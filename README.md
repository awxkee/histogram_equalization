# Histogram equalization in Rust

There is some implementation of CLAHE (contrast-limited adaptive histogram equalization), AHE (adaptive histogram equalization), 
and histogram equalization performed in different color spaces.

All methods may perform histogram equalization in:
- [x] YUV (YCgCo subtype) always with 256 hist bins for performance purposes.
- [x] HSV
- [x] HSL
- [x] CIE L\*a\*b
- [x] CIE L\*u\*v
- [x] Oklab
- [x] Jzazbz
- [x] Oklch

All color spaces as it is have different properties and of course results.

**There is no implementation for gray images.**

# Example

```rust
clahe_luv_rgb(
    src_bytes,
    stride as u32,
    &mut dst_bytes,
    stride as u32,
    dimensions.0,
    dimensions.1,
    4f32,
    ClaheGridSize::new(8, 8),
    128,
);
```

## How to use with `image` crate

```rust
let img = ImageReader::open("assets/asset_1.jpg")
    .unwrap()
    .decode()
    .unwrap();
let dimensions = img.dimensions();
let channels = 3;
let stride = dimensions.0 as usize * channels;
let mut dst_bytes: Vec<u8> = vec![0; stride * dimensions.1 as usize];
let src_bytes = img.as_bytes();
hist_equal_hsv_rgb(
    src_bytes,
    stride as u32,
    &mut dst_bytes,
    stride as u32,
    dimensions.0,
    dimensions.1,
    128,
);
image::save_buffer(
    "converted_eq_hsv.jpg",
    &dst_bytes,
    dimensions.0,
    dimensions.1,
    image::ExtendedColorType::Rgb8,
)
.unwrap();
```