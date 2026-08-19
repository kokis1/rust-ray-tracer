use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};
use std::io::Write;
use crate::common::{self, clamp};

#[derive(Copy, Clone, Default, Debug)]
pub struct Vec3 {
   e: [f64; 3],
}

impl Vec3 {
   pub fn new(x: f64, y: f64, z: f64) -> Self{
      Vec3 {e: [x, y, z]}
   }

   pub fn random() -> Self {
        Vec3::new(
            common::random_f64(),
            common::random_f64(),
            common::random_f64(),
        )
   }

   pub fn random_range(min: f64, max: f64) -> Self {
        Vec3::new(
            common::random_f64_range(min, max),
            common::random_f64_range(min, max),
            common::random_f64_range(min, max),
        )
   }

   pub fn random_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
   }

   pub fn random_unit_vector() -> Vec3 {
        unit_vector(Vec3::random_unit_sphere())
   }

   pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2.0 * dot(v, n) * n
   }

   pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = f64::min(dot(uv, n), 1.0);
        let r_out_perp = etai_over_etat * (uv + cos_theta * n);
        let r_out_parallel = -f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared())) * n;
        r_out_perp + r_out_parallel
   }

   pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(
                common::random_f64_range(-1.0, 1.0),
                common::random_f64_range(-1.0, 1.0),
                0.0,
            );

            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
   }

   pub fn near_zero(&self) -> bool {
        const EPS: f64 = 1.0e-8;

        // returns true if the Vec3 is almost zero in all dimensions
        self.e[0].abs() < EPS && self.e[1].abs() < EPS && self.e[2].abs() < EPS
   }

   pub fn x(&self) -> f64 {
      self.e[0]
   }

   pub fn y(&self) -> f64 {
      self.e[1]
   }

   pub fn z(&self) -> f64 {
      self.e[2]
   }

   pub fn length(&self) -> f64 {
      f64::sqrt(self.length_squared())
   }

   pub fn length_squared(&self) -> f64 {
      self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
   }

}

impl Neg for Vec3 {
   type Output = Vec3;

   fn neg(self) -> Vec3 {
      Vec3::new(-self.x(), -self.y(), -self.z())
   }
}

// Vec3 += Vec3
impl AddAssign for Vec3 {
    fn add_assign(&mut self, v: Vec3) {
        *self = *self + v;
    }
}
 
// Vec3 *= f64
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        *self = *self * t;
    }
}
 
// Vec3 /= f64
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self = *self / t;
    }
}
 
// Vec3 + Vec3
impl Add for Vec3 {
    type Output = Vec3;
 
    fn add(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() + v.x(), self.y() + v.y(), self.z() + v.z())
    }
}
 
// Vec3 - Vec3
impl Sub for Vec3 {
    type Output = Vec3;
 
    fn sub(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() - v.x(), self.y() - v.y(), self.z() - v.z())
    }
}
 
// Vec3 * Vec3
impl Mul for Vec3 {
    type Output = Vec3;
 
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() * v.x(), self.y() * v.y(), self.z() * v.z())
    }
}
 
// f64 * Vec3
impl Mul<Vec3> for f64 {
    type Output = Vec3;
 
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self * v.x(), self * v.y(), self * v.z())
    }
}
 
// Vec3 * f64
impl Mul<f64> for Vec3 {
    type Output = Vec3;
 
    fn mul(self, t: f64) -> Vec3 {
        Vec3::new(self.x() * t, self.y() * t, self.z() * t)
    }
}
 
// Vec3 / f64
impl Div<f64> for Vec3 {
    type Output = Vec3;
 
    fn div(self, t: f64) -> Vec3 {
        Vec3::new(self.x() / t, self.y() / t, self.z() / t)
    }
}
 
pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}
 
pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0],
    )
}
 
pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}



// type aliases to help with the ray-tracer
pub type Point3 = Vec3;
pub type Colour = Vec3;

pub fn write_colour(out: &mut impl Write, pixel_colour: Colour, samples_per_pixel: i32) {
    let mut r = pixel_colour.x();
    let mut g = pixel_colour.y();
    let mut b = pixel_colour.z();
 
    // Divide the colour by the number of samples and gamma correct for gamma = 2.0
    let scale = 1.0 / samples_per_pixel as f64;
    r = f64::sqrt(scale * r);
    g = f64::sqrt(scale * g);
    b = f64::sqrt(scale * b);
 
    // Write the translated [0, 255] value of each colour component
    writeln!(
        out,
        "{} {} {}",
        (256.0 * clamp(r, 0.0, 0.999)) as i32,
        (256.0 * clamp(g, 0.0, 0.999)) as i32,
        (256.0 * clamp(b, 0.0, 0.999)) as i32,
    )
    .expect("writing colour");
}