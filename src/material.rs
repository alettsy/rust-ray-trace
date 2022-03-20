use crate::{hittable::HitRecord, ray::Ray, vec3::Vec3};

pub trait Scatterable {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &Vec3, scattered: &Ray) -> bool;
}
