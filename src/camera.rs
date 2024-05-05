use std::io::{stderr, Write};

use crate::color::Color;
use crate::hit::{Hit, World};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vector::{Point3, Vec3};

pub struct Camera {
    image_width: u64,
    image_height: u64,
    camera_center: Point3, // Camera center
    pixel00_loc: Point3,   // Location of pixel 0, 0
    pixel_delta_u: Vec3,   // Offset to pixel to the right
    pixel_delta_v: Vec3,   // Offset to pixel below
}

impl Camera {
    pub fn new(image_width: u64, image_height: u64) -> Self {
        // Set up the camera.
        let focal_length = 1.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = viewport_height * image_width as f64 / image_height as f64;
        let camera_center = Point3::new(0.0, 0.0, 0.0);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Camera {
            image_height,
            image_width,
            camera_center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: &Vec<Box<dyn Hit>>) {
        // Render the image header.
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        // Render the image content.
        for y in 0..self.image_height {
            // Show some progress on stderr.
            eprint!("\rScanlines remaining: {:3}", self.image_height - y - 1);
            stderr().flush().unwrap();

            for x in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (self.pixel_delta_u * x as f64)
                    + (self.pixel_delta_v * y as f64);
                let ray_direction = pixel_center - self.camera_center;
                let ray = Ray::new(self.camera_center, ray_direction);

                let c = self.ray_color(&ray, &world);
                println!("{}", c.as_ppm_tuple());
            }
        }

        // Send a final update to stderr.
        eprintln!("\nDone.");
    }

    fn ray_color(&self, ray: &Ray, world: &World) -> Color {
        if let Some(rec) = world.hit(ray, &Interval::new(0.0, f64::INFINITY)) {
            let n = (rec.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5;
            Color::new(n.x(), n.y(), n.z())
        } else {
            let unit_direction = ray.direction().unit_vector();
            let a = (unit_direction.y() + 1.0) * 0.5;
            let r = 1.0 - a + a * 0.5;
            let g = 1.0 - a + a * 0.7;
            let b = 1.0 - a + a * 1.0;
            Color::new(r, g, b)
        }
    }
}
