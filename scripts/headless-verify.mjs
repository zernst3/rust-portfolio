#!/usr/bin/env node
// Headless-browser verification for the rust-portfolio Dioxus fullstack app.
//
// WHY THIS EXISTS (ORCH-VERIFY-RUNTIME-1): curl can prove assets return 200 but
// it never executes the WASM, so it cannot tell you whether hydration actually
// works or whether an interaction (hover highlight, subtitle rotation) fires.
// Every white-screen / frozen-tab regression in this app's history slipped past
// curl. This harness drives a REAL headless Chrome over the DevTools Protocol
// and asserts live interactivity, not just "the page loaded."
//
// ZERO npm installs: uses only Node built-ins (global fetch + global WebSocket,
// stable in Node 24+) and the Google Chrome already installed on this machine.
//
// Usage:  node scripts/headless-verify.mjs [http://127.0.0.1:8080]
// Exit 0 = all required checks passed. Exit 1 = at least one required check
// failed (and a JSON report is printed to stdout either way).

import { spawn } from "node:child_process";
import { mkdtempSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const BASE = (process.argv[2] || "http://127.0.0.1:8080").replace(/\/$/, "");
const CDP_PORT = 9333;
const CHROME =
  "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome";

const results = [];
function record(name, required, pass, detail) {
  results.push({ name, required, pass: !!pass, detail: detail ?? "" });
  const tag = pass ? "PASS" : required ? "FAIL" : "warn";
  console.error(`[${tag}] ${name}${detail ? " — " + detail : ""}`);
}
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

// ── 1. Wait for the dev server to actually answer ───────────────────────────
async function waitForServer(timeoutMs = 180000) {
  const start = Date.now();
  while (Date.now() - start < timeoutMs) {
    try {
      const r = await fetch(BASE, { redirect: "manual" });
      if (r.status > 0) return true;
    } catch {
      /* server not up yet */
    }
    await sleep(1500);
  }
  return false;
}

// ── 2. Minimal CDP client over the page target's WebSocket ──────────────────
class CDP {
  constructor(ws) {
    this.ws = ws;
    this.id = 0;
    this.pending = new Map();
    this.exceptions = [];
    this.consoleErrors = [];
    ws.addEventListener("message", (ev) => {
      const msg = JSON.parse(ev.data);
      if (msg.id && this.pending.has(msg.id)) {
        const { resolve, reject } = this.pending.get(msg.id);
        this.pending.delete(msg.id);
        msg.error ? reject(new Error(JSON.stringify(msg.error))) : resolve(msg.result);
      } else if (msg.method === "Runtime.exceptionThrown") {
        this.exceptions.push(
          msg.params?.exceptionDetails?.exception?.description ||
            msg.params?.exceptionDetails?.text ||
            "unknown exception"
        );
      } else if (
        msg.method === "Runtime.consoleAPICalled" &&
        msg.params?.type === "error"
      ) {
        this.consoleErrors.push(
          (msg.params.args || []).map((a) => a.value ?? a.description).join(" ")
        );
      }
    });
  }
  send(method, params = {}) {
    const id = ++this.id;
    return new Promise((resolve, reject) => {
      this.pending.set(id, { resolve, reject });
      this.ws.send(JSON.stringify({ id, method, params }));
      setTimeout(() => {
        if (this.pending.has(id)) {
          this.pending.delete(id);
          reject(new Error(`CDP timeout: ${method}`));
        }
      }, 30000);
    });
  }
  // Evaluate JS in the page and return the value (by value, awaiting promises).
  async eval(expression) {
    const r = await this.send("Runtime.evaluate", {
      expression,
      returnByValue: true,
      awaitPromise: true,
    });
    if (r.exceptionDetails) {
      throw new Error(
        r.exceptionDetails.exception?.description || r.exceptionDetails.text
      );
    }
    return r.result?.value;
  }
  async mouseMove(x, y) {
    await this.send("Input.dispatchMouseEvent", {
      type: "mouseMoved",
      x,
      y,
      buttons: 0,
    });
  }
}

async function connectCDP(userDataDir) {
  const child = spawn(
    CHROME,
    [
      "--headless=new",
      `--remote-debugging-port=${CDP_PORT}`,
      "--disable-gpu",
      "--no-first-run",
      "--no-default-browser-check",
      "--force-device-scale-factor=1",
      "--window-size=1440,900",
      `--user-data-dir=${userDataDir}`,
      BASE,
    ],
    { stdio: "ignore" }
  );

  // Find the page target's WebSocket debugger URL.
  let wsUrl = null;
  for (let i = 0; i < 40 && !wsUrl; i++) {
    await sleep(500);
    try {
      const list = await (await fetch(`http://127.0.0.1:${CDP_PORT}/json`)).json();
      const page = list.find((t) => t.type === "page" && t.webSocketDebuggerUrl);
      if (page) wsUrl = page.webSocketDebuggerUrl;
    } catch {
      /* devtools endpoint not ready */
    }
  }
  if (!wsUrl) {
    child.kill("SIGKILL");
    throw new Error("Could not reach Chrome DevTools endpoint");
  }

  const ws = new WebSocket(wsUrl);
  await new Promise((res, rej) => {
    ws.addEventListener("open", res, { once: true });
    ws.addEventListener("error", rej, { once: true });
  });
  const cdp = new CDP(ws);
  await cdp.send("Runtime.enable");
  await cdp.send("Page.enable");
  return { cdp, child, ws };
}

// ── 3. The checks ───────────────────────────────────────────────────────────
async function run() {
  if (!(await waitForServer())) {
    record("server-reachable", true, false, `no response from ${BASE}`);
    return;
  }
  record("server-reachable", true, true, BASE);

  const userDataDir = mkdtempSync(join(tmpdir(), "rp-verify-"));
  let session;
  try {
    session = await connectCDP(userDataDir);
  } catch (e) {
    record("chrome-launch", true, false, e.message);
    rmSync(userDataDir, { recursive: true, force: true });
    return;
  }
  const { cdp, child, ws } = session;

  try {
    // Navigate fresh (cache-busted) so a stale build can't masquerade as a pass.
    await cdp.send("Page.navigate", { url: `${BASE}/?cb=${Date.now()}` });
    await sleep(2500);

    // CHECK: SSR shell present.
    const homePresent = await cdp.eval(
      `!!document.querySelector('#Home')`
    );
    record("ssr-shell", true, homePresent, "#Home in DOM");

    // CHECK: nav items rendered (5 page links).
    const navCount = await cdp.eval(
      `document.querySelectorAll('#Home .pageLinks .navbarItem').length`
    );
    record("nav-items", true, navCount === 5, `count=${navCount}`);

    // CHECK (HYDRATION — the load-bearing one): the subtitle rotation is driven
    // by a Rust async task that only runs after WASM hydration. If the text
    // changes on its own, the WASM is alive. This is the single most important
    // assertion: it is impossible to pass without working hydration.
    const t0 = await cdp.eval(`document.querySelector('#Home h3')?.textContent`);
    // 3s interval + 0.4s fade → changes within ~3.5s; poll up to 6s.
    let changed = false;
    let t1 = t0;
    for (let i = 0; i < 12 && !changed; i++) {
      await sleep(600);
      t1 = await cdp.eval(`document.querySelector('#Home h3')?.textContent`);
      if (t1 && t1 !== t0) changed = true;
    }
    record(
      "hydration-subtitle-rotates",
      true,
      changed,
      changed ? `"${(t0 || "").slice(0, 24)}…" -> "${(t1 || "").slice(0, 24)}…"` : `stuck on "${t0}"`
    );

    // CHECK: subtitle has an opacity transition (fade in/out, Bug 3).
    const hasFade = await cdp.eval(
      `(()=>{const h=document.querySelector('#Home h3');if(!h)return false;` +
        `const s=getComputedStyle(h);return s.transitionProperty.includes('opacity')||s.transitionProperty==='all';})()`
    );
    record("subtitle-fade-style", false, hasFade, "h3 transitions opacity");

    // CHECK: nav-highlight bar appears when hovering a PAGE-LINK item (Bug 1).
    // The previous failure mode: the bar only fired for the Links item's CSS
    // hover, never for Manifesto/AI Architecture/Work/Writing/Contact Me.
    const workRect = await cdp.eval(
      `(()=>{const it=document.querySelectorAll('#Home .pageLinks .navbarItem')[2];` +
        `if(!it)return null;const r=it.getBoundingClientRect();` +
        `return {x:r.left+r.width/2,y:r.top+r.height/2,top:it.offsetTop,h:it.offsetHeight};})()`
    );
    if (!workRect) {
      record("hover-highlight-work", true, false, "Work nav item not found");
    } else {
      await cdp.mouseMove(workRect.x, workRect.y);
      await sleep(450);
      const hl = await cdp.eval(
        `(()=>{const b=document.querySelector('#Home .pageLinks .nav-highlight');` +
          `if(!b)return null;const s=getComputedStyle(b);` +
          `return {opacity:parseFloat(s.opacity),top:parseFloat(s.top)||b.offsetTop};})()`
      );
      const visible = hl && hl.opacity > 0.4;
      const aligned = hl && Math.abs(hl.top - workRect.top) < 8;
      record(
        "hover-highlight-work",
        true,
        visible,
        hl ? `opacity=${hl.opacity}` : "no .nav-highlight element"
      );
      record(
        "hover-highlight-aligned",
        false,
        aligned,
        hl ? `bar.top=${hl.top} item.top=${workRect.top}` : "n/a"
      );
    }

    // CHECK: Links item highlight does NOT overflow the menu width (Bug 2).
    const overflow = await cdp.eval(
      `(()=>{const li=document.querySelector('#Home .links li');` +
        `const mc=document.querySelector('#Home .menuContainer');` +
        `if(!li||!mc)return null;` +
        `return {liRight:li.getBoundingClientRect().right,mcRight:mc.getBoundingClientRect().right};})()`
    );
    record(
      "links-no-overflow",
      true,
      overflow ? overflow.liRight <= overflow.mcRight + 1.5 : false,
      overflow ? `li.right=${overflow.liRight.toFixed(1)} menu.right=${overflow.mcRight.toFixed(1)}` : "elements not found"
    );

    // CHECK: a secondary route hydrates too (page-enter animation present).
    await cdp.send("Page.navigate", { url: `${BASE}/work?cb=${Date.now()}` });
    await sleep(2500);
    const workPage = await cdp.eval(
      `(()=>{const r=document.querySelector('.page-enter');return !!r;})()`
    );
    record("work-page-page-enter", false, workPage, ".page-enter root present");

    // CHECK: no uncaught exceptions / console errors anywhere above.
    record(
      "no-js-exceptions",
      true,
      cdp.exceptions.length === 0,
      cdp.exceptions.slice(0, 3).join(" | ") || "none"
    );
    record(
      "no-console-errors",
      false,
      cdp.consoleErrors.length === 0,
      cdp.consoleErrors.slice(0, 3).join(" | ") || "none"
    );
  } catch (e) {
    record("harness", true, false, e.message);
  } finally {
    try { ws.close(); } catch {}
    child.kill("SIGKILL");
    rmSync(userDataDir, { recursive: true, force: true });
  }
}

await run();

const requiredFails = results.filter((r) => r.required && !r.pass);
console.log(
  JSON.stringify(
    {
      base: BASE,
      passed: results.filter((r) => r.pass).length,
      total: results.length,
      requiredFailures: requiredFails.map((r) => r.name),
      results,
    },
    null,
    2
  )
);
process.exit(requiredFails.length === 0 ? 0 : 1);
