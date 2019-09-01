extern crate rand;

use super::vec3::{Vec3, cross};
use super::ray::Ray;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64) -> Camera {
        let theta = vfov * std::f64::consts::PI/180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        
        let w = Vec3::unit_vector(&(Vec3::copy(&lookfrom) - Vec3::copy(&lookat)));
        let u = Vec3::unit_vector(&cross(&vup, &w));
        let v = cross(&w, &u);

        let origin = lookfrom;
        let lower_left_corner = Vec3::copy(&origin) - Vec3::copy(&u) * half_width * focus_dist - Vec3::copy(&v) * half_height * focus_dist - Vec3::copy(&w) * focus_dist;
        let horizontal = Vec3::copy(&u) * 2.0 * half_width * focus_dist;
        let vertical =  Vec3::copy(&v) * 2.0 * half_height * focus_dist;
        Camera{
            lower_left_corner: lower_left_corner,
            horizontal: horizontal,
            vertical: vertical,
            origin: origin,
            u: u,
            v: v,
            w: w,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: f64, t :f64) -> Ray {
        let rd = Camera::random_in_unit_disk() * self.lens_radius;
        let offset = Vec3::copy(&self.u) * rd.xyz().0 + Vec3::copy(&self.v) * rd.xyz().1;
        let llc = Vec3::copy(&self.lower_left_corner);
        let h = Vec3::copy(&self.horizontal);
        let ver = Vec3::copy(&self.vertical);
        let orig = Vec3::copy(&self.origin);
        
        Ray::new(
            &(Vec3::copy(&self.origin) +  Vec3::copy(&offset)),
            &Vec3::copy(&(llc + h * s + ver * t - orig - offset)))
    }

    fn random_in_unit_disk() -> Vec3 {
        let f = || {
            let rnd_vec = Vec3::new(rand::random::<f64>(), rand::random::<f64>(), 0.0);
            rnd_vec * 2.0 - Vec3::new(1.0, 1.0, 0.0)
        };

        let mut p = f();
        while super::vec3::dot(&p, &p) >= 1.0 {
            p = f();
        }

        return p;
    }
}