use super::hittable::*;
use super::material::*;
use super::ray::*;
use super::vec3::*;
use std::rc::Rc;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot(&r.direction(), &oc);
        let c = oc.length_squared() - self.radius.powf(2.0);

        let discriminant = half_b.powf(2.0) - a * c;
        if discriminant < 0.0 {
            return None;
        }

        // Find the nearest root that lies in the acceptable range
        let sqrtd = discriminant.sqrt();
        let first_root = (-half_b - sqrtd) / a;
        let second_root = (-half_b + sqrtd) / a;
        let roots = [first_root, second_root];

        if let Some(root) = roots.into_iter().find(|&x| t_min <= x && x <= t_max) {
            let mut rec = HitRecord::new();
            rec.t = root;
            rec.p = r.at(rec.t);
            let outward_normal = (rec.p - self.center) / self.radius;
            rec.set_face_normal(r, &outward_normal);
            rec.material = self.material.clone();

            Some(rec)
        } else {
            None
        }
    }
}
