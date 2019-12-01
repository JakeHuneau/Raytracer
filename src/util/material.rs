use crate::shapes::hitable::HitRecord;
use crate::shapes::sphere::random_in_unit_sphere;
use crate::util::random::rand_num;
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

#[allow(unused)]
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

#[allow(unused)]
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

#[derive(Clone)]
pub struct Dialectric {
    ref_ind: f32,
}

impl Dialectric {
    pub fn new(ri: f32) -> Self {
        Self { ref_ind: ri }
    }
}

impl Material for Dialectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vector3D,
        scattered: &mut Ray,
    ) -> bool {
        let outward_normal: Vector3D;
        let reflected = reflect(&r_in.direction(), &rec.normal);
        let ni_over_nt: f32;
        *attenuation = Vector3D::new(1., 1., 1.);
        let mut refracted = Vector3D::new(0., 0., 0.);
        let reflect_prob: f32;
        let cosine: f32;
        match r_in.direction().dot(&rec.normal) > 0. {
            true => {
                outward_normal = -rec.normal;
                ni_over_nt = self.ref_ind;
                cosine =
                    self.ref_ind * r_in.direction().dot(&rec.normal) / r_in.direction().length();
            }
            false => {
                outward_normal = rec.normal;
                ni_over_nt = 1. / self.ref_ind;
                cosine = -r_in.direction().dot(&rec.normal) / r_in.direction().length();
            }
        };
        match refract(
            &r_in.direction(),
            &outward_normal,
            ni_over_nt,
            &mut refracted,
        ) {
            true => {
                reflect_prob = schlick(&cosine, &self.ref_ind);
            }
            false => {
                *scattered = Ray::new(&rec.p, &reflected);
                reflect_prob = 1.;
            }
        };
        match rand_num() < reflect_prob {
            true => *scattered = Ray::new(&rec.p, &reflected),
            false => *scattered = Ray::new(&rec.p, &refracted),
        };
        true
    }

    fn box_clone(&self) -> Box<Material> {
        Box::new((*self).clone())
    }
}

pub fn reflect(v: &Vector3D, n: &Vector3D) -> Vector3D {
    v.clone() - n.clone() * v.dot(&n) * 2.
}

pub fn refract(v: &Vector3D, n: &Vector3D, ni_over_nt: f32, refracted: &mut Vector3D) -> bool {
    let uv = unit_vector(&v);
    let dt = uv.dot(&n);
    let discriminant = 1. - ni_over_nt * ni_over_nt * (1. - dt * dt);
    match discriminant > 0. {
        true => {
            *refracted = (uv - n.clone() * dt) * ni_over_nt - n.clone() * discriminant.sqrt();
            true
        }
        false => false,
    }
}

pub fn schlick(cosine: &f32, ref_idx: &f32) -> f32 {
    let mut r0 = (1. - ref_idx) / (1. + ref_idx);
    r0 *= r0;
    r0 + (1. - r0) * (1. - cosine).powi(5)
}
