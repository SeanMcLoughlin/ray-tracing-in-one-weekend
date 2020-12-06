mod camera;
mod hittable;
mod material;
mod ray;
mod vec3;

extern crate pbr;
#[allow(unused_imports)]
#[macro_use]
extern crate float_cmp;
extern crate rand;

use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::vec3::{Color, Point3, Vec3};

use hittable::sphere::Sphere;
use material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};
use pbr::ProgressBar;
use rand::Rng;
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
    println!("Rendering...");

    let samples_per_pixel = 100;
    let max_depth = 50;

    let mut file = File::create("image.ppm")?;
    let mut pb = ProgressBar::new(image_height as u64);

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 0.0);

    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            material_ground,
        )),
        Box::new(Sphere::new(
            Point3::new(0.0, 0.0, -1.0),
            0.5,
            material_center,
        )),
        Box::new(Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            -0.4,
            material_left,
        )),
        Box::new(Sphere::new(
            Point3::new(1.0, 0.0, -1.0),
            0.5,
            material_right,
        )),
    ];

    let camera = Camera::new(aspect_ratio, 2.0, 1.0);

    writeln!(file, "P3\n{} {}\n255", image_width, image_height)?;

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = Color::default();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random_double()) / (image_width - 1) as f64;
                let v = (j as f64 + random_double()) / (image_height - 1) as f64;

                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, max_depth);
            }
            write_color(&mut file, pixel_color, samples_per_pixel)?;
        }
        pb.inc();
    }

    pb.finish_print("Render finished!");

    Ok(())
}

fn write_color(
    file: &mut File,
    pixel_color: Vec3,
    samples_per_pixel: i32,
) -> Result<(), Box<dyn Error>> {
    let scale = 1.0 / samples_per_pixel as f64;

    // Divide the color by the number of samples and gamma-correct for gamma=2.0
    let (r, g, b) = (
        (pixel_color.x * scale).sqrt(),
        (pixel_color.y * scale).sqrt(),
        (pixel_color.z * scale).sqrt(),
    );

    let ir = (256.0 * clamp(r, 0.0, 0.999)) as i32;
    let ig = (256.0 * clamp(g, 0.0, 0.999)) as i32;
    let ib = (256.0 * clamp(b, 0.0, 0.999)) as i32;

    writeln!(file, "{} {} {}", ir, ig, ib)?;

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

fn random_double() -> f64 {
    rand::thread_rng().gen_range(0.0, 1.0)
}
