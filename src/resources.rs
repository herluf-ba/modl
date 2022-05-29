use std::{
    io::{BufReader, Cursor},
    path::PathBuf,
};

use glam::{Vec2, Vec3};
use wgpu::util::DeviceExt;

use crate::{model, texture};

pub async fn load_string(file_name: &str) -> anyhow::Result<String> {
    let path = std::path::Path::new(env!("OUT_DIR"))
        .join("assets")
        .join(file_name);

    let txt = std::fs::read_to_string(path)?;
    Ok(txt)
}

pub async fn load_binary(file_name: &str) -> anyhow::Result<Vec<u8>> {
    let path = std::path::Path::new(env!("OUT_DIR"))
        .join("assets")
        .join(file_name);
    let data = std::fs::read(path)?;
    Ok(data)
}

pub async fn load_texture(
    path: &str,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
) -> anyhow::Result<texture::Texture> {
    let data = load_binary(path).await?;
    texture::Texture::from_bytes(device, queue, &data, path)
}

// TODO: consider if this should be a Model::load instead
pub async fn load_model(
    file_name: &str,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    layout: &wgpu::BindGroupLayout,
) -> anyhow::Result<model::Model> {
    let obj_text = load_string(file_name).await?;
    let obj_cursor = Cursor::new(obj_text);
    let mut obj_reader = BufReader::new(obj_cursor);

    let (models, obj_materials) = tobj::load_obj_buf_async(
        &mut obj_reader,
        &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
        |p| async move {
            let mut path = PathBuf::from(file_name);
            path.set_file_name(p);
            let path = path.as_path().display().to_string();
            let mat_text = load_string(&path).await.unwrap();
            tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(mat_text)))
        },
    )
    .await?;

    let mut materials = Vec::new();
    for m in obj_materials? {
        let mut path_buf = PathBuf::from(file_name);
        path_buf.set_file_name(m.diffuse_texture);
        let diffuse_texture_file = path_buf.as_path().display().to_string();
        let diffuse_texture = load_texture(&diffuse_texture_file, device, queue).await?;
        path_buf.set_file_name(m.normal_texture);
        let normal_texture_file = path_buf.as_path().display().to_string();
        let normal_texture = load_texture(&normal_texture_file, device, queue).await?;

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&normal_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Sampler(&normal_texture.sampler),
                },
            ],
        });

        materials.push(model::Material {
            name: m.name,
            diffuse_texture,
            normal_texture,
            bind_group,
        })
    }

    let meshes = models
        .into_iter()
        .map(|m| {
            let mut vertices = (0..m.mesh.positions.len() / 3)
                .map(|i| model::ModelVertex {
                    position: [
                        m.mesh.positions[i * 3],
                        m.mesh.positions[i * 3 + 1],
                        m.mesh.positions[i * 3 + 2],
                    ],
                    tex_coords: [m.mesh.texcoords[i * 2], m.mesh.texcoords[i * 2 + 1]],
                    normal: [
                        m.mesh.normals[i * 3],
                        m.mesh.normals[i * 3 + 1],
                        m.mesh.normals[i * 3 + 2],
                    ],
                    tangent: [0.0; 3],
                    bitangent: [0.0; 3],
                })
                .collect::<Vec<_>>();

            // Compute tangents and bitangents
            let indices = &m.mesh.indices;
            let mut triangles_included = (0..vertices.len()).collect::<Vec<_>>();
            for c in indices.chunks(3) {
                // Read triangle and uvs
                let v0 = vertices[c[0] as usize];
                let v1 = vertices[c[1] as usize];
                let v2 = vertices[c[2] as usize];
                let pos0 = Vec3::from(v0.position);
                let pos1 = Vec3::from(v1.position);
                let pos2 = Vec3::from(v2.position);
                let uv0 = Vec2::from(v0.tex_coords);
                let uv1 = Vec2::from(v1.tex_coords);
                let uv2 = Vec2::from(v2.tex_coords);

                // Compute tangent and bitangent
                let delta_pos1 = pos1 - pos0;
                let delta_pos2 = pos2 - pos0;
                let delta_uv1 = uv1 - uv0;
                let delta_uv2 = uv2 - uv0;
                let r = 1.0 / (delta_uv1.x * delta_uv2.y - delta_uv1.y * delta_uv2.x);
                let tangent = (delta_pos1 * delta_uv2.y - delta_pos2 * delta_uv1.y) * r;
                let bitangent = (delta_pos2 * delta_uv1.x - delta_pos1 * delta_uv2.x) * -r;

                // Write tangent and bitangent
                vertices[c[0] as usize].tangent =
                    (Vec3::from(v0.tangent) + Vec3::from(tangent)).to_array();
                vertices[c[1] as usize].tangent =
                    (Vec3::from(v1.tangent) + Vec3::from(tangent)).to_array();
                vertices[c[2] as usize].tangent =
                    (Vec3::from(v2.tangent) + Vec3::from(tangent)).to_array();
                vertices[c[0] as usize].bitangent =
                    (Vec3::from(v0.bitangent) + Vec3::from(bitangent)).to_array();
                vertices[c[1] as usize].bitangent =
                    (Vec3::from(v1.bitangent) + Vec3::from(bitangent)).to_array();
                vertices[c[2] as usize].bitangent =
                    (Vec3::from(v2.bitangent) + Vec3::from(bitangent)).to_array();

                // Used for averaging tangents/bitangents later
                triangles_included[c[0] as usize] += 1;
                triangles_included[c[1] as usize] += 1;
                triangles_included[c[2] as usize] += 1;
            }

            // Average tangents and bitangents
            for (i, n) in triangles_included.into_iter().enumerate() {
                let denominator = 1.0 / n as f32;
                let mut v = &mut vertices[i];
                v.tangent = (Vec3::from(v.tangent) * denominator).normalize().to_array();
                v.bitangent = (Vec3::from(v.bitangent) * denominator)
                    .normalize()
                    .to_array();
            }

            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{:?} Vertex buffer", file_name)),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{:?} Index buffer", file_name)),
                contents: bytemuck::cast_slice(&m.mesh.indices),
                usage: wgpu::BufferUsages::INDEX,
            });

            model::Mesh {
                name: file_name.to_string(),
                vertex_buffer,
                index_buffer,
                num_elements: m.mesh.indices.len() as u32,
                material: m.mesh.material_id.unwrap_or(0),
            }
        })
        .collect::<Vec<_>>();

    Ok(model::Model { meshes, materials })
}
