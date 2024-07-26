use std::{fs::File, path::Path};

extern crate ply_rs;
use ply_rs::{
    self as ply,
    ply::{Property, PropertyAccess},
};

use crate::{Model3D, ModelError};

#[derive(Debug, Default, Clone, Copy)]
struct Vertex {
    x: f32,
    y: f32,
    z: f32,
    x_norm: Option<f32>,
    y_norm: Option<f32>,
    z_norm: Option<f32>,
    tex_x: Option<f32>,
    tex_y: Option<f32>,
}

impl PropertyAccess for Vertex {
    fn new() -> Self {
        Vertex {
            ..Default::default()
        }
    }
    fn set_property(&mut self, key: String, property: Property) {
        match (key.as_ref(), property) {
            ("x", Property::Float(v)) => self.x = v,
            ("y", Property::Float(v)) => self.y = v,
            ("z", Property::Float(v)) => self.z = v,

            ("nx", Property::Float(v)) => self.x_norm = Some(v),
            ("ny", Property::Float(v)) => self.y_norm = Some(v),
            ("nz", Property::Float(v)) => self.z_norm = Some(v),
            // NOTE: Blender3D exports texture coordinates as s,t tuples
            ("u" | "s" | "tx" | "texture_u", Property::Float(v)) => self.tex_x = Some(v),
            ("v" | "t" | "ty" | "texture_v", Property::Float(v)) => self.tex_y = Some(v),
            (k, _) => eprintln!("Vertex: Unexpected key/value combination: key: {}", k),
        }
    }
}

#[derive(Debug)]
struct Face {
    vertex_index: Vec<u32>,
}

impl PropertyAccess for Face {
    fn new() -> Self {
        Face {
            vertex_index: Vec::new(),
        }
    }
    fn set_property(&mut self, key: String, property: Property) {
        match (key.as_ref(), property) {
            ("vertex_index" | "vertex_indices", Property::ListUInt(vec)) => self.vertex_index = vec,
            (k, _) => eprintln!("Face: Unexpected key/value combination: key: {}", k),
        }
    }
}

pub fn load(path: &Path) -> Result<Model3D, ModelError> {
    let mut file = File::open(path).map_err(|e| ModelError::OpenFile(e.to_string()))?;
    let mut reader = std::io::BufReader::new(&mut file);

    // Create a parser for each struct. Parsers are cheap objects.
    let vertex_parser = ply::parser::Parser::<Vertex>::new();
    let face_parser = ply::parser::Parser::<Face>::new();

    // lets first consume the header
    // We also could use `face_parser`, The configuration is a parser's only state.
    // The reading position only depends on `f`.
    let header = vertex_parser.read_header(&mut reader).unwrap();

    // Depending on the header, read the data into our structs..
    let mut vertex_list = Vec::new();
    let mut face_list = Vec::new();
    for (_ignore_key, element) in &header.elements {
        // we could also just parse them in sequence, but the file format might change
        match element.name.as_ref() {
            "vertex" => {
                vertex_list = vertex_parser
                    .read_payload_for_element(&mut reader, element, &header)
                    .unwrap();
            }
            "face" => {
                face_list = face_parser
                    .read_payload_for_element(&mut reader, element, &header)
                    .unwrap();
            }
            _ => panic!("Enexpeced element!"),
        }
    }
    let mut vertices = Vec::new();
    for face in face_list {
        // Every face (triangle) has 3 Vertices
        let vertex = vertex_list[face.vertex_index[0] as usize];
        let vertex1 = vertex_list[face.vertex_index[1] as usize];
        let vertex2 = vertex_list[face.vertex_index[2] as usize];

        let mut normal = None;
        let mut tex_coord = None;
        if let Some(x) = vertex.x_norm {
            if let Some(y) = vertex.z_norm {
                if let Some(z) = vertex.z_norm {
                    normal = Some([x, y, z])
                }
            }
        }
        if let Some(x) = vertex.tex_x {
            if let Some(y) = vertex.tex_y {
                tex_coord = Some([x, y])
            }
        }

        let v1 = crate::Vertex {
            position: [vertex.x, vertex.y, vertex.z],
            tex_coord,
            color: None,
            normal,
        };
        if let Some(x) = vertex1.x_norm {
            if let Some(y) = vertex1.z_norm {
                if let Some(z) = vertex1.z_norm {
                    normal = Some([x, y, z])
                }
            }
        }
        if let Some(x) = vertex1.tex_x {
            if let Some(y) = vertex1.tex_y {
                tex_coord = Some([x, y])
            }
        }
        let v2 = crate::Vertex {
            position: [vertex1.x, vertex1.y, vertex1.z],
            tex_coord,
            color: None,
            normal,
        };
        if let Some(x) = vertex2.x_norm {
            if let Some(y) = vertex2.z_norm {
                if let Some(z) = vertex2.z_norm {
                    normal = Some([x, y, z])
                }
            }
        }
        if let Some(x) = vertex2.tex_x {
            if let Some(y) = vertex2.tex_y {
                tex_coord = Some([x, y])
            }
        }
        let v3 = crate::Vertex {
            position: [vertex2.x, vertex2.y, vertex2.z],
            tex_coord,
            color: None,
            normal,
        };

        vertices.push(v1);
        vertices.push(v2);
        vertices.push(v3);
    }
    let mesh = crate::Mesh {
        vertices,
        indices: None,
        material_index: None,
        mode: crate::RenderMode::TriangleFan,
        name: None,
    };

    Ok(Model3D {
        meshes: vec![mesh],
        materials: vec![],
        format: crate::ModelFormat::PLY,
    })
}
