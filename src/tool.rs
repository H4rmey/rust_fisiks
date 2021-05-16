
#![allow(non_snake_case)]
pub mod Tool
{
    use cgmath::Vector2;

    pub fn new_vec2_with_angle(len: f64, angle: f64 /*in radians*/) -> Vector2<f64>
    {
        Vector2::new(angle.cos() * len, angle.sin() * len)    
    }

    pub fn normalize_between(
            num     : f64,
            from_min: f64,
            from_max: f64,
            to_min  : f64,  
            to_max  : f64,  
        ) -> f64
    {
        to_min + (num-from_min)/(from_max - from_min)*(to_max-to_min)
    }
}