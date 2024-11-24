
use nalgebra_glm::{Vec3, Vec2};
use crate::color::Color;

pub struct Fragment {
    pub position: Vec2,
    pub color: Color,
    pub depth: f32,
    pub normal: Vec3,
    pub intensity: f32,
    pub vertex_position: Vec3,
    pub uv: Option<Vec2>,
}

impl Fragment {
    pub fn new(
        position: Vec2,
        color: Color,
        depth: f32,
        normal: Vec3,
        intensity: f32,
        vertex_position: Vec3,
        uv: Option<Vec2>
    ) -> Self {  
        Fragment {
            position,
            color,
            depth,
            normal,
            intensity,
            vertex_position,
            uv
        }
    }
}