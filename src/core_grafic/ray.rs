use super::vec3::{Vec3, RGB};

pub struct Ray {
    pub orig: Vec3,
    pub direct: Vec3,
}

impl<'a> Ray {
    pub fn new(a: &'a Vec3, b: &'a Vec3) -> Self {
        Ray{orig: Vec3::copy(a), direct: Vec3::copy(b)}
    }

    pub fn origin(&self) -> &Vec3 {
        &self.orig
    }
    
    pub fn direction(&self) -> &Vec3 {
        &self.direct
    }

    pub fn point_at(&self, t: f64) -> Vec3 { 
        let b = Vec3::new(self.direct[RGB::R], self.direct[RGB::G], self.direct[RGB::B]);
        let a = Vec3::new(self.orig[RGB::R], self.orig[RGB::G], self.orig[RGB::B]);
        a + b * t
    }
}