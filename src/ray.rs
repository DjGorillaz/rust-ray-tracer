use crate::vec3::{Point3, Vec3};

#[derive(Default)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn origin_test() {
        let orig = Point3::new(1.0, 2.0, 3.0);
        let dir = Vec3::new(4.0, 5.0, 6.0);
        let r = Ray::new(orig, dir);
        assert_eq!(r.origin(), orig);
    }

    #[test]
    fn dir_test() {
        let orig = Point3::new(1.0, 2.0, 3.0);
        let dir = Vec3::new(4.0, 5.0, 6.0);
        let r = Ray::new(orig, dir);
        assert_eq!(r.direction(), dir);
    }

    #[test]
    fn at_test() {
        let orig = Point3::new(1.0, 2.0, 3.0);
        let dir = Vec3::new(4.0, 5.0, 6.0);
        let t = 2.0;
        let r = Ray::new(orig, dir);
        assert_eq!(
            r.at(t),
            Point3 {
                e: [9.0, 12.0, 15.0]
            }
        );
    }
}
