use super::Color;
use super::HitRecord;
use super::Material;
use super::Ray;
use super::Scatter;
use super::Vec3;

pub struct Metal {
    tint: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(r: f64, g: f64, b: f64, fuzz: f64) -> Self {
        Self::from(Color::from(r, g, b), fuzz)
    }

    pub fn from(tint: Color, fuzz: f64) -> Self {
        Metal { tint, fuzz }
    }
}

unsafe impl Sync for Metal {}
unsafe impl Send for Metal {}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Scatter {
        let reflected = ray.reflect(&rec.normal);
        if reflected.dot(&rec.normal) > 0. {
            let scattered = Ray::from(
                rec.p,
                reflected + Vec3::random_unit_sphere().unit_vector() * self.fuzz,
            );
            let attenuation = self.tint;
            return Scatter::Scattered(attenuation, scattered);
        }
        Scatter::Absorbed
    }
}
