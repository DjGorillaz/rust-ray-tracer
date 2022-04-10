mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod vec3;

use image::ImageBuffer;
use rand::Rng;
use std::{f64::INFINITY, rc::Rc};

use camera::*;
use hittable::*;
use ray::Ray;
use sphere::*;
use vec3::*;

use crate::material::{Dielectric, Lambertian, Metal};

fn ray_color(r: &Ray, world: &dyn Hittable, depth: usize) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, INFINITY) {
        if let Some((attenuation, scattered)) = rec.material.scatter(r, &rec) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }

    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

fn create_pixel(pixel_color: &Color, samples_per_pixel: usize) -> image::Rgb<u8> {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Divide the color by the number of samples and gamma-correct for gamma=2.0
    let scale = 1.0 / samples_per_pixel as f64;
    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();

    image::Rgb([
        (256.0 * clamp(r, 0.0, 0.999)) as u8,
        (256.0 * clamp(g, 0.0, 0.999)) as u8,
        (256.0 * clamp(b, 0.0, 0.999)) as u8,
    ])
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    // World
    let mut world = HittableList { objects: vec![] };
    let sphere = Rc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    ));
    let sphere2 = Rc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    let sphere3 = Rc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    ));
    let sphere4 = Rc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    ));

    world.add(sphere);
    world.add(sphere2);
    world.add(sphere3);
    world.add(sphere4);

    // Camera
    let cam = Camera::new();

    // Render
    let mut img = ImageBuffer::new(image_width, image_height);
    let mut rng = rand::thread_rng();

    for j in 0..image_height {
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _s in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen_range(0.0..1.0)) / (image_width - 1) as f64;
                let v = (image_height as f64 - j as f64 + rng.gen_range(0.0..1.0)) as f64
                    / (image_height - 1) as f64;
                let r = cam.get_ray(u, v);

                pixel_color += ray_color(&r, &world, max_depth);
            }
            let pixel = create_pixel(&pixel_color, samples_per_pixel);
            img.put_pixel(i, j, pixel);
        }
    }

    img.save("image.png").expect("failed to save image!");

    println!("Done!");
}
