use colorutils_rs::rgb_to_rgba;
use image::io::Reader as ImageReader;
use image::GenericImageView;

use histogram_equalization::{
    clahe_jzazbz_bgra, clahe_lab_bgra, clahe_luv_bgra, clahe_oklab_bgra, clahe_oklch_bgra,
    ClaheGridSize,
};

fn main() {
    let img = ImageReader::open("assets/asset_1.jpg")
        .unwrap()
        .decode()
        .unwrap();
    let dimensions = img.dimensions();
    let mut src_bytes = img.as_bytes();

    let mut dst_rgba = vec![];
    dst_rgba.resize(4usize * dimensions.0 as usize * dimensions.1 as usize, 0u8);
    rgb_to_rgba(
        &src_bytes,
        3u32 * dimensions.0,
        &mut dst_rgba,
        4u32 * dimensions.0,
        dimensions.0,
        dimensions.1,
        255,
    );

    let stride = dimensions.0 * 4;
    for y in 0..dimensions.1 {
        for x in 0..dimensions.0 {
            let b = dst_rgba[(y * stride + x * 4) as usize];
            dst_rgba[(y * stride + x * 4) as usize] = dst_rgba[(y * stride + x * 4 + 2) as usize];
            dst_rgba[(y * stride + x * 4 + 2) as usize] = b;
        }
    }

    src_bytes = &dst_rgba;

    let channels = 4;
    let stride = dimensions.0 as usize * channels;
    let mut dst_bytes: Vec<u8> = vec![0; stride * dimensions.1 as usize];

    clahe_oklch_bgra(
        src_bytes,
        stride as u32,
        &mut dst_bytes,
        stride as u32,
        dimensions.0,
        dimensions.1,
        1.2f32,
        ClaheGridSize::new(8, 8),
        102,
    );

    let stride = dimensions.0 * 4;
    for y in 0..dimensions.1 {
        for x in 0..dimensions.0 {
            let b = dst_bytes[(y * stride + x * 4) as usize];
            dst_bytes[(y * stride + x * 4) as usize] = dst_bytes[(y * stride + x * 4 + 2) as usize];
            dst_bytes[(y * stride + x * 4 + 2) as usize] = b;
        }
    }

    if channels == 4 {
        image::save_buffer(
            "converted_oklch.png",
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
