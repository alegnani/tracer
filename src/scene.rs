use rayon::prelude::*;
use std::{
    cell::Cell,
    rc::Rc,
    sync::{mpsc, Arc},
};
use threadpool::ThreadPool;

use crate::{
    camera::Camera,
    geometries::{Hittable, HittableList},
    materials::Material,
    utils::{
        image::Image,
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
    pool: ThreadPool,
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
    pub fn new(workers: usize) -> Self {
        let pool = ThreadPool::new(workers);
        let camera = Camera::default();
        let world = HittableList::new();
        let materials = vec![];
        let config = Config::default();

        Scene {
            camera,
            world,
            materials,
            pool,
            config,
        }
    }

    pub fn from(
        camera: Camera,
        world: HittableList,
        materials: Vec<Arc<dyn Material>>,
        config: Config,
        workers: usize,
    ) -> Self {
        let pool = ThreadPool::new(workers);
        Scene {
            camera,
            world,
            materials,
            pool,
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

    pub fn render(&self) {
        let height = self.config.height;
        let width = (self.config.height as f64 * self.config.aspect_ratio) as usize;
        let samples = self.config.samples;
        let depth = self.config.depth;
        let world = &self.world;
        let camera = &self.camera;

        let mut image = vec![vec![Vec3::new(); width]; height];
        let a = &mut image;
        rayon::scope(|s| {
            let (tx, rx) = mpsc::channel();
            s.spawn(move |s| {
                for (x, y, pixel) in rx.iter() {
                    a[y][x] = pixel;
                }
            });

            for j in 0..height {
                for i in 0..width {
                    let tx = tx.clone();
                    let ref_im = &image;
                    s.spawn(move |_| {
                        let mut pixel = Color::new();
                        for _ in 0..samples {
                            let x = (i as f64 + rand::random::<f64>()) / (width as f64 - 1.);
                            let y = (j as f64 + rand::random::<f64>()) / (height as f64 - 1.);
                            let r = camera.get_ray(x, y);
                            pixel += r.color(world, depth);
                        }
                        tx.send((i, j, pixel));
                    });
                }
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
    }

    // pub fn render(&self) {
    //     let mut image =
    //         vec![vec![Vec3::new(); self.image.height as usize]; self.image.width as usize];
    //     let (tx, rx) = channel();
    //     for j in 0..self.image.height {
    //         for i in 0..self.image.width {
    //             let tx = tx.clone();

    //             self.pool.execute(move || {
    //                 let color = Color::new();
    //                 for _ in 0..self.image.samples {
    //                     let x = (i as f64 + rand::random::<f64>()) / (self.image.width as f64 - 1.);
    //                     let y =
    //                         (j as f64 + rand::random::<f64>()) / (self.image.height as f64 - 1.);
    //                     let r = self.camera.get_ray(x, y);
    //                     color += r.color(&self.world, self.image.max_depth);
    //                 }
    //                 tx.send((i, j, color));
    //             });
    //         }
    //     }

    //     let file = PPM::from(
    //         &(self.image.name + ".ppm"),
    //         self.image.width,
    //         self.image.height,
    //     );
    //     image.reverse();
    //     for row in image {
    //         for pixel in row {
    //             let rgb = pixel.to_rgb(self.image.samples);
    //             file.push(rgb.0, rgb.1, rgb.2);
    //         }
    //     }
    // }
}
