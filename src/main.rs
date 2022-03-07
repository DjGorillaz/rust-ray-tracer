mod vec3;

use vec3::{Color, Point3, Vec3};

fn write_color(pixel_color: Color) {
    println!(
        "{} {} {}",
        (255.999 * pixel_color.x()) as i32,
        (255.999 * pixel_color.y()) as i32,
        (255.999 * pixel_color.z()) as i32
    );
}

fn main() {
    let image_width = 256;
    let image_height = 256;

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let r = i as f64 / (image_width as f64 - 1_f64);
            let g = j as f64 / (image_height as f64 - 1_f64);
            let b = 0.25;
            
            write_color(Color::new(r, g, b));
        }
    }

    eprintln!("Done!");
}
