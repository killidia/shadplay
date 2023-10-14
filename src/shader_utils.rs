//!
//! Most of the boilerplate to make a custom shader work lives here.
//!
use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::*,
    sprite::Material2d,
    // window::PrimaryWindow,
};

use crate::drag_n_drop::UserAddedTexture;

pub mod common;

// ************************************ //
//                3D                    //
// ************************************ //
/// The 3D shader.
#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "a3d71c04-d054-4946-80f8-ba6cfbc90cad"]
pub struct YourShader {
    #[uniform(0)]
    pub color: Color, //RGBA
}
// 3d impl
impl Material for YourShader {
    fn fragment_shader() -> ShaderRef {
        "shaders/myshader.wgsl".into()
    }
}

// ************************************ //
//                2D                    //
// ************************************ //
/// The 2D shadertoy like shader
#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone, Deref)]
#[uuid = "f528511f-dcf2-4b0b-9522-a9df3a1a795b"]
pub struct YourShader2D {
    /// Mouse X and Mouse Y
    #[uniform(0)]
    pub(crate) mouse_pos: MousePos,
    // /// to replicate iChannel
    #[texture(1, dimension = "2d")]
    #[sampler(2)]
    pub img: Handle<Image>,
}

#[derive(ShaderType, Debug, Clone)]
pub struct MousePos {
    pub x: f32,
    pub y: f32,
}

impl Material2d for YourShader2D {
    fn fragment_shader() -> ShaderRef {
        "shaders/myshader_2d.wgsl".into()
    }
}

impl YourShader2D {
    pub fn set_current_tex(&self, idx: usize, user_added_textures: &UserAddedTexture) {
        todo!()
    }
}

// ---- ---- ---- ---- ---- ---- ---- ---- ----
// For an example of how you can pass larger ammounts of data from bevy -> yourshadercode
// You need to use this sort of structure, for example with the dotted_line.wgsl shader.
// ---- ---- ---- ---- ---- ---- ---- ---- ----
// dotted-line
#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "c74e039a-3df7-4f71-bd1d-7fe4b25a2230"]
struct DottedLineShader {
    #[uniform(0)]
    uniforms: Holder, //RGBA
}

/// Simplified holding struct to make passing across uniform(n) simpler.
#[derive(ShaderType, Default, Clone, Debug)]
struct Holder {
    tint: Color,
    /// How wide do you want the line as a % of its availablu uv space: 0.5 would be 50% of the surface of the geometry
    line_width: f32,
    /// How many segments (transparent 'cuts') do you want?
    segments: f32,
    /// How fast do you want the animation to be? set 0.0 to disable.
    phase: f32,
    /// How far spaced apart do you want these lines?
    line_spacing: f32,
}

impl Material for DottedLineShader {
    fn fragment_shader() -> ShaderRef {
        "shaders/dotted_line.wgsl".into()
    }
}
