fn main() {
    const IMGW: u64 = 256;
    const IMGH: u64 = 256;

    println!("P3");
    println!("{} {}", IMGW, IMGH);
    println!("255"); // TODO ??

    for j in 0..IMGH {
        for i in 0..IMGW {
            let r = i/(IMGW - 1);
            let g = j/(IMGW - 1);
            let b = 0.25 as u64;

            let ir = (255.999 as u64) * r;
            let ig = (255.999 as u64) * g;
            let ib = (255.999 as u64) * b;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
