use std::io::{stderr, Write};

mod color;
mod hit;
mod interval;
mod ray;
mod sphere;
mod vector;

use color::Color;
use hit::{Hit, World};
use interval::Interval;
use ray::Ray;
use sphere::Sphere;
use vector::{Point3, Vec3};

fn main() {
    const IMAGE_HEIGHT: u64 = 1200;
    const IMAGE_WIDTH: u64 = IMAGE_HEIGHT * 16 / 9; // 16:9 aspect ratio.

    // World
    let mut world = World::new();
    world.push(Box::new(Sphere::new(Point3::new(0.5, 0.5, -2.0), 0.5)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -2.0), 0.5)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -2.0), 100.0)));

    // Set up the camera.
    const FOCAL_LENGTH: f64 = 1.0;
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64;
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / IMAGE_WIDTH as f64;
    let pixel_delta_v = viewport_v / IMAGE_HEIGHT as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, FOCAL_LENGTH) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    // Render the image header.
    println!("P3");
    println!("{IMAGE_WIDTH} {IMAGE_HEIGHT}");
    println!("255");

    // Render the image content.
    for y in 0..IMAGE_HEIGHT {
        // Show some progress on stderr.
        eprint!("\rScanlines remaining: {:3}", IMAGE_HEIGHT - y - 1);
        stderr().flush().unwrap();

        for x in 0..IMAGE_WIDTH {
            let pixel_center =
                pixel00_loc + (pixel_delta_u * x as f64) + (pixel_delta_v * y as f64);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let c = ray_color(&ray, &world);
            println!("{}", c.as_ppm_tuple());
        }
    }

    // Send a final update to stderr.
    eprintln!("\nDone.");
}

fn ray_color(ray: &Ray, world: &World) -> Color {
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
