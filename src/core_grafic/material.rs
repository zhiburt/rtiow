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
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Metal{
    pub fn new(a: Vec3, fuzz: f64) -> Self {
        Metal{albedo: a, fuzz}
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(&Vec3::unit_vector(r_in.direction()), &rec.normal);
        let scattered  = Ray::new_with_move(Vec3::copy(&rec.p), reflected + random_in_unit_sphere() * self.fuzz);
        let attenuation = Vec3::copy(&self.albedo);
        
        match dot(scattered.direction(), &rec.normal) > 0.0 {
            true => Some((attenuation, scattered)),
            _ => None
        }
    }
}

#[derive(Clone)]
pub struct Dielectric {
    pub ref_index: f64,
}

impl Dielectric {
    pub fn new(ref_index: f64) -> Self {
        Dielectric {ref_index}
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let mut outword_normal: Vec3;
        let mut  ni_over_nt = 0.0;
        let mut cosine = 0.0;
        let mut reflect_prob = 0.0;
        let reflected = reflect(r_in.direction(), &rec.normal);
        if dot(r_in.direction(), &rec.normal) > 0.0 {
            outword_normal =  Vec3::copy(&rec.normal) * -1.0;
            ni_over_nt = self.ref_index;
            cosine = self.ref_index * dot(r_in.direction(), &rec.normal) / r_in.direction().length();
        } else {
            outword_normal =  Vec3::copy(&rec.normal);
            ni_over_nt = 1.0 / self.ref_index;
            cosine = (-1.0 * dot(r_in.direction(), &rec.normal)) / r_in.direction().length();
        }

        let refracted = match refract(r_in.direction(), &outword_normal, ni_over_nt) {
            Some(r) => {
                reflect_prob = schlick(cosine, self.ref_index);
              
                r
            },
            None => {
                reflect_prob = 1.0;
              
                Vec3::new(0.0, 0.0, 0.0)
            }
        };

        match rand::random::<f64>() < reflect_prob {
            true => Some((Vec3::new(1., 1., 1.), Ray::new_with_move(Vec3::copy(&rec.p), reflected))),
            false => Some((Vec3::new(1., 1., 1.), Ray::new_with_move(Vec3::copy(&rec.p), refracted))),
        }
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    Vec3::copy(v) - Vec3::copy(n) * dot(v, n) * 2.0
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = Vec3::unit_vector(v);
    let dt =  dot(&uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt*dt);
    if discriminant > 0.0 {
        let cn = Vec3::copy(n);
        let cn1 = Vec3::copy(n);
        return Some((uv - cn*dt) * ni_over_nt - cn1 * discriminant.sqrt())
    }

    return Option::None;
}

fn schlick(cosine: f64, ref_index: f64) -> f64 {
    let mut r0 = (1.0 - ref_index) / (1.0 + ref_index);
    r0 = r0 * r0;
    
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}