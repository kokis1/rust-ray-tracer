use std::rc::Rc;

use crate::hitable::{HitRecord, Hittable};
use crate::materials::Material;
use crate::ray::Ray;
use crate::vec3::{self, Point3};


pub struct Sphere {
   centre: Point3,
   radius: f64,
   mat: Rc<dyn Material>,
}

impl Sphere {
   pub fn new(cen: Point3, rad: f64, m: Rc<dyn Material>) -> Self {
         Sphere {
            centre: cen,
            radius: rad,
            mat: m,
         }
   }
}

impl Hittable for Sphere {
   fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
       let oc = ray.origin() - self.centre;
        let a = ray.direction().length_squared();
        let half_b = vec3::dot(oc, ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
 
        let sqrt_d = f64::sqrt(discriminant);
 
        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrt_d) / a;
        if root <= t_min || t_max <= root {
            root = (-half_b + sqrt_d) / a;
            if root <= t_min || t_max <= root {
                return false;
            }
        }
 
        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = (rec.p - self.centre) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        rec.mat = Some(self.mat.clone());
        true
   }
}