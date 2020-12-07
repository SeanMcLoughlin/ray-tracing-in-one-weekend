use crate::hittable::sphere::Sphere;
use crate::hittable::Hittable;
use crate::material::dielectric::Dielectric;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use crate::vec3::{Color, Point3};
use rand::Rng;

#[allow(dead_code)]
pub fn test_world() -> Vec<Box<dyn Hittable>> {
    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 0.0);
    vec![
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
            0.5,
            material_left,
        )),
        Box::new(Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            -0.45,
            material_left,
        )),
        Box::new(Sphere::new(
            Point3::new(1.0, 0.0, -1.0),
            0.5,
            material_right,
        )),
    ]
}

#[allow(dead_code)]
pub fn book_cover_scene() -> Vec<Box<dyn Hittable>> {
    let mut scene: Vec<Box<dyn Hittable>> = Vec::new();

    let mut rng = rand::thread_rng();

    // Ground
    scene.push(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(Color::new(0.5, 0.5, 0.5)),
    )));

    // Scattered spheres
    for a in -11..11 {
        for b in -11..11 {
            let a = a as f64;
            let b = b as f64;

            let choose_mat = rng.gen_range(0.0, 1.0);
            let center = Point3::new(
                a + 0.9 * rng.gen_range(0.0, 1.0),
                0.2,
                b + 0.9 * rng.gen_range(0.0, 1.0),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let albedo = Color::random(0.0, 1.0) * Color::random(0.0, 1.0);
                scene.push(Box::new(Sphere::new(center, 0.2, Lambertian::new(albedo))));
            } else if choose_mat < 0.95 {
                let albedo = Color::random(0.5, 1.0);
                let fuzz = rng.gen_range(0.0, 0.5);
                scene.push(Box::new(Sphere::new(center, 0.2, Metal::new(albedo, fuzz))));
            } else {
                scene.push(Box::new(Sphere::new(center, 0.2, Dielectric::new(1.5))));
            };
        }
    }

    // Three large main spheres
    scene.push(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Dielectric::new(1.5),
    )));

    scene.push(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Lambertian::new(Color::new(0.4, 0.2, 0.1)),
    )));

    scene.push(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Metal::new(Color::new(0.7, 0.6, 0.5), 0.0),
    )));

    scene
}
