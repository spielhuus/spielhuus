use cgmath::Vector3;

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

impl From<Color> for u32 {
    fn from(color: Color) -> u32 {
        (color.b as u32) | ((color.g as u32) << 8) | ((color.r as u32) << 16)
    }
}

impl From<Vector3<f32>> for Color {
    fn from(color: Vector3<f32>) -> Color {
        Color::new(
            (color.x * 255.0) as u8,
            (color.y * 255.0) as u8,
            (color.z * 255.0) as u8,
            255,
        )
    }
}
