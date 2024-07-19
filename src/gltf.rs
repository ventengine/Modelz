use std::{fs, path::Path};

use gltf::Mesh;

use crate::{Indices, Model3D, ModelError, Vertex};

pub fn load(path: &Path) -> Result<Model3D, ModelError> {
    let gltf = gltf::Gltf::from_reader(fs::File::open(path).unwrap()).unwrap();

    let path = path.parent().unwrap_or_else(|| Path::new("./"));

    let buffer_data = gltf::import_buffers(&gltf.document, Some(path), gltf.blob)
        .expect("Failed to Load glTF Buffers");

    let mut materials = Vec::new();
    for material in gltf.document.materials() {
        materials.push(crate::Material {
            name: material.name().map(|s| s.to_string()),
        })
    }

    let mut meshes = Vec::new();
    for mesh in gltf.document.meshes() {
        meshes.append(&mut load_mesh(mesh, &buffer_data))
    }

    Ok(Model3D {
        meshes,
        materials,
        format: crate::ModelFormat::GLTF,
    })
}

// fn load_node(
//     node: gltf::Node<'_>,
//     buffer_data: &[gltf::buffer::Data],
// ) {
//     if let Some(mesh) = node.mesh() {
//         load_mesh(
//             mesh,
//             buffer_data,
//         );
//     }
//     node.children().enumerate().for_each(|(i, child)| {
//         load_node(
//             child,
//             buffer_data,
//         )
//     });
// }

fn load_mesh(mesh: Mesh, buffer_data: &[gltf::buffer::Data]) -> Vec<crate::Mesh> {
    let mut meshes = Vec::new();
    for primitive in mesh.primitives() {
        let (vertices, indices) = load_primitive(buffer_data, &primitive);
        meshes.push(crate::Mesh {
            vertices,
            indices,
            material_index: primitive.material().index(),
            name: mesh.name().map(|s| s.to_string()),
        })
    }
    meshes
}

fn load_primitive<'a>(
    buffer_data: &'a [gltf::buffer::Data],
    primitive: &gltf::Primitive<'a>,
) -> (Vec<Vertex>, Option<Indices>) {
    let reader = primitive.reader(|buffer| Some(&buffer_data[buffer.index()]));

    let mut vertices: Vec<Vertex> = reader
        .read_positions()
        .unwrap()
        .map(|position| Vertex {
            position,
            tex_coord: None,
            normal: None,
        })
        .collect();

    if let Some(normal_attribute) = reader.read_normals() {
        for (normal_index, normal) in normal_attribute.enumerate() {
            vertices[normal_index].normal = Some(normal);
        }
    }

    if let Some(tex_coord_attribute) = reader.read_tex_coords(0).map(|v| v.into_f32()) {
        for (tex_coord_index, tex_coord) in tex_coord_attribute.enumerate() {
            vertices[tex_coord_index].tex_coord = Some(tex_coord);
        }
    }

    let indices = if let Some(indcies) = reader.read_indices() {
        Some(match indcies {
            gltf::mesh::util::ReadIndices::U8(indices) => Indices::U8(indices.collect::<Vec<_>>()),
            gltf::mesh::util::ReadIndices::U16(indices) => {
                Indices::U16(indices.collect::<Vec<_>>())
            }
            gltf::mesh::util::ReadIndices::U32(indices) => {
                Indices::U32(indices.collect::<Vec<_>>())
            }
        })
    } else {
        None
    };
    (vertices, indices)
}
