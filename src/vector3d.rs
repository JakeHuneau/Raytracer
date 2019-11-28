use std::fmt;
use std::ops::*;

#[derive(Clone, Copy, Debug)]
pub struct Vector3D {
    pub e: [f32; 3],
}

impl Vector3D {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3D {
        Vector3D { e: [x, y, z] }
    }

    pub fn x(&self) -> f32 {
        self.e[0]
    }

    pub fn y(&self) -> f32 {
        self.e[1]
    }

    pub fn z(&self) -> f32 {
        self.e[2]
    }

    pub fn r(&self) -> f32 {
        self.e[0]
    }

    pub fn g(&self) -> f32 {
        self.e[1]
    }

    pub fn b(&self) -> f32 {
        self.e[2]
    }

    pub fn length(&self) -> f32 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
    }

    pub fn squared_length(&self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn make_unit_vector(&mut self) {
        let k = 1.0 / self.length();
        self.e[0] *= k;
        self.e[1] *= k;
        self.e[2] *= k;
    }

    pub fn dot(&self, v2: &Vector3D) -> f32 {
        self.e[0] * v2.e[0] + self.e[1] * v2.e[1] + self.e[2] * v2.e[2]
    }

    pub fn cross(&self, v2: &Vector3D) -> Vector3D {
        Vector3D {
            e: [
                self.e[1] * v2.e[2] - self.e[2] * v2.e[1],
                -(self.e[0] * v2.e[2] - self.e[2] * v2.e[0]),
                self.e[0] * v2.e[1] - self.e[1] * v2.e[0],
            ],
        }
    }
}

impl Add for Vector3D {
    type Output = Vector3D;

    fn add(self, v2: Vector3D) -> Vector3D {
        Vector3D {
            e: [
                self.e[0] + v2.e[0],
                self.e[1] + v2.e[1],
                self.e[2] + v2.e[2],
            ],
        }
    }
}

impl Sub for Vector3D {
    type Output = Vector3D;

    fn sub(self, v2: Vector3D) -> Vector3D {
        Vector3D {
            e: [
                self.e[0] - v2.e[0],
                self.e[1] - v2.e[1],
                self.e[2] - v2.e[2],
            ],
        }
    }
}

impl Mul<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn mul(self, v2: Vector3D) -> Vector3D {
        Vector3D {
            e: [
                self.e[0] * v2.e[0],
                self.e[1] * v2.e[1],
                self.e[2] * v2.e[2],
            ],
        }
    }
}

impl Mul<f32> for Vector3D {
    type Output = Vector3D;

    fn mul(self, c: f32) -> Vector3D {
        Vector3D {
            e: [self.e[0] * c, self.e[1] * c, self.e[2] * c],
        }
    }
}

impl Div<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn div(self, v2: Vector3D) -> Vector3D {
        Vector3D {
            e: [
                self.e[0] / v2.e[0],
                self.e[1] / v2.e[1],
                self.e[2] / v2.e[2],
            ],
        }
    }
}

impl Div<f32> for Vector3D {
    type Output = Vector3D;

    fn div(self, c: f32) -> Vector3D {
        Vector3D {
            e: [self.e[0] / c, self.e[1] / c, self.e[2] / c],
        }
    }
}

impl Neg for Vector3D {
    type Output = Vector3D;

    fn neg(self) -> Vector3D {
        Vector3D {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl AddAssign for Vector3D {
    fn add_assign(&mut self, v2: Vector3D) {
        self.e[0] += v2.e[0];
        self.e[1] += v2.e[1];
        self.e[2] += v2.e[2];
    }
}

impl SubAssign for Vector3D {
    fn sub_assign(&mut self, v2: Vector3D) {
        self.e[0] -= v2.e[0];
        self.e[1] -= v2.e[1];
        self.e[2] -= v2.e[2];
    }
}

impl MulAssign<Vector3D> for Vector3D {
    fn mul_assign(&mut self, v2: Vector3D) {
        self.e[0] *= v2.e[0];
        self.e[1] *= v2.e[1];
        self.e[2] *= v2.e[2];
    }
}

impl DivAssign<Vector3D> for Vector3D {
    fn div_assign(&mut self, v2: Vector3D) {
        self.e[0] /= v2.e[0];
        self.e[1] /= v2.e[1];
        self.e[2] /= v2.e[2];
    }
}

impl MulAssign<f32> for Vector3D {
    fn mul_assign(&mut self, c: f32) {
        self.e[0] *= c;
        self.e[1] *= c;
        self.e[2] *= c;
    }
}

impl DivAssign<f32> for Vector3D {
    fn div_assign(&mut self, c: f32) {
        self.e[0] /= c;
        self.e[1] /= c;
        self.e[2] /= c;
    }
}

impl PartialEq for Vector3D {
    fn eq(&self, v2: &Vector3D) -> bool {
        self.e[0] == v2.e[0] && self.e[1] == v2.e[1] && self.e[2] == v2.e[2]
    }
}

impl fmt::Display for Vector3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.e[0], self.e[1], self.e[2])
    }
}

