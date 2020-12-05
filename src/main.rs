mod vec3;

extern crate pbr;
use pbr::ProgressBar;
extern crate float_cmp;
use crate::vec3::Color;
use std::error::Error;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    // Image
    let image_width = 256;
    let image_height = 256;

    // Progress bar
    let mut pb = ProgressBar::new(image_height);

    let mut file = File::create("image.ppm")?;

    // Render
    println!("Rendering...");
    writeln!(file, "P3\n{} {}\n255", image_width, image_height)?;

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let pixel_color = Color {
                x: i as f64 / (image_width - 1) as f64,
                y: j as f64 / (image_height - 1) as f64,
                z: 0.25,
            };

            writeln!(
                file,
                "{} {} {}",
                (255.999 * pixel_color.x) as i32,
                (255.999 * pixel_color.y) as i32,
                (255.999 * pixel_color.z) as i32
            )?;
        }
        pb.inc();
    }

    pb.finish_print("Render finished!");

    Ok(())
}
