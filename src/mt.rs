use std::sync::Arc;

use threadpool::ThreadPool;

use crate::{
    geometries::{sphere::Sphere, Hittable},
    materials::dielectric::Dielectric,
    ray::Ray,
    utils::vec3::{Point3, Vec3},
};

fn test() {
    let pool = ThreadPool::new(2);
    pool.execute(|| println!("dio"));
    let glass = Arc::new(Dielectric::new(1.5));
    let sphere_og = Arc::new(Sphere::from(Point3::from(0., 0., 0.), 1., glass.clone()));
    let sphere = sphere_og.clone();
    pool.execute(move || {
        let r = Ray::from(Point3::new(), Vec3::from(1., 1., 1.));
        sphere.hit(&r, 0., 50.);
    });
    pool.join()
}
