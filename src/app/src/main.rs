use image::{DynamicImage, EncodableLayout, GenericImageView, ImageReader};
use std::time::Instant;

use histogram_equalization::{clahe_yuv_rgb, ClaheGridSize};

fn main() {
    let img: DynamicImage = ImageReader::open("assets/asset_1.jpg")
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

    clahe_yuv_rgb(
        src_bytes,
        stride as u32,
        &mut dst_bytes,
        stride as u32,
        dimensions.0,
        dimensions.1,
        1.3f32,
        ClaheGridSize::new(8, 8),
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