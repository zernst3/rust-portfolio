# AGENTS.md — rust-portfolio

Brief for AI agents working in this repo. Read this in full at the start of every session.

## What this repo is

A full-stack Rust port of Zach Ernst's portfolio site (zachary-ernst.dev). The current site lives at `~/Documents/Repos/MyPortfolioSite/` (Vite + React + TypeScript + three.js + framer-motion). The contact-form backend lives at `~/Documents/Repos/MyPortfolioSiteFunctions/` (Python Lambda + Mailgun + GeoIP).

This port collapses both into a **single Rust binary** built on **Axum + Dioxus Fullstack + Bevy**. The new repo is `~/Documents/Repos/rust-portfolio/`.

The current site's behavior is the spec. The bot's job is to preserve user-visible behavior and aesthetic exactly while moving the implementation to Rust.

## Locked decisions

Read `docs/decisions/2026-06-05-portfolio-port-direction.md` in full at the start of every session. Twelve decisions are locked there. Any choice the locked-decisions doc covers is settled; do not re-litigate.

If you encounter a question the doc doesn't cover, apply `docs/CONVENTIONS.md` (the local rule library) and the camerata universal principles before deciding anything yourself.

## Source repos (read-only inputs to the port)

- `~/Documents/Repos/MyPortfolioSite/` — React source. Bot reads this to translate components to Dioxus RSX. **Bot MUST NOT modify this repo.**
- `~/Documents/Repos/MyPortfolioSiteFunctions/` — Python Lambda. Bot reads this for the contact-form + pageview email shapes. **Bot MUST NOT modify this repo.**
- `~/Documents/Repos/camerata-ai/principles/` — universal rule library. Bot reads this for citation IDs (RUST-DOMAIN-*, RUST-DIOXUS-*, ORCH-*, ARCH-*, etc.).
- `~/Documents/Repos/rust-chorale/docs/CONVENTIONS.md` — chorale's rule pattern. Reference for shape, not for content (chorale rules are library-specific).

## Working principles

### Clear-winner test (per `orch-clear-winner-1`)

1. **Documented convention exists?** Apply it, cite the ID in the commit, continue.
2. **`PORT-ROBUST-1` (robustness > terseness) breaks the tie?** Apply the more-robust option, document it as a new rule in `docs/CONVENTIONS.md` listing alternatives + why-each-was-rejected, cite the new ID in the commit, append to `.overnight-portfolio-auto-calls-ledger.md`, continue.
3. **Neither resolves it?** Log to `.overnight-portfolio-decisions_needed.md`, set the pause flag at `~/Library/Logs/claude/overnight-portfolio-paused.flag`, exit.

### One-way-door gate (per `orch-one-way-door-1` and `PORT-ROUTE-1`)

Even with a clear winner, ROUTE to Zach when the choice:

- Adds, splits, or removes a crate from the workspace.
- Adds an external dependency.
- Changes a public route shape (path, method, request/response DTO).
- Changes the env var contract.
- Touches anything in `docs/decisions/` (those are Zach-edited only).

### Hard guards (always abort, never auto)

- **Cross-repo edits.** You may ONLY modify files inside `~/Documents/Repos/rust-portfolio/`. Touching anything else triggers a hard pause.
- **Pushing.** This repo has no remote yet. Do NOT run `git remote add`. Do NOT run `git push`. Local commits only until Zach configures the remote and explicitly enables push.
- **License / Cargo.toml top-level metadata** (author, repository, license fields). ROUTE.
- **`cargo publish`** — never. This is an app, not a library.

### Operating model

The bot reads `.overnight-portfolio-work-queue.md` to find the active phase and current item. It works through items in order. Each item produces one focused commit (or a small series of related commits) that compiles clean, passes clippy clean, and passes tests.

After each item, the bot re-reads the queue, picks the next, and continues until: (a) the per-session budget is exhausted, (b) it routes a decision to Zach for a hard block, or (c) the entire v0.1 work queue is empty.

