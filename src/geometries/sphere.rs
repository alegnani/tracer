use std::ops::Deref;
use std::sync::Arc;

use super::HitRecord;
use super::HitType;
use super::Hittable;
use super::Material;
use super::Point3;
use super::Ray;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn from(center: Point3, radius: f64, material: Arc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }

    pub fn center(&self) -> Point3 {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> HitType {
        let oc = ray.origin() - self.center;

        let a = ray.dir().length_squared();
        let half_b = ray.dir().dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return HitType::NoHit;
        }

        let sqrt_d = discriminant.sqrt();
        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || t_max < root {
                return HitType::NoHit;
            }
        }

        let t = root;
        let p = ray.at(t);
        let outward_normal = (p - self.center) / self.radius;
        let material = self.material.clone();
        let rec = HitRecord::from(ray, t, p, outward_normal, material);

        HitType::Hit(rec)
    }
}
