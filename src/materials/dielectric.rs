use super::Color;
use super::HitRecord;
use super::Material;
use super::Ray;
use super::Scatter;

pub struct Dielectric {
    refraction: f64,
}

impl Dielectric {
    pub fn new(refraction: f64) -> Self {
        Dielectric { refraction }
    }

    pub fn reflectance(cosine: f64, ref_ratio: f64) -> f64 {
        let r0 = (1. - ref_ratio) / (1. + ref_ratio);
        let r0 = r0 * r0;
        r0 + (1. - r0) * (1. - cosine).powf(5.)
    }
}

unsafe impl Sync for Dielectric {}
unsafe impl Send for Dielectric {}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Scatter {
        let refraction_ratio = if rec.front_face {
            1. / self.refraction
        } else {
            self.refraction
        };
        let unit_dir = ray.dir().unit_vector();
        let cos_theta = rec.normal.dot(&(-unit_dir)).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.;
        let attenuation = Color::from(1., 1., 1.);

        if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > rand::random() {
            let dir = ray.reflect(&rec.normal);
            return Scatter::Scattered(attenuation, Ray::from(rec.p, dir));
        } else {
            let dir = ray.refract(&rec.normal, refraction_ratio);
            return Scatter::Scattered(attenuation, Ray::from(rec.p, dir));
        }
    }
}
