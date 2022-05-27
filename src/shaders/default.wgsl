//Vertex shader

struct Camera {
    position: vec3<f32>;
    view_projection: mat4x4<f32>;
};

struct Light { 
    position: vec3<f32>;
    color: vec3<f32>;
};

struct InstanceInput {
    [[location(5)]] transform_matrix_0: vec4<f32>;
    [[location(6)]] transform_matrix_1: vec4<f32>;
    [[location(7)]] transform_matrix_2: vec4<f32>;
    [[location(8)]] transform_matrix_3: vec4<f32>;
    [[location(9)]] normal_matrix_0: vec3<f32>;
    [[location(10)]] normal_matrix_1: vec3<f32>;
    [[location(11)]] normal_matrix_2: vec3<f32>;
};

struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] tex_coords: vec2<f32>;
    [[location(2)]] normal: vec3<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] tex_coords: vec2<f32>;
    [[location(1)]] world_normal: vec3<f32>;
    [[location(2)]] world_position: vec3<f32>;
};

[[group(1), binding(0)]] var<uniform> camera: Camera;
[[group(2), binding(0)]] var<uniform> light: Light;

[[stage(vertex)]]
fn vs_main(
    vert: VertexInput,
    instance: InstanceInput
) -> VertexOutput {
    let transform = mat4x4<f32>(
        instance.transform_matrix_0,
        instance.transform_matrix_1,
        instance.transform_matrix_2,
        instance.transform_matrix_3,
    );
    let normal_matrix = mat3x3<f32>(
        instance.normal_matrix_0,
        instance.normal_matrix_1,
        instance.normal_matrix_2,
    );
    var out: VertexOutput;
    out.tex_coords = vert.tex_coords;
    out.world_normal = normal_matrix * vert.normal;
    var world_position = transform * vec4<f32>(vert.position, 1.0);
    out.world_position = world_position.xyz;
    out.clip_position = camera.view_projection * world_position;
    return out;
}

// Fragment shader
[[group(0), binding(0)]]
var t_diffuse: texture_2d<f32>;
[[group(0), binding(1)]]
var s_diffuse: sampler;

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    // Sample texture for pure diffuse color
    let texture_color = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    
    // Compute ambient light
    let ambient_strength = 0.1;
    let ambient_color = light.color * ambient_strength;

    // Compute diffuse light
    let light_dir = normalize(light.position - in.world_position);
    let diffuse_strength = max(0.0, dot(in.world_normal, light_dir));
    let diffuse_color = light.color * diffuse_strength;

    //Compute specular light
    let view_dir = normalize(camera.position.xyz - in.world_position);
    let half_dir = normalize(view_dir + light_dir);
    let specular_strength = pow(max(dot(in.world_normal, half_dir), 0.0), 32.0);
    let specular_color = specular_strength * light.color;

    let result = (ambient_color + diffuse_color + specular_color) * texture_color.xyz;
    return vec4<f32>(result, texture_color.a);
}