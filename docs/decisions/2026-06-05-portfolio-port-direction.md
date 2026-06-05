---
date: 2026-06-05
status: locked
---

# Portfolio site port direction (2026-06-05)

The foundational decision lock for porting `MyPortfolioSite` (Vite + React + three.js + framer-motion) and `MyPortfolioSiteFunctions` (Python Lambda) into a single full-stack Rust monolith at `rust-portfolio`.

## What was decided

Twelve decisions, locked in the 2026-06-05 session before any code was written.

### 1. Database scope: NONE

Pure SPA + small contact-form backend. No SeaORM, no Postgres, no SQL migrations, no hexagonal domain/repository layer. The portfolio is content-driven and read-only at the data tier; the only mutable state is two outbound emails per contact form submission.

Cuts ~70% of the agora-rs crate surface that would otherwise carry over.

### 2. Render model: Dioxus Fullstack (SSR + hydration)

First paint is server-rendered HTML; client-side WASM hydrates. Better SEO, social cards out of the box, faster first contentful paint than the current Vite + React CSR build.

### 3. 3D background: Bevy fullscreen canvas behind Dioxus DOM

A single `<canvas>` element fills the viewport at `z-index: -1`. Bevy owns the WebGL context and renders the 3D scene. Dioxus owns the rest of the page (text, navigation, pickers) as normal DOM. Background is purely decorative, no interop needed between Bevy and Dioxus.

**Scene fidelity:** the bot first produces a *faithful port* of the current three.js + react-three-fiber scene as the "safe baseline" (geometry, materials, postprocessing, animation loop translated to Bevy primitives). After the safe baseline ships, the bot drafts 2–3 alternate scene options for Zach to compare. The safe baseline is the fallback if no alternate wins.

### 4. Animation library: vanilla CSS transitions + signal helpers

No `dioxus-motion`, no framer-motion equivalent. Most current animations (sliding picker highlight, page transitions, hover effects, Halo-style affordances) are CSS-shape today; Dioxus emits the same class toggles React does and CSS transitions Just Work. For the few cases that need imperative animation (e.g., audio-cue timing on hover), write small 20-line helpers on top of `use_effect` + `request_animation_frame`.

### 5. CSS approach: port vanilla files verbatim

Move the existing `.css` files into the rust-portfolio `assets/` folder and reference them via Dioxus's stylesheet macro. Zero visual drift; the Halo-style aesthetic is preserved exactly. No Tailwind rewrite.

### 6. Backend: two distinct Axum routes, not one with a discriminator

- `POST /api/contact` — visitor submits `{name, email, subject, message}`. Server sends thank-you email to visitor via Mailgun template `thank_you_email`, then sends notification email to Zach via Mailgun template `email_to_me`. Two outbound emails per submission.
- `POST /api/pageview` — visitor's browser pings (no body, or empty body). Server sends single notification email to Zach: "Someone visited your site." **No IP capture. No GeoIP lookup. No user-agent. No path. No referrer. No metadata at all.**

The `MyPortfolioSiteFunctions` Lambda's `action`-discriminated single-handler shape is rejected. Each operation is a separate Axum route with its own handler, its own validation DTO, its own integration tests.

### 7. Diagram porting: mechanical Dioxus RSX port, one per session

The five React diagram components in `~/Documents/Repos/MyPortfolioSite/src/components/ProfessionalExperience/*Diagram/` (812 LoC total: AgentWorkflowDiagram, CICDPipelineDiagram, InfrastructureDiagram, LayeredArchitectureDiagram, RealTimeSyncDiagram) port one per bot session. Same SVG output, same animations driven by CSS classes. Bot does visual-diff verification before marking each one done.

### 8. Workspace structure: two-crate Cargo workspace

`server` (Axum routes + Bevy scene + static asset serving + Dioxus Fullstack server entry) and `ui` (Dioxus components + Fullstack hydration entry + diagram components). Shared DTOs live in `server/src/dto.rs`; no separate `domain` crate (no DB, no domain layer needed). Easy to add a `domain` crate later if scope grows.

The full hexagonal six-crate setup from agora-rs is rejected as overkill for a portfolio site that has no domain model.

### 9. Routing: Dioxus Router. State: Dioxus Signals. Audio: `web-sys` Audio.

Defaults applied without ceremony — the React choices map 1:1 to Dioxus equivalents. Zustand's single global `isMuted` signal becomes one Dioxus context-provided Signal. react-router-dom v6 becomes Dioxus Router. `use-sound` becomes a thin `web-sys` Audio wrapper.

### 10. Image assets: copy verbatim

