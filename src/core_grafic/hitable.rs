use super::ray::Ray;
use super::vec3::Vec3;
use super::material::{Material, Metal};
use std::rc::Rc;

pub struct HitRecord{
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Rc<Material>,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord{
            t: 0.0,
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            material: Rc::new(Metal::new(Vec3::new(0.0, 0.0, 0.0), 0.0)),
        }
    }
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}