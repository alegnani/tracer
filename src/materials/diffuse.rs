use super::HitRecord;
use super::Material;
use super::Ray;
use super::Scatter;
use super::{Color, Vec3};

pub struct Diffuse {
    tint: Color,
}

impl Diffuse {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self::from(Color::from(r, g, b))
    }

    pub fn from(tint: Color) -> Self {
        Diffuse { tint }
    }
}

unsafe impl Sync for Diffuse {}
unsafe impl Send for Diffuse {}

impl Material for Diffuse {
    fn scatter(&self, _ray: &crate::ray::Ray, rec: &HitRecord) -> Scatter {
        loop {
            let scatter_dir = rec.normal + Vec3::random_unit_sphere().unit_vector();
            if !scatter_dir.near_zero() {
                let scattered_ray = Ray::from(rec.p, scatter_dir);
                let attenuation = self.tint;
                return Scatter::Scattered(attenuation, scattered_ray);
            }
        }
    }
}
