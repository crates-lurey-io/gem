//! Creates a PNG image with a red box and a semi-transparent blue square in the middle.
//!
//! This example demonstrates how to use the `gem` crate to create a simple PNG image.

use gem::prelude::*;

fn main() {
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("red_box.png");
    let mut red_box_50x50 = vec![Abgr8888::from_abgr(0xFF, 0x00, 0x00, 0xFF); 50 * 50];

    // Make a semi-transparent blue box in the middle of the image
    for y in 0..50 {
        for x in 0..50 {
            if (10..40).contains(&x) && (10..40).contains(&y) {
                red_box_50x50[y * 50 + x] = Abgr8888::from_abgr(0x7F, 0x7F, 0, 0xFF);
            }
        }
    }

    let mut encoder = png::Encoder::new(std::fs::File::create(&file_path).unwrap(), 50_u32, 50_u32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer
        .write_image_data(bytemuck::cast_slice(&red_box_50x50))
        .unwrap();

    println!("Red box image created at: {}", file_path.display());
    open::that(file_path).unwrap_or_else(|_| {
        eprintln!("Failed to open the image file.");
    });
}
