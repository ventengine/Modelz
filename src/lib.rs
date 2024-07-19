use std::path::Path;

#[cfg(feature = "gltf")]
mod gltf;
#[cfg(feature = "obj")]
mod obj;

pub struct Model3D {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,

    /// The format which was used to load the Model
    pub format: ModelFormat,
}

impl Model3D {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ModelError> {
        let format = get_format(&path)?;
        Self::from_format(path, format)
    }

    pub fn from_format<P: AsRef<Path>>(path: P, format: ModelFormat) -> Result<Self, ModelError> {
        match format {
            #[cfg(feature = "obj")]
            ModelFormat::OBJ => return obj::load(path.as_ref()),
            #[cfg(feature = "gltf")]
            ModelFormat::GLTF => return gltf::load(path.as_ref()),
        }
    }
}

pub enum ModelFormat {
    #[cfg(feature = "obj")]
    // Wavefront obj, .obj
    OBJ,
    #[cfg(feature = "gltf")]
    // gltf 2.0, .gltf | .glb
    GLTF,
}

#[derive(Debug)]
pub enum ModelError {
    // Format is not supported for you may have to enable it as a crate feature
    UnknowFormat,
    // Given file does not exist
    FileNotExists,

    ModelParsing(String),
}

fn get_format<P: AsRef<Path>>(path: &P) -> Result<ModelFormat, ModelError> {
    let path = path.as_ref();
    if !path.exists() {
        return Err(ModelError::FileNotExists);
    }

    let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
    match extension {
        #[cfg(feature = "obj")]
        "obj" => Ok(ModelFormat::OBJ),
        #[cfg(feature = "gltf")]
        "gltf" | "glb" => Ok(ModelFormat::GLTF),
        _ => Err(ModelError::UnknowFormat),
    }
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Option<Indices>,
    pub material_index: Option<usize>,
    pub name: Option<String>,
}

pub struct Material {
    pub name: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Vertex {
    position: [f32; 3],
    tex_coord: Option<[f32; 2]>,
    normal: Option<[f32; 3]>,
}

#[derive(Clone, Debug)]
pub enum Indices {
    U8(Vec<u8>),
    U16(Vec<u16>),
    U32(Vec<u32>),
}