pub fn unit_vector(v: &Vector3D) -> Vector3D {
    *v / v.length()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_getters() {
        let v = Vector3D { e: [1.0, 2.0, 3.0] };
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
        assert_eq!(v.r(), 1.0);
        assert_eq!(v.g(), 2.0);
        assert_eq!(v.b(), 3.0);
    }

    #[test]
    fn test_length() {
        let v = Vector3D { e: [1.0, 2.0, 3.0] };
        let r: f32 = 14.0;
        assert_eq!(v.length(), r.sqrt());
    }

    #[test]
    fn test_squared_length() {
        let v = Vector3D { e: [1.0, 2.0, 3.0] };
        assert_eq!(v.squared_length(), 14.0);
    }

    #[test]
    fn test_make_unit_vector() {
        let mut v = Vector3D { e: [1.0, 2.0, 3.0] };
        let v_len = v.length();
        v.make_unit_vector();
        let v2 = Vector3D {
            e: [1.0 / v_len, 2.0 / v_len, 3.0 / v_len],
        };
        assert_eq!(v, v2);
        let v = Vector3D { e: [1.0, 2.0, 3.0] };
        assert_eq!(unit_vector(&v), v2);
    }

    #[test]
    fn test_dot() {
        let v = Vector3D { e: [1.0, 2.0, 3.0] };
        assert_eq!(v.dot(&v), 14.0);
    }

    #[test]
    fn test_cross() {
        let v = Vector3D { e: [1.0, 2.0, 3.0] };
        let v2 = Vector3D { e: [4.0, 5.0, 6.0] };
        let v3 = Vector3D {
            e: [-3.0, 6.0, -3.0],
        };
        assert_eq!(v.cross(&v2), v3);
    }

    #[test]
    fn test_addition() {
        let mut v = Vector3D { e: [1.0, 2.0, 3.0] };
        let v2 = Vector3D { e: [4.0, 5.0, 6.0] };
        let v3 = Vector3D { e: [5.0, 7.0, 9.0] };
        assert_eq!(v + v2, v3);
        v += v2;
        assert_eq!(v, v3);
    }

    #[test]
    fn test_subtraction() {
        let mut v = Vector3D { e: [1.0, 2.0, 3.0] };
        let v2 = Vector3D { e: [4.0, 5.0, 6.0] };
        let v3 = Vector3D {
            e: [-3.0, -3.0, -3.0],
        };
        assert_eq!(v - v2, v3);
        v -= v2;
        assert_eq!(v, v3);
    }

    #[test]
    fn test_multiplication() {
        let mut v = Vector3D { e: [1.0, 2.0, 3.0] };
        let v2 = Vector3D { e: [4.0, 5.0, 6.0] };
        let v3 = Vector3D {
            e: [4.0, 10.0, 18.0],
        };
        let k = 10.0;
        let v4 = Vector3D {
            e: [10.0, 20.0, 30.0],
        };
        assert_eq!(v * v2, v3);
        assert_eq!(v * k, v4);
        v *= v2;
        assert_eq!(v, v3);
        let mut v = Vector3D { e: [1.0, 2.0, 3.0] };
        v *= k;
        assert_eq!(v, v4);
    }

    #[test]
    fn test_division() {
        let mut v = Vector3D { e: [1.0, 2.0, 3.0] };
        let v2 = Vector3D { e: [4.0, 5.0, 6.0] };
        let v3 = Vector3D {
            e: [0.25, 2.0 / 5.0, 3.0 / 6.0],
        };
        let k = 10.0;
        let v4 = Vector3D { e: [0.1, 0.2, 0.3] };
        assert_eq!(v / v2, v3);
        assert_eq!(v / k, v4);
        v /= v2;
        assert_eq!(v, v3);
        let mut v = Vector3D { e: [1.0, 2.0, 3.0] };
        v /= k;
        assert_eq!(v, v4);
    }
}
