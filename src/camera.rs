use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, viewport_height: f64, focal_length: f64) -> Camera {
        let viewport_width = aspect_ratio * viewport_height;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::camera::Camera;
    use crate::ray::Ray;
    use crate::vec3::Vec3;

    #[test]
    fn new_camera() {
        let camera = Camera::new(2.0, 3.0, 1.0);

        assert_eq!(camera.origin, Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(camera.horizontal, Vec3::new(6.0, 0.0, 0.0));
        assert_eq!(camera.vertical, Vec3::new(0.0, 3.0, 0.0));
        assert_eq!(camera.lower_left_corner, Vec3::new(-3.0, -1.5, -1.0));
    }

    #[test]
    fn get_ray() {
        let camera = Camera::new(2.0, 3.0, 1.0);
        let result = camera.get_ray(10.0, 10.0);

        assert_eq!(
            result,
            Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(57.0, 28.5, -1.0))
        )
    }
}
