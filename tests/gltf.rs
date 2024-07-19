#[cfg(test)]
mod gltf {
    use modelz::Model3D;

    #[test]
    fn load_gltf() {
        let model_path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/cube.glb");

        let model = Model3D::from_format(model_path, modelz::ModelFormat::GLTF)
            .expect("Failed to load gltf model");
        for mesh in model.meshes {
            println!("{}", mesh.name.unwrap()); // obj meshes have always a name
            for vert in mesh.vertices {
                println!("{:?}", vert)
            }
        }
        for material in model.materials {
            println!("{}", material.name.unwrap_or("Unknown".to_string()))
        }
    }
}
