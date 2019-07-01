use super::vec3::{Vec3, RGB};

pub struct Ray<'a> {
    pub orig: &'a Vec3,
    pub direct: &'a Vec3,
}

impl<'a> Ray<'a> {
    pub fn new(a: &'a Vec3, b: &'a Vec3) -> Self {
        Ray{orig: a, direct: b}
    }

    pub fn origin(&self) -> &Vec3 {
        self.orig
    }
    
    pub fn direction(&self) -> &Vec3 {
        self.direct
    }

    pub fn point_at(&self, t: f64) -> Vec3 { 
        let b = Vec3::new(self.direct[RGB::R], self.direct[RGB::G], self.direct[RGB::B]);
        let a = Vec3::new(self.orig[RGB::R], self.orig[RGB::G], self.orig[RGB::B]);
        a + b * t
    }
}