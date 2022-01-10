use std::sync::Arc;

use camera::Camera;
use geometries::{sphere::Sphere, HittableList};
use materials::{dielectric::Dielectric, diffuse::Diffuse, light::Light, metal::Metal, Material};
use utils::{ppm::PPM, vec3::*};

mod camera;
mod geometries;
mod materials;
mod mt;
mod ray;
mod scene;
mod utils;

fn main() {
    // Image
    let aspect_ratio = 16. / 9.;
    let image_width = 1920;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    let samples = 200;
    let max_depth = 50;

    //World
    // let mut world = random_scene();
    let world = random_scene();

    // let light = Arc::new(Light::new(100., 100., 100.));
    // let sun = Arc::new(Sphere::from(Point3::from(0., 0., -1.), 0.5, light.clone()));
    // world.add(sun);

    //Camera
    let from = Point3::from(-2., 2., 1.);
    let at = Point3::from(0., 0., -1.);
    let vup = Vec3::from(0., 1., 0.);
    let camera = Camera::new(from, at, vup, 90., aspect_ratio);

    let mut file = PPM::from(String::from("light_random.ppm"), image_width, image_height);

    // Iterate from top to bottom, left to right
    for j in (0..image_height).rev() {
        println!("Line: {} of {}", j, image_height);
        for i in 0..image_width {
            let mut pixel_color = Color::new();
            for _ in 0..samples {
                let x = (i as f64 + rand::random::<f64>()) / (image_width as f64 - 1.);
                let y = (j as f64 + rand::random::<f64>()) / (image_height as f64 - 1.);
                let r = camera.get_ray(x, y);
                pixel_color += r.color(&world, max_depth);
            }
            let rgb = pixel_color.to_rgb(samples);
            file.push(rgb.0, rgb.1, rgb.2);
        }
    }
    file.write().unwrap();
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let ground = Arc::new(Diffuse::new(0.5, 0.5, 0.5));
    let light = Arc::new(Light::new(1., 1., 0.8));
    let metal = Arc::new(Metal::from(Color::from(0.5, 0.5, 0.5), 0.05));
    world.add(Arc::new(Sphere::from(
        Point3::from(0., -1000., 0.),
        1000.,
        ground.clone(),
    )));

    world.add(Arc::new(Sphere::from(
        Point3::from(8., 2., -0.6),
        0.3,
        light.clone(),
    )));

    world.add(Arc::new(Sphere::from(
        Point3::from(0., 2., -0.4),
        0.3,
        light.clone(),
    )));

    world.add(Arc::new(Sphere::from(
        Point3::from(4., 1., 0.),
        1.,
        metal.clone(),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let mat_choice: f64 = rand::random();
            let center = Point3::from(
                0.9 * rand::random::<f64>() + a as f64,
                0.2,
                0.9 * rand::random::<f64>() + b as f64,
            );
            if (center - Point3::from(4., 0.2, 0.)).length() > 0.9 {
                let material: Arc<dyn Material + Sync + Send> = if mat_choice < 0.8 {
                    let tint = Vec3::random() * Vec3::random();
                    Arc::new(Diffuse::from(tint))
                } else if mat_choice < 0.95 {
                    let tint = Vec3::random();
                    let fuzz = rand::random::<f64>() / 2.;
                    Arc::new(Metal::from(tint, fuzz))
                } else {
                    Arc::new(Dielectric::new(1.5))
                };
                let sphere = Arc::new(Sphere::from(center, 0.2, material));
                world.add(sphere);
            }
        }
    }
    world
}

fn small_scene() -> HittableList {
    // let from = Point3::from(-2., 1.5, 2.);
    // let at = Point3::from(0., 0., -1.);
    // let vup = Vec3::from(0., 1., 0.);
    // let camera = Camera::new(from, at, vup, 90., aspect_ratio);

    let mut world = HittableList::new();
    let ground = Arc::new(Diffuse::new(0.8, 0.8, 0.0));
    let light = Arc::new(Light::new(1., 1., 0.8));
    let red = Arc::new(Diffuse::new(0.8, 0.2, 0.2));
    let blue = Arc::new(Metal::new(0.2, 0.2, 0.8, 0.5));
    let glass = Arc::new(Dielectric::new(1.5));

    world.add(Arc::new(Sphere::from(
        Point3::from(0., -100.5, -1.),
        100.,
        ground.clone(),
    )));

    world.add(Arc::new(Sphere::from(
        Point3::from(-1., 0., -1.),
        0.5,
        red.clone(),
    )));

    world.add(Arc::new(Sphere::from(
        Point3::from(1., 0., -1.),
        0.5,
        blue.clone(),
    )));

    world.add(Arc::new(Sphere::from(
        Point3::from(-0., 1.5, -1.),
        0.5,
        light.clone(),
    )));

    world.add(Arc::new(Sphere::from(
        Point3::from(-0., 1.4, -0.5),
        0.5,
        blue.clone(),
    )));

    world.add(Arc::new(Sphere::from(
        Point3::from(0.1, -0.2, -0.6),
        0.3,
        glass.clone(),
    )));

    world
}

fn only_light() -> HittableList {
    let mut world = HittableList::new();
    let light = Arc::new(Light::new(1., 1., 1.));
    let ground = Arc::new(Diffuse::new(0.8, 0.8, 0.0));

    world.add(Arc::new(Sphere::from(
        Point3::from(0., -100.5, -1.),
        100.,
        ground.clone(),
    )));

    world.add(Arc::new(Sphere::from(
        Point3::from(-0., 1.5, -1.),
        0.5,
        light.clone(),
    )));
    world
}
