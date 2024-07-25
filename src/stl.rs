use std::{fs::File, path::Path};

use crate::{Model3D, ModelError, Vertex};

pub fn load(path: &Path) -> Result<Model3D, ModelError> {
    let stl = stl_io::read_stl(&mut File::open(path).unwrap()).unwrap();

    let mut vertices = Vec::new();
    for face in stl.faces {
        let normal = [face.normal[0], face.normal[1], face.normal[2]];
        let pos1 = stl.vertices[face.vertices[0]];
        let pos2 = stl.vertices[face.vertices[1]];
        let pos3 = stl.vertices[face.vertices[2]];

        // Every face (triangle) has 3 Vertices
        let v1 = Vertex {
            position: [pos1[0], pos1[1], pos1[2]],
            tex_coord: None,
            normal: Some(normal),
        };
        let v2 = Vertex {
            position: [pos2[0], pos2[1], pos2[2]],
            tex_coord: None,
            normal: Some(normal),
        };
        let v3 = Vertex {
            position: [pos3[0], pos3[1], pos3[2]],
            tex_coord: None,
            normal: Some(normal),
        };

        vertices.push(v1);
        vertices.push(v2);
        vertices.push(v3);
    }
    let mesh = crate::Mesh {
        vertices,
        indices: None,
        material_index: None,
        name: None,
    };

    Ok(Model3D {
        meshes: vec![mesh],
        materials: vec![],
        format: crate::ModelFormat::STL,
    })
}
