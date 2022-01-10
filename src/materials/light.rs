use crate::{geometries::HitRecord, ray::Ray, utils::vec3::Color};

use super::{Material, Scatter};

pub struct Light {
    tint: Color,
}

impl Light {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self::from(Color::from(r, g, b))
    }

    pub fn from(tint: Color) -> Self {
        Light { tint }
    }
}

unsafe impl Sync for Light {}
unsafe impl Send for Light {}

impl Material for Light {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Scatter {
        Scatter::Light(self.tint)
    }
}
