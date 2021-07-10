mod vec;
use vec::{Color};

fn main() {
    const IMGW: u64 = 256;
    const IMGH: u64 = 256;

    println!("P3");
    println!("{} {}", IMGW, IMGH);
    println!("255"); // TODO ??

    for j in 0..IMGH {
        eprintln!("Scanlines remaining: {}", IMGH - j - 1); // print to stderr
        for i in 0..IMGW {
            let pixel_color =
                Color::new((i as f64) / ((IMGW - 1) as f64),
                           (j as f64) / ((IMGH - 1) as f64),
                           0.25);

            println!("{}", pixel_color.format_color());
        }
    }
    eprintln!("Done.");
}
