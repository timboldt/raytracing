use std::io::{stderr, Write};

mod color;
use color::Color;

mod vector;
use vector::Vec3;

fn main() {
    const IMAGE_WIDTH: u64 = 1024;
    const IMAGE_HEIGHT: u64 = 1024;

    println!("P3");
    println!("{IMAGE_WIDTH} {IMAGE_HEIGHT}");
    println!("255");

    for y in 0..IMAGE_HEIGHT {
        eprint!("\rScanlines remaining: {:3}", IMAGE_HEIGHT - y - 1);
        stderr().flush().unwrap();

        for x in 0..IMAGE_WIDTH {
            let c = Color::new(
                x as f64 / (IMAGE_WIDTH - 1) as f64,
                y as f64 / (IMAGE_HEIGHT - 1) as f64,
                0.5,
            );
            println!("{}", c.as_ppm_tuple());
        }
    }

    eprintln!("\nDone.");
}
