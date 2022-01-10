use crate::geometries::HitRecord;
use crate::ray::Ray;
use crate::utils::vec3::{Color, Vec3};

pub mod dielectric;
pub mod diffuse;
pub mod light;
pub mod metal;
pub mod mirror;

pub enum Scatter {
    Absorbed,
    Scattered(Color, Ray),
    Light(Color),
}

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Scatter;
}
