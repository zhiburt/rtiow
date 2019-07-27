use super::vec3::{Vec3, RGB, dot};
use super::ray::Ray;
use super::hitable::HitRecord;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;
}

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Vec3
}

impl Lambertian{
    pub fn new(a: Vec3) -> Self {
        Lambertian{albedo: a}
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let target = Vec3::copy(&rec.p) + Vec3::copy(&rec.normal) + random_in_unit_sphere();
        
        Some((Vec3::copy(&self.albedo), Ray::new_with_move(Vec3::copy(&rec.p), target - Vec3::copy(&rec.p))))
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let f = || Vec3::new(rand::random::<f64>(), rand::random::<f64>(), rand::random::<f64>()) * 2.0 - Vec3::new(1.0, 1.0, 1.0) ;
    let mut p = f();
    while p.squared_lenght() >= 1.0 {
        p = f();
    }

    return p;
}

#[derive(Clone)]
pub struct Metal {
    pub albedo: Vec3
}

impl Metal{
    pub fn new(a: Vec3) -> Self {
        Metal{albedo: a}
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(&Vec3::unit_vector(r_in.direction()), &rec.normal);
        let scattered  = Ray::new_with_move(Vec3::copy(&rec.p), reflected);
        let attenuation = Vec3::copy(&self.albedo);
        
        match dot(scattered.direction(), &rec.normal) > 0.0 {
            true => Some((attenuation, scattered)),
            _ => None
        }
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    Vec3::copy(v) - Vec3::copy(n) * dot(v, n) * 2.0
}