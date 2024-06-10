use colorutils_rs::rgb_to_rgba;
use image::GenericImageView;
use image::io::Reader as ImageReader;

use histogram_equalization::{clahe_hsl_rgba, clahe_hsv_bgra, clahe_hsv_rgba, clahe_lab_rgb, clahe_lab_rgba, clahe_luv_bgra, clahe_luv_rgba, ClaheGridSize, hist_equal_luv_rgb};

fn main() {
    let img = ImageReader::open("assets/IMG_5902.jpg")
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
    src_bytes = &dst_rgba;

    let channels = 4;
    let stride = dimensions.0 as usize * channels;
    let mut dst_bytes: Vec<u8> = vec![0; stride * dimensions.1 as usize];

    clahe_hsv_bgra(
        src_bytes,
        stride as u32,
        &mut dst_bytes,
        stride as u32,
        dimensions.0,
        dimensions.1,
        3f32,
        ClaheGridSize::new(8, 8),
        128,
    );

    if channels == 4 {
        image::save_buffer(
            "converted_luv.png",
            &dst_bytes,
            dimensions.0,
            dimensions.1,
            image::ExtendedColorType::Rgba8,
        )
            .unwrap();
    } else {
        image::save_buffer(
            "converted_luv.jpg",
            &dst_bytes,
            dimensions.0,
            dimensions.1,
            image::ExtendedColorType::Rgb8,
        )
            .unwrap();
    }
}
