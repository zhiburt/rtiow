use super::vec3::Vec3;
use super::ray::Ray;

pub struct Camera {
    origin: Vec3,
    lofer_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Default for Camera {
    fn default() -> Self{
        Camera{
            lofer_left_corner: Vec3::new(-2.0, -1.0, -1.0),
            origin: Vec3::new(0.0, 0.0, 0.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
        }
    }
}

impl Camera {
    pub fn get_ray(&self, u: f64, v :f64) -> Ray {
        let llc = Vec3::copy(&self.lofer_left_corner);
        let h = Vec3::copy(&self.horizontal);
        let ver = Vec3::copy(&self.vertical);
        let orig = Vec3::copy(&self.origin);
        let r = Ray::new(
            &Vec3::copy(&self.origin),
            &Vec3::copy(&(llc + h * u + ver * v - orig)));
        
        return r;
    }
}