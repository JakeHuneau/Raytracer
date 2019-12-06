extern crate rand;
extern crate rayon;

use self::rand::{thread_rng, Rng};
use rayon::prelude::*;

use std::env;
use std::f32;
use std::time::SystemTime;

use raytrace::shapes::hitable::{HitRecord, Hitable, HitableList};
use raytrace::shapes::sphere::Sphere;
use raytrace::util::camera::Camera;
use raytrace::util::material::Material;
use raytrace::util::ppm::PPM;

use raytrace::util::ray::Ray;
use raytrace::util::vector3d::{unit_vector, Vector3D};

pub fn random_scene() -> HitableList {
    let mut rng = thread_rng();
    let mut list = HitableList::new(vec![]);
    list.list.push(Box::new(Sphere::new(
        Vector3D::new(0., -1000., 0.),
        1000.,
        Material::Lambertian {
            albedo: Vector3D::new(0.5, 0.5, 0.5),
        },
    )));
    list.list.push(Box::new(Sphere::new(
        Vector3D::new(0., 1., 0.),
        1.,
        Material::Dialectric { ref_ind: 1.5 },
    )));
    list.list.push(Box::new(Sphere::new(
        Vector3D::new(-4., 1., 0.),
        1.,
        Material::Lambertian {
            albedo: Vector3D::new(0.4, 0.2, 0.1),
        },
    )));
    list.list.push(Box::new(Sphere::new(
        Vector3D::new(4., 1., 0.),
        1.,
        Material::Metal {
            albedo: Vector3D::new(0.7, 0.6, 0.5),
            fuzziness: 0.,
        },
    )));
    for a in -11..10 {
        for b in -11..10 {
            let choose_mat = rng.gen::<f32>();
            let center = Vector3D::new(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );
            if (center - Vector3D::new(4., 0.2, 0.)).length() > 0.9 {
                match choose_mat {
                    choose_mat if choose_mat < 0.8 => {
                        list.list.push(Box::new(Sphere::new(
                            center,
                            0.2,
                            Material::Lambertian {
                                albedo: Vector3D::new(
                                    rng.gen::<f32>() * rng.gen::<f32>(),
                                    rng.gen::<f32>() * rng.gen::<f32>(),
                                    rng.gen::<f32>() * rng.gen::<f32>(),
                                ),
                            },
                        )));
                    }
                    choose_mat if choose_mat < 0.95 => {
                        list.list.push(Box::new(Sphere::new(
                            center,
                            0.2,
                            Material::Metal {
                                albedo: Vector3D::new(
                                    0.5 * (1. + rng.gen::<f32>()),
                                    0.5 * (1. + rng.gen::<f32>()),
                                    0.5 * (1. + rng.gen::<f32>()),
                                ),
                                fuzziness: 0.5 * rng.gen::<f32>(),
                            },
                        )));
                    }
                    _ => {
                        list.list.push(Box::new(Sphere::new(
                            center,
                            0.2,
                            Material::Dialectric { ref_ind: 1.5 },
                        )));
                    }
                }
            }
        }
    }
    list
}

pub fn color(r: &Ray, world: &HitableList, depth: i32) -> Vector3D {
    let mut rec = HitRecord::new(Material::DummyMat {
        albedo: Vector3D::new(0., 0., 0.),
    });
    match world.hit(&r, 0.001, f32::MAX, &mut rec) {
        true => {
            let v1 = Vector3D::new(0., 0., 0.);
            let v2 = Vector3D::new(0., 0., 0.);
            let mut scattered = Ray::new(&v1, &v2);
            let mut attenuation = Vector3D::new(0., 0., 0.);
            match depth < 50
                && rec
                    .material
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

pub fn calculate_pixel(
    ns: u32,
    cam: &Camera,
    world: &HitableList,
    i: u32,
    j: u32,
    nx: u32,
    ny: u32,
    max_color: u32,
) -> [u32; 3] {
    let mut col: Vector3D = (0..ns)
        .into_par_iter()
        .map_init(rand::thread_rng, |rng, _| -> Vector3D {
            let u = (i as f32 + rng.gen::<f32>()) / (nx as f32);
            let v = (j as f32 + rng.gen::<f32>()) / (ny as f32);
            let r = cam.get_ray(u, v);
            color(&r, &world, 0)
        })
        .sum();
    col /= ns as f32;
    col = Vector3D::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());
    [
        (max_color as f32 * col.e[0]) as u32,
        (max_color as f32 * col.e[1]) as u32,
        (max_color as f32 * col.e[2]) as u32,
    ]
}

fn main() {
    let start = SystemTime::now();

    let args: Vec<String> = env::args().collect();

    let nx = args[1].parse::<u32>().unwrap(); // image width
    let ny = args[2].parse::<u32>().unwrap(); // image height
    let ns = args[3].parse::<u32>().unwrap(); // antialiasing samples per pixel
    let max_color = 256;

    let filename = "out.ppm";
    let mut ppm = PPM::new(&filename, ny, nx, max_color);

    let world = random_scene();

    let lookfrom = Vector3D::new(13., 2., 3.);
    let lookat = Vector3D::new(0., 0., -1.);
    let cam = Camera::new(
        &lookfrom,
        &lookat,
        Vector3D::new(0., 1., 0.),
        30.,
        nx as f32 / ny as f32,
        0.1,
        10.,
    );

    for j in (0..ny).rev() {
        println!("Starting row {} at {} ms", j, start.elapsed().unwrap().as_millis());
        let pixels: Vec<[u32; 3]> = (0..nx)
            .into_par_iter()
            .map(|i| calculate_pixel(ns, &cam, &world, i, j, nx, ny, max_color))
            .collect();

        for pixel in pixels {
            ppm.write_row(pixel);
        }
    }
    println!("Finished in {} ms", start.elapsed().unwrap().as_millis());
}
