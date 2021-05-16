
#![allow(non_snake_case)]
pub mod Tool
{
    use cgmath::Vector2;

    pub fn new_vec2_with_angle(len: f64, angle: f64 /*in radians*/) -> Vector2<f64>
    {
        Vector2::new(angle.cos() * len, angle.sin() * len)    
    }
}