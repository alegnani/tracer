use std::{
    sync::{mpsc, Arc},
    time::Instant,
};

use crate::{
    camera::Camera,
    geometries::{Hittable, HittableList},
    materials::Material,
    utils::{
        ppm::PPM,
        vec3::{Color, Vec3},
    },
};

pub struct Config {
    pub name: String,
    pub height: usize,
    pub aspect_ratio: f64,
    pub samples: u32,
    pub depth: u32,
    pub background: Color,
}

pub struct Scene {
    camera: Camera,
    world: HittableList,
    materials: Vec<Arc<dyn Material>>,
    config: Config,
}

impl Config {
    pub fn default() -> Self {
        let name = String::from("scene");
        let height = 400;
        let samples = 40;
        let depth = 20;
        let aspect_ratio = 16. / 9.;
        let background = Color::from(0.3, 0.3, 0.8);
        Config {
            name,
            height,
            aspect_ratio,
            samples,
            depth,
            background,
        }
    }
}

impl Scene {
    pub fn new() -> Self {
        let camera = Camera::default();
        let world = HittableList::new();
        let materials = vec![];
        let config = Config::default();

        Scene {
            camera,
            world,
            materials,
            config,
        }
    }

    pub fn from(
        camera: Camera,
        world: HittableList,
        materials: Vec<Arc<dyn Material>>,
        config: Config,
    ) -> Self {
        Scene {
            camera,
            world,
            materials,
            config,
        }
    }

    pub fn add_material(&mut self, material: &Arc<dyn Material>) {
        self.materials.push(material.clone());
    }

    pub fn add_object(&mut self, obj: Arc<dyn Hittable>) {
        self.world.add(obj);
    }

    pub fn set_config(&mut self, conf: Config) {
        self.config = conf;
    }

    pub fn set_camera(&mut self, cam: Camera) {
        self.camera = cam;
    }

    pub fn set_world(&mut self, world: HittableList) {
        self.world = world;
    }

    pub fn set_materials(&mut self, materials: Vec<Arc<dyn Material>>) {
        self.materials = materials;
    }

    pub fn print_info(&self) {
        println!(
            "Rendering scene:\nResolution: {} x {}\nSamples: {}\nDepth: {}\nObjects: {}",
            (self.config.height as f64 * self.config.aspect_ratio) as usize,
            self.config.height,
            self.config.samples,
            self.config.depth,
            self.world.size()
        );
    }

    pub fn render(&self) {
        self.print_info();
        let start = Instant::now();
        let height = self.config.height;
        let width = (self.config.height as f64 * self.config.aspect_ratio) as usize;
        let samples = self.config.samples;
        let depth = self.config.depth;
        let world = &self.world;
        let camera = &self.camera;

        let milestone = height / 10;

        let mut image = vec![vec![Vec3::new(); width]; height];
        let a = &mut image;
        rayon::scope(|s| {
            let (tx, rx) = mpsc::channel::<(usize, Vec<Vec3>)>();
            s.spawn(move |_| {
                let mut counter = 0;
                for (y, row) in rx.iter() {
                    counter += 1;
                    if counter % milestone == 0 {
                        println!("Status: {}%", counter / milestone * 10);
                    }
                    a[y] = row;
                }
            });

            for j in 0..height {
                let tx = tx.clone();
                s.spawn(move |_| {
                    let mut row = vec![];
                    for i in 0..width {
                        let mut pixel = Color::new();
                        for _ in 0..samples {
                            let x = (i as f64 + rand::random::<f64>()) / (width as f64 - 1.);
                            let y = (j as f64 + rand::random::<f64>()) / (height as f64 - 1.);
                            let r = camera.get_ray(x, y);
                            pixel += r.color(world, self.config.background, depth);
                        }
                        row.push(pixel);
                    }

                    tx.send((j, row)).unwrap();
                });
            }
        });

        let file_name = format!("{}.ppm", self.config.name);
        let mut file = PPM::from(file_name, width as u32, self.config.height as u32);
        image.reverse();
        for row in image {
            for pixel in row {
                let rgb = pixel.to_rgb(self.config.samples);
                file.push(rgb.0, rgb.1, rgb.2);
            }
        }
        file.write().unwrap();
        println!("Took: {}s", start.elapsed().as_secs());
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use crate::{
        camera::Camera,
        geometries::{sphere::Sphere, HittableList},
        materials::{
            dielectric::Dielectric, diffuse::Diffuse, light::Light, metal::Metal, Material,
        },
        utils::vec3::{Color, Point3, Vec3},
    };

    use super::Scene;

    #[test]
    fn test_scene() {
        let mut scene = Scene::new();

        let from = Point3::from(-2., 1.5, 2.);
        let at = Point3::from(0., 0., -1.);
        let vup = Vec3::from(0., 1., 0.);
        let camera = Camera::new(from, at, vup, 90., 16. / 9.);

        let world = random_scene();

        scene.set_world(world);
        scene.set_camera(camera);

        scene.render();
    }

    pub fn random_scene() -> HittableList {
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
}
