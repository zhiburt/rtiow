mod core_grafic;

use core_grafic::vec3::{RGB, Vec3};
use core_grafic::ray::Ray;

fn color(r: Ray) -> Vec3 {
    let unit_direction =  Vec3::unit_vector(r.direct);
    let (_, y, _) = unit_direction.xyz();
    let t = 0.5 *  (y  + 1.0);
    
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t)+ Vec3::new(0.5, 0.7, 1.9) * t
}

fn main() {
    let (nx, ny) = (200, 100);
    let offset = 255.99;
    println!("P3\n{} {} \n255", nx, ny);

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    for j in (0..ny).rev(){
        for i in 0..nx {
            let u = i as f64 / nx as f64;
            let v = j as f64 / ny as f64;

            let llc = Vec3::new(lower_left_corner[RGB::R], lower_left_corner[RGB::G], lower_left_corner[RGB::B]);
            let hor = Vec3::new(horizontal[RGB::R], horizontal[RGB::G], horizontal[RGB::B]);
            let ver = Vec3::new(vertical[RGB::R], vertical[RGB::G], vertical[RGB::B]);

            let t = llc + hor * u + ver * v;
            let r = Ray::new(&origin, &t);
            let vec = color(r);

            println!("{} {} {}", (vec[RGB::R] * offset) as isize, (vec[RGB::G] * offset) as isize, (vec[RGB::B] * offset) as isize);
        }
    }
}
