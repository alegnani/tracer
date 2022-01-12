use std::sync::Arc;

use crate::materials::Material;
use crate::ray::Ray;
use crate::utils::vec3::{Point3, Vec3};

pub mod sphere;

pub enum HitType {
    NoHit,
    Hit(HitRecord),
}

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn from(
        ray: &Ray,
        t: f64,
        p: Point3,
        outward_normal: Vec3,
        material_ptr: Arc<dyn Material>,
    ) -> Self {
        let front_face = outward_normal.dot(&ray.dir()) < 0.;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        let material = material_ptr.clone();
        HitRecord {
            p,
            normal,
            t,
            front_face,
            material,
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> HitType;
}

#[derive(Clone)]
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        let objects = vec![];
        HittableList { objects }
    }

    pub fn from(objects: Vec<Arc<dyn Hittable>>) -> Self {
        HittableList { objects }
    }

    pub fn clear(&mut self) {
        self.objects = vec![];
    }

    pub fn size(&self) -> usize {
        self.objects.len()
    }

    pub fn add(&mut self, obj: Arc<dyn Hittable>) {
        self.objects.push(obj);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> HitType {
        let mut closest = t_max;
        let mut rec = HitType::NoHit;

        for obj in &self.objects {
            if let HitType::Hit(tmp) = obj.hit(ray, t_min, closest) {
                closest = tmp.t;
                rec = HitType::Hit(tmp);
            }
        }
        rec
    }
}
