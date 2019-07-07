extern crate rand;
use rand::Rng;

mod core_grafic;

use core_grafic::vec3::{RGB, Vec3, dot};
use core_grafic::ray::Ray;
use core_grafic::hitable::{Hitable, HitRecord};
use core_grafic::sphere::Sphere;
use core_grafic::hitable_list::HitableList;
use core_grafic::camera::Camera;

fn color(r: &Ray, world: &mut Hitable) -> Vec3 {
    let mut rec = HitRecord{t: 0.0, p: Vec3::new(0.0, 0.0, 0.0), normal: Vec3::new(0.0, 0.0, 0.0)};
    if world.hit(r, 0.0, std::f64::MAX, &mut rec) {
        return Vec3::new(rec.normal.xyz().0+1.0, rec.normal.xyz().1+1.0, rec.normal.xyz().2+1.0) * 0.5;
    }

    let unit_direction =  Vec3::unit_vector(&r.direct);
    let (_, y, _) = unit_direction.xyz();
    let t = 0.5 *  (y  + 1.0);
    
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t)+ Vec3::new(0.5, 0.7, 1.9) * t
}

fn main() {
    let (nx, ny) = (200, 100);
    let ns = 100;
    let offset = 255.99;
    let mut rng = rand::thread_rng();

    let mut world = HitableList::new(vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
    ]);

    println!("P3\n{} {} \n255", nx, ny);

    let cam: Camera = Camera::default();
    for j in (0..ny).rev(){
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for s in 0..ns {
                let u = (i as f64 + rng.gen::<f64>()) / nx as f64;
                let v = (j as f64 + rng.gen::<f64>()) / ny as f64;
                let r = cam.get_ray(u, v);
                col += color(&r, &mut world);
            }
            col /= ns as f64;

            println!("{} {} {}", (col[RGB::R] * offset) as isize, (col[RGB::G] * offset) as isize, (col[RGB::B] * offset) as isize);
        }
    }
}
