use super::vec3::{Vec3, cross};
use super::ray::Ray;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect: f64) -> Camera {
        let theta = vfov * std::f64::consts::PI;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        
        let w = Vec3::unit_vector(&(Vec3::copy(&lookfrom) - Vec3::copy(&lookat)));
        let u = Vec3::unit_vector(&cross(&vup, &w));
        let v = cross(&w, &u);

        let origin = lookfrom;
        // let lower_left_corner = Vec3::new(-half_width, -half_height, -1.0);
        let lower_left_corner = Vec3::copy(&origin) - Vec3::copy(&u) * half_width - Vec3::copy(&v) * half_height - w;
        let horizontal = u * 2.0 * half_width;
        let vertical =  v * 2.0 * half_height;
        Camera{
            lower_left_corner: lower_left_corner,
            horizontal: horizontal,
            vertical: vertical,
            origin: origin,
        }
    }

    pub fn get_ray(&self, u: f64, v :f64) -> Ray {
        let llc = Vec3::copy(&self.lower_left_corner);
        let h = Vec3::copy(&self.horizontal);
        let ver = Vec3::copy(&self.vertical);
        let orig = Vec3::copy(&self.origin);
        let r = Ray::new(
            &Vec3::copy(&self.origin),
            &Vec3::copy(&(llc + h * u + ver * v - orig)));
        
        return r;
    }
}