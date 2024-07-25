#[cfg(test)]
mod stl {
    use modelz::Model3D;

    #[test]
    fn load_stl() {
        let model_path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/cube.stl");

        let model = Model3D::from_format(model_path, modelz::ModelFormat::STL)
            .expect("Failed to load stl model");
        for mesh in model.meshes {
            for vert in mesh.vertices {
                println!("{:?}", vert)
            }
        }
    }
}
