// Faithful port of Schematic.tsx PulseShader to WGSL (PORT-SCENE-1).
// Vertex: sine/cosine architectural breathing.  Fragment: traveling pulse + fog.

#import bevy_pbr::mesh_functions::get_world_from_local
#import bevy_pbr::mesh_view_bindings::view
#import bevy_pbr::view_transformations::position_world_to_view

// 4 × f32 = 16 bytes — clean WGSL uniform-buffer alignment.
struct LineMaterial {
    time: f32,
    phase: f32,
    hover_intensity: f32,
    _pad: f32,
}

@group(2) @binding(0)
var<uniform> material: LineMaterial;

struct VertexInput {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) v_pos: f32,
    @location(1) view_z: f32,
}

@vertex
fn vertex(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    var pos = in.position;

    // Architectural Breathing: sine wave on Z, cosine wave on Y (port of PulseShader vertex).
    let wave = sin(pos.x * 0.15 + material.time * 0.6) * 1.2;
    pos.z += wave * (1.0 + material.hover_intensity * 0.8);
    pos.y += cos(pos.x * 0.08 + material.time * 0.4) * 0.6;

    let world_from_local = get_world_from_local(in.instance_index);
    let world_pos = world_from_local * vec4<f32>(pos, 1.0);

    // View-space Z for fog — view looks down -Z so negate to get positive depth.
    let view_pos = position_world_to_view(world_pos.xyz);
    out.view_z = -view_pos.z;

    out.v_pos = pos.x;
    out.clip_position = view.clip_from_world * world_pos;
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // Primary pulse: sharpened to roughly 5% of line length.
    let pulse = pow(max(0.0, sin(in.v_pos * 0.2 - material.phase)), 15.0);
    // Secondary faster pulses at 2.25× phase.
    let pulse2 = pow(max(0.0, sin(in.v_pos * 0.5 - material.phase * 2.25)), 20.0) * 0.5;

    // Base color #d1d7d6 in sRGB-normalized floats.
    var color = vec3<f32>(0.8196, 0.8431, 0.8392);
    let alpha_raw = 0.1 + (pulse + pulse2) * (1.0 + material.hover_intensity * 2.0);

    // Hover glint boost (v0.1: hover_intensity always 0.0).
    if material.hover_intensity > 0.1 {
        color = mix(color, vec3<f32>(1.0, 1.0, 1.0), material.hover_intensity * 0.5);
    }

    // Exponential fog matching THREE.FogExp2('#0a1f1c', density=0.0075).
    // Formula: fogFactor = exp(-density^2 * depth^2)  applied to alpha so
    // distant additive lines fade to zero (background #0a1f1c shows through).
    let fog_factor = exp(-0.0075 * 0.0075 * in.view_z * in.view_z);

    return vec4<f32>(color, alpha_raw * fog_factor);
}
