#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_clip}

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) barycentric: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) barycentric: vec3<f32>,
};

struct EdgeMaterialData {
    color: vec4<f32>,
    settings: vec4<f32>,
};

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> material: EdgeMaterialData;

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = mesh_position_local_to_clip(
        get_world_from_local(vertex.instance_index),
        vec4<f32>(vertex.position, 1.0),
    );
    out.barycentric = vertex.barycentric;
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let bary = in.barycentric;

    // Get line thickness from settings.x (default to 1.0 if zero)
    let line_thickness = max(material.settings.x, 1.0);
    let thickness = line_thickness * 0.02;

    // Find minimum barycentric coordinate (distance to nearest edge)
    let min_bary = min(min(bary.x, bary.y), bary.z);

    // Use smoothstep for anti-aliased edges
    let edge = 1.0 - smoothstep(0.0, thickness, min_bary);

    // Discard pixels that are not on edges
    if edge < 0.5 {
        discard;
    }

    return material.color;
}
