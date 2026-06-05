# rust-portfolio Conventions

Canonical rule library for the rust-portfolio repo. Scoped to this repo; the universal rules referenced are sourced from camerata-ai principles and the agora-rs CONVENTIONS.md.

Every rule has a stable ID. Every commit that applies a convention MUST cite the rule ID in the commit body.

---

## PORT-CC-1: Cite rule IDs in commits

Every commit that applies a convention from this file (or from camerata / agora-rs) MUST cite the rule ID in the commit body. Format:

```
feat(server): wire /api/contact handler to Mailgun

Applied PORT-API-1 (two distinct routes per operation), PORT-ROBUST-1
(named struct fields over tuples), and RUST-DOMAIN-4 (thiserror enum
in the handler crate). Chose `reqwest::Client::new()` per call over a
shared client because the handler runs once per submission and connection
reuse savings don't justify the shared-state plumbing.
```

If applying a camerata principle, cite the camerata ID (e.g. `RUST-DIOXUS-3`, `RUST-DOMAIN-2`). If applying an agora-rs convention, cite the agora-rs ID (e.g. `MONOLITH-1`). Otherwise cite the local `PORT-*` ID.

---

## PORT-ROBUST-1: Explicit > terse

Default toward explicit, robust code over clever-terse code. Examples:

- Named struct fields over multi-element tuples past two fields.
- Distinct error variants per failure mode in `thiserror` enums, not a catch-all `Other(String)`.
- `EmailAddress` newtype over bare `String` for the contact-form `email` field.
- Explicit `tracing::info_span!` around each handler instead of bare `println!`.

**Why:** matches chorale's ROBUSTNESS-1 and the [[robustness_over_terseness]] feedback. AI agents bear the boilerplate cost; debugging cost on cryptic code is high.

---

## PORT-MONOLITH-1: Single binary, single process, single port

The entire stack ships as one Rust binary. Axum's root `Router` mounts: the Dioxus Fullstack SSR handler (HTML routes), the two `/api/*` routes, and a `tower_http::services::ServeDir` for `assets/` (static files). One process owns SSR rendering, route handling, static serving, and the Bevy WebGL canvas's WASM bundle. No microservices, no separate API service, no separate static-site host, no separate Lambda for the contact form.

**Why:** agora-rs MONOLITH-1 applies verbatim. No new infrastructure cost; one deploy artifact; no cross-service auth or networking.

