extern crate rand;
use rand::Rng;

mod core_grafic;

use core_grafic::vec3::{RGB, Vec3, dot};
use core_grafic::ray::Ray;
use core_grafic::hitable::{Hitable, HitRecord};
use core_grafic::sphere::Sphere;
use core_grafic::hitable_list::HitableList;
use core_grafic::camera::Camera;
use core_grafic::material::{Lambertian, Metal, Dielectric};

fn random_in_unit_sphere() -> Vec3 {
    let f = || Vec3::new(rand::random::<f64>(), rand::random::<f64>(), rand::random::<f64>()) * 2.0 - Vec3::new(1.0, 1.0, 1.0) ;
    let mut p = f();
    while p.squared_lenght() >= 1.0 {
        p = f();
    }

    return p;
}

fn color(r: Ray, world: &mut Hitable, depth: i32) -> Vec3 {
    let mut rec = HitRecord::new();
    if world.hit(&r, 0.001, std::f64::MAX, &mut rec) {
        return match rec.material.scatter(&r, &rec) {
            Some((attenuation, scattered)) =>
                if depth < 50 {
                    color(scattered, world, depth+1) * attenuation
                } else {
                    Vec3::new(0.0, 0.0, 0.0)
                }
            _ =>{
                Vec3::new(0.0, 0.0, 0.0)
            },
        }
    }

    let unit_direction =  Vec3::unit_vector(&r.direct);
    let (_, y, _) = unit_direction.xyz();
    let t = 0.5 *  (y  + 1.0);
    
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t)+ Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() {
    let (nx, ny) = (200, 100);
    let ns = 100;
    let offset = 255.99;
    let mut rng = rand::thread_rng();

    let mut world = HitableList::new(vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, std::rc::Rc::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3))))),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, std::rc::Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))))),
        Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, std::rc::Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.3)))),
        Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, std::rc::Rc::new(Dielectric{ref_index: 1.5}))),
        Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.45, std::rc::Rc::new(Dielectric{ref_index: 1.5}))),
    ]);

    println!("P3\n{} {} \n255", nx, ny);


    let lookfrom = Vec3::new(3.0, 3.0, 2.0);
    let lookat = Vec3::new(0.0, 0.0, -1.0);
    let dist_to_focus = (Vec3::copy(&lookfrom) - Vec3::copy(&lookat)).length();
    let aperture = 2.0;
    let cam: Camera = Camera::new(lookfrom, lookat, Vec3::new(0.0, 1.0, 0.0), 20.0, nx as f64 / ny as f64, aperture, dist_to_focus);
    for j in (0..ny).rev(){
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for s in 0..ns {
                let u = (i as f64 + rng.gen::<f64>()) / nx as f64;
                let v = (j as f64 + rng.gen::<f64>()) / ny as f64;
                let r = cam.get_ray(u, v);
                col += color(r, &mut world, 0);
            }
            col /= ns as f64;
            col = Vec3::new(col.rgb().0.sqrt(), col.rgb().1.sqrt(), col.rgb().2.sqrt());

            println!("{} {} {}", (col[RGB::R] * offset) as isize, (col[RGB::G] * offset) as isize, (col[RGB::B] * offset) as isize);
        }
    }
}
