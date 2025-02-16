use crate::types::vec3::Vec3;
use std::io::Write;

pub type Color = Vec3;

pub fn write_color<T: Write>(out: &mut T, pixel_color: Color) {
    let rbyte = (255.999 * pixel_color.x()) as i32;
    let gbyte = (255.999 * pixel_color.y()) as i32;
    let bbyte = (255.999 * pixel_color.z()) as i32;

    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte).expect("failed to write color")
}
