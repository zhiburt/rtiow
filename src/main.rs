fn main() {
    let (nx, ny) = (200, 100);
    let offset = 255.99;

    println!("P3\n{} {} \n255", nx, ny);
    for j in (0..ny).rev(){
        for i in 0..nx {
            let r = i as f64 / nx as f64;
            let g = j as f64 / ny as f64;
            let b = 0.2;
            
            println!("{} {:?} {}", (r * offset) as isize, j, (b * offset) as isize);
        }
    }
}
