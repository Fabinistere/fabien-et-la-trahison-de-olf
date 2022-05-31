pub fn linear(t: f32) -> f32 {
    t
}

pub fn ease_in_cubic(t: f32) -> f32 {
    t.powi(3)
}

pub fn ease_in_sine(t: f32) -> f32 {
    1.0 - ((t * std::f32::consts::PI) / 2.0).cos()
}

pub fn ease_out_sine(t: f32) -> f32 {
    ((t * std::f32::consts::PI) / 2.0).sin()
}
