use crate::vec3::Point3;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Point3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Point3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use crate::Ray;
    use crate::Vec3;

    #[test]
    fn test_new() {
        let origin = Vec3::new(1.0, 2.0, 3.0);
        let direction = Vec3::new(4.0, 5.0, 6.0);
        let result = Ray::new(origin, direction);

        assert_eq!(result.origin.x, origin.x);
        assert_eq!(result.origin.y, origin.y);
        assert_eq!(result.origin.z, origin.z);
        assert_eq!(result.direction.x, direction.x);
        assert_eq!(result.direction.x, direction.x);
        assert_eq!(result.direction.x, direction.x);
    }

    #[test]
    fn test_at() {
        let origin = Vec3::new(1.0, 2.0, 3.0);
        let direction = Vec3::new(4.0, 5.0, 6.0);
        let ray = Ray::new(origin, direction);

        let result = ray.at(10.0);

        assert_eq!(result.x, 41.0);
        assert_eq!(result.y, 52.0);
        assert_eq!(result.z, 63.0);
    }
}
