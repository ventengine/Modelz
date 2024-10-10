#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![expect(clippy::single_call_fn)]
#![expect(clippy::exhaustive_enums)]
#![expect(clippy::exhaustive_structs)]

use std::path::{Path, PathBuf};

#[cfg(feature = "gltf")]
mod gltf;
#[cfg(feature = "obj")]
mod obj;
#[cfg(feature = "stl")]
mod ply;
#[cfg(feature = "ply")]
mod stl;

pub struct Model3D {
    /// All meshes the Model has.
    ///
    /// Some 3D Formats do not have multiple meshes and have just vertices, In this case there will be one Mesh with all the Vertices
    pub meshes: Vec<Mesh>,
    /// All Materials the Model has.
    ///
    /// Some 3D Formats do not support Materials/Textures, In this case the Vec will be empty
    pub materials: Vec<Material>,

    /// The format which was used to load the Model
    pub format: ModelFormat,
}

impl Model3D {
    /// Load an Full 3D Model from the Given File extension
    ///
    /// # Examples
    ///
    /// ```
    /// use modelz::Model3D;
    ///
    /// let model = Model3D::load("model.gltf");
    ///
    ///  for mesh in model.meshes {
    ///     println!("{}", mesh.name.unwrap());
    ///     for vert in mesh.vertices {
    ///        println!("{:?}", vert)
    ///     }
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an Error is loading the Model was unsuccessful
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ModelError> {
        let format = get_format(&path)?;
        Self::from_format(path, &format)
    }

    /// Load an Full 3D Model from the Given `ModelFormat`
    ///
    /// # Examples
    ///
    /// ```
    /// use modelz::Model3D;
    ///
    /// let model = Model3D::from_format("model.gltf", ModelFormat::GLTF);
    ///
    /// let model = Model3D::from_format("model", ModelFormat::GLTF);
    /// ```
    /// # Errors
    ///
    /// Returns an Error is loading the Model was unsuccessful
    pub fn from_format<P: AsRef<Path>>(path: P, format: &ModelFormat) -> Result<Self, ModelError> {
        match format {
            #[cfg(feature = "obj")]
            ModelFormat::OBJ => obj::load(path.as_ref()),
            #[cfg(feature = "gltf")]
            ModelFormat::GLTF => gltf::load(path.as_ref()),
            #[cfg(feature = "stl")]
            ModelFormat::STL => stl::load(path.as_ref()),
            #[cfg(feature = "ply")]
            ModelFormat::PLY => ply::load(path.as_ref()),
        }
    }
}

