pub struct Vec3 {
    e: [f64;3]
}

impl Vec3 {
    pub fn new(e0:f64, e1:f64, e2:f64) -> Self {
        return Vec3{e: [e0, e1, e2]};
    }

    pub fn rgb(&self) -> (f64, f64, f64) {
        (self.e[0], self.e[1], self.e[2])
    }

    fn xyz(&self) -> (f64, f64, f64) {
        (self.e[0], self.e[1], self.e[2])
    }

    fn lenght(&self) -> f64 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
    }
    
    fn squared_lenght(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        let (lhs_r, lhs_g, lhs_b) = self.rgb();
        let (rhs_r, rhs_g, rhs_b) = rhs.rgb();

        Vec3::new(lhs_r + rhs_r, lhs_g + rhs_g, lhs_b + rhs_b)    
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];        
        self.e[1] += rhs.e[1];        
        self.e[2] += rhs.e[2];        
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self {
        let (lhs_r, lhs_g, lhs_b) = self.rgb();
        let (rhs_r, rhs_g, rhs_b) = rhs.rgb();

        Vec3::new(lhs_r - rhs_r, lhs_g - rhs_g, lhs_b - rhs_b)    
    }
}

impl std::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.e[0] -= rhs.e[0];        
        self.e[1] -= rhs.e[1];        
        self.e[2] -= rhs.e[2];        
    }
}

impl std::ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.e[0] *= rhs.e[0];        
        self.e[1] *= rhs.e[1];        
        self.e[2] *= rhs.e[2];        
    }
}

impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;        
        self.e[1] *= rhs;        
        self.e[2] *= rhs;        
    }
}

impl std::ops::DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.e[0] /= rhs.e[0];        
        self.e[1] /= rhs.e[1];        
        self.e[2] /= rhs.e[2];        
    }
}

impl std::ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.e[0] /= rhs;        
        self.e[1] /= rhs;        
        self.e[2] /= rhs;        
    }
}

pub enum RGB{
    R,
    G,
    B,
}

impl std::ops::Index<RGB> for Vec3 {
    type Output = f64;

    fn index(&self, rgb: RGB) -> &Self::Output {
        match rgb {
            RGB::R => &self.e[0],
            RGB::G => &self.e[1],
            RGB::B => &self.e[2],
        }
    }
}

impl std::cmp::PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.rgb() == other.rgb()
    }
}

impl std::cmp::Eq for Vec3 { }

impl std::fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (r, g, b) = self.rgb();
        write!(f, "r={}, g={}, b={}", r, g, b)
    }
}

fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
    v1[RGB::R] * v1[RGB::R] + v1[RGB::G] * v1[RGB::G] + v1[RGB::B] * v1[RGB::B]  
}

fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3::new(v1.e[1] * v2.e[2] - v1.e[2] * v2.e[1],
                    -(v1.e[0] * v2.e[2] - v1.e[2] * v2.e[0]),
                      v1.e[0] * v2.e[1] - v1.e[1] * v2.e[0])
}
