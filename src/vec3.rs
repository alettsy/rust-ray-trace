use std::{fmt, ops};

pub type Color = Vec3;
pub type Point3 = Vec3;

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { r: x, g: y, b: z }
    }

    fn length(&self) -> f64 {
        (self.length_squared()).sqrt()
    }

    fn length_squared(&self) -> f64 {
        self.r * self.r + self.g * self.g + self.b * self.b
    }

    fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            r: self.g * other.b - self.b * other.g,
            g: self.b * other.r - self.r * other.b,
            b: self.r * other.g - self.g * other.r,
        }
    }

    fn dot(self, other: Vec3) -> f64 {
        self.r * other.r + self.g * other.g + self.b * other.b
    }

    pub fn unit_vector(v: Vec3) -> Vec3 {
        v / v.length()
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.r = self.r + rhs.r;
        self.g = self.g + rhs.g;
        self.b = self.b + rhs.b;
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3 {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            r: rhs.r * self,
            g: rhs.g * self,
            b: rhs.b * self,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            r: rhs.r * self.r,
            g: rhs.g * self.g,
            b: rhs.b * self.b,
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.r = self.r * rhs;
        self.g = self.g * rhs;
        self.b = self.b * rhs;
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        match index {
            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            _ => panic!("Index out of range!"),
        }
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        match index {
            0 => &mut self.r,
            1 => &mut self.g,
            2 => &mut self.b,
            _ => panic!("Index out of range!"),
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            r: self.r * -1.0,
            g: self.g * -1.0,
            b: self.b * -1.0,
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
        write!(f, "X: {}, Y: {}, Z: {}", self.r, self.g, self.b)
    }
}

#[cfg(test)]
mod tests {
    use crate::Vec3;

    #[test]
    fn create_new_vec3() {
        let test = Vec3::new(43.0, 44.0, 45.0);
        assert_eq!(test.r, 43.0);
        assert_eq!(test.g, 44.0);
        assert_eq!(test.b, 45.0);
    }

    #[test]
    fn add_two_vec3() {
        let test1 = Vec3::new(43.0, 44.0, 45.0);
        let test2 = Vec3::new(5.0, 6.0, 7.0);
        let result = test1 + test2;
        assert_eq!(result.r, 48.0);
        assert_eq!(result.g, 50.0);
        assert_eq!(result.b, 52.0);
    }

    #[test]
    fn add_assign_two_vec3() {
        let mut test1 = Vec3::new(43.0, 44.0, 45.0);
        let test2 = Vec3::new(5.0, 6.0, 7.0);
        test1 += test2;
        assert_eq!(test1.r, 48.0);
        assert_eq!(test1.g, 50.0);
        assert_eq!(test1.b, 52.0);
    }

    #[test]
    fn mul_assign_vec3() {
        let mut test1 = Vec3::new(10.0, 11.0, 9.0);
        test1 *= 10.0;
        assert_eq!(test1.r, 100.0);
        assert_eq!(test1.g, 110.0);
        assert_eq!(test1.b, 90.0);
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
        assert_eq!(test1.r, 1.0);
        assert_eq!(test1.g, 1.1);
        assert_eq!(test1.b, 0.9);
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
        assert_eq!(result.r, 40.0);
        assert_eq!(result.g, 38.0);
        assert_eq!(result.b, 39.0);
    }

    #[test]
    fn div_f64_vec3() {
        let test1 = Vec3::new(45.0, 40.0, 35.0);
        let result = test1 / 5.0;
        assert_eq!(result.r, 9.0);
        assert_eq!(result.g, 8.0);
        assert_eq!(result.b, 7.0);
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
        assert_eq!(result.r, 25.0);
        assert_eq!(result.g, 30.0);
        assert_eq!(result.b, 35.0);
    }

    #[test]
    fn mul_f64_vec3_right() {
        let test1 = Vec3::new(5.0, 6.0, 7.0);
        let result = 5.0 * test1;
        assert_eq!(result.r, 25.0);
        assert_eq!(result.g, 30.0);
        assert_eq!(result.b, 35.0);
    }

    #[test]
    fn mul_vec3_vec3() {
        let test1 = Vec3::new(5.0, 6.0, 7.0);
        let test2 = Vec3::new(1.0, 2.0, 3.0);
        let result = test1 * test2;
        assert_eq!(result.r, 5.0);
        assert_eq!(result.g, 12.0);
        assert_eq!(result.b, 21.0);
    }

    #[test]
    fn cross() {
        let test1 = Vec3::new(5.0, 6.0, 7.0);
        let test2 = Vec3::new(1.0, 2.0, 3.0);
        let result = test1.cross(test2);
        assert_eq!(result.r, 4.0);
        assert_eq!(result.g, -8.0);
        assert_eq!(result.b, 4.0);
    }

    #[test]
    fn dot() {
        let test1 = Vec3::new(5.0, 6.0, 7.0);
        let test2 = Vec3::new(1.0, 2.0, 3.0);
        let result = test1.dot(test2);
        assert_eq!(result, 38.0);
    }
}
