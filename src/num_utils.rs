pub fn lerp(val: f32, from: f32, to: f32) -> f32 {
    from + (to - from) * val
}