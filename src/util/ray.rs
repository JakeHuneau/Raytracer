use super::vector3d::Vector3D;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    a: Vector3D,
    b: Vector3D,
}

impl Ray {
    pub fn new(a: &Vector3D, b: &Vector3D) -> Self {
        Self {
            a: a.clone(),
            b: b.clone(),
        }
    }

    pub fn origin(&self) -> Vector3D {
        self.a
    }

    pub fn direction(&self) -> Vector3D {
        self.b
    }

    pub fn point_at_parameter(&self, t: f32) -> Vector3D {
        self.a.clone() + self.b.clone() * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor() {
        let v1 = Vector3D { e: [1.0, 1.0, 1.0] };
        let v2 = Vector3D { e: [2.0, 2.0, 2.0] };
        let ray = Ray::new(&v1, &v2);
        assert_eq!(ray.origin(), v1);
        assert_eq!(ray.direction(), v2);
    }

    #[test]
    fn test_point_at_parameter() {
        let v1 = Vector3D { e: [1.0, 1.0, 1.0] };
        let v2 = Vector3D { e: [2.0, 2.0, 2.0] };
        let ray = Ray::new(&v1, &v2);
        let v3 = Vector3D { e: [3.0, 3.0, 3.0] };
        assert_eq!(ray.point_at_parameter(1.0), v3);
    }
}
