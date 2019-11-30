use crate::util::ray::Ray;
use crate::util::vector3d::Vector3D;

pub struct Camera {
    origin: Vector3D,
    lower_left_corner: Vector3D,
    horizontal: Vector3D,
    vertical: Vector3D,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            origin: Vector3D::new(0., 0., 0.),
            lower_left_corner: Vector3D::new(-2., -1., -1.),
            vertical: Vector3D::new(0., 2., 0.),
            horizontal: Vector3D::new(4., 0., 0.),
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let dest = self.lower_left_corner.clone()
            + self.horizontal.clone() * u
            + self.vertical.clone() * v
            - self.origin.clone();
        Ray::new(&self.origin, &dest)
    }
}
