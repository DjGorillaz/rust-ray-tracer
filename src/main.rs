fn main() {
    let image_width = 256;
    let image_height = 256;

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0.. image_width {
            let r = i as f64 / (image_width as f64 - 1_f64);
            let g = j as f64 / (image_height as f64 - 1_f64);
            let b = 0.25;

            let ir = 255.999 * r;
            let ig = 255.99 * g;
            let ib = 255.99 * b;

            println!("{} {} {}", ir as i32, ig as i32, ib as i32);
        }
    }

    eprintln!("Done!");
}
