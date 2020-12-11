mod camera;
mod hittable;
mod material;
mod ray;
mod vec3;
mod world;

#[allow(unused_imports)]
#[macro_use]
extern crate float_cmp;
extern crate image;
extern crate indicatif;
extern crate rand;

use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::vec3::{Color, Point3, Vec3};

use image::{ImageBuffer, Rgb};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rand::prelude::ThreadRng;
use rand::Rng;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use std::error::Error;

#[derive(Clone, Copy)]
struct Bounds {
    width: u32,
    height: u32,
}

#[derive(Clone, Copy)]
struct RenderParams {
    samples_per_pixel: usize,
    depth: i32,
}

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    let aspect_ratio = 16.0 / 9.0;

    let bounds = Bounds {
        width: 3840,
        height: (3840.0 / aspect_ratio) as u32,
    };

    let world = world::book_cover_scene();

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
        samples_per_pixel: 500,
        depth: 50,
    };

    let pixels = render_image(bounds, &world, camera, params)?;
    write_image(pixels, bounds, params);

    Ok(())
}

fn write_image(pixels: Vec<Color>, bounds: Bounds, params: RenderParams) {
    let mut image_buf: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::new(bounds.width, bounds.height);

    for (i, pixel) in pixels.iter().enumerate() {
        let scale = 1.0 / params.samples_per_pixel as f64;

        // Divide the color by the number of samples and gamma-correct for gamma=2.0
        let (r, g, b) = (
            (pixel.x * scale).sqrt(),
            (pixel.y * scale).sqrt(),
            (pixel.z * scale).sqrt(),
        );

        let ir = (256.0 * clamp(r, 0.0, 0.999)) as u8;
        let ig = (256.0 * clamp(g, 0.0, 0.999)) as u8;
        let ib = (256.0 * clamp(b, 0.0, 0.999)) as u8;

        let x_coord = i as u32 % bounds.width;
        let y_coord = i as u32 / bounds.width;

        let pixel = image_buf.get_pixel_mut(x_coord, y_coord);
        *pixel = image::Rgb([ir, ig, ib]);
    }

    image_buf
        .save("image.png")
        .expect("Image rendered, but failed to save!");
}

fn render_image(
    bounds: Bounds,
    world: &[Box<dyn Hittable + Send + Sync>],
    camera: Camera,
    params: RenderParams,
) -> Result<Vec<Color>, Box<dyn Error + Sync + Send>> {
    let bar = ProgressBar::new(bounds.height as u64).with_style(
        ProgressStyle::default_bar()
            .template("Rendering: [{eta_precise}] {bar:40.cyan/blue} {pos:>7}/{len:} scanlines"),
    );

    let pixels = (0..bounds.height)
        .into_par_iter()
        .rev()
        .progress_with(bar)
        .flat_map(|y| {
            (0..bounds.width).into_par_iter().map(move |x| {
                let collect_sample = |rng: &mut ThreadRng| {
                    let u = (x as f64 + rng.gen_range(0.0, 1.0)) / (bounds.width - 1) as f64;
                    let v = (y as f64 + rng.gen_range(0.0, 1.0)) / (bounds.height - 1) as f64;

                    let ray = camera.get_ray(u, v);
                    ray_color(&ray, &world, params.depth)
                };

                let mut rng = rand::thread_rng();
                (0..params.samples_per_pixel)
                    .map(|_| collect_sample(&mut rng))
                    .sum()
            })
        })
        .collect();
    Ok(pixels)
}

fn ray_color(ray: &Ray, world: &[Box<dyn Hittable + Send + Sync>], depth: i32) -> Color {
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
