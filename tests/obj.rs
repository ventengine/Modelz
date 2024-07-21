#[cfg(test)]
mod obj {
    use modelz::Model3D;

    #[test]
    fn load_obj() {
        let model_path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/cube.obj");

        let model = Model3D::from_format(model_path, modelz::ModelFormat::OBJ)
            .expect("Failed to load obj model");
        for mesh in model.meshes {
            println!("{}", mesh.name.unwrap()); // obj meshes have always a name
            for vert in mesh.vertices {
                println!("{:?}", vert)
            }
        }
        for material in model.materials {
            if let Some(diffuse) = material.diffuse_texture {
                println!("{:?}", diffuse.name)
            }
            println!("{}", material.name.unwrap_or("Unknown".to_string()))
        }
    }
}
