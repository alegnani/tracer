use std::sync::Arc;

use crate::{
    materials::Material,
    ray::Ray,
    utils::vec3::{Point3, Vec3},
};

use super::{triangle::Triangle, HitType, Hittable, HittableList};

pub struct Rectangle {
    triangles: HittableList,
}

impl Rectangle {
    pub fn new(
        origin: Point3,
        horizontal: Vec3,
        vertical: Vec3,
        material: Arc<dyn Material>,
    ) -> Self {
        let br = origin + horizontal;
        let tl = origin + vertical;
        let tr = br + vertical;
        let t1 = Box::new(Triangle::new(tl, origin, br, material.clone()));
        let t2 = Box::new(Triangle::new(tl, br, tr, material));
        let triangles = HittableList::from(vec![t1, t2]);
        Rectangle { triangles }
    }
}

impl Hittable for Rectangle {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> HitType {
        self.triangles.hit(ray, t_min, t_max)
    }
}
