use crate::vec3::{self, Colour, Vec3};
use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::common;

pub trait Material: Send + Sync {
   fn scatter(
      &self, 
      r_in: &Ray, 
      rec: &HitRecord, 
      attenuation: &mut Colour, 
      scattered: &mut Ray
   ) -> bool;
}

pub struct Lambertian {
   albedo: Colour,
}

impl Lambertian {
   pub fn new(a: Colour) -> Self {
      Lambertian { albedo: a }
   }
}

impl Material for Lambertian {
   fn scatter(
      &self,
      _r_in: &Ray,
      rec: &HitRecord,
      attenuation: &mut Colour,
      scattered: &mut Ray
   ) -> bool {
      let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

      // Catch degenerate scatter direction
      if scatter_direction.near_zero() {
         scatter_direction = rec.normal;
      }

      *attenuation = self.albedo;
      *scattered = Ray::new(rec.p, scatter_direction);
      true
   }
}


pub struct Metal {
   albedo: Colour,
   fuzz: f64
}

impl Metal {
   pub fn new(a: Colour, fuzz: f64) -> Self {
      Metal { 
      albedo: a,
      fuzz: if fuzz < 1.0 { fuzz } else { 1.0 }}
   }
}

impl Material for Metal {
   fn scatter(
      &self,
      r_in: &Ray,
      rec: &HitRecord,
      attenuation: &mut Colour,
      scattered: &mut Ray,
   ) -> bool {
      let reflected = Vec3::reflect(vec3::unit_vector(r_in.direction()), rec.normal);

      *attenuation = self.albedo;
      *scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_unit_sphere());

      vec3::dot(scattered.direction(), rec.normal) > 0.0
   }
}


pub struct Dialectric {
   ir: f64,
}

impl Dialectric {
   pub fn new(index_of_refractions: f64) -> Self {
      Dialectric { ir: index_of_refractions }
   }

   fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
      // Use Schlick's approximation for reflectance
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * f64::powf(1.0 - cosine, 5.0)
   }
}

impl Material for Dialectric {
   fn scatter(
         &self, 
         r_in: &Ray, 
         rec: &HitRecord, 
         attenuation: &mut Colour, 
         scattered: &mut Ray
      ) -> bool
   {
       let refraction_ratio = if rec.front_face {
         1.0 / self.ir
       } else {
         self.ir
       };

       let unit_direction = vec3::unit_vector(r_in.direction());
      let cos_theta = f64::min(vec3::dot(-unit_direction, rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);
 
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract
            || Self::reflectance(cos_theta, refraction_ratio) > common::random_f64()
        {
            Vec3::reflect(unit_direction, rec.normal)
        } else {
            Vec3::refract(unit_direction, rec.normal, refraction_ratio)
        };
 
        *attenuation = Colour::new(1.0, 1.0, 1.0);
        *scattered = Ray::new(rec.p, direction);

       true
   }
}