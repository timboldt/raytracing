use std::io::{stderr, Write};

fn main() {
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = 256;

    println!("P3");
    println!("{IMAGE_WIDTH} {IMAGE_HEIGHT}");
    println!("255");

    for y in 0..IMAGE_HEIGHT {
        eprint!("\rScanlines remaining: {:3}", IMAGE_HEIGHT - y - 1);
        stderr().flush().unwrap();

        for x in 0..IMAGE_WIDTH {
            let r = x as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = y as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.0;

            let almost_256 = 255.999;
            let ir = (almost_256 * r) as u64;
            let ig = (almost_256 * g) as u64;
            let ib = (almost_256 * b) as u64;

            println!("{ir} {ig} {ib}");
        }
    }
    eprintln!("\nDone.");
}
