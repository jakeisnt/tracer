use super::vec::{Vec3, Point3};

pub struct Ray {
    // identical: both are vec3s internally
    orig: Point3,
    dir: Vec3
}

impl Ray {
    // creates a new ray at a point in a direction
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction
        }
    }

    // fetches oriign of ray
    pub fn origin(&self) -> Point3 {
        self.orig
    }

    // determines direction of array
    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    // gets result of ray at value t
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}
