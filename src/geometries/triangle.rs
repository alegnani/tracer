use std::sync::Arc;

use crate::{
    materials::Material,
    ray::Ray,
    utils::vec3::{Point3, Vec3},
};

use super::{HitRecord, HitType, Hittable};

pub struct Triangle {
    a: Point3,
    b: Point3,
    c: Point3,
    edge1: Vec3,
    edge2: Vec3,
    material: Arc<dyn Material>,
}

impl Triangle {
    pub fn new(a: Point3, b: Point3, c: Point3, material: Arc<dyn Material>) -> Self {
        let edge1 = b - a;
        let edge2 = c - a;
        Triangle {
            a,
            b,
            c,
            edge1,
            edge2,
            material,
        }
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> HitType {
        let epsilon = 0.0001;
        let h = ray.dir().cross(&self.edge2);
        let a = self.edge1.dot(&h);
        if a > -epsilon && a < epsilon {
            return HitType::NoHit;
        }
        let f = 1. / a;
        let s = ray.origin() - self.a;
        let u = s.dot(&h) * f;
        if u < 0. || u > 1. {
            return HitType::NoHit;
        }
        let q = s.cross(&self.edge1);
        let v = ray.dir().dot(&q) * f;
        if v < 0. || u + v > 1. {
            return HitType::NoHit;
        }

        let t = self.edge2.dot(&q) * f;
        if t < epsilon {
            return HitType::NoHit;
        }
        let outward_normal = self.edge1.cross(&self.edge2).unit_vector();
        let p = ray.at(t);
        let material = self.material.clone();
        let rec = HitRecord::from(ray, t, p, outward_normal, material);
        HitType::Hit(rec)
    }
}
