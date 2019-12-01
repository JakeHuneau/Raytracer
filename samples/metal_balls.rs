use std::f32;
use std::time::SystemTime;

use raytrace::shapes::hitable::{HitRecord, Hitable, HitableList};
use raytrace::shapes::sphere::Sphere;
use raytrace::util::camera::Camera;
use raytrace::util::material::{DummyMat, Lambertian, Metal};
use raytrace::util::ppm::PPM;
use raytrace::util::random::rand_num;
use raytrace::util::ray::Ray;
use raytrace::util::vector3d::{unit_vector, Vector3D};

#[macro_export]
macro_rules! make_sphere {
    ( $m:ident, $x:expr, $y:expr, $r:expr ) => {{
        Box::new(Sphere::new($y, $r, Box::new($m::new($x))))
    }};
    ( $m:ident, $x:expr, $f:expr, $y:expr, $r:expr ) => {{
        Box::new(Sphere::new($y, $r, Box::new($m::new($x, $f))))
    }};
}

pub fn color(r: &Ray, world: &Hitable, depth: i32) -> Vector3D {
    let mut rec = HitRecord::new(Box::new(DummyMat::new()));
    match world.hit(&r, 0.001, f32::MAX, &mut rec) {
        true => {
            let v1 = Vector3D::new(0., 0., 0.);
            let v2 = Vector3D::new(0., 0., 0.);
            let mut scattered = Ray::new(&v1, &v2);
            let mut attenuation = Vector3D::new(0., 0., 0.);
            match depth < 50
                && rec
                    .material
                    .as_ref()
                    .scatter(r, &rec, &mut attenuation, &mut scattered)
            {
                true => attenuation * color(&scattered, world, depth + 1),
                false => Vector3D::new(0., 0., 0.),
            }
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

    let s1 = make_sphere!(
        Lambertian,
        Vector3D::new(0.8, 0.3, 0.3),
        Vector3D::new(0., 0., -1.),
        0.5
    );
    let s2 = make_sphere!(
        Lambertian,
        Vector3D::new(0.8, 0.8, 0.),
        Vector3D::new(0., -100.5, -1.),
        100.
    );
    let s3 = make_sphere!(
        Metal,
        Vector3D::new(0.8, 0.6, 0.2),
        1.,
        Vector3D::new(1., 0., -1.),
        0.5
    );
    let s4 = make_sphere!(
        Metal,
        Vector3D::new(0.8, 0.8, 0.8),
        0.3,
        Vector3D::new(-1., 0., -1.),
        0.5
    );

    let world = HitableList::new(vec![s1, s2, s3, s4]);

    let cam = Camera::new();

    for j in (0..ppm.height - 1).rev() {
        for i in 0..ppm.width {
            let mut col = Vector3D::new(0., 0., 0.);
            for _ in 0..ns {
                let u = (i as f32 + rand_num()) / (ppm.width as f32);
                let v = (j as f32 + rand_num()) / (ppm.height as f32);
                let r = cam.get_ray(u, v);
                col += color(&r, &world, 0);
            }
            col /= ns as f32;
            col = Vector3D::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());

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
