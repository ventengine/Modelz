use std::{
    fs::{self},
    path::Path,
};

use gltf::Mesh;

use crate::{Indices, Model3D, ModelError, Vertex};

pub fn load(path: &Path) -> Result<Model3D, ModelError> {
    let gltf = gltf::Gltf::from_reader(
        fs::File::open(path).map_err(|e| ModelError::OpenFile(e.to_string()))?,
    )
    .map_err(|e| ModelError::ModelParsing(e.to_string()))?;

    let path = path.parent().unwrap_or_else(|| Path::new("./"));

    let buffer_data = gltf::import_buffers(&gltf.document, Some(path), gltf.blob)
        .expect("Failed to Load glTF Buffers");

    let mut materials = Vec::new();
    let len = gltf.document.materials().len();
    for (i, material) in gltf.document.materials().enumerate() {
        log::debug!(
            "Loading Material {} {}/{}",
            material.name().unwrap_or("Unknown"),
            i + 1,
            len,
        );
        materials.push(load_material(path, material, &buffer_data)?);
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

fn load_material<'a>(
    model_dir: &'a Path,
    material: gltf::Material<'a>,
    buffer_data: &'a [gltf::buffer::Data],
) -> Result<crate::Material, ModelError> {
    let pbr = material.pbr_metallic_roughness();

    let diffuse_texture = if let Some(texture) = pbr.base_color_texture() {
        let texture = texture.texture();
        match texture.source().source() {
            gltf::image::Source::View { view, mime_type } => {
                let parent_buffer_data = &buffer_data[view.buffer().index()].0;
                let begin = view.offset();
                let end = begin + view.length();
                let encoded_image = &parent_buffer_data[begin..end];

                let sampler = texture.sampler();
                let image = crate::Image::Memory {
                    data: encoded_image.to_vec(), // idk
                    mime_type: Some(mime_type.to_string()),
                };
                Some(crate::Texture {
                    image,
                    sampler: convert_sampler(&sampler),
                    name: texture.name().map(|s| s.to_string()),
                })
            }
            gltf::image::Source::Uri { uri, mime_type } => {
                let sampler = texture.sampler();

                let image = crate::Image::Path {
                    path: model_dir.join(uri),
                    mime_type: mime_type.map(|s| s.to_string()),
                };

                Some(crate::Texture {
                    image,
                    sampler: convert_sampler(&sampler),
                    name: texture.name().map(|s| s.to_string()),
                })
            }
        }
    } else {
        None
    };
    let alpha_mode = convert_alpha_mode(material.alpha_mode());

    Ok(crate::Material {
        diffuse_texture,
        alpha_mode,
        double_sided: material.double_sided(),
        name: material.name().map(|s| s.to_string()),
        base_color: Some(pbr.base_color_factor()),
        alpha_cutoff: material.alpha_cutoff(),
    })
}

fn convert_sampler<'a>(sampler: &'a gltf::texture::Sampler<'a>) -> crate::Sampler {
    let mag_filter = sampler.mag_filter().map(|filter| match filter {
        gltf::texture::MagFilter::Nearest => crate::MagFilter::Nearest,
        gltf::texture::MagFilter::Linear => crate::MagFilter::Linear,
    });

    let min_filter = sampler.min_filter().map(|filter| match filter {
        gltf::texture::MinFilter::Nearest => crate::MinFilter::Nearest,
        gltf::texture::MinFilter::Linear => crate::MinFilter::Linear,
        gltf::texture::MinFilter::NearestMipmapNearest => crate::MinFilter::NearestMipmapNearest,
        gltf::texture::MinFilter::LinearMipmapNearest => crate::MinFilter::LinearMipmapNearest,
        gltf::texture::MinFilter::NearestMipmapLinear => crate::MinFilter::NearestMipmapLinear,
        gltf::texture::MinFilter::LinearMipmapLinear => crate::MinFilter::LinearMipmapLinear,
    });

    let wrap_s = convert_wrapping_mode(sampler.wrap_s());
    let wrap_t = convert_wrapping_mode(sampler.wrap_t());

    crate::Sampler {
        mag_filter,
        min_filter,
        wrap_s,
        wrap_t,
        name: sampler.name().map(|s| s.to_string()),
    }
}

#[must_use]
const fn convert_alpha_mode(mode: gltf::material::AlphaMode) -> crate::AlphaMode {
    match mode {
        gltf::material::AlphaMode::Opaque => crate::AlphaMode::Opaque,
        gltf::material::AlphaMode::Mask => crate::AlphaMode::Mask,
        gltf::material::AlphaMode::Blend => crate::AlphaMode::Blend,
    }
}

#[must_use]
const fn convert_wrapping_mode(mode: gltf::texture::WrappingMode) -> crate::WrappingMode {
    match mode {
        gltf::texture::WrappingMode::ClampToEdge => crate::WrappingMode::ClampToEdge,
        gltf::texture::WrappingMode::MirroredRepeat => crate::WrappingMode::MirroredRepeat,
        gltf::texture::WrappingMode::Repeat => crate::WrappingMode::Repeat,
    }
}

#[must_use]
const fn convert_mode(mode: gltf::mesh::Mode) -> crate::RenderMode {
    match mode {
        gltf::mesh::Mode::Points => crate::RenderMode::Points,
        gltf::mesh::Mode::Lines => crate::RenderMode::Lines,
        gltf::mesh::Mode::LineLoop => crate::RenderMode::LineLoop,
        gltf::mesh::Mode::LineStrip => crate::RenderMode::LineStrip,
        gltf::mesh::Mode::Triangles => crate::RenderMode::Triangles,
        gltf::mesh::Mode::TriangleStrip => crate::RenderMode::TriangleStrip,
        gltf::mesh::Mode::TriangleFan => crate::RenderMode::TriangleFan,
    }
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
    for (i, primitive) in mesh.primitives().enumerate() {
        log::debug!(
            "         Loading Mesh Primtive {}/{}",
            i + 1,
            mesh.primitives().len()
        );
        let (vertices, indices) = load_primitive(buffer_data, &primitive);
        meshes.push(crate::Mesh {
            vertices,
            indices,
            mode: convert_mode(primitive.mode()),
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
            color: None,
            tex_coord: None,
            normal: None,
        })
        .collect();

    if let Some(normal_attribute) = reader.read_normals() {
        for (normal_index, normal) in normal_attribute.enumerate() {
            vertices[normal_index].normal = Some(normal);
        }
    }

    if let Some(color_attribute) = reader.read_colors(0).map(|v| v.into_rgba_f32()) {
        for (color_index, color) in color_attribute.enumerate() {
            vertices[color_index].color = Some(color);
        }
    }

    if let Some(tex_coord_attribute) = reader.read_tex_coords(0).map(|v| v.into_f32()) {
        for (tex_coord_index, tex_coord) in tex_coord_attribute.enumerate() {
            vertices[tex_coord_index].tex_coord = Some(tex_coord);
        }
    }

    let indices = reader.read_indices().map(|indcies| match indcies {
        gltf::mesh::util::ReadIndices::U8(indices) => Indices::U8(indices.collect::<Vec<_>>()),
        gltf::mesh::util::ReadIndices::U16(indices) => Indices::U16(indices.collect::<Vec<_>>()),
        gltf::mesh::util::ReadIndices::U32(indices) => Indices::U32(indices.collect::<Vec<_>>()),
    });

    (vertices, indices)
}
