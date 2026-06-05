//! Bevy WASM scene — faithful port of MyPortfolioSite/src/components/Background3D/.
//!
//! Compiles to wasm32-unknown-unknown (cdylib). JS in the Dioxus app loads it on
//! DOMContentLoaded; Bevy mounts to `<canvas id="bevy-canvas">` at z-index:-1
//! per PORT-BEVY-1. Scene spec documented in docs/bevy-scene-spec-v0.1.md.

use bevy::{
    asset::load_internal_asset,
    core_pipeline::bloom::{BloomCompositeMode, BloomPrefilterSettings, BloomSettings},
    pbr::{
        FogFalloff, FogSettings, Material, MaterialMeshBundle, MaterialPipeline,
        MaterialPipelineKey, MaterialPlugin,
    },
    prelude::*,
    render::{
        mesh::MeshVertexBufferLayoutRef,
        render_asset::RenderAssetUsages,
        render_resource::{
            AsBindGroup, BlendComponent, BlendFactor, BlendOperation, BlendState,
            RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
    },
    window::PrimaryWindow,
};
use wasm_bindgen::prelude::*;

const LINE_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(0x9d4f_82a1_5c3e_7b60_u128);

// Scene constants — verbatim from Schematic.tsx.
const NUM_Y: usize = 6;
const NUM_Z: usize = 10;
const TOTAL: usize = NUM_Y * NUM_Z;
const Z_STEP: f32 = 24.0;
const DEPTH_RANGE: f32 = Z_STEP * NUM_Z as f32;
const Z_RECYCLE: f32 = 22.0;
const FLY_SPEED: f32 = 7.0;
const LINE_LENGTH: f32 = 300.0;

/// JS entry point — constructs and runs the Bevy app targeting `#bevy-canvas`.
#[wasm_bindgen(start)]
pub fn start() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#bevy-canvas".into()),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb_u8(10, 31, 28)))
        .add_plugins(BevyScenePlugin)
        .run();
}

struct BevyScenePlugin;

impl Plugin for BevyScenePlugin {
    fn build(&self, app: &mut App) {
        // Embed the WGSL shader in the WASM binary — no runtime HTTP fetch needed.
        load_internal_asset!(
            app,
            LINE_SHADER_HANDLE,
            "shaders/line_material.wgsl",
            Shader::from_wgsl
        );
        app.add_plugins(MaterialPlugin::<LineMaterial>::default())
            .add_systems(Startup, setup_scene)
            .add_systems(Update, (camera_controller, animate_lines, animate_group));
    }
}

// ─── Components ──────────────────────────────────────────────────────────────

/// Per-line animation state not needed in the GPU uniform.
#[derive(Component)]
struct LineState {
    pulse_speed: f32,
}

/// Marker for the line group entity (holds group-level rotation animation).
#[derive(Component)]
struct LineGroup;

/// Marker for the camera entity (used for camera controller query filtering).
#[derive(Component)]
struct SceneCamera;

// ─── Material ────────────────────────────────────────────────────────────────

/// Custom material for the pulse-shader lines.
///
/// Four f32 fields = 16 bytes — satisfies WGSL uniform-buffer alignment exactly.
/// The `_pad` field is unused by the shader; it pads the struct to 16 bytes so
/// encase does not insert implicit padding that would misalign the WGSL struct.
#[derive(Clone, Default, Asset, TypePath, AsBindGroup)]
struct LineMaterial {
    #[uniform(0)]
    time: f32,
    #[uniform(0)]
    phase: f32,
    #[uniform(0)]
    hover_intensity: f32,
    #[uniform(0)]
    _pad: f32,
}

