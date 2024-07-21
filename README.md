### Modelz
[![Latest Version]][crates.io]

[Latest Version]: https://img.shields.io/crates/v/modelz.svg
[crates.io]: https://crates.io/crates/modelz

Modelz is a Rust library to load various 3D file formats into a shared, in-memory representation.

### Getting Started
You can fully load an 3D Model/Scene using just one line of code
```rust
let model = Model3D::load("model.gltf").except("Failed to load");
```
You can also specify a format Modelz should use
```rust
let model = Model3D::from_format("model", ModelFormat::OBJ).except("Failed to load")
```

### What is supported

- [x] Wavefront OBJ
- [x] glTF 2.0 

### Contributing

Contributions are welcome!. If you'd like to help improve the library or add support for new formats, feel free to submit a pull request.

### Note

Modelz is inspired by the popular open-source library [Assimp](https://github.com/assimp/assimp).