**How to apply:** new functionality goes in this binary. If a future feature genuinely needs different scaling characteristics, route to Zach per `orch-one-way-door-1` (it's a structural change).

---

## PORT-NO-DB-1: No database

This repo does NOT include SeaORM, Postgres, SQLite, or any persistence layer. The portfolio is content-driven and read-only at the data tier. The two `/api/*` routes are stateless: they take a request, fire one or two emails via Mailgun, return a response. No outbox, no retry table, no audit log.

**Why:** zero state to persist. Adding a DB would mean adding hosting cost, migration ceremony, and the entire hexagonal domain/repository layer for nothing.

**How to apply:** if a future feature seems to need persistence (analytics, form-submission log, etc.), route to Zach. Almost any such feature should go in a separate service instead of growing this one.

---

## PORT-API-1: Two distinct routes per operation, no action discriminator

The current `MyPortfolioSiteFunctions` Lambda routes two operations through a single endpoint by sniffing a `payload.action` field. This repo rejects that pattern. Each operation is a separate Axum route:

- `POST /api/contact` — visitor submits `{name, email, subject, message}`. Handler validates, sends visitor thank-you via Mailgun template `thank_you_email`, sends Zach notification via template `email_to_me`, returns `204 No Content` on success.
- `POST /api/pageview` — visitor's browser pings. Handler ignores any request body. Sends single email to Zach: `"Someone visited your site."`. Returns `204 No Content` on success.

**Why:** the two operations have different validation requirements, different failure modes, different rate-limit shapes, and different observability needs. Sharing a handler entangles them. Two routes makes each independently testable and independently rate-limitable.

**How to apply:** if a third operation is ever added (newsletter signup, etc.), it gets its own `POST /api/<name>` route. Never a generic `/api/notify` or `/api/action`.

---

## PORT-API-2: Pageview captures zero metadata

`POST /api/pageview` MUST NOT capture, log, send to Mailgun, or otherwise record: client IP, GeoIP location, user-agent, referrer, path, timestamp, or any other request metadata. The email body is the fixed string `"Someone visited your site."`. Nothing more.

**Why:** Zach's explicit ask 2026-06-05 — "remove the IP tracking entirely, just hit me with a 'someone visited your site' email, that is enough." Privacy-by-architecture: code that doesn't collect the data can't leak it.

**How to apply:** the handler signature is `async fn pageview() -> StatusCode`. No `axum::extract::ConnectInfo`, no `headers: HeaderMap`. Mailgun call body is a constant.

---

## PORT-BEVY-1: Bevy renders the background only

Bevy's role in this repo is rendering the fullscreen 3D background canvas behind the Dioxus DOM. Bevy does NOT render UI widgets, does NOT receive click events for application logic, does NOT mount Dioxus components inside its UI layer. The canvas sits at `z-index: -1`; Dioxus owns every interactive element above it.

**Why:** the background is purely decorative; there is no design need for the 3D layer to interact with the DOM. Bevy-owns-everything would add weeks of integration work for no portfolio benefit.

**How to apply:** the Bevy app lives in its own workspace crate at `portfolio_scene/` (lib type `cdylib`, target `wasm32-unknown-unknown`). The `#[wasm_bindgen(start)]` entry constructs the `App` and runs it; JS in the Dioxus app loads the resulting `.wasm` on `DOMContentLoaded` and Bevy mounts to the canvas at `#bevy-canvas`. The `server` and `ui` crates NEVER `use bevy::*`. Name history: the original PORT-BEVY-1 text referenced `server/src/bevy_scene.rs` (early-draft path); corrected 2026-06-05 to `bevy_scene/` after PORT-DECISION-bevy-crate-location; renamed 2026-06-05 to `portfolio_scene/` after a Cargo name collision with the Bevy ecosystem's own `bevy_scene` crate broke `cargo build -p bevy_scene`.

---

## PORT-SCENE-1: Faithful three.js port first, alternates second

The first Bevy scene the bot delivers is a faithful translation of the current React Three Fiber scene in `~/Documents/Repos/MyPortfolioSite/src/`. Geometry, materials, postprocessing chain, and animation loop are translated to Bevy primitives that match as closely as Bevy allows. This becomes the "safe baseline" that ships with v0.1.

After the safe baseline is in `main`, the bot drafts 2–3 alternate scene options (each a separate branch or a separate `portfolio_scene/src/alt_<n>.rs` module behind a feature flag) for Zach to compare. The chosen alternate replaces the safe baseline; if no alternate wins, the safe baseline ships unchanged.

**Why:** Zach's explicit ask 2026-06-05 — "A combo of A and B. I'd like to see some options, but create a port of the original as a backup."

**How to apply:** bot completes the faithful port as a separate work-queue item before any alternate is started. Alternates do NOT block v0.1 ship.

---

## PORT-CSS-1: Vanilla CSS files port verbatim

The 30+ `.css` files in `~/Documents/Repos/MyPortfolioSite/src/components/**/*.css` copy as-is into `rust-portfolio/assets/styles/` (preserving subdirectory structure for organization). Dioxus references them via the `dx::stylesheet!` macro or via `<link rel="stylesheet" href="/assets/styles/<path>">` in the document head.

NO rewrite to Tailwind. NO conversion to CSS-in-RSX. NO bundler-aware imports — the CSS is copied raw and served as static files by the same Axum binary.

**Why:** the current vanilla CSS is the proven aesthetic. Rewriting it during the port would introduce visual drift for zero engineering benefit. The bot's job is to translate the component tree, not to redesign the look.

**How to apply:** the bot copies each `.css` file when porting its corresponding `.tsx` component. Class names in the ported RSX MUST match the React originals byte-for-byte so the CSS rules apply.

---

## PORT-DIAGRAMS-1: One diagram per session, visual-diff verified

The five React diagram components — `AgentWorkflowDiagram`, `CICDPipelineDiagram`, `InfrastructureDiagram`, `LayeredArchitectureDiagram`, `RealTimeSyncDiagram` (812 LoC total) — port one per bot session. Each port produces a Dioxus component emitting the same SVG output the React component does. Animations are driven by CSS classes (same `.css` files copied per PORT-CSS-1).

After each port the bot does a side-by-side visual diff: render the React original at a known viewport, render the Dioxus port at the same viewport, save both PNGs, and write a short verification note in the commit body. If the diff reveals drift the bot can't trivially explain, ROUTE to Zach.

**Why:** the diagrams are the page's strongest signals (the Layered Architecture diagram in "Built End to End" especially). One per session preserves bandwidth for verification.

**How to apply:** treat each diagram as a single work-queue item. Do not batch two diagrams in one commit.

---

## PORT-EMAIL-1: Mailgun via `reqwest`, no retry layer

Both `/api/contact` and `/api/pageview` send their outbound emails by issuing one `reqwest::Client::post()` call per email to `https://api.mailgun.net/v3/<domain>/messages` with Basic auth. If Mailgun returns 5xx or the call times out, the handler returns `503 Service Unavailable` and the visitor sees an error on their side. NO durable retry table, NO outbox pattern, NO async retry task.

**Why:** agora-rs EMAIL-1 prescribes the `pending_emails` outbox pattern for transactional emails that MUST eventually deliver. Portfolio contact-form emails are not in that category — if Mailgun is down for 30 seconds, the visitor can re-submit. The complexity cost of an outbox is not warranted at this scale.

**How to apply:** Mailgun API key + domain come from env vars (`MAILGUN_API_KEY`, `MAILGUN_DOMAIN`). Handler builds the form payload, fires the POST, maps non-2xx to a `503` response. Done.

---

## PORT-AUTO-CALLS-1: Self-made decisions are documented + ledger-logged

When the bot encounters an architectural choice not covered by an existing rule and the clear-winner test (camerata `orch-clear-winner-1`) resolves it, the bot:

1. Applies the winner.
2. Writes a new `PORT-*` rule to this file documenting the call, with `## Alternatives considered` listing every alternative weighed and the specific reason the winner beat each.
3. Cites the new rule ID in the commit.
4. Appends a one-line entry to `.overnight-portfolio-auto-calls-ledger.md` (gitignored — local-only) for the weekly review pass.

**Why:** mirrors chorale's auto-calls ledger pattern. Per-decision gates miss aggregate drift; the ledger surfaces accumulated calls for Zach to review periodically.

**How to apply:** the morning DM (when one is added) surfaces the count + headlines of new auto-calls since the last review.

---

## PORT-ROUTE-1: Structural / topology / public-API changes route to Zach

Even with a clear winner under the test, the bot ROUTES to Zach (writes to `.overnight-portfolio-decisions_needed.md`, sets pause flag, exits) when the choice:

- Adds, splits, or removes a crate from the workspace.
- Adds an external dependency to `[workspace.dependencies]` or a crate's `[dependencies]`.
- Changes the shape of a `/api/*` route (path, method, request/response DTO).
- Changes the env var contract (adds a required env var the deploy story has to know about).
- Touches anything in `docs/decisions/` (those are append-only on Zach's call).

**Why:** these are one-way doors. Even if the call seems clear, the cost of getting it wrong is much higher than the cost of waiting one session for Zach to confirm.

---

## Adding new rules

Append below this line. New rule IDs follow the pattern `PORT-<TOPIC>-<NUMBER>` (e.g. `PORT-API-3`, `PORT-BEVY-2`, `PORT-ROBUST-2`). Every new rule includes: summary, why, how to apply, alternatives considered.

---

## PORT-AUDIO-1: Use `document::eval` for audio playback, not `web-sys`

Audio playback in Dioxus WASM uses `document::eval(js_script)` to fire a JS `new Audio(...).play()` call rather than the `web-sys::HtmlAudioElement` API that decision #9 originally named.

**Why:** Adding `web-sys` to `[dependencies]` is a PORT-ROUTE-1 one-way door (dep additions always route to Zach). `dioxus` already pulls `web-sys` as a transitive dep, but the bot must not rely on undeclared transitives. Using `document::eval` (available from the existing `dioxus` dep via `dioxus::prelude::document`) achieves the same behavioral goal with zero new dependencies.

**How to apply:** Fire-and-forget audio:
```rust
pub fn play_sound(path: &str, volume: f64) {
    let _ = document::eval(&format!(
        "(()=>{{var a=new Audio('{path}');a.volume={volume};a.play().catch(()=>{{}});}})()"
    ));
}
```
`eval(script)` dispatches the JS immediately via `Function::new_with_args(...).call1(...)` in the `WebEvaluator`. Dropping the `Eval` result is safe for fire-and-forget. On the server (SSR) the `NoOpEvaluator` discards the call silently.

**Alternatives considered:**
- `web-sys::HtmlAudioElement` — correct per locked decision #9, but requires adding `web-sys` as an explicit dep with feature flags. Blocked by PORT-ROUTE-1. Preferred if Zach ever opens the dep gate.
- `wasm-bindgen::JsValue` eval — same end state as `document::eval` but requires `wasm-bindgen` dep. Blocked by PORT-ROUTE-1 for same reason.
- `#[cfg(target_arch = "wasm32")]` no-op on server — valid but the `NoOpEvaluator` already handles the server side, so the cfg guard adds noise without benefit.

---

## PORT-ANIM-1: CSS keyframe animations replace framer-motion; lives in `transitions.css`

All framer-motion page transitions and entrance animations port to CSS `@keyframes` defined in `assets/styles/transitions.css`. The App component loads this file via `document::Stylesheet` alongside the verbatim-ported CSS files.

**Why:** Decision #4 mandates vanilla CSS transitions. The framer-motion variants (`pageVariants`, `pageTransition`) passed as props in the React app become CSS class toggles (`page-enter`, `name-fade-in`, `header-fade-in`, etc.). Dioxus re-keying (changing a node's `key` prop) triggers remount + animation re-fire, replacing `AnimatePresence`.

**How to apply:** Wrap page root elements with `class: "page-enter"`. For elements with delayed entrance, use `fade-in-delayed` / `fade-in-delayed-long`. For rotating headers, use `header-fade-in` on a `key`-rebound element. Do NOT add these animation classes to the verbatim-ported CSS files from the React source.

**Alternatives considered:**
- `dioxus-motion` crate — rejected per decision #4 (less mature, some animations need CSS fallback anyway).
- Inline JS `requestAnimationFrame` helpers — valid for imperative animation (e.g. audio-cue timing per decision #4), but overkill for simple entrance fades.