impl Material for LineMaterial {
    fn vertex_shader() -> ShaderRef {
        LINE_SHADER_HANDLE.into()
    }
    fn fragment_shader() -> ShaderRef {
        LINE_SHADER_HANDLE.into()
    }
    fn alpha_mode(&self) -> AlphaMode {
        // Transparent pass so depth-sort runs; additive blend set in specialize.
        AlphaMode::Blend
    }
    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // Only need POSITION — no normals, no UVs on line meshes.
        let vertex_layout = layout
            .0
            .get_layout(&[Mesh::ATTRIBUTE_POSITION.at_shader_location(0)])?;
        descriptor.vertex.buffers = vec![vertex_layout];

        // Line strip topology to match THREE.Line rendering.
        descriptor.primitive.topology = bevy::render::render_resource::PrimitiveTopology::LineStrip;
        descriptor.primitive.cull_mode = None;

        // Additive blending: matches THREE.AdditiveBlending.
        let additive = Some(BlendState {
            color: BlendComponent {
                src_factor: BlendFactor::One,
                dst_factor: BlendFactor::One,
                operation: BlendOperation::Add,
            },
            alpha: BlendComponent {
                src_factor: BlendFactor::One,
                dst_factor: BlendFactor::One,
                operation: BlendOperation::Add,
            },
        });
        if let Some(fragment) = descriptor.fragment.as_mut() {
            for target in &mut fragment.targets {
                if let Some(t) = target.as_mut() {
                    t.blend = additive;
                }
            }
        }

        // No depth write: matches THREE `depthWrite: false`.
        if let Some(ds) = descriptor.depth_stencil.as_mut() {
            ds.depth_write_enabled = false;
        }

        Ok(())
    }
}

// ─── Systems ─────────────────────────────────────────────────────────────────

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LineMaterial>>,
) {
    // Camera with HDR for bloom.
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                clear_color: ClearColorConfig::Custom(Color::srgb_u8(10, 31, 28)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 2.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
            projection: Projection::Perspective(PerspectiveProjection {
                fov: 75.0_f32.to_radians(),
                far: 2000.0,
                ..default()
            }),
            ..default()
        },
        BloomSettings {
            intensity: 0.8,
            low_frequency_boost: 0.0,
            low_frequency_boost_curvature: 0.0,
            high_pass_frequency: 0.7,
            prefilter_settings: BloomPrefilterSettings {
                threshold: 0.25,
                threshold_softness: 0.8,
            },
            composite_mode: BloomCompositeMode::Additive,
        },
        FogSettings {
            color: Color::srgb_u8(10, 31, 28),
            falloff: FogFalloff::Exponential { density: 0.0075 },
            ..default()
        },
        SceneCamera,
    ));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.5,
    });

    // Build 6 shared line meshes — one per Y slot.
    // Y slot: ((y_idx as f32) - (NUM_Y - 1) / 2) * 2.4  matches Schematic.tsx yPos formula.
    let mut mesh_handles: Vec<Handle<Mesh>> = Vec::with_capacity(NUM_Y);
    for y_idx in 0..NUM_Y {
        let y_pos = (y_idx as f32 - (NUM_Y as f32 - 1.0) / 2.0) * 2.4;
        // 151 points: x = -150, -148, …, 150 (step 2, matches `x <= LINE_LENGTH / 2` loop).
        let point_count = ((LINE_LENGTH / 2.0) as usize + 1) * 2 - 1; // = 151
        let mut positions: Vec<[f32; 3]> = Vec::with_capacity(point_count);
        let mut x = -(LINE_LENGTH / 2.0);
        while x <= LINE_LENGTH / 2.0 + f32::EPSILON {
            positions.push([x, y_pos, 0.0]);
            x += 2.0;
        }
        let mut mesh = Mesh::new(
            bevy::render::render_resource::PrimitiveTopology::LineStrip,
            RenderAssetUsages::RENDER_WORLD,
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh_handles.push(meshes.add(mesh));
    }

    // Line group — initial X rotation 0.2 rad (from React `rotation={[0.2, 0, 0]}`).
    let group = commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_rotation(Quat::from_rotation_x(0.2)),
                ..default()
            },
            LineGroup,
        ))
        .id();

    // 60 line entities: 10 Z-slices × 6 Y-slots.
    for i in 0..TOTAL {
        let y_idx = i % NUM_Y;
        let mesh_handle = mesh_handles[y_idx].clone_weak();

        // Deterministic pseudo-random Z spread (sin-based; no rand dep per PORT-ROUTE-1).
        let pseudo_rand = (i as f32 * 7.539_f32 + 1.234_f32).sin().abs();
        let initial_z = Z_RECYCLE - pseudo_rand * DEPTH_RANGE;

        // Per-line base pulse speed: 0.8 + sin(i * 0.5) * 0.3, matching Schematic.tsx baseSpeed.
        let base_speed = 0.8 + ((i as f32) * 0.5).sin() * 0.3;

        let mat_handle = materials.add(LineMaterial::default());

        let child = commands
            .spawn((
                MaterialMeshBundle::<LineMaterial> {
                    mesh: mesh_handle,
                    material: mat_handle,
                    transform: Transform::from_xyz(0.0, 0.0, initial_z),
                    ..default()
                },
                LineState {
                    pulse_speed: base_speed,
                },
            ))
            .id();

        commands.entity(group).add_child(child);
    }
}

