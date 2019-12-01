use crate::shapes::hitable::HitRecord;
use crate::shapes::sphere::random_in_unit_sphere;
use crate::util::ray::Ray;
use crate::util::vector3d::{unit_vector, Vector3D};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vector3D,
        scattered: &mut Ray,
    ) -> bool;
    fn box_clone(&self) -> Box<Material>;
}

#[derive(Clone)]
pub struct DummyMat {
    pub albedo: Vector3D,
}

impl Clone for Box<Material> {
    fn clone(&self) -> Box<Material> {
        self.box_clone()
    }
}

impl DummyMat {
    pub fn new() -> Self {
        Self {
            albedo: Vector3D::new(0., 0., 0.),
        }
    }
}

impl Material for DummyMat {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vector3D,
        scattered: &mut Ray,
    ) -> bool {
        true
    }
    fn box_clone(&self) -> Box<Material> {
        Box::new((*self).clone())
    }
}

#[derive(Clone)]
pub struct Lambertian {
    albedo: Vector3D,
}

impl Lambertian {
    pub fn new(a: Vector3D) -> Self {
        Self { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vector3D,
        scattered: &mut Ray,
    ) -> bool {
        let dir = rec.normal.clone() + random_in_unit_sphere();
        *scattered = Ray::new(&rec.p, &dir);
        *attenuation = self.albedo.clone();
        true
    }
    fn box_clone(&self) -> Box<Material> {
        Box::new((*self).clone())
    }
}

#[derive(Clone)]
pub struct Metal {
    albedo: Vector3D,
    fuzziness: f32,
}

impl Metal {
    pub fn new(a: Vector3D, f: f32) -> Self {
        Self {
            albedo: a,
            fuzziness: if f < 1. { f } else { 1. },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vector3D,
        scattered: &mut Ray,
    ) -> bool {
        let u = unit_vector(&r_in.direction()).clone();
        let reflected = reflect(&u, &rec.normal) + random_in_unit_sphere() * self.fuzziness;
        *scattered = Ray::new(&rec.p, &reflected);
        *attenuation = self.albedo.clone();
        scattered.direction().dot(&rec.normal) > 0.
    }
    fn box_clone(&self) -> Box<Material> {
        Box::new((*self).clone())
    }
}

pub fn reflect(v: &Vector3D, n: &Vector3D) -> Vector3D {
    v.clone() - n.clone() * v.dot(&n) * 2.
}
