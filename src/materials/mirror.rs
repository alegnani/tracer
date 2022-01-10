use super::Color;
use super::HitRecord;
use super::Material;
use super::Ray;
use super::Scatter;

pub struct Mirror;

impl Mirror {
    pub fn new() -> Self {
        Mirror
    }
}

unsafe impl Sync for Mirror {}
unsafe impl Send for Mirror {}

impl Material for Mirror {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Scatter {
        let attenuation = Color::from(1., 1., 1.);
        let dir = ray.reflect(&rec.normal);
        return Scatter::Scattered(attenuation, Ray::from(rec.p, dir));
    }
}