fn camera_controller(
    time: Res<Time>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut cameras: Query<&mut Transform, With<SceneCamera>>,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };
    let Ok(mut cam) = cameras.get_single_mut() else {
        return;
    };

    let t = time.elapsed_seconds();
    // Idle breathing drift — the signature atmospheric-menu feel.
    let breathe_x = (t * 0.12).sin() * 1.2;
    let breathe_y = (t * 0.09).cos() * 0.6;

    // Normalize cursor to [-1, 1]; default to zero when cursor leaves window.
    let mouse = if let Some(cursor) = window.cursor_position() {
        let w = window.width().max(1.0);
        let h = window.height().max(1.0);
        Vec2::new(
            (cursor.x / w) * 2.0 - 1.0,
            -((cursor.y / h) * 2.0 - 1.0), // invert Y (screen is top-down)
        )
    } else {
        Vec2::ZERO
    };

    let target_x = mouse.x * 4.0 + breathe_x;
    let target_y = mouse.y * 2.0 + 5.0 + breathe_y;

    // Slow lerp (0.06) for cinematic glide — matches CameraController.tsx.
    cam.translation.x += (target_x - cam.translation.x) * 0.06;
    cam.translation.y += (target_y - cam.translation.y) * 0.06;
    cam.look_at(Vec3::ZERO, Vec3::Y);
}

fn animate_lines(
    time: Res<Time>,
    mut materials: ResMut<Assets<LineMaterial>>,
    mut query: Query<(&mut Transform, &Handle<LineMaterial>, &LineState)>,
) {
    let t = time.elapsed_seconds();
    let delta = time.delta_seconds();

    for (mut transform, mat_handle, state) in &mut query {
        if let Some(mat) = materials.get_mut(mat_handle) {
            mat.time = t;
            // Accumulate phase per-line at its own base speed (no audio in v0.1).
            mat.phase += delta * state.pulse_speed * 2.0;
        }

        // Forward flight: pull lines toward camera; recycle to far back when passed.
        transform.translation.z += FLY_SPEED * delta;
        if transform.translation.z > Z_RECYCLE {
            transform.translation.z -= DEPTH_RANGE;
        }
    }
}

fn animate_group(time: Res<Time>, mut query: Query<&mut Transform, With<LineGroup>>) {
    let t = time.elapsed_seconds();
    let Ok(mut transform) = query.get_single_mut() else {
        return;
    };
    // Keep base X rotation 0.2 rad; animate Z rotation per groupRef in Schematic.tsx.
    transform.rotation = Quat::from_euler(EulerRot::XYZ, 0.2, 0.0, (t * 0.1).sin() * 0.05);
}
