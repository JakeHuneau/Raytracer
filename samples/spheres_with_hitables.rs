use std::f32;
use std::time::SystemTime;

use raytrace::shapes::hitable::{HitRecord, Hitable, HitableList};
use raytrace::shapes::sphere::Sphere;
use raytrace::util::ppm::PPM;
use raytrace::util::ray::Ray;
use raytrace::util::vector3d::{unit_vector, Vector3D};

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
    let filename = "out.ppm";
    let mut ppm = PPM::new(&filename, 500, 1000, 256);

    let lower_left_corner = Vector3D {
        e: [-2.0, -1.0, -1.0],
    };
    let horizontal = Vector3D { e: [4.0, 0.0, 0.0] };
    let vertical = Vector3D { e: [0.0, 2.0, 0.0] };
    let origin = Vector3D { e: [0.0, 0.0, 0.0] };

    let s1 = Box::new(Sphere::new(Vector3D::new(0., 0., -1.), 0.5));
    let s2 = Box::new(Sphere::new(Vector3D::new(0., -100.5, -1.), 100.));
    let world = HitableList::new(vec![s1, s2]);

    for j in (0..ppm.height - 1).rev() {
        for i in 0..ppm.width {
            let u = (i as f32) / (ppm.width as f32);
            let v = (j as f32) / (ppm.height as f32);
            let dest = lower_left_corner + horizontal * u + vertical * v;
            let r = Ray::new(&origin, &dest);
            let col = color(&r, &world);

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
