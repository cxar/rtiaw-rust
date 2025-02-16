mod types;
use crate::types::color::write_color;
use crate::types::color::Color;
use std::fs;
use std::fs::OpenOptions;

fn main() {
    let image_width = 256;
    let image_height = 256;
    let data = format!("P3\n{image_width} {image_height}\n255\n");
    fs::write("./test.ppm", data).expect("Unable to write file");

    let mut file = OpenOptions::new().append(true).open("./test.ppm").unwrap();

    for j in 0..image_height {
        print!("\rScanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_color = Color::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.0,
            );
            write_color(&mut file, pixel_color);
        }
    }
}
