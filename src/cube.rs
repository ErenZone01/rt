use crate::hittable::*;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Vec3, Point3};

pub struct Cube {
    min: Point3,
    max: Point3,
    material: Material,
}

impl Cube {
    pub fn new(min: Point3, max: Point3, material: Material) -> Cube {
        Cube { min, max, material }
    }
}

impl Hittable for Cube {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut t_min = t_min;
        let mut t_max = t_max;

        for i in 0..3 {
            let inv_d = 1.0 / r.direction()[i];
            let t0 = (self.min[i] - r.origin()[i]) * inv_d;
            let t1 = (self.max[i] - r.origin()[i]) * inv_d;

            let (t0, t1) = if inv_d < 0.0 { (t1, t0) } else { (t0, t1) };

            t_min = if t0 > t_min { t0 } else { t_min };
            t_max = if t1 < t_max { t1 } else { t_max };

            if t_max <= t_min {
                return None;
            }
        }

        let t = t_min;
        let p = r.point_at_parameter(t);
        let normal = if (p.x() - self.min.x()).abs() < 0.0001 {
            Vec3::new(-1.0, 0.0, 0.0)
        } else if (p.x() - self.max.x()).abs() < 0.0001 {
            Vec3::new(1.0, 0.0, 0.0)
        } else if (p.y() - self.min.y()).abs() < 0.0001 {
            Vec3::new(0.0, -1.0, 0.0)
        } else if (p.y() - self.max.y()).abs() < 0.0001 {
            Vec3::new(0.0, 1.0, 0.0)
        } else if (p.z() - self.min.z()).abs() < 0.0001 {
            Vec3::new(0.0, 0.0, -1.0)
        } else {
            Vec3::new(0.0, 0.0, 1.0)
        };

        Some(HitRecord {
            t,
            p,
            normal,
            material: self.material,
        })
    }
}
