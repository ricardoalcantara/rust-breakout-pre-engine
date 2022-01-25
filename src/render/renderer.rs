use super::texture::Texture;
use crate::{font::Font, shapes::rectangle::Rect};


pub struct RenderQuad {
    pub size: glam::Vec2,
    pub position: glam::Vec2,
    pub scale: glam::Vec2,
    pub rotate: f32,
    pub center_origin: bool,
    pub color: glam::Vec4,
}

pub struct RenderText<'a> {
    pub text: &'a str,
    pub font: &'a Font,
    pub size: u32,
    pub position: glam::Vec2,
    pub scale: glam::Vec2,
    pub color: glam::Vec4,
}

pub struct RenderTexture<'a> {
    pub texture: &'a Texture,
    pub rect: Option<Rect>,
    pub position: glam::Vec2,
    pub scale: glam::Vec2,
    pub rotate: f32,
    pub center_origin: bool,
    pub color: glam::Vec4,
}

pub struct RenderVertices<'a> {
    pub texture: Option<&'a Texture>,
    pub vertices: &'a [glam::Vec3; 4],
    pub color: glam::Vec4,
    pub texture_coords: &'a [glam::Vec2; 4],
}
