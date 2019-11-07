use crate::Vec2f;

pub fn lerp(val: f32, from: f32, to: f32) -> f32 {
    from + (to - from) * val
}

pub fn vec2_modulo(a: Vec2f, b: Vec2f) -> Vec2f {
    Vec2f::new(a.x % b.x, a.y % b.y)
}
