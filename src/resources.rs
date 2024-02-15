use std::io::{BufReader, Cursor};

use cfg_if::cfg_if;
use wgpu::util::DeviceExt;
use image::{DynamicImage, Rgb, RgbImage};
use console_log::log;
use log::info;
use nalgebra::{Vector3, Point3};


use crate::{model, texture};



pub async fn load_texture(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
) -> anyhow::Result<texture::Texture> {
    let mut rgb_img = RgbImage::new(1, 1);
    rgb_img.put_pixel(0,0,Rgb([150, 150,150]));
    let dynamic_image = DynamicImage::ImageRgb8(rgb_img);

    let file_name = "red pixel";

    texture::Texture::from_image(device, queue, &dynamic_image, Some(file_name))
}

pub async fn load_model(
    bytestream: &[u8],
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    layout: &wgpu::BindGroupLayout,
) -> anyhow::Result<model::Model> {
    let mut buffer = [0; 4];
    buffer.copy_from_slice(&bytestream[80..84]);
    let num_triangles = u32::from_le_bytes(buffer);
    let expected_byte_count = 50*num_triangles+84;
    let acual_byte_count = bytestream.len() as u32;


    let mut materials;
    let mut meshes;

    if bytestream.starts_with(b"solid") || expected_byte_count==acual_byte_count {
     
        materials = Vec::new();

        let diffuse_texture = load_texture(device, queue).await?;
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
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
            ],
            label: None,
        });

        materials.push(model::Material {
            name: "material".to_string(),
            diffuse_texture,
            bind_group,
        });

        let mut stl_cursor = Cursor::new(bytestream);
        let stl = stl_io::read_stl(&mut stl_cursor).unwrap();
        
        //let (vertices, indices) = stl::process_stl(stl);
        let centroid_vertices: Vec<(f32, f32, f32)> = stl.vertices.iter()
            .map(|v| {            
                (v[0], v[1], v[2])
            })
            .collect();
        let centroid = calculate_centroid(&centroid_vertices);
        let offset = (-centroid.0, -centroid.1, -centroid.2);
        let max_dimension = calculate_max_dimension(&centroid_vertices);
        
        // Determine the scaling factor needed to achieve the desired height
        let desired_height = 6.0; // Example desired height
        let scaling_factor = desired_height / max_dimension;

        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for (index, face) in stl.faces.iter().enumerate() {

            for i in 0..3{
                let current_index = index*3+i;
                indices.push(current_index);

                let vertice_index = face.vertices[i];
                let current_vertex = stl.vertices[vertice_index].clone();
                let offset_vertex = [
                    (current_vertex[0] + offset.0)*scaling_factor,
                    (current_vertex[1] + offset.1)*scaling_factor,
                    (current_vertex[2] + offset.2)*scaling_factor,
                ];
                let model_vertex = model::ModelVertex {
                    position: offset_vertex,
                    tex_coords: [0.0, 0.0],
                    normal: face.normal.into(),
                };
                vertices.push(model_vertex);

            }
    
        
        }
    

        //info!("vertices: {:#?}",vertices);
        //info!("indices: {:#?}", indices);

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("STL Vertex Buffer")),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });


        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("STL Index Buffer")),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let mesh_model = model::Mesh {
            name: "STL model".to_string(),
            vertex_buffer,
            index_buffer,
            num_elements: indices.len() as u32,
            material: 0,
        };
        
        /*
        let centroid_vertices: Vec<(f32, f32, f32)> = stl.vertices.iter()
            .map(|v| {            
                (v[0], v[1], v[2])
            })
            .collect();
        let centroid = calculate_centroid(&centroid_vertices);
        let offset = (-centroid.0, -centroid.1, -centroid.2);
        let max_dimension = calculate_max_dimension(&centroid_vertices);
        
        // Determine the scaling factor needed to achieve the desired height
        let desired_height = 6.0; // Example desired height
        let scaling_factor = desired_height / max_dimension;

        let vertices = stl
            .vertices
            .iter()
            .enumerate()
            .map(|(i, vertex)| {
                //check if normal data exists
                let normal = if i < stl.faces.len() {
                    (stl.faces[i].normal).into()
                } else {
                    [0.0, 0.0, 0.0] 
                };
                //info!("vertex {} position: {:#?}", i, vertex);
                let offset_vertex = [
                    (vertex[0] + offset.0)*scaling_factor,
                    (vertex[1] + offset.1)*scaling_factor,
                    (vertex[2] + offset.2)*scaling_factor,
                ];
                //info!("offet vertex {} position: {:#?}", i, offset_vertex);

                model::ModelVertex {
                    position: offset_vertex,//(*vertex).into()
                    tex_coords: [0.0, 0.0],
                    normal,
                }
            })
            .collect::<Vec<_>>();
            

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("STL Vertex Buffer")),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let mut indices = Vec::new();

        for face in &stl.faces {
            let vertex_indices = [face.vertices[0], face.vertices[1], face.vertices[2]];
            indices.extend_from_slice(&vertex_indices);
        }

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("STL Index Buffer")),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let mesh_model = model::Mesh {
            name: "STL model".to_string(),
            vertex_buffer,
            index_buffer,
            num_elements: indices.len() as u32,
            material: 0,
        };
        */
        meshes = vec![mesh_model];

        
    }else{

        let obj_cursor = Cursor::new(bytestream);
        let mut obj_reader = BufReader::new(obj_cursor);

        let (models, obj_materials) = tobj::load_obj_buf_async(
            &mut obj_reader,
            &tobj::LoadOptions {
                triangulate: true,
                single_index: true,
                ..Default::default()
            },
            |p| async move {
                let mat_text = "newmtl Material.001
                Ns 323.999994
                Ka 1.000000 1.000000 1.000000
                Kd 0.800000 0.800000 0.800000
                Ks 0.500000 0.500000 0.500000
                Ke 0.000000 0.000000 0.000000
                Ni 1.450000
                d 1.000000
                illum 2";
                tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(mat_text)))
            },
        )
        .await?;

        materials = Vec::new();
        for m in obj_materials? {
            let diffuse_texture = load_texture(device, queue).await?;
            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
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
                ],
                label: None,
            });

            materials.push(model::Material {
                name: "material".to_string(),
                diffuse_texture,
                bind_group,
            })
        }


        meshes = models
            .into_iter()
            .map(|m| {
                
                let vertices = (0..m.mesh.positions.len() / 3)
                    .map(|i| model::ModelVertex {
                        position: [
                            m.mesh.positions[i * 3],
                            m.mesh.positions[i * 3 + 1],
                            m.mesh.positions[i * 3 + 2],
                        ],
                        tex_coords: [m.mesh.texcoords[i * 2], 1.0 - m.mesh.texcoords[i * 2 + 1]],
                        normal: [
                            m.mesh.normals[i * 3],
                            m.mesh.normals[i * 3 + 1],
                            m.mesh.normals[i * 3 + 2],
                        ],
                    })
                    .collect::<Vec<_>>();

                    info!("vertices: {:#?}",vertices);
                    info!("indices: {:#?}", m.mesh.indices);

                let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("mesh Vertex Buffer")),
                    contents: bytemuck::cast_slice(&vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                });
                let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("mesh Index Buffer")) ,
                    contents: bytemuck::cast_slice(&m.mesh.indices),
                    usage: wgpu::BufferUsages::INDEX,
                });

                model::Mesh {
                    name: "mesh".to_string(),
                    vertex_buffer,
                    index_buffer,
                    num_elements: m.mesh.indices.len() as u32,
                    material: m.mesh.material_id.unwrap_or(0),
                }
            })
            .collect::<Vec<_>>();

    }

    Ok(model::Model { meshes, materials })

}

fn calculate_centroid(vertices: &[(f32, f32, f32)]) -> (f32, f32, f32) {
    let (sum_x, sum_y, sum_z) = vertices.iter()
        .fold((0.0, 0.0, 0.0), |acc, &v| (acc.0 + v.0, acc.1 + v.1, acc.2 + v.2));
    let num_vertices = vertices.len() as f32;
    (sum_x / num_vertices, sum_y / num_vertices, sum_z / num_vertices)
}


fn calculate_max_dimension(vertices: &[(f32, f32, f32)]) -> f32 {
    let min_x = vertices.iter().map(|v| v.0).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_x = vertices.iter().map(|v| v.0).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let width = max_x - min_x;

    let min_y = vertices.iter().map(|v| v.1).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_y = vertices.iter().map(|v| v.1).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let height = max_y-min_y;

    let min_z = vertices.iter().map(|v| v.2).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_z = vertices.iter().map(|v| v.2).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let depth = max_z - min_z;

    width.max(height).max(depth)
}