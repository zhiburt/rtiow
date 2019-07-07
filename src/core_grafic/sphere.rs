use super::hitable::{HitRecord, Hitable};
use super::ray::Ray;
use super::vec3::{dot, Vec3};

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(c: Vec3, r: f64) -> Sphere {
        Sphere {
            center: c,
            radius: r,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let (ray_origin_cp, center_cp) = (Vec3::copy(r.origin()), Vec3::copy(&self.center));

        let oc = &(ray_origin_cp - center_cp);
        let a = dot(r.direction(), r.direction());
        let b = dot(oc, r.direction());
        let c = dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let mut temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at(rec.t);
                rec.normal = (Vec3::copy(&rec.p) - Vec3::copy(&self.center)) / self.radius;

                return true;
            }
            temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at(rec.t);
                rec.normal = (Vec3::copy(&rec.p) - Vec3::copy(&self.center)) / self.radius;

                return true;
            }
        }

        return false;
    }
}