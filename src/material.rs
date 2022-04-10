use super::hittable::*;
use super::ray::*;
use super::vec3::*;
pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        // let target = rec.p + rec.normal + Vec3::random_in_unit_sphere(); // lambertian approximation
        // let target = rec.p + rec.normal + Vec3::random_unit_vector(); // true lambertian reflection
        // let target = rec.p + Vec3::random_in_hemishpere(&rec.normal); // alternative diffuse method

        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        if fuzz > 1.0 {
            Metal { albedo, fuzz: 1.0 }
        } else {
            Metal { albedo, fuzz }
        }
    }

    fn reflect(&self, v: &Vec3, n: &Vec3) -> Vec3 {
        *v - 2.0 * dot(v, n) * (*n)
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = self.reflect(&unit_vector(r_in.direction()), &rec.normal);

        let scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        let attenuation = self.albedo;

        if dot(&scattered.direction(), &rec.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}
