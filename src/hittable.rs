use crate::material::Material;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HitRecord<'material> {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: &'material Material,
}

impl<'material> HitRecord<'material> {
    pub fn new_empty() -> HitRecord<'material> {
        HitRecord {
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            material: &Material::None,
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

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Sphere>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: vec![] }
    }

    pub fn clear(&mut self) {
        self.objects = vec![];
    }

    pub fn add(&mut self, object: Sphere) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if let Some(record) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = record.t;
            } else {
                return None;
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        hittable::{HitRecord, HittableList},
        material::Material,
        ray::Ray,
        sphere::Sphere,
        vec3::Vec3,
    };

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

    #[test]
    fn new_hittable_list() {
        let list = HittableList::new();
        assert_eq!(list.objects.len(), 0);
    }

    #[test]
    fn add_hittable_list() {
        let mut list = HittableList::new();

        let item1 = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 100.0, Material::None);
        list.add(item1);

        assert_eq!(list.objects.len(), 1);

        let item2 = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 100.0, Material::None);
        list.add(item2);

        assert_eq!(list.objects.len(), 2);
    }

    #[test]
    fn clear_hittable_list() {
        let mut list = HittableList::new();

        let item1 = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 100.0, Material::None);
        list.add(item1);
        let item2 = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 100.0, Material::None);
        list.add(item2);

        list.clear();

        assert_eq!(list.objects.len(), 0);
    }
}
