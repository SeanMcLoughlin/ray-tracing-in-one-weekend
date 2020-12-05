mod ray;
mod vec3;

extern crate pbr;
#[allow(unused_imports)]
#[macro_use]
extern crate float_cmp;

use crate::ray::Ray;
use crate::vec3::{Color, Point3, Vec3};
use pbr::ProgressBar;
use std::error::Error;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    render_image(aspect_ratio, image_width, image_height)
}

fn render_image(
    aspect_ratio: f64,
    image_width: i32,
    image_height: i32,
) -> Result<(), Box<dyn Error>> {
    let mut file = File::create("image.ppm")?;
    let mut pb = ProgressBar::new(image_height as u64);

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;
    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    println!("Rendering...");
    writeln!(file, "P3\n{} {}\n255", image_width, image_height)?;

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;

            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&ray);

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

fn ray_color(ray: &Ray) -> Color {
    let unit_directions = ray.direction.unit_vector();
    let t = 0.5 * (unit_directions.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
