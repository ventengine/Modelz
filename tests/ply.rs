#[cfg(test)]
mod ply {
    use modelz::Model3D;

    #[test]
    fn load_ply() {
        let model_path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/cube.ply");

        let model = Model3D::from_format(model_path, &modelz::ModelFormat::PLY)
            .expect("Failed to load ply model");
        for mesh in model.meshes {
            for vert in mesh.vertices {
                println!("{:?}", vert)
            }
        }
    }
}
