use crate::hist_support::{cdf, make_histogram_region, minmax};
use yuvutils_rs::YuvRange;

#[allow(dead_code)]
pub(crate) fn equalize_histogram_yuv_impl<const CHANNELS: u8>(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    destructuring: fn(
        &mut [u8],
        u32,
        &mut [u8],
        u32,
        &mut [u8],
        u32,
        &[u8],
        u32,
        u32,
        u32,
        YuvRange,
    ),
    structuring: fn(
        &[u8],
        u32,
        &[u8],
        u32,
        &[u8],
        u32,
        &[u8],
        u32,
        &mut [u8],
        u32,
        u32,
        u32,
        YuvRange,
        bool,
    ),
) {
    let bins_count = 256;

    let mut y_plane: Vec<u8> = vec![0u8; width as usize * height as usize];
    y_plane.resize(width as usize * height as usize, 0u8);

    let mut u_plane: Vec<u8> = vec![0u8; width as usize * height as usize];
    u_plane.resize(width as usize * height as usize, 0u8);

    let mut v_plane: Vec<u8> = vec![0u8; width as usize * height as usize];
    let mut a_plane = if CHANNELS == 4 {
        vec![0u8; width as usize * height as usize]
    } else {
        Vec::new()
    };

    destructuring(
        &mut y_plane,
        width,
        &mut u_plane,
        width,
        &mut v_plane,
        width,
        src,
        src_stride,
        width,
        height,
        YuvRange::Full,
    );
    if CHANNELS == 4 {
        let mut a_shift = 0usize;
        let mut y_shift = 0usize;
        for _ in 0usize..height as usize {
            for x in 0usize..width as usize {
                unsafe {
                    *a_plane.get_unchecked_mut(a_shift + x) =
                        *src.get_unchecked(y_shift + x * 4 + 3);
                }
            }
            y_shift += src_stride as usize;
            a_shift += width as usize;
        }
    }

    let histogram =
        make_histogram_region::<0, 1, u8>(&y_plane, width, 0, width, 0, height, bins_count);
    let mut bins = histogram.bins;

    cdf(&mut bins);

    let pixels_count = width * height;

    let (min_bin, _) = minmax(&bins);

    let distance_r = 1f64 / (pixels_count as f64 - min_bin as f64);

    if distance_r != 0f64 {
        for i in 0..256usize {
            unsafe {
                *bins.get_unchecked_mut(i) =
                    (255f64 * (*bins.get_unchecked(i) as f64 - min_bin as f64) * distance_r)
                        .round()
                        .min(255f64) as u64;
            }
        }
    }

    let mut y_shift = 0usize;
    for _ in 0usize..height as usize {
        for x in 0usize..width as usize {
            unsafe {
                let value = *y_plane.get_unchecked(y_shift + x) as usize;
                let bin_value = *bins.get_unchecked(value);
                *y_plane.get_unchecked_mut(y_shift + x) = bin_value as u8;
            }
        }
        y_shift += width as usize;
    }

    structuring(
        &y_plane,
        width,
        &u_plane,
        width,
        &v_plane,
        width,
        &a_plane,
        width,
        dst,
        dst_stride,
        width,
        height,
        YuvRange::Full,
        false,
    );
}
