# Bevy Scene Spec — v0.1 Faithful Port

This document enumerates every `@react-three/fiber`, `@react-three/drei`, and
`@react-three/postprocessing` usage in `MyPortfolioSite/src/components/Background3D/`
and maps each to its Bevy equivalent. Written as the deliverable for v0.1-bevy item 1.

---

## Source Files

- `Background3D.tsx` — Canvas setup, camera, camera controller, light, post-processing
- `Schematic.tsx` — Custom shader line field with forward-flight animation

---

## Dependency Inventory

| React dependency | Version | Usage |
|---|---|---|
| `@react-three/fiber` | `useFrame`, `Canvas` | animation loop, WebGL canvas |
| `@react-three/postprocessing` | `EffectComposer`, `Bloom` | post-process bloom |
| `three` (direct) | `THREE.*` | math, geometry, materials, fog, shaders |

No `@react-three/drei` usage found in either file (imported but unused).

---

## Scene Elements and Bevy Mapping

### 1. WebGL Canvas

React: `<Canvas dpr={[1,2]} gl={{ antialias: false, powerPreference: "high-performance", alpha: true, stencil: false, depth: true }}`

Bevy: `App::new()` with `DefaultPlugins` configured for WebGL2 (via the `webgl2` feature). Mount via `Window` plugin targeting `<canvas id="bevy-canvas">`. DPR equivalent via `Window::resolution.set_scale_factor_override(None)`.

**Mapping: clean.**

---

### 2. Background Color and Fog

React: `<color attach="background" args={['#0a1f1c']}/>` + `scene.fog = new THREE.FogExp2('#0a1f1c', 0.0075)`

Bevy:
- `ClearColor(Color::srgb_u8(10, 31, 28))` — matches `#0a1f1c`
- `FogSettings { color: Color::srgb_u8(10, 31, 28), falloff: FogFalloff::Exponential { density: 0.0075 }, .. }` — matches FogExp2

**Mapping: clean.**

---

### 3. Camera

React: `camera={{ position: [0, 2, 15], fov: 75, far: 2000 }}`

Bevy: `Camera3dBundle { transform: Transform::from_xyz(0.0, 2.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y), projection: Projection::Perspective(PerspectiveProjection { fov: 75.0_f32.to_radians(), far: 2000.0, .. }), .. }`

**Mapping: clean.**

---

### 4. Camera Controller

React animation (runs every frame via `useFrame`):
```
breatheX = sin(t * 0.12) * 1.2
breatheY = cos(t * 0.09) * 0.6
targetX = mouse.x * 4 + breatheX
targetY = mouse.y * 2 + 5 + breatheY
camera.position.x = lerp(camera.position.x, targetX, 0.06)
camera.position.y = lerp(camera.position.y, targetY, 0.06)
camera.lookAt(0, 0, 0)
```

Bevy: Bevy system that reads `Time`, `Windows::single().cursor_position()` (normalized to [-1,1]), and mutates the camera `Transform` with the same math. `lerp` via `Vec3::lerp` or manual float lerp.

**Mapping: clean.** (Mouse cursor in Bevy WASM is available via `CursorMoved` events or `Window::cursor_position()` mapped to normalized device coords.)

---

### 5. Ambient Light

React: `<ambientLight intensity={0.5} />`

Bevy: `AmbientLight { color: Color::WHITE, brightness: 0.5 }`

**Mapping: clean.**

---

### 6. Post-Processing Bloom

React: `<EffectComposer enableNormalPass={false}><Bloom luminanceThreshold={0.25} luminanceSmoothing={0.8} intensity={0.8} radius={0.3} /></EffectComposer>`

Bevy: `BloomSettings { intensity: 0.8, low_frequency_boost: 0.0, low_frequency_boost_curvature: 0.0, high_pass_frequency: 1.0 - 0.3, prefilter_settings: BloomPrefilterSettings { threshold: 0.25, threshold_softness: 0.8 }, composite_mode: BloomCompositeMode::Additive }` on the camera entity (Bevy 0.14).

Note: Bevy 0.14's `BloomSettings` has a slightly different parameter set than three.js's Bloom. The threshold and softness map directly; `radius` maps to `high_pass_frequency`; `intensity` maps directly.

**Mapping: clean (minor color/parameter drift within the acceptable range stated in PORT-SCENE-1).**

---

### 7. Schematic Line Field — Geometry

React: 60 `THREE.Line` objects, each with:
- `LINE_LENGTH = 300` world units wide, sampled every 2 units (151 points per line)
- Y positions: 6 slots at `((i % 6) - 2.5) * 2.4` = approx. {-6.0, -3.6, -1.2, 1.2, 3.6, 6.0}
- Z positions: randomly spread over 240 world unit range [-218, 22]
- `THREE.BufferGeometry().setFromPoints(...)` with `THREE.Line`

Bevy: 60 entities, each with a custom `Mesh` using `PrimitiveTopology::LineList` or `PrimitiveTopology::LineStrip`. Points generated the same way.

