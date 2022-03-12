use super::ray::*;
use super::vec3::*;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = Camera::ASPECT_RATIO * Camera::VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f64 = 1.0;

    pub fn new() -> Camera {
        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(Camera::VIEWPORT_WIDTH, 0.0, 0.0);
        let vertical = Vec3::new(0.0, Camera::VIEWPORT_HEIGHT, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, Camera::FOCAL_LENGTH);

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