#[non_exhaustive]
/// `ModelFormat` represents the 3D Format being used to Load an File
pub enum ModelFormat {
    #[cfg(feature = "obj")]
    // Wavefront obj, .obj
    OBJ,
    #[cfg(feature = "gltf")]
    // gltf 2.0, .gltf | .glb
    GLTF,
    #[cfg(feature = "stl")]
    // STL .stl
    STL,
    #[cfg(feature = "ply")]
    // Polygon File Format .ply
    PLY,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ModelError {
    // Format is not supported for you may have to enable it as a crate feature
    UnknowFormat,
    // Given file does not exist
    FileNotExists,
    // Failed to open file
    OpenFile(String),
    // Error while loading general 3D File
    ModelParsing(String),
    // Error loading Material
    MaterialLoad(String),
}

fn get_format<P: AsRef<Path>>(path: &P) -> Result<ModelFormat, ModelError> {
    let path = path.as_ref();
    if !path.exists() {
        return Err(ModelError::FileNotExists);
    }

    let extension = path
        .extension()
        .and_then(|ext| return ext.to_str())
        .expect("Failed to get File extension");
    match extension {
        #[cfg(feature = "obj")]
        "obj" => Ok(ModelFormat::OBJ),
        #[cfg(feature = "gltf")]
        "gltf" | "glb" => Ok(ModelFormat::GLTF),
        #[cfg(feature = "stl")]
        "stl" => Ok(ModelFormat::STL),
        _ => Err(ModelError::UnknowFormat),
    }
}

pub struct Mesh {
    /// All the Vertices the Mesh has.
    pub vertices: Vec<Vertex>,
    /// All the Indices the Mesh has.
    pub indices: Option<Indices>,
    /// The Render Mode that should be used to Render the Mesh
    pub mode: RenderMode,
    /// The index from the Vec Materials Vec in `Model3D`
    ///
    /// # Examples
    ///
    /// ```
    /// let material = model.materials[mesh.material_index];
    /// ```
    pub material_index: Option<usize>,
    /// Name of the Mesh.
    ///
    /// Some File Formats do not support Mesh names, In this case this will be `None`
    pub name: Option<String>,
}

#[non_exhaustive]
pub struct Material {
    /// The optional diffuse Texture
    pub diffuse_texture: Option<Texture>,
    /// The alpha rendering mode of the material.  The material's alpha rendering
    /// mode enumeration specifying the interpretation of the alpha value of the main
    /// factor and texture.
    pub alpha_mode: AlphaMode,
    ///  The Alpha cutoff value of the material.
    pub alpha_cutoff: Option<f32>,
    /// Specifies whether the material is double-sided.
    ///
    /// When disabled, back-face culling is enabled
    /// When enabled, back-face culling is disabled
    pub double_sided: bool,
    /// The Base color of the Material.
    ///
    /// Usally used to mutiple the diffuse texture
    ///
    /// Some File Formats do not support Material names, In this case this will be `None`
    /// # Examples
    ///
    /// ```
    /// vec4 texture = texture(texture_diffuse, tex_coord) * material.base_color;
    /// ```
    pub base_color: Option<[f32; 4]>,
    /// Name of the Material.
    ///
    /// Some File Formats do not support Material names, In this case this will be `None`
    pub name: Option<String>,
}

pub struct Texture {
    /// The image from the `image` crate, Which is loaded into RAM
    pub image: Image,
    /// Sampler which beining used on the Image
    pub sampler: Sampler,
    /// Name of the Texture.
    ///
    /// Some File Formats do not support Texture names, In this case this will be `None`
    pub name: Option<String>,
}

pub enum Image {
    Memory {
        data: Vec<u8>,
        mime_type: Option<String>,
    },
    Path {
        path: PathBuf,
        mime_type: Option<String>,
    },
}

#[derive(Default)]
pub struct Sampler {
    pub mag_filter: Option<MagFilter>,
    pub min_filter: Option<MinFilter>,
    pub wrap_s: WrappingMode,
    pub wrap_t: WrappingMode,
    pub name: Option<String>,
}

/// Mag Filter
///
/// # Rendering
///
/// Vulkan: Corresponds to `vk::Filter`
/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFilter.html>
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MagFilter {
    /// Corresponds to `GL_NEAREST` or `vk::Filter::NEAREST`.
    Nearest = 1,

    /// Corresponds to `GL_LINEAR` or `vk::Filter::LINEAR`.
    Linear,
}

/// Mag Filter
///
/// # Rendering
///
/// Vulkan: Corresponds to `vk::Filter` & `vk::SamplerMipmapMode`
/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFilter.html>
/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerMipmapMode.html>
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MinFilter {
    /// Corresponds to `GL_NEAREST` or `vk::Filter::NEAREST`.
    Nearest = 1,

    /// Corresponds to `GL_LINEAR` or `vk::Filter::LINEAR`.
    Linear,

    /// Corresponds to `GL_NEAREST_MIPMAP_NEAREST` or (`vk::Filter::NEAREST`, `vk::SamplerMipmapMode::NEAREST`).
    NearestMipmapNearest,

    /// Corresponds to `GL_LINEAR_MIPMAP_NEAREST` or (`vk::Filter::LINEAR`, `vk::SamplerMipmapMode::NEAREST`).
    LinearMipmapNearest,

    /// Corresponds to `GL_NEAREST_MIPMAP_LINEAR` or (`vk::Filter::NEAREST`, `vk::SamplerMipmapMode::LINEAR`).
    NearestMipmapLinear,

    /// Corresponds to `GL_LINEAR_MIPMAP_LINEAR` or (`vk::Filter::LINEAR`, `vk::SamplerMipmapMode::LINEAR`).
    LinearMipmapLinear,
}

/// Wrapping Mode
///
/// # Rendering
///
/// Vulkan: Corresponds to `vk::SamplerAddressMode`
/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerAddressMode.html>
#[derive(Default, Clone, Copy, Debug, Eq, PartialEq)]
pub enum WrappingMode {
    /// Corresponds to `GL_CLAMP_TO_EDGE` or `vk::SamplerAddressMode::CLAMP_TO_EDGE`.
    ClampToEdge = 1,

    /// Corresponds to [`GL_MIRRORED_REPEAT`] or [`vk::SamplerAddressMode::MIRRORED_REPEAT`].
    MirroredRepeat,

    /// Corresponds to `GL_REPEAT` or `vk::SamplerAddressMode::REPEAT`.
    #[default]
    Repeat,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AlphaMode {
    /// The alpha value is ignored and the rendered output is fully opaque.
    Opaque = 1,

    /// The rendered output is either fully opaque or fully transparent depending on
    /// the alpha value and the specified alpha cutoff value.
    Mask,

    /// The alpha value is used, to determine the transparency of the rendered output.
    /// The alpha cutoff value is ignored.
    Blend,
}

/// The type of primitives to render.
///
/// # Rendering
///
/// Vulkan: Corresponds to `vk::PrimitiveTopology`
/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPrimitiveTopology.html>
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RenderMode {
    /// Corresponds to `GL_POINTS` or `vk::PrimitiveTopology::POINT_LIST`.
    Points = 1,

    /// Corresponds to `GL_LINES` or `vk::PrimitiveTopology::LINE_LIST`.
    Lines,

    /// Corresponds to [`GL_LINE_LOOP`] or [`vk::PrimitiveTopology::LINE_LIST`].
    LineLoop,

    /// Corresponds to `GL_LINE_STRIP` or `vk::PrimitiveTopology::LINE_STRIP`.
    LineStrip,

    /// Corresponds to `GL_TRIANGLES` or `vk::PrimitiveTopology::TRIANGLE_LIST`.
    Triangles,

    /// Corresponds to `GL_TRIANGLE_STRIP`or `vk::PrimitiveTopology::TRIANGLE_STRIP`.
    TriangleStrip,

    /// Corresponds to `GL_TRIANGLE_FAN` or `vk::PrimitiveTopology::TRIANGLE_FAN`.
    TriangleFan,
}

#[derive(Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: Option<[f32; 4]>, // rgba f32
    pub tex_coord: Option<[f32; 2]>,
    pub normal: Option<[f32; 3]>,
}

#[derive(Clone, Debug)]
/// Indicies
///
/// # Rendering
///
/// Vulkan: Corresponds to `vk::IndexType`
/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkIndexType.html>
pub enum Indices {
    U8(Vec<u8>),
    U16(Vec<u16>),
    U32(Vec<u32>),
}
