use std::{fmt, ops};

use rand::{thread_rng, Rng};

pub type Color = Vec3;
pub type Point3 = Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    fn length(&self) -> f64 {
        (self.length_squared()).sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn random() -> Vec3 {
        let mut rng = thread_rng();
        Vec3 {
            x: rng.gen_range(0.0..=1.0),
            y: rng.gen_range(0.0..=1.0),
            z: rng.gen_range(0.0..=1.0),
        }
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        let mut rng = thread_rng();
        Vec3 {
            x: rng.gen_range(min..=max),
            y: rng.gen_range(min..=max),
            z: rng.gen_range(min..=max),
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    pub fn reflect(self, n: Vec3) -> Vec3 {
        self - 2.0 * self.dot(n) * n
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        other.x == self.x && other.y == self.y && other.z == self.z
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: rhs.x * self.x,
            y: rhs.y * self.y,
            z: rhs.z * self.z,
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of range!"),
        }
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of range!"),
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        if rhs == 0.0 {
            panic!("divide by zero error");
        } else {
            self * (1.0 / rhs)
        }
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        if rhs == 0.0 {
            panic!("divide by zero error");
        } else {
            *self *= 1.0 / rhs
        }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "X: {}, Y: {}, Z: {}", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use crate::vec3::Vec3;

    #[test]
    fn create_new_vec3() {
        let test = Vec3::new(43.0, 44.0, 45.0);
        assert_eq!(test.x, 43.0);
        assert_eq!(test.y, 44.0);
        assert_eq!(test.z, 45.0);
    }

    #[test]
    fn add_two_vec3() {
        let test1 = Vec3::new(43.0, 44.0, 45.0);
        let test2 = Vec3::new(5.0, 6.0, 7.0);
        let result = test1 + test2;
        assert_eq!(result.x, 48.0);
        assert_eq!(result.y, 50.0);
        assert_eq!(result.z, 52.0);
    }

    #[test]
    fn add_assign_two_vec3() {
        let mut test1 = Vec3::new(43.0, 44.0, 45.0);
        let test2 = Vec3::new(5.0, 6.0, 7.0);
        test1 += test2;
        assert_eq!(test1.x, 48.0);
        assert_eq!(test1.y, 50.0);
        assert_eq!(test1.z, 52.0);
    }

    #[test]
    fn mul_assign_vec3() {
        let mut test1 = Vec3::new(10.0, 11.0, 9.0);
        test1 *= 10.0;
        assert_eq!(test1.x, 100.0);
        assert_eq!(test1.y, 110.0);
        assert_eq!(test1.z, 90.0);
    }

    #[test]
    fn index_of_vec3() {
        let test1 = Vec3::new(10.0, 11.0, 9.0);
        assert_eq!(test1[0], 10.0);
        assert_eq!(test1[1], 11.0);
        assert_eq!(test1[2], 9.0);
    }

    #[test]
    #[should_panic]
    fn invalid_index_of_vec3() {
        let test1 = Vec3::new(10.0, 11.0, 9.0);
        assert_eq!(test1[4], 10.0);
    }

    #[test]
    fn neg_of_vec3() {
        let test1 = Vec3::new(10.0, 11.0, 9.0);
        let result = -test1;
        assert_eq!(result[0], -10.0);
        assert_eq!(result[1], -11.0);
        assert_eq!(result[2], -9.0);
    }

    #[test]
    fn div_assign_vec3() {
        let mut test1 = Vec3::new(10.0, 11.0, 9.0);
        test1 /= 10.0;
        assert_eq!(test1.x, 1.0);
        assert_eq!(test1.y, 1.1);
        assert_eq!(test1.z, 0.9);
    }

    #[test]
    #[should_panic]
    fn invalid_div_assign_vec3() {
        let mut test1 = Vec3::new(10.0, 11.0, 9.0);
        test1 /= 0.0;
    }

    #[test]
    fn length_squared() {
        let test1 = Vec3::new(10.0, 11.0, 9.0);
        let result = test1.length_squared();
        assert_eq!(result, 302.0);
    }

    #[test]
    fn length() {
        let test1 = Vec3::new(10.0, 11.0, 9.0);
        let result = test1.length();
        assert_eq!(result, 17.378147196982766);
    }

    #[test]
    fn sub_f64_vec3() {
        let test1 = Vec3::new(43.0, 44.0, 45.0);
        let test2 = Vec3::new(3.0, 6.0, 6.0);
        let result = test1 - test2;
        assert_eq!(result.x, 40.0);
        assert_eq!(result.y, 38.0);
        assert_eq!(result.z, 39.0);
    }

    #[test]
    fn div_f64_vec3() {
        let test1 = Vec3::new(45.0, 40.0, 35.0);
        let result = test1 / 5.0;
        assert_eq!(result.x, 9.0);
        assert_eq!(result.y, 8.0);
        assert_eq!(result.z, 7.0);
    }

    #[test]
    #[should_panic]
    fn invalid_div_f64_vec3() {
        let test1 = Vec3::new(45.0, 40.0, 35.0);
        let _result = test1 / 0.0;
    }

    #[test]
    fn mul_f64_vec3_left() {
        let test1 = Vec3::new(5.0, 6.0, 7.0);
        let result = test1 * 5.0;
        assert_eq!(result.x, 25.0);
        assert_eq!(result.y, 30.0);
        assert_eq!(result.z, 35.0);
    }

    #[test]
    fn mul_f64_vec3_right() {
        let test1 = Vec3::new(5.0, 6.0, 7.0);
        let result = 5.0 * test1;
        assert_eq!(result.x, 25.0);
        assert_eq!(result.y, 30.0);
        assert_eq!(result.z, 35.0);
    }

    #[test]
    fn mul_vec3_vec3() {
        let test1 = Vec3::new(5.0, 6.0, 7.0);
        let test2 = Vec3::new(1.0, 2.0, 3.0);
        let result = test1 * test2;
        assert_eq!(result.x, 5.0);
        assert_eq!(result.y, 12.0);
        assert_eq!(result.z, 21.0);
    }

    #[test]
    fn cross() {
        let test1 = Vec3::new(5.0, 6.0, 7.0);
        let test2 = Vec3::new(1.0, 2.0, 3.0);
        let result = test1.cross(test2);
        assert_eq!(result.x, 4.0);
        assert_eq!(result.y, -8.0);
        assert_eq!(result.z, 4.0);
    }

    #[test]
    fn dot() {
        let test1 = Vec3::new(5.0, 6.0, 7.0);
        let test2 = Vec3::new(1.0, 2.0, 3.0);
        let result = test1.dot(test2);
        assert_eq!(result, 38.0);
    }

    #[test]
    fn equality() {
        let test1 = Vec3::new(5.0, 6.0, 7.0);
        let test2 = Vec3::new(1.0, 2.0, 3.0);
        assert_ne!(test1, test2);

        let test3 = Vec3::new(5.0, 6.0, 7.0);
        assert_eq!(test1, test3);
    }
}
