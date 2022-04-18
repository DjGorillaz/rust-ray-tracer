mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod vec3;

use image::ImageBuffer;
use std::{
    f64::INFINITY,
    sync::{Arc, Mutex},
};

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

pub fn random_scene() -> HittableList {
    let mut world = HittableList::default();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double(0.0..1.0);
            let center = Point3::new(
                a as f64 + 0.9 * random_double(0.0..1.0),
                0.2,
                b as f64 + 0.9 * random_double(0.0..1.0),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random(0.0..1.0) * Color::random(0.0..1.0);
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random(0.5..1.0);
                    let fuzz = random_double(0.0..0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}

fn main() {
    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 500;
    let max_depth = 50;

    // World
    let world = Arc::new(random_scene());

    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Arc::new(Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    ));

    // Render
    let img = Arc::new(Mutex::new(ImageBuffer::new(image_width, image_height)));

    let threads = num_cpus::get();
    let rows_per_band = image_height as usize / threads + 1;
    {
        let rows: Vec<_> = (0..image_height).collect();
        let bands: Vec<_> = rows.chunks(rows_per_band as usize).collect();

        crossbeam::scope(|spawner| {
            for band_chunks in bands.into_iter() {
                let cam = cam.clone();
                let world = world.clone();
                let img = img.clone();

                spawner.spawn(move |_| {
                    for j in band_chunks {
                        for i in 0..image_width {
                            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                            for _s in 0..samples_per_pixel {
                                let u =
                                    (i as f64 + random_double(0.0..1.0)) / (image_width - 1) as f64;
                                let v = (image_height as f64 - *j as f64 + random_double(0.0..1.0))
                                    as f64
                                    / (image_height - 1) as f64;
                                let r = cam.get_ray(u, v);

                                pixel_color += ray_color(&r, &(*world), max_depth);
                            }
                            let pixel = create_pixel(&pixel_color, samples_per_pixel);
                            img.lock().unwrap().put_pixel(i, *j, pixel);
                        }
                    }
                });
            }
        })
        .expect("failed to spawn threads");
    }

    img.lock()
        .unwrap()
        .save("image.png")
        .expect("failed to save image!");

    println!("Done!");
}
