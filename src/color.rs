use std::fmt;

#[derive(Clone, Copy, Debug, Default)]
pub struct Color {
    rgba: [f64; 4],
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color {
            rgba: [r, g, b, 0.0],
        }
    }

    pub fn as_ppm_tuple(&self) -> String {
        const SCALE: f64 = 255.999;
        format!(
            "{} {} {}",
            (SCALE * self.rgba[0]) as u64,
            (SCALE * self.rgba[1]) as u64,
            (SCALE * self.rgba[2]) as u64
        )
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Color({}, {}, {}, {})",
            self.rgba[0], self.rgba[1], self.rgba[2], self.rgba[3]
        )
    }
}
