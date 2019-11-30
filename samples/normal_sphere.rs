use util::ppm::PPM;
use util::ray::Ray;
use util::vector3d::{unit_vector, Vector3D};

pub fn hit_sphere(center: &Vector3D, radius: f32, r: &Ray) -> f32 {
    let oc = r.origin() - *center;
    let a = r.direction().dot(&r.direction());
    let b = oc.dot(&r.direction()) * 2.0;
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    let res = discriminant < 0.0;
    match res {
        true => -1.0,
        false => (-b - discriminant.sqrt()) / (2.0 * a),
    }
}

pub fn color(r: &Ray) -> Vector3D {
    let center = Vector3D {
        e: [0.0, 0.0, -1.0],
    };
    let t = hit_sphere(&center, 0.5, &r);
    let res = t > 0.0;
    match res {
        true => {
            let d = r.point_at_parameter(t)
                - Vector3D {
                    e: [0.0, 0.0, -1.0],
                };
            let normal = unit_vector(&d);
            Vector3D {
                e: [normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0],
            } * 0.5
        }
        false => {
            let direction = r.direction();
            let unit_direction = unit_vector(&direction);

            let t: f32 = 0.5 * (unit_direction.y() + 1.0);
            let v1 = Vector3D { e: [1.0, 1.0, 1.0] } * (1.0 - t);
            let v2 = Vector3D { e: [0.5, 0.7, 1.0] } * t;
            v1 + v2
        }
    }
}

fn main() {
    let filename = "out.ppm";
    let mut ppm = PPM::new(&filename, 500, 1000, 256);

    let lower_left_corner = Vector3D {
        e: [-2.0, -1.0, -1.0],
    };
    let horizontal = Vector3D { e: [4.0, 0.0, 0.0] };
    let vertical = Vector3D { e: [0.0, 2.0, 0.0] };
    let origin = Vector3D { e: [0.0, 0.0, 0.0] };

    for j in (0..ppm.height - 1).rev() {
        for i in 0..ppm.width {
            let u = (i as f32) / (ppm.width as f32);
            let v = (j as f32) / (ppm.height as f32);
            let dest = lower_left_corner + horizontal * u + vertical * v;
            let r = Ray::new(&origin, &dest);
            let col = color(&r);

            let v1 = [
                (ppm.max as f32 * col.e[0]) as u32,
                (ppm.max as f32 * col.e[1]) as u32,
                (ppm.max as f32 * col.e[2]) as u32,
            ];
            ppm.write_row(&v1);
        }
    }
}
