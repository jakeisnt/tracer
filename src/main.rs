mod vec;
mod ray;
use vec::{Vec3, Point3, Color};
use ray::Ray;

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - center;
    let a = r.direction().dot(r.direction());
    let b = 2.0 * oc.dot(r.direction());
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / ( 2.0 + a)
    }
}

fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r);

    if t > 0.0 {
        let n = (r.at(t) - Vec3::new(0.0, 0.0, -1.0)).normalized();
        return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }

    let unit_dir = r.direction().normalized();
    let t = 0.5 * (unit_dir.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // image configuration
    const ASPECT: f64 = 16.0 / 9.0;
    const IMGW: u64 = 256;
    const IMGH: u64 = ((IMGW as f64) / ASPECT) as u64;

    // camera configuration
    let port_h = 2.0;
    let port_w = ASPECT * port_h;
    let focal_len = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(port_w, 0.0, 0.0);
    let vertical = Vec3::new(0.0, port_h, 0.0);
    let low_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_len);

    println!("P3");
    println!("{} {}", IMGW, IMGH);
    println!("255");

    for j in 0..IMGH {
        eprintln!("Scanlines remaining: {}", IMGH - j - 1); // print to stderr

        for i in 0..IMGW {
            let u = (i as f64) / ((IMGW - 1) as f64);
            let v = (j as f64) / ((IMGH - 1) as f64);

            let r = Ray::new(origin, low_left_corner + u * horizontal + v * vertical - origin);
            let pixel_color = ray_color(&r);

            println!("{}", pixel_color.format_color());
        }
    }
    eprintln!("Done.");
}
