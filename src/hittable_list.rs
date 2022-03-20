use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: vec![] }
    }

    pub fn clear(&mut self) {
        self.objects = vec![];
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new_empty();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        hit_anything
    }
}

#[cfg(test)]
mod tests {
    use crate::{hittable_list::HittableList, sphere::Sphere, vec3::Vec3};

    #[test]
    fn new_hittable_list() {
        let list = HittableList::new();
        assert_eq!(list.objects.len(), 0);
    }

    #[test]
    fn add_hittable_list() {
        let mut list = HittableList::new();

        let item1 = Box::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 100.0));
        list.add(item1);

        assert_eq!(list.objects.len(), 1);

        let item2 = Box::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 100.0));
        list.add(item2);

        assert_eq!(list.objects.len(), 2);
    }

    #[test]
    fn clear_hittable_list() {
        let mut list = HittableList::new();

        let item1 = Box::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 100.0));
        list.add(item1);
        let item2 = Box::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 100.0));
        list.add(item2);

        list.clear();

        assert_eq!(list.objects.len(), 0);
    }
}
