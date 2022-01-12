use crate::{
    geometries::{HitType, Hittable},
    materials::Scatter,
    utils::vec3::{Color, Point3, Vec3},
};

pub struct Ray {
    origin: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn from(origin: Point3, dir: Vec3) -> Self {
        Ray { origin, dir }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.dir * t
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn dir(&self) -> Vec3 {
        self.dir
    }
}

impl Ray {
    pub fn color(&self, world: &dyn Hittable, background: Color, depth: u32) -> Color {
        if depth == 0 {
            return Vec3::new();
        }

        if let HitType::Hit(rec) = world.hit(&self, 0.001, f64::INFINITY) {
            let scattered = rec.material.scatter(&self, &rec);
            if let Scatter::Scattered(attenuation, scattered_ray) = scattered {
                return attenuation * scattered_ray.color(world, background, depth - 1);
            }
            let ret = match scattered {
                Scatter::Scattered(attenuation, scattered_ray) => {
                    attenuation * scattered_ray.color(world, background, depth - 1)
                }
                Scatter::Light(tint) => tint,
                Scatter::Absorbed => Vec3::new(),
            };
            return ret;
        }

        // let unit_dir = self.dir.unit_vector();
        // map y-component to 1 (top of viewport) and 0 (bottom of viewport)
        // let t = 0.5 * (unit_dir.y() + 1.);
        // return gradient from blue (top) and white (bottom)
        // Color::from(0.3, 0.3, 0.9) * t + Color::from(0.5, 0.7, 1.) * (1. - t)
        background
    }

    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        let in_ray = self.dir.unit_vector();
        return in_ray - *normal * in_ray.dot(normal) * 2.;
    }

    pub fn refract(&self, normal: &Vec3, eta_over_etap: f64) -> Vec3 {
        let uv = self.dir.unit_vector();
        let cos_theta = normal.dot(&(-uv)).min(1.);
        let out_perp = (uv + *normal * cos_theta) * eta_over_etap;
        let out_para = *normal * -(1. - out_perp.length_squared()).abs().sqrt();
        out_para + out_perp
    }
}
