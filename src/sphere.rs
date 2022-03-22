use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut empty_record = HitRecord::new_empty();

        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;

        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        empty_record.t = root;
        empty_record.p = r.at(empty_record.t);
        let outward_normal = (empty_record.p - self.center) / self.radius;
        empty_record.set_face_normal(r, outward_normal);
        empty_record.material = self.material;

        return Some(empty_record);
    }
}

#[cfg(test)]
mod tests {
    use crate::{material::Material, sphere::Sphere, vec3::Vec3};

    #[test]
    fn new_sphere() {
        let sphere = Sphere::new(Vec3::new(10.0, 11.0, 12.0), 50.0, Material::None);

        assert_eq!(sphere.center, Vec3::new(10.0, 11.0, 12.0));
        assert_eq!(sphere.radius, 50.0);
    }
}
