use colorutils_rs::{rgb_to_rgba, TransferFunction};
use image::{EncodableLayout, GenericImageView, ImageReader};
use std::time::Instant;

use histogram_equalization::{ahe_hsl_rgb, ahe_hsv_rgb, ahe_lab_rgb, ahe_yuv_rgb, clahe_jzazbz_bgra, clahe_jzazbz_rgb, clahe_lab_bgra, clahe_lab_rgb, clahe_luv_bgra, clahe_luv_rgb, clahe_oklab_bgra, clahe_oklab_rgb, clahe_oklab_rgba, clahe_oklch_bgra, clahe_yuv_bgra, clahe_yuv_rgb, hist_equal_hsl_rgb, hist_equal_lab_rgb, hist_equal_luv_rgb, ClaheGridSize};

fn main() {
    let img = ImageReader::open("assets/forest.jpg")
        .unwrap()
        .decode()
        .unwrap();
    let img = img.to_rgb8();
    let dimensions = img.dimensions();
    let mut src_bytes = img.as_bytes();

    let channels = 3;

    let stride = dimensions.0 as usize * channels;
    let mut dst_bytes: Vec<u8> = vec![0; stride * dimensions.1 as usize];

    let start_time = Instant::now();

    clahe_oklab_rgb(
        src_bytes,
        stride as u32,
        &mut dst_bytes,
        stride as u32,
        dimensions.0,
        dimensions.1,
        1.3f32,
        ClaheGridSize::new(8, 8),
        128,
    );

    println!("exec time {:?}", start_time.elapsed());

    if channels == 4 {
        image::save_buffer(
            "converted_oklab.png",
            &dst_bytes,
            dimensions.0,
            dimensions.1,
            image::ExtendedColorType::Rgba8,
        )
        .unwrap();
    } else {
        image::save_buffer(
            "converted_oklab.jpg",
            &dst_bytes,
            dimensions.0,
            dimensions.1,
            image::ExtendedColorType::Rgb8,
        )
        .unwrap();
    }
}
