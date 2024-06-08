use image::GenericImageView;
use image::io::Reader as ImageReader;

use histogram_equalization::hist_equal_luv_rgb;

fn main() {
    let img = ImageReader::open("assets/asset_1.jpg")
        .unwrap()
        .decode()
        .unwrap();
    let dimensions = img.dimensions();
    let channels = 3;
    let stride = dimensions.0 as usize * channels;
    let mut dst_bytes: Vec<u8> = vec![0; stride * dimensions.1 as usize];
    let src_bytes = img.as_bytes();

    hist_equal_luv_rgb(
        src_bytes,
        stride as u32,
        &mut dst_bytes,
        stride as u32,
        dimensions.0,
        dimensions.1,
        128,
    );

    if channels == 4 {
        image::save_buffer(
            "converted_eq_luv.png",
            &dst_bytes,
            dimensions.0,
            dimensions.1,
            image::ExtendedColorType::Rgba8,
        )
            .unwrap();
    } else {
        image::save_buffer(
            "converted_eq_luv.jpg",
            &dst_bytes,
            dimensions.0,
            dimensions.1,
            image::ExtendedColorType::Rgb8,
        )
            .unwrap();
    }
}
