mod hittable;
mod ray;
mod sphere;
mod vec3;

use hittable::*;
use image::ImageBuffer;
use ray::Ray;
use sphere::*;
use std::{f64::INFINITY, rc::Rc};
use vec3::*;

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    if let Some(rec) = world.hit(r, 0.0, INFINITY) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn create_pixel(pixel_color: &Color) -> image::Rgb<u8> {
    image::Rgb([
        (255.999 * pixel_color.x()) as u8,
        (255.999 * pixel_color.y()) as u8,
        (255.999 * pixel_color.z()) as u8,
    ])
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // World
    let mut world = HittableList { objects: vec![] };
    let sphere = Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    let sphere2 = Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));
    let sphere3 = Rc::new(Sphere::new(Point3::new(-0.5, 0.0, -0.7), 0.2));
    world.add(sphere);
    world.add(sphere2);
    world.add(sphere3);
    
    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render
    let mut img = ImageBuffer::new(image_width, image_height);

    for j in 0..image_height {
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = (image_height - j) as f64 / (image_height - 1) as f64;

            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&r, &world);
            let pixel = create_pixel(&pixel_color);
            img.put_pixel(i, j, pixel);
        }
    }

    img.save("image.png").expect("failed to save image!");

    println!("Done!");
}