**Phase advancement is the bot's job (updated 2026-06-05).** When all items in the current `## ACTIVE PHASE` are done, the bot edits the `## ACTIVE PHASE: <name>` line to the next `## Phase v0.1-*` heading in the file and continues working without pausing. Zach explicitly authorized this — phase completions are soft transitions, not routing events. The only "completion" that stops the session is finishing the entire v0.1 work queue (every phase done), at which point the bot sets the pause flag with a `PORT-V0.1-COMPLETE` entry.

### Code quality gates

Every commit MUST pass:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --workspace -- -D warnings
cargo check --workspace
cargo test --workspace
```

Workspace lints are configured in the root `Cargo.toml`. `unsafe_code` is forbidden; `unwrap_used`, `expect_used`, `panic!`, `todo!`, `unimplemented!` are warnings (treat as errors for bot commits).

### Commit shape

Commits cite rule IDs in the body per `PORT-CC-1`. Subject is conventional-commits style (`feat(scope): ...`, `fix(scope): ...`, `wip(port): ...`). Long bodies are encouraged when the call is non-obvious; short bodies are fine when the change is purely mechanical and covered by an established rule.

Prefix all overnight-routine commits with `wip(port):` so they're visually distinct from any manual commits Zach makes.

## Camerata / agora-rs / chorale references

| Origin | What it carries |
|---|---|
| **camerata `agentic/orch-*`** | Orchestration policy (clear-winner, one-way-door, novelty, training-cutoff, tiered-escalation, autocalls ledger, env-gated quality, etc.). Apply all. |
| **camerata `rust-domain-*`** | Newtype IDs, validated strings, thiserror per crate, async all the way down. Apply DOMAIN-2, -3, -4, -5. DOMAIN-1 (single domain crate) is N/A — no domain crate here. DOMAIN-6/-7 (UoW) are N/A — no DB. |
| **camerata `rust-dioxus-*`** | All 14 Dioxus rules apply (file structure, functional components, Signals, context providers, effects, async resources, event handlers, RSX patterns, server functions, auth `_can`, fullstack SSR, SVG inline, forms newtype, primitives first). |
| **camerata `api-layer/arch-*`** | Apply arch-api-dtos-1, arch-boundary-validation-1, arch-structured-errors-1, arch-utc-timestamps-1, arch-middleware-first-1. Skip the repo / aggregate / cursor-pagination rules (no DB). |
| **camerata `ci-cd/*`** | Applies once deploy lands. Deferred until Zach picks a host. |
| **camerata `universal/*`** | All apply: cite-convention-id, regression-test, document-decisions, file-size, optimize, robustness. |
| **agora-rs `MONOLITH-1`** | Applies verbatim. See `PORT-MONOLITH-1`. |
| **agora-rs `QUEUE-1`** | Applies. No external brokers; no `mpsc` needed either (no async pipelines). |
| **agora-rs `WORKERS-1`, `EMAIL-1`** | N/A (no scheduled jobs, no transactional-retry need; see `PORT-EMAIL-1`). |
| **chorale `CC-1`, `ROBUSTNESS-1`** | Re-codified as `PORT-CC-1`, `PORT-ROBUST-1` here for self-containment. |

When in doubt: prefer to cite an existing camerata rule rather than invent a new local rule. Local `PORT-*` rules are for portfolio-specific shapes only.

## Communication

Zach reads commit messages, the auto-calls ledger, and any entries in `.overnight-portfolio-decisions_needed.md`. The bot's job is to make those artifacts complete and easy to scan — if a decision needs Zach's input, the entry includes: what the choice is, what options were considered, what the bot would pick under the clear-winner test, why it's routing rather than applying.

No Slack DM is wired for this routine (yet). Inbox + morning-consolidator integration may come later if Zach wants regular cadence.
