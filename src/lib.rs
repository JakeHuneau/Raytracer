use std::ops::*;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub e: [f32; 3]
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { e: [x, y, z] }
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

    pub fn squared_lenght(&self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn make_unit_vector(&mut self) {
        let k = 1.0 / self.length();
        self.e[0] *= k;
        self.e[1] *= k;
        self.e[2] *= k;
    }

    pub fn dot(&self, v2: &Vec3) -> f32 {
        self.e[0] * v2.e[1] + self.e[1] * v2.e[1] + self.e[2] * v2.e[2]
    }
    
    pub fn cross(&self, v2: &Vec3) -> Vec3 {
        Vec3 { e: [
            self.e[1] * v2.e[2] - self.e[2] * v2.e[1],
            -(self.e[0] * v2.e[2] - self.e[2] * v2.e[0]),
            self.e[0] * v2.e[1] - self.e[1] * v2.e[0]
        ]}
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, v2: Vec3) -> Vec3 {
        Vec3 { e: [self.e[0] + v2.e[0], self.e[1] + v2.e[1], self.e[2] + v2.e[2]] }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, v2: Vec3) -> Vec3 {
        Vec3 { e: [self.e[0] - v2.e[0], self.e[1] - v2.e[1], self.e[2] - v2.e[2]] }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, v2: Vec3) -> Vec3 {
        Vec3 { e: [self.e[0] * v2.e[0], self.e[1] * v2.e[1], self.e[2] * v2.e[2]] }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, c: f32) -> Vec3 {
        Vec3 { e: [self.e[0] * c, self.e[1] * c, self.e[2] * c] }
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, v2: Vec3) -> Vec3 {
        Vec3 { e: [self.e[0] / v2.e[0], self.e[1] / v2.e[1], self.e[2] / v2.e[2]] }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, c: f32) -> Vec3 {
        Vec3 { e: [self.e[0] / c, self.e[1] / c, self.e[2] / c] }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 { e: [-self.e[0], -self.e[1], -self.e[2]] }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, v2: Vec3) {
        self.e[0] += v2.e[0];
        self.e[1] += v2.e[1];
        self.e[2] += v2.e[2];
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, v2: Vec3) {
        self.e[0] -= v2.e[0];
        self.e[1] -= v2.e[1];
        self.e[2] -= v2.e[2];
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, v2: Vec3) {
        self.e[0] *= v2.e[0];
        self.e[1] *= v2.e[1];
        self.e[2] *= v2.e[2];
    }
}

impl DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, v2: Vec3) {
        self.e[0] /= v2.e[0];
        self.e[1] /= v2.e[1];
        self.e[2] /= v2.e[2];
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, c: f32) {
        self.e[0] *= c;
        self.e[1] *= c;
        self.e[2] *= c;
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, c: f32) {
        self.e[0] /= c;
        self.e[1] /= c;
        self.e[2] /= c;
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length() {
        let v = Vec3{ e: [1.0, 2.0, 3.0]};
        let r: f32 = 14.0;
        assert_eq!(v.length(), r.sqrt());
    }
}