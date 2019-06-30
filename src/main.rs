mod core_grafic;

use core_grafic::vec3::{RGB, Vec3};

fn main() {
    let (nx, ny) = (200, 100);
    let offset = 255.99;
    println!("P3\n{} {} \n255", nx, ny);
    for j in (0..ny).rev(){
        for i in 0..nx {
            let vec = Vec3::new(i as f64 / nx as f64, j as f64 / ny as f64, 0.2);

            println!("{} {} {}", (vec[RGB::R] * offset) as isize, (vec[RGB::G] * offset) as isize, (vec[RGB::B] * offset) as isize);
        }
    }

    let a = core_grafic::vec3::Vec3::new(1.0, 1.0, 1.0);

    println!("{}", a.rgb().1);
}
