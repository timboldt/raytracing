use std::fmt;

/// Represents an RGBA (Red-Green-Blue-Alpha) color value.
///
/// All component values must be between 0.0 and 1.0, inclusive.
#[derive(Clone, Copy, Debug, Default)]
pub struct Color {
    rgba: [f64; 4],
}

impl Color {
    /// Creates a new Color, with Alpha defaulted to 1.0.
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        assert!(r >= 0.0 && r <= 1.0);
        assert!(g >= 0.0 && g <= 1.0);
        assert!(b >= 0.0 && b <= 1.0);
        Color {
            rgba: [r, g, b, 1.0],
        }
    }

    /// Produces a string formatted according to the PPM image representation.
    ///
    /// TODO: This should really be part of a PPM image module instead of this module.
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

/// Pretty-prints a Color.
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Color({}, {}, {}, {})",
            self.rgba[0], self.rgba[1], self.rgba[2], self.rgba[3]
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;

    #[test]
    fn new_works() {
        let c = Color::new(0.1, 0.2, 0.3);
        assert_eq!(c.rgba[0], 0.1);
        assert_eq!(c.rgba[1], 0.2);
        assert_eq!(c.rgba[2], 0.3);
        assert_eq!(c.rgba[3], 1.0);
    }

    #[test]
    fn ppm_tuple_works() {
        let c = Color::new(0.1, 0.0, 1.0);
        assert_eq!(c.as_ppm_tuple(), "25 0 255");
    }

    #[test]
    fn format_works() {
        let c = Color::new(0.0, 0.5, 0.9);
        assert_eq!(format!("{c}"), "Color(0, 0.5, 0.9, 1)");
    }
}
