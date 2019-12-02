use std::f32::consts::PI;

use crate::util::random::rand_num;
use crate::util::ray::Ray;
use crate::util::vector3d::{unit_vector, Vector3D};

pub struct Camera {
    origin: Vector3D,
    lower_left_corner: Vector3D,
    horizontal: Vector3D,
    vertical: Vector3D,
    u: Vector3D,
    v: Vector3D,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        lookfrom: &Vector3D,
        lookat: &Vector3D,
        vup: Vector3D,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let theta = vfov * PI / 180.;
        let half_height = (theta / 2.).tan();
        let half_width = aspect * half_height;
        let w = unit_vector(&(lookfrom.clone() - lookat.clone()));
        let u = unit_vector(&(vup.cross(&w)));
        let v = w.cross(&u);
        Self {
            origin: lookfrom.clone(),
            lower_left_corner: lookfrom.clone()
                - u * half_width * focus_dist
                - v * half_height * focus_dist
                - w * focus_dist,
            vertical: v * 2. * half_height * focus_dist,
            horizontal: u * 2. * half_width * focus_dist,
            lens_radius: aperture / 2.,
            u: u,
            v: v,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = self.u.clone() * rd.x() + self.v.clone() * rd.y();
        let dest = self.lower_left_corner.clone()
            + self.horizontal.clone() * u
            + self.vertical.clone() * v
            - self.origin.clone()
            - offset;
        Ray::new(&(self.origin + offset), &dest)
    }
}

pub fn random_in_unit_disk() -> Vector3D {
    let mut p: Vector3D;
    loop {
        p = Vector3D::new(rand_num(), rand_num(), 0.) * 2. - Vector3D::new(1., 1., 0.);
        if p.dot(&p) < 1. {
            return p;
        }
    }
}
