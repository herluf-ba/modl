//Vertex shader

struct CameraUniform {
    view_projection: mat4x4<f32>;
};

struct InstanceInput {
    [[location(5)]] model_matrix_0: vec4<f32>;
    [[location(6)]] model_matrix_1: vec4<f32>;
    [[location(7)]] model_matrix_2: vec4<f32>;
    [[location(8)]] model_matrix_3: vec4<f32>;
};

struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] tex_coords: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] tex_coords: vec2<f32>;
};

[[group(1), binding(0)]] 
var<uniform> camera: CameraUniform;

[[stage(vertex)]]
fn vs_main(
    vert: VertexInput,
    instance: InstanceInput
) -> VertexOutput {
    let transform = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );
    var out: VertexOutput;
    out.tex_coords = vert.tex_coords;
    out.position = camera.view_projection * transform * vec4<f32>(vert.position, 1.0);
    return out;
}

// Fragment shader
[[group(0), binding(0)]]
var t_diffuse: texture_2d<f32>;
[[group(0), binding(1)]]
var s_diffuse: sampler;

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}