extern crate pbr;
use pbr::ProgressBar;
use std::fs::File;
use std::error::Error;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>>{

    // Image
    let image_width = 256;
    let image_height = 256;

    // Progress bar
    let mut pb = ProgressBar::new(image_height);

    let mut file = File::create("image.ppm")?;

    // Render
    println!("Rendering...");
    write!(file, "P3\n{} {}\n255\n", image_width, image_height)?;

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let r: f64 = i as f64 / (image_width - 1) as f64;
            let g: f64 = j as f64 / (image_height - 1) as f64;
            let b: f64 = 0.25;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            write!(file, "{} {} {}\n", ir, ig, ib)?;
        }
        pb.inc();
    }

    pb.finish_print("Render finished!");

    Ok(())
}
