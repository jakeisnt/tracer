use super::vec::{Vec3, Color};
use super::ray::Ray;
use super::hit::HitRecord;

pub trait Scatter {
    // potentially rcattur the ray
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub fn new(a: Color) -> Lambertian {
        Lambertian {
            // measure of diffuse of radiation incident to the angle
            albedo: a
        }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_in_unit_sphere().normalized();
        // prevent normalizing to 0, at cost of slightly more radical distribution
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new(rec.p, scatter_direction);

        Some((self.albedo, scattered))
    }
}

pub struct Metal {
    albedo: Color
}

impl Metal {
    pub fn new(a: Color) -> Metal {
        Metal {
            albedo: a
        }
    }
}

impl Scatter for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        // reflect off of the normal, then make unit vector
        let reflected = r_in.direction().reflect(rec.normal).normalized();
        // prod ray in dir of collision
        let scattered = Ray::new(rec.p, reflected);

        if scattered.direction().dot(rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
