mod camera;
mod color;
mod hit;
mod interval;
mod ray;
mod sphere;
mod vector;

use camera::Camera;
use hit::World;
use sphere::Sphere;
use vector::Point3;

const IMAGE_WIDTH: u64 = IMAGE_HEIGHT * 16 / 9; // 16:9 aspect ratio.
const IMAGE_HEIGHT: u64 = 1200;

fn main() {
    // Set up the world.
    let mut world = World::new();
    world.push(Box::new(Sphere::new(Point3::new(0.0, 1.05, -3.0), 0.3)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.5, -3.0), 0.4)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -0.1, -3.0), 0.5)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -2.0), 100.0)));

    // Render it.
    let camera = Camera::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    camera.render(&world);
}

