use super::hitable::{HitRecord, Hitable};
use crate::util::ray::Ray;
use crate::util::vector3d::Vector3D;

pub struct Sphere {
    pub center: Vector3D,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vector3D, radius: f32) -> Self {
        Self {
            center: center,
            radius: radius,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = r.origin().clone() - self.center.clone();
        let a = r.direction().dot(&r.direction());
        let b = oc.dot(&r.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        let res = discriminant > 0.;
        match res {
            true => {
                let mut temp = (-b - (b * b - a * c).sqrt()) / a;
                if temp < t_max && temp > t_min {
                    rec.t = temp;
                    rec.p = r.point_at_parameter(rec.t);
                    rec.normal = (rec.p - self.center) / self.radius;
                    return true;
                }
                temp = (-b + (b * b - a * c).sqrt()) / a;
                if temp < t_max && temp > t_min {
                    rec.t = temp;
                    rec.p = r.point_at_parameter(rec.t);
                    rec.normal = (rec.p - self.center) / self.radius;
                    return true;
                }
                return false;
            }
            false => false,
        }
    }
}
