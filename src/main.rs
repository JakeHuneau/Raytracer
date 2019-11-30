use std::f32;
use std::time::SystemTime;

use raytrace::shapes::hitable::{HitRecord, Hitable, HitableList};
use raytrace::shapes::sphere::Sphere;
use raytrace::util::camera::Camera;
use raytrace::util::ppm::PPM;
use raytrace::util::ray::Ray;
use raytrace::util::vector3d::{unit_vector, Vector3D};
use raytrace::util::random::rand_num;

pub fn color(r: &Ray, world: &Hitable) -> Vector3D {
    let mut rec = HitRecord::new();
    match world.hit(&r, 0., f32::MAX, &mut rec) {
        true => {
            Vector3D::new(
                rec.normal.x() + 1.,
                rec.normal.y() + 1.,
                rec.normal.z() + 1.,
            ) / 2.
        }
        false => {
            let unit_direction = unit_vector(&r.direction());
            let t = (unit_direction.y() + 1.) / 2.;
            Vector3D::new(1., 1., 1.) * (1. - t) + Vector3D::new(0.5, 0.7, 1.) * t
        }
    }
}

fn main() {
    let start = SystemTime::now();

    let nx = 500;
    let ny = 1000;
    let ns = 100;

    let filename = "out.ppm";
    let mut ppm = PPM::new(&filename, nx, ny, 256);

    let s1 = Box::new(Sphere::new(Vector3D::new(0., 0., -1.), 0.5));
    let s2 = Box::new(Sphere::new(Vector3D::new(0., -100.5, -1.), 100.));
    let world = HitableList::new(vec![s1, s2]);

    let cam = Camera::new();

    for j in (0..ppm.height - 1).rev() {
        for i in 0..ppm.width {
            let mut col = Vector3D::new(0., 0., 0.);
            for _ in 0..ns {
                let u = (i as f32 + rand_num()) / (ppm.width as f32);
                let v = (j as f32 + rand_num()) / (ppm.height as f32);
                let r = cam.get_ray(u, v);
                col += color(&r, &world);
            }
            col /= ns as f32;

            let v1 = [
                (ppm.max as f32 * col.e[0]) as u32,
                (ppm.max as f32 * col.e[1]) as u32,
                (ppm.max as f32 * col.e[2]) as u32,
            ];
            ppm.write_row(&v1);
        }
    }
    println!("Finished in {} ms", start.elapsed().unwrap().as_millis());
}