**Mapping: clean.**

---

### 8. Schematic — Custom GLSL Shader

React: `THREE.ShaderMaterial` with custom vertex + fragment GLSL:

**Vertex shader:**
- Sine wave: `pos.z += sin(pos.x * 0.15 + time * 0.6) * 1.2 * (1 + hoverIntensity * 0.8)`
- Cosine wave: `pos.y += cos(pos.x * 0.08 + time * 0.4) * 0.6`

**Fragment shader:**
- Traveling pulse: `sin(pos.x * 0.2 - phase)` raised to power 15
- Secondary pulse: `sin(pos.x * 0.5 - phase * 2.25)` raised to power 20, × 0.5
- Base alpha 0.1, modulated by pulse × (1 + hoverIntensity × 2)
- Color: `#d1d7d6` (light gray-blue), blended toward white on hover

**Uniforms:** `time`, `phase`, `color`, `hoverIntensity`, `pulseSpeed`
**Blending:** `THREE.AdditiveBlending`, `transparent: true`, `depthWrite: false`

Bevy mapping: `AsBindGroup` + `Material` trait (for 3D) or custom `RenderPipelineDescriptor`. GLSL → WGSL translation:
- `gl_FragColor` → `@location(0) out vec4 fragment_output`
- `gl_Position = projectionMatrix * modelViewMatrix * vec4(pos, 1.0)` → `@builtin(position) out vec4 = view_bindings::perspective_projection * view_bindings::view.view * model_matrix * vec4(pos, 1.0)`
- `sin`, `cos`, `pow`, `max`, `mix` → identical functions in WGSL
- Additive blending via pipeline `BlendState::ADDITIVE`

The WGSL translation is straightforward math. The shader is standard vertex+fragment with no compute, no raymarching, no texture sampling. **Mapping: clean.**

---

### 9. Schematic — Animation Loop

React:
- `phase.value += delta * pulseSpeed * 2.0` (per line, accumulated phase)
- `line.position.z += FLY_SPEED * delta * (1 + audioIntensity * 0.6)` (forward flight)
- Recycle: when `pos.z > 22`, subtract 240
- Group z-rotation: `sin(t * 0.1) * 0.05`
- Group x-rotation offset: 0.2 radians

Bevy: Bevy system reading `Time::delta_secs()` and mutating uniform buffers + `Transform` components per frame. The flight, recycle, and group rotation are plain Rust math.

**Mapping: clean.**

---

### 10. Audio Reactivity

React: `useAudioAnalyzer` reads Web Audio API frequency data (0-255). Modulates `hoverIntensity`, `pulseSpeed`, and flight speed. `isHovering` from Zustand.

Bevy WASM: Web Audio API is not natively accessible from Bevy. Options:
- Omit audio reactivity for v0.1 faithful port (scene looks correct; audio is an enhancement)
- Expose frequency data via `document::eval` + JS `AnalyserNode`, then read back via Bevy's WASM interop

**Decision for v0.1 faithful port:** Omit audio reactivity (PORT-SCENE-1 says faithful geometry/materials/animation — audio modulation is enhancement, not core geometry). `hoverIntensity` defaults to 0.0, `pulseSpeed` uses base value. The scene still animates continuously with traveling pulses and flight. Audio reactivity can be added as a v0.2 enhancement.

**Mapping: acceptable simplification, within PORT-SCENE-1 faithful-port-first policy.**

---

## Summary

All five scene elements (camera, fog, lights, bloom, line shader, flight animation) map cleanly to Bevy 0.14 primitives. No usage requires a third-party Bevy plugin or an unimplementable feature.

**No ROUTE required on scene-mapping grounds** (per v0.1-bevy item 1 condition).

**ROUTE required on dependency grounds** (per PORT-ROUTE-1): `bevy` dep is in `[workspace.dependencies]` but NOT yet in any crate's `[dependencies]`. Wiring the Bevy scene requires either:
1. Adding `bevy = { workspace = true }` to `ui/Cargo.toml` (dep addition → PORT-ROUTE-1), OR
2. Adding a new `bevy_scene` crate to the workspace (crate addition → PORT-ROUTE-1)

Additionally, PORT-BEVY-1 references `server/src/bevy_scene.rs` as the Bevy entry point — but `server` is a native binary, not compiled to WASM. The actual Bevy WASM scene must live in the `ui` crate or a dedicated WASM crate. This architectural clarification is also needed from Zach.

See `.overnight-portfolio-decisions_needed.md` for the routing entry.

---

**2026-06-05 update — crate renamed.** Option 2 was chosen and implemented, but the crate was initially named `bevy_scene` which collided with Bevy's own ecosystem crate `bevy_scene@0.14.2` (transitively required by the `bevy` umbrella crate). `cargo build -p bevy_scene` was ambiguous between the two. Crate renamed to `portfolio_scene` the same day; references throughout the codebase updated. References to `bevy_scene` above remain as the historical record of what was considered + first-built.
