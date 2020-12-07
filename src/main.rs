mod camera;
mod hittable;
mod material;
mod ray;
mod vec3;
mod world;

extern crate pbr;
#[allow(unused_imports)]
#[macro_use]
extern crate float_cmp;
extern crate image;
extern crate rand;

use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::vec3::{Color, Point3, Vec3};

use image::{imageops, ImageBuffer, Rgb};
use pbr::ProgressBar;
use rand::Rng;
use std::error::Error;

struct Bounds {
    width: u32,
    height: u32,
}

struct RenderParams {
    samples_per_pixel: usize,
    depth: i32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    render_image(
        aspect_ratio,
        Bounds {
            width: image_width,
            height: image_height,
        },
    )
}

fn render_image(aspect_ratio: f64, bounds: Bounds) -> Result<(), Box<dyn Error>> {
    println!("Rendering...");

    let world = world::test_world();

    let camera = Camera::new(
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.1,
        10.0,
    );

    let params = RenderParams {
        samples_per_pixel: 100,
        depth: 50,
    };

    let mut image_buf: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::new(bounds.width, bounds.height);

    render_band(bounds, &mut image_buf, &world, camera, params)?;

    image_buf = imageops::rotate180(&image_buf);

    image_buf
        .save("image.png")
        .expect("Image rendered, but failed to save!");

    Ok(())
}

fn render_band(
    bounds: Bounds,
    buffer: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    world: &[Box<dyn Hittable>],
    camera: Camera,
    params: RenderParams,
) -> Result<(), Box<dyn Error>> {
    let mut pb = ProgressBar::new(bounds.height as u64);
    let mut rng = rand::thread_rng();

    for x in 0..bounds.width {
        for y in 0..bounds.height {
            let mut pixel_color = Color::default();
            for _ in 0..params.samples_per_pixel {
                let u = (x as f64 + rng.gen_range(0.0, 1.0)) / (bounds.width - 1) as f64;
                let v = (y as f64 + rng.gen_range(0.0, 1.0)) / (bounds.height - 1) as f64;

                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, params.depth);
            }

            let scale = 1.0 / params.samples_per_pixel as f64;

            // Divide the color by the number of samples and gamma-correct for gamma=2.0
            let (r, g, b) = (
                (pixel_color.x * scale).sqrt(),
                (pixel_color.y * scale).sqrt(),
                (pixel_color.z * scale).sqrt(),
            );

            let ir = (256.0 * clamp(r, 0.0, 0.999)) as u8;
            let ig = (256.0 * clamp(g, 0.0, 0.999)) as u8;
            let ib = (256.0 * clamp(b, 0.0, 0.999)) as u8;

            let pixel = buffer.get_pixel_mut(x, y);
            *pixel = image::Rgb([ir, ig, ib]);
        }
        pb.inc();
    }
    pb.finish_print("Render finished!");

    Ok(())
}

fn ray_color(ray: &Ray, world: &[Box<dyn Hittable>], depth: i32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    // This is the recursion base-case.
    if depth <= 0 {
        return Color::default();
    }

    if let Some(record) = world.hit(ray, 0.001, f64::INFINITY) {
        if let Some(scatter) = record.material.scatter(*ray, &record) {
            return scatter.attenuation * ray_color(&scatter.ray, world, depth - 1);
        }
    }

    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn clamp<T: PartialOrd>(input: T, min: T, max: T) -> T {
    assert!(min <= max);
    if input < min {
        return min;
    }
    if input > max {
        return max;
    }
    input
}
