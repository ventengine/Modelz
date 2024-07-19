use std::path::Path;

use crate::{Model3D, ModelError, Vertex};

pub fn load(path: &Path) -> Result<Model3D, ModelError> {
    let (models, materials) = match tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS) {
        Ok(r) => r,
        Err(e) => return Err(ModelError::ModelParsing(format!("{}", e))),
    };

    let path = path.parent().unwrap_or_else(|| Path::new("./"));

    let mut final_materials = Vec::new();

    let materials = match materials {
        Ok(r) => r,
        Err(e) => {
            return Err(ModelError::MaterialLoad(format!(
                "Failed to load MTL file, {}",
                e
            )))
        }
    };

    for material in materials {
        final_materials.push(load_material(&material, path)?)
    }

    let mut meshes = Vec::new();

    for model in models {
        let mesh = model.mesh;
        let vertices = load_mesh(&mesh);
        meshes.push(crate::Mesh {
            vertices,
            indices: {
                if mesh.indices.is_empty() {
                    None
                } else {
                    Some(crate::Indices::U32(mesh.indices.clone())) // OBJ only has u32 indices
                }
            },
            name: Some(model.name),
            material_index: mesh.material_id,
        })
    }

    Ok(Model3D {
        meshes,
        materials: final_materials,
        format: crate::ModelFormat::OBJ,
    })
}

fn load_material(
    material: &tobj::Material,
    model_dir: &Path,
) -> Result<crate::Material, ModelError> {
    let base_color = material.diffuse.as_ref().map(|d| [d[0], d[1], d[2], 1.0]);

    let diffuse_texture = if let Some(texture) = &material.diffuse_texture {
        match image::open(model_dir.join(texture)) {
            Ok(image) => Some(image),
            Err(err) => return Err(ModelError::MaterialLoad(err.to_string())),
        }
    } else {
        None
    };
    Ok(crate::Material {
        diffuse_texture,
        base_color,
        name: Some(material.name.clone()),
    })
}

fn load_mesh(mesh: &tobj::Mesh) -> Vec<Vertex> {
    (0..mesh.positions.len() / 3)
        .map(|i| Vertex {
            position: [
                mesh.positions[i * 3],
                mesh.positions[i * 3 + 1],
                mesh.positions[i * 3 + 2],
            ],
            tex_coord: {
                if mesh.texcoords.is_empty() {
                    None
                } else {
                    Some([mesh.texcoords[i * 2], mesh.texcoords[i * 2 + 1]])
                }
            },
            normal: {
                if mesh.normals.is_empty() {
                    None
                } else {
                    Some([
                        mesh.normals[i * 3],
                        mesh.normals[i * 3 + 1],
                        mesh.normals[i * 3 + 2],
                    ])
                }
            },
        })
        .collect::<Vec<_>>()
}
