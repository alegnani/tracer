use std::sync::Arc;

use camera::Camera;
use geometries::{rectangle::Rectangle, sphere::Sphere, triangle::Triangle, HittableList};
use materials::{dielectric::Dielectric, diffuse::Diffuse, light::Light, metal::Metal, Material};
use scene::{Config, Scene};
use utils::vec3::*;

mod camera;
mod geometries;
mod materials;
mod ray;
mod scene;
mod utils;

fn main() {
    rectangle_test();
    //triangle_test();
    // let mut scene = Scene::new();

    // let from = Point3::from(-2., 1.5, 2.);
    // let at = Point3::from(0., 0., -1.);
    // let vup = Vec3::from(0., 1., 0.);
    // let camera = Camera::new(from, at, vup, 90., 16. / 9.);

    // let world = small_scene();

    // let config = Config {
    //     name: String::from("dio"),
    //     height: 1440,
    //     samples: 200,
    //     depth: 20,
    //     aspect_ratio: 16. / 9.,
    //     background: Color::from(0.05, 0.05, 0.05),
    // };

    // scene.set_config(config);
    // scene.set_world(world);
    // scene.set_camera(camera);

    // scene.render();
}

pub fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let ground = Arc::new(Diffuse::new(0.5, 0.5, 0.5));
    let light = Arc::new(Light::new(1., 1., 0.8));
    let metal = Arc::new(Metal::from(Color::from(0.5, 0.5, 0.5), 0.05));
    world.add(Box::new(Sphere::from(
        Point3::from(0., -1000., 0.),
        1000.,
        ground.clone(),
    )));

    world.add(Box::new(Sphere::from(
        Point3::from(8., 2., -0.6),
        0.3,
        light.clone(),
    )));

    world.add(Box::new(Sphere::from(
        Point3::from(0., 2., -0.4),
        0.3,
        light.clone(),
    )));

    world.add(Box::new(Sphere::from(
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
                let material: Arc<dyn Material + Sync + Send> = if mat_choice < 0.6 {
                    let tint = Vec3::random() * Vec3::random();
                    Arc::new(Diffuse::from(tint))
                } else if mat_choice < 0.85 {
                    let tint = Vec3::random();
                    let fuzz = rand::random::<f64>() / 2.;
                    Arc::new(Metal::from(tint, fuzz))
                } else {
                    Arc::new(Dielectric::new(1.5))
                };
                let sphere = Box::new(Sphere::from(center, 0.2, material));
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

    world.add(Box::new(Sphere::from(
        Point3::from(0., -100.5, -1.),
        100.,
        ground.clone(),
    )));

    world.add(Box::new(Sphere::from(
        Point3::from(-1., 0., -1.),
        0.5,
        red.clone(),
    )));

    world.add(Box::new(Sphere::from(
        Point3::from(1., 0., -1.),
        0.5,
        blue.clone(),
    )));

    world.add(Box::new(Sphere::from(
        Point3::from(-0., 1.5, -1.),
        0.5,
        light.clone(),
    )));

    world.add(Box::new(Sphere::from(
        Point3::from(-0., 1.4, -0.5),
        0.5,
        blue.clone(),
    )));

    world.add(Box::new(Sphere::from(
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

    world.add(Box::new(Sphere::from(
        Point3::from(0., -100.5, -1.),
        100.,
        ground.clone(),
    )));

    world.add(Box::new(Sphere::from(
        Point3::from(-0., 1.5, -1.),
        0.5,
        light.clone(),
    )));
    world
}

fn triangle_test() {
    let mut scene = Scene::new();
    let red = Arc::new(Metal::new(0.2, 0.2, 0.8, 0.2));
    let mut world = HittableList::new();
    let v1 = Point3::from(1., -0.5, -2.);
    let v2 = Point3::from(-1., -0.5, -1.);
    let v3 = Point3::from(0., 1., -1.);
    let sphere = Box::new(Sphere::from(v2, 0.5, red.clone()));
    let triangle = Box::new(Triangle::new(v1, v2, v3, red));

    let ground = Arc::new(Diffuse::new(0.8, 0.8, 0.0));

    world.add(Box::new(Sphere::from(
        Point3::from(0., -100.5, -1.),
        100.,
        ground.clone(),
    )));
    world.add(triangle);
    world.add(sphere);
    scene.set_world(world);
    scene.render();
}

fn rectangle_test() {
    let mut scene = Scene::new();
    let red = Arc::new(Metal::new(1., 0., 0., 0.2));
    let blue = Arc::new(Diffuse::new(0., 0., 0.8));
    let green = Arc::new(Metal::new(0.3, 1., 0.2, 0.1));
    let ground = Arc::new(Diffuse::new(0.8, 0.8, 0.0));
    let light = Arc::new(Light::from(Color::from(0.9, 0.9, 0.9)));

    let mut world = HittableList::new();

    world.add(Box::new(Sphere::from(
        Point3::from(0., -100.5, -1.),
        100.,
        ground.clone(),
    )));

    let origin = Point3::from(0., 0., -1.);
    let bot = Vec3::from(0., -0.5, 0.);
    let left = Vec3::from(-1., 0., -1.).unit_vector() * 0.5;
    let right = Vec3::from(1., 0., -1.).unit_vector() * 0.5;

    let top = Box::new(Rectangle::new(origin, left, right, red.clone()));
    let side_left = Box::new(Rectangle::new(origin, bot, left, blue));
    let side_right = Box::new(Rectangle::new(origin, bot, right, green));

    let sphere = Box::new(Sphere::from(Point3::from(1., 0., -1.), 0.5, red));
    let sun = Box::new(Sphere::from(Point3::from(0., 1., -3.), 0.2, light));

    let mut camera = Camera::default();
    camera.translate(0., 0.5, 0.);
    scene.set_camera(camera);

    let mut config = Config::default();
    let background = Color::from(0.1, 0.1, 0.2);
    config.background = background;
    config.samples = 400;
    config.depth = 100;
    scene.set_config(config);

    world.add(top);
    world.add(side_left);
    world.add(side_right);
    world.add(sphere);
    world.add(sun);
    scene.set_world(world);
    scene.render();
}