The 9 PNGs and 3 SVGs in `MyPortfolioSite/public/` (Camerata screenshots, Chorale screenshots, portrait, QR code, finance/city icons) copy as-is into `rust-portfolio/assets/` and serve under the same root paths. Subdirectories (`ShootAR_images/`, `crm_images/`, `ecommerce_images/`, `translation_chat_app/`) likewise.

### 11. Deploy target: DEFERRED

Azure infra is reserved for agora.new and explicitly NOT available for personal projects. Zach is researching Digital Ocean. The bot's work is local-only until a deploy target lands. No GitHub Actions workflows, no Dockerfile until then.

### 12. Routine cadence: fire-once, no launchd plist

One-shot manual fire via `~/.claude/scripts/portfolio-port-direct-fire.sh`. Runs until the per-session $20 budget cap or until it routes to Zach for a decision. Zach re-fires when ready for another session. No nightly schedule. Reflects the explicit "not a high-priority item time-wise" framing.

## Why this shape

The decisions above derive from three inputs:

1. **Agora's locked decisions** — every architectural decision Zach made for the agora-rs Rust port applies here unless explicitly overridden. MONOLITH-1, WORKERS-1 (in-process scheduled tasks — not used here since there are no scheduled jobs), EMAIL-1 (fire-and-forget email, durable retry — not needed here since there are no transactional emails to retry), QUEUE-1 (no external brokers — applies), the seven RUST-DOMAIN-* rules, the fourteen RUST-DIOXUS-* rules, the seven async-efficiency rules.

2. **Chorale's locked decisions** — the routine architecture pattern, the clear-winner + one-way-door + novelty gates, the auto-calls ledger, the recon-gated phase advancement, the cite-rule-IDs-in-commits discipline (CC-1 → PORT-CC-1).

3. **Portfolio-specific net-new decisions** — the twelve listed above, locked in the 2026-06-05 session.

## Alternatives considered and rejected

- **Keep three.js by interoperating from Dioxus** — rejected on "full-stack Rust" goal. Zach explicitly chose option 2 (Bevy) over option 1 (JS interop) when offered the tradeoff.
- **`dioxus-motion` for animations** — rejected. Less mature than framer-motion; small community; some animations port to it 1:1 but others require CSS fallback anyway. Hand-rolled CSS + small helpers is the simpler steady state.
- **Tailwind rewrite during the port** — rejected. Current vanilla CSS is dialed in; rewriting it would add weeks of churn for zero aesthetic improvement.
- **Full hexagonal six-crate workspace** — rejected as overkill given no database and no domain layer.
- **Single-crate flat module layout** — rejected (mildly). Workspace with `server` + `ui` crates gives the compile-time boundary check Dioxus Fullstack expects between rendering and server code, without significant scaffolding cost.
- **CSR (match current site)** — rejected. Zach chose SSR + hydration for SEO and social-card benefit.
- **Fold both Lambda operations into a single `/api/notify` route with an `action` discriminator** — rejected. Zach was explicit that they must be separate endpoints. Two operations, two routes, two DTOs.
- **Keep IP capture and GeoIP for pageview** — rejected. Zach: "remove the IP tracking entirely, just hit me with a 'someone visited your site' email, that is enough." No metadata flows on the pageview path.
- **Azure App Service deploy** — rejected (load-bearing). Azure infra is reserved for Agora; this is a personal project on a different cloud.
- **Nightly launchd schedule** — rejected. One-shot fire matches the "not high priority" framing.

## Related

- Camerata's universalized rules: every `agentic/orch-*`, `rust-domain-*`, `rust-dioxus-*`, `api-layer/arch-*`, `ci-cd/*`, `iac/arch-iac-1`, `universal/*` principle applies to this port unless this document overrides.
- Agora-rs's MONOLITH-1, QUEUE-1 (universal-shaped, applies). WORKERS-1 / EMAIL-1 (defined in agora-rs but not invoked here — no scheduled jobs, no transactional retry pattern).
- Chorale's CC-1 (cite rule IDs in commits) and ROBUSTNESS-1 (explicit > terse) apply — re-codified as PORT-CC-1 and PORT-ROBUST-1 in `docs/CONVENTIONS.md` to keep this repo self-contained.

## How to apply

The bot reads this file at the start of every session via `AGENTS.md`. Any decision in this file is locked unless Zach edits this file with a follow-up decision dated later than 2026-06-05.

Any time the bot encounters an architectural choice not covered here, the clear-winner test runs (camerata `orch-clear-winner-1`); if it doesn't resolve, the bot routes to Zach per `orch-one-way-door-1`.
