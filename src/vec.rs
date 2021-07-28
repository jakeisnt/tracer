use std::ops::{Index, IndexMut, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use std::fmt;
use std::fmt::Display;
use rand::Rng; // i keep forgetting to import the trait!
use core::ops::Range;

#[derive(Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3]

}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 {
            e: [e0, e1, e2]
        }
    }

    pub fn x(self) -> f64 {
        self[0]
    }

    pub fn y(self) -> f64 {
        self[1]
    }

    pub fn z(self) -> f64 {
        self[2]
    }

    pub fn dot(self, other: Vec3) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x()
            ]
        }
    }

    pub fn normalized(self) -> Vec3 {
        self / self.length()
    }

    pub fn format_color(self, samples_per_pixel: u64) -> String {
        // sqrt: gamma correction. TODO what is gamma correction? sqrt approximates it.
        let ir = (256.0 * (self[0] / (samples_per_pixel as f64)).sqrt().clamp(0.0, 0.999)) as u64;
        let ig = (256.0 * (self[1] / (samples_per_pixel as f64)).sqrt().clamp(0.0, 0.999)) as u64;
        let ib = (256.0 * (self[2] / (samples_per_pixel as f64)).sqrt().clamp(0.0, 0.999)) as u64;

        format!("{} {} {}", ir, ig, ib)
    }

    pub fn random(r: Range<f64>) -> Vec3 {
        let mut rng = rand::thread_rng();

        Vec3 {
            e: [rng.gen_range(r.clone()), rng.gen_range(r.clone()), rng.gen_range(r.clone())]
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        // generate new vectors untio we get a unit vector? i guess this works...
        loop {
            let v = Vec3::random(-1.0..1.0);
            if v.length() < 1.0 {
                return v;
            }
        }
    }

    // to prevent unit vector opposite normal from summing to zero, we need to avoid near zeroes
    pub fn near_zero(self) -> bool {
        const EPS: f64 = 1.0e-8;
        self[0].abs() < EPS && self[1].abs() < EPS && self[2].abs() < EPS
    }

    pub fn reflect(self, n: Vec3) -> Vec3 {
        // dir is v + 2(normal) * magnitude n (unit vector)
        self - 2.0 * self.dot(n) * n
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        &mut self.e[index]
    }
}

impl Add for Vec3 {
    type Output = Vec3; // necessary for many traits to clarify what the trait outputs? type safety?

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self.x() + other.x(), self.y() + other.y(), self.z() + other.z()]
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            e: [self.x() + other.x(), self.y() + other.y(), self.z() + other.z()]
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self.x() - other.x(), self.y() - other.y(), self.z() - other.z()]
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            e: [self.x() - other.x(), self.y() - other.y(), self.z() - other.z()]
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3 {
            e: [self.x() * other, self.y() * other, self.z() * other]
        }
    }
}


impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self.x() * other.x(), self.y() * other.y(), self.z() * other.z()]
        }
    }
}


impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Vec3 {
            e: [self.x() * other, self.y() * other, self.z() * other]
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self * other.x(), self * other.y(), self * other.z()]
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        Vec3 {
            e: [self.x() / other, self.y() / other, self.z() / other]
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) -> () {
        *self = Vec3 {
            e: [self.x() / other, self.y() / other, self.z() / other]
        };
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x(), self.y(), self.z())
    }
}
