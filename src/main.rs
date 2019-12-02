use std::f32;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread;
use std::time::SystemTime;

use raytrace::shapes::hitable::{HitRecord, Hitable, HitableList};
use raytrace::shapes::sphere::Sphere;
use raytrace::util::camera::Camera;
use raytrace::util::material::{Dialectric, DummyMat, Lambertian, Metal};
use raytrace::util::ppm::PPM;
use raytrace::util::random::rand_num;
use raytrace::util::ray::Ray;
use raytrace::util::vector3d::{unit_vector, Vector3D};

const NUM_WORKERS: usize = 4;

#[macro_export]
macro_rules! make_sphere {
    ( $m:ident, $x:expr, $y:expr, $r:expr ) => {{
        Box::new(Sphere::new($y, $r, Box::new($m::new($x))))
    }};
    ( $m:ident, $x:expr, $f:expr, $y:expr, $r:expr ) => {{
        Box::new(Sphere::new($y, $r, Box::new($m::new($x, $f))))
    }};
}

pub fn random_scene() -> HitableList {
    let mut list = HitableList::new(vec![]);
    list.list.push(make_sphere!(
        Lambertian,
        Vector3D::new(0.5, 0.5, 0.5),
        Vector3D::new(0., -1000., 0.),
        1000.
    ));
    list.list
        .push(make_sphere!(Dialectric, 1.5, Vector3D::new(0., 1., 0.), 1.));
    list.list.push(make_sphere!(
        Lambertian,
        Vector3D::new(0.4, 0.2, 0.1),
        Vector3D::new(-4., 1., 0.),
        1.
    ));
    list.list.push(make_sphere!(
        Metal,
        Vector3D::new(0.7, 0.6, 0.5),
        0.0,
        Vector3D::new(4., 1., 0.),
        1.
    ));
    for a in -11..10 {
        for b in -11..10 {
            let choose_mat = rand_num();
            let center = Vector3D::new(
                a as f32 + 0.9 * rand_num(),
                0.2,
                b as f32 + 0.9 * rand_num(),
            );
            if (center - Vector3D::new(4., 0.2, 0.)).length() > 0.9 {
                match choose_mat {
                    choose_mat if choose_mat < 0.8 => {
                        list.list.push(make_sphere!(
                            Lambertian,
                            Vector3D::new(
                                rand_num() * rand_num(),
                                rand_num() * rand_num(),
                                rand_num() * rand_num()
                            ),
                            center,
                            0.2
                        ));
                    }
                    choose_mat if choose_mat < 0.95 => {
                        list.list.push(make_sphere!(
                            Metal,
                            Vector3D::new(
                                0.5 * (1. + rand_num()),
                                0.5 * (1. + rand_num()),
                                0.5 * (1. + rand_num())
                            ),
                            0.5 * rand_num(),
                            center,
                            0.2
                        ));
                    }
                    _ => {
                        list.list.push(make_sphere!(Dialectric, 1.5, center, 0.2));
                    }
                }
            }
        }
    }
    list
}

pub fn color(r: &Ray, world: &Arc<HitableList>, depth: i32) -> Vector3D {
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

fn get_color(
    i: f32,
    j: f32,
    nx: f32,
    ny: f32,
    cam: &Arc<Camera>,
    world: &Arc<HitableList>,
) -> Vector3D {
    let u = (i as f32 + rand_num()) / (nx as f32);
    let v = (j as f32 + rand_num()) / (ny as f32);
    let r = cam.get_ray(u, v);
    color(&r, world, 0)
}

fn exec_worker(
    cam: &Arc<Camera>,
    world: &Arc<HitableList>,
    rx: Receiver<Option<(f32, f32, f32, f32)>>,
    cx: Sender<Option<Vector3D>>,
) {
    loop {
        match rx.recv().unwrap() {
            Some(arg) => {
                let r = get_color(arg.0, arg.1, arg.2, arg.3, cam, world);
                cx.send(Some(r)).unwrap();
            }
            None => {
                return;
            }
        }
    }
}

fn main() {
    let start = SystemTime::now();

    let nx = 1000;
    let ny = 500;
    let ns = 100;

    let filename = "out.ppm";
    let mut ppm = PPM::new(&filename, ny, nx, 256);

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
    let mut workers = vec![];
    let mut handles = vec![];
    let world_arc = Arc::new(world);
    let cam_arc = Arc::new(cam);
    let (calc_tx, calc_rx) = channel::<Option<Vector3D>>();

    for _ in 0..NUM_WORKERS {
        let world = world_arc.clone();
        let cam = cam_arc.clone();
        let (worker_tx, worker_rx) = channel::<Option<(f32, f32, f32, f32)>>();
        workers.push(worker_tx.clone());
        let c_tx = calc_tx.clone();
        handles.push(thread::spawn(move || {
            exec_worker(&cam, &world, worker_rx, c_tx)
        }));
    }

    for j in (0..ppm.height).rev() {
        println!("Row {}", j);
        for i in 0..ppm.width {
            let mut col = Vector3D::new(0., 0., 0.);
            for cnt in 0..ns {
                let offset = cnt % NUM_WORKERS;
                let req = workers[offset].clone();
                req.send(Some((i as f32, j as f32, nx as f32, ny as f32)))
                    .unwrap();
            }
            for _ in 0..ns {
                match calc_rx.recv().unwrap() {
                    Some(ret) => col += ret,
                    None => break,
                }
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

    for worker in workers {
        let req = worker.clone();
        req.send(None).unwrap();
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Finished in {} ms", start.elapsed().unwrap().as_millis());
}
