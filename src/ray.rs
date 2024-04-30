use crate::vector::{Point3, Vec3};

/// Represents a Ray of light, as an origin point and a directional vector.
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    /// Constructs a new Ray from a point and a vector.
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            orig: origin,
            dir: direction,
        }
    }

    /// Returns the starting point (origin) of the Ray.
    pub fn origin(&self) -> Point3 {
        self.orig
    }

    /// Returns the direction vector of the Ray.
    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    /// Returns a point projected along the Ray from the Origin at `t` times the direction vector.
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ray::Ray,
        vector::{Point3, Vec3},
    };

    #[test]
    fn new_works() {
        let r = Ray::new(Point3::new(1.0, 2.0, 3.0), Vec3::new(4.0, 5.0, 6.0));
        assert_eq!(r.origin(), Point3::new(1.0, 2.0, 3.0));
        assert_eq!(r.direction(), Vec3::new(4.0, 5.0, 6.0));
    }

    #[test]
    fn at_works() {
        let r = Ray::new(Point3::new(0.1, -0.1, 0.2), Vec3::new(3.0, 4.0, 5.0));
        assert_eq!(r.at(1.0), Point3::new(3.1, 3.9, 5.2));
        assert_eq!(r.at(-1.0), Point3::new(-2.9, -4.1, -4.8));
        assert_eq!(r.at(2.0), Point3::new(6.1, 7.9, 10.2));
    }
}
