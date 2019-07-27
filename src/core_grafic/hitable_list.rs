use super::hitable::{HitRecord, Hitable};
use super::ray::Ray;
use super::vec3::{dot, Vec3};

pub struct HitableList {
    list: Vec<Box<Hitable>>,
}

impl HitableList {
    pub fn new(v: Vec<Box<Hitable>>) -> HitableList {
        HitableList {
            list: v,
        }
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut record: HitRecord = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for obj in self.list.iter() {
            if obj.hit(r, t_min, closest_so_far, &mut record) {
                hit_anything = true;
                closest_so_far = record.t;
                rec.t = record.t; 
                rec.p = Vec3::copy(&record.p);
                rec.normal = Vec3::copy(&record.normal);
                rec.material = record.material.clone();
            }
        }

        return hit_anything;
    }
}