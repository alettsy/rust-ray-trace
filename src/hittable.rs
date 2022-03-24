use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Material,
}

impl HitRecord {
    pub fn new_empty() -> HitRecord {
        HitRecord {
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            material: Material::None,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[cfg(test)]
mod tests {
    use crate::{hittable::HitRecord, ray::Ray, vec3::Vec3};

    #[test]
    fn new_empty_hit_record() {
        let empty = HitRecord::new_empty();

        assert_eq!(empty.p, Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(empty.normal, Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(empty.t, 0.0);
        assert!(!empty.front_face);
    }

    #[test]
    fn set_face_normal_hit_record_false() {
        let mut record = HitRecord::new_empty();
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
        let normal = Vec3::new(10.0, 10.0, 10.0);

        record.set_face_normal(&ray, normal);

        assert_eq!(record.normal, Vec3::new(-10.0, -10.0, -10.0));
    }

    #[test]
    fn set_face_normal_hit_record_true() {
        let mut record = HitRecord::new_empty();
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(-1.0, -1.0, -1.0));
        let normal = Vec3::new(10.0, 10.0, 10.0);

        record.set_face_normal(&ray, normal);

        assert_eq!(record.normal, Vec3::new(10.0, 10.0, 10.0));
    }
}
