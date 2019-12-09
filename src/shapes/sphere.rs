use super::hitable::{HitRecord, Hitable};
use crate::util::material::Material;
use crate::util::random::rand_num;
use crate::util::ray::Ray;
use crate::util::vector3d::Vector3D;

pub struct Sphere {
    pub center: Vector3D,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vector3D, radius: f32, m: Material) -> Self {
        Self {
            center: center,
            radius: radius,
            material: m,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = r.origin().clone() - self.center.clone();
        let a = r.direction().dot(r.direction());
        let b = oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        let res = discriminant > 0.;
        match res {
            true => {
                let mut temp = (-b - (b * b - a * c).sqrt()) / a;
                if temp < t_max && temp > t_min {
                    rec.t = temp;
                    rec.p = r.point_at_parameter(rec.t);
                    rec.normal = (rec.p - self.center) / self.radius;
                    rec.material = self.material.clone();
                    return true;
                }
                temp = (-b + (b * b - a * c).sqrt()) / a;
                if temp < t_max && temp > t_min {
                    rec.t = temp;
                    rec.p = r.point_at_parameter(rec.t);
                    rec.normal = (rec.p - self.center) / self.radius;
                    rec.material = self.material.clone();
                    return true;
                }
                return false;
            }
            false => false,
        }
    }
}

pub fn random_in_unit_sphere() -> Vector3D {
    loop {
        let p = Vector3D::new(rand_num(), rand_num(), rand_num()) * 2. - Vector3D::new(1., 1., 1.);
        if p.squared_length() >= 1. {
            return p;
        }
    }
}
