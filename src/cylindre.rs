use crate::hittable::*;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Cylinder {
    base_center: Vec3,
    height: f32,
    radius: f32,
    material: Material,
}

impl Cylinder {
    pub fn cylinder(base_center: Vec3, height: f32, radius: f32, material: Material) -> Cylinder {
        Cylinder {
            base_center,
            height,
            radius,
            material,
        }
    }
}

impl Hittable for Cylinder {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - self.base_center;
        let dir = r.direction();

        // Ignore the y-component to treat the cylinder as infinite
        let a = dir.x() * dir.x() + dir.z() * dir.z();
        let b = 2.0 * (oc.x() * dir.x() + oc.z() * dir.z());
        let c = oc.x() * oc.x() + oc.z() * oc.z() - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            let sqrt_d = discriminant.sqrt();
            let mut temp = (-b - sqrt_d) / (2.0 * a);
            if temp < t_max && temp > t_min {
                let hit_point = r.point_at_parameter(temp);
                let y = hit_point.y() - self.base_center.y();
                if y >= 0.0 && y <= self.height {
                    return Some(HitRecord {
                        t: temp,
                        p: hit_point,
                        normal: Vec3::unit_vector(&Vec3::new(hit_point.x() - self.base_center.x(), 0.0, hit_point.z() - self.base_center.z())),
                        material: self.material,
                    });
                }
            }

            temp = (-b + sqrt_d) / (2.0 * a);
            if temp < t_max && temp > t_min {
                let hit_point = r.point_at_parameter(temp);
                let y = hit_point.y() - self.base_center.y();
                if y >= 0.0 && y <= self.height {
                    return Some(HitRecord {
                        t: temp,
                        p: hit_point,
                        normal: Vec3::unit_vector(&Vec3::new(hit_point.x() - self.base_center.x(), 0.0, hit_point.z() - self.base_center.z())),
                        material: self.material,
                    });
                }
            }
        }

        None
    }
}
