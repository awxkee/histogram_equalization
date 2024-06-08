use colorutils_rs::{
    bgra_to_hsl, bgra_to_hsv, hsl_to_bgra, hsl_to_rgb, hsl_to_rgba, hsv_to_bgra, hsv_to_rgb,
    hsv_to_rgba, rgb_to_hsl, rgb_to_hsv, rgba_to_hsl, rgba_to_hsv,
};

use crate::hist_equal_impl::equalize_histogram_impl;
use crate::lab::{bgra_to_lab, lab_to_bgra, lab_to_rgb, lab_to_rgba, rgb_to_lab, rgba_to_lab};
use crate::luv::{bgra_to_luv, luv_to_bgra, luv_to_rgb, luv_to_rgba, rgb_to_luv, rgba_to_luv};

/// Converts image to HSV, performs histogram equalization and reverts back into RGB
///
/// # Arguments
///
///
/// # Panics
///
/// This function panics if the lengths of the planes or the input data are not valid based
/// on the specified width, height, and strides
pub fn hist_equal_hsv_rgb(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    bins_count: usize,
) {
    equalize_histogram_impl::<3, 2>(
        src, src_stride, dst, dst_stride, width, height, bins_count, rgb_to_hsv, hsv_to_rgb,
    );
}

/// Converts image to HSV, performs histogram equalization and reverts back into RGBA
///
/// # Arguments
///
///
/// # Panics
///
/// This function panics if the lengths of the planes or the input data are not valid based
/// on the specified width, height, and strides
pub fn hist_equal_hsv_rgba(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    bins_count: usize,
) {
    equalize_histogram_impl::<4, 2>(
        src,
        src_stride,
        dst,
        dst_stride,
        width,
        height,
        bins_count,
        rgba_to_hsv,
        hsv_to_rgba,
    );
}

/// Converts image to HSV, performs histogram equalization and reverts back into BGRA
///
/// # Arguments
///
///
/// # Panics
///
/// This function panics if the lengths of the planes or the input data are not valid based
/// on the specified width, height, and strides
pub fn hist_equal_hsv_bgra(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    bins_count: usize,
) {
    equalize_histogram_impl::<4, 2>(
        src,
        src_stride,
        dst,
        dst_stride,
        width,
        height,
        bins_count,
        bgra_to_hsv,
        hsv_to_bgra,
    );
}

/// Converts image to HSL, performs histogram equalization and reverts back into RGB
///
/// # Arguments
///
///
/// # Panics
///
/// This function panics if the lengths of the planes or the input data are not valid based
/// on the specified width, height, and strides
pub fn hist_equal_hsl_rgb(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    bins_count: usize,
) {
    equalize_histogram_impl::<3, 2>(
        src, src_stride, dst, dst_stride, width, height, bins_count, rgb_to_hsl, hsl_to_rgb,
    );
}

/// Converts image to HSL, performs histogram equalization and reverts back into RGBA
///
/// # Arguments
///
///
/// # Panics
///
/// This function panics if the lengths of the planes or the input data are not valid based
/// on the specified width, height, and strides
pub fn hist_equal_hsl_rgba(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    bins_count: usize,
) {
    equalize_histogram_impl::<4, 2>(
        src,
        src_stride,
        dst,
        dst_stride,
        width,
        height,
        bins_count,
        rgba_to_hsl,
        hsl_to_rgba,
    );
}

/// Converts image to HSL, performs histogram equalization and reverts back into BGRA
///
/// # Arguments
///
///
/// # Panics
///
/// This function panics if the lengths of the planes or the input data are not valid based
/// on the specified width, height, and strides
pub fn hist_equal_hsl_bgra(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    bins_count: usize,
) {
    equalize_histogram_impl::<4, 2>(
        src,
        src_stride,
        dst,
        dst_stride,
        width,
        height,
        bins_count,
        bgra_to_hsl,
        hsl_to_bgra,
    );
}

/// Converts image to LAB, performs histogram equalization and reverts back into RGB
///
/// # Arguments
///
///
/// # Panics
///
/// This function panics if the lengths of the planes or the input data are not valid based
/// on the specified width, height, and strides
pub fn hist_equal_lab_rgb(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    bins_count: usize,
) {
    equalize_histogram_impl::<3, 0>(
        src, src_stride, dst, dst_stride, width, height, bins_count, rgb_to_lab, lab_to_rgb,
    );
}

/// Converts image to LAB, performs histogram equalization and reverts back into RGBA
///
/// # Arguments
///
///
/// # Panics
///
/// This function panics if the lengths of the planes or the input data are not valid based
/// on the specified width, height, and strides
pub fn hist_equal_lab_rgba(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    bins_count: usize,
) {
    equalize_histogram_impl::<4, 0>(
        src,
        src_stride,
        dst,
        dst_stride,
        width,
        height,
        bins_count,
        rgba_to_lab,
        lab_to_rgba,
    );
}

/// Converts image to LAB, performs histogram equalization and reverts back into BGRA
///
/// # Arguments
///
///
/// # Panics
///
/// This function panics if the lengths of the planes or the input data are not valid based
/// on the specified width, height, and strides
pub fn hist_equal_lab_bgra(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    bins_count: usize,
) {
    equalize_histogram_impl::<4, 0>(
        src,
        src_stride,
        dst,
        dst_stride,
        width,
        height,
        bins_count,
        bgra_to_lab,
        lab_to_bgra,
    );
}


/// Converts image to LUV, performs histogram equalization and reverts back into RGB
///
/// # Arguments
///
///
/// # Panics
///
/// This function panics if the lengths of the planes or the input data are not valid based
/// on the specified width, height, and strides
pub fn hist_equal_luv_rgb(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    bins_count: usize,
) {
    equalize_histogram_impl::<3, 0>(
        src, src_stride, dst, dst_stride, width, height, bins_count, rgb_to_luv, luv_to_rgb,
    );
}

/// Converts image to LUV, performs histogram equalization and reverts back into RGBA
///
/// # Arguments
///
///
/// # Panics
///
/// This function panics if the lengths of the planes or the input data are not valid based
/// on the specified width, height, and strides
pub fn hist_equal_luv_rgba(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    bins_count: usize,
) {
    equalize_histogram_impl::<4, 0>(
        src,
        src_stride,
        dst,
        dst_stride,
        width,
        height,
        bins_count,
        rgba_to_luv,
        luv_to_rgba,
    );
}

/// Converts image to LUV, performs histogram equalization and reverts back into BGRA
///
/// # Arguments
///
///
/// # Panics
///
/// This function panics if the lengths of the planes or the input data are not valid based
/// on the specified width, height, and strides
pub fn hist_equal_luv_bgra(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    bins_count: usize,
) {
    equalize_histogram_impl::<4, 0>(
        src,
        src_stride,
        dst,
        dst_stride,
        width,
        height,
        bins_count,
        bgra_to_luv,
        luv_to_bgra,
    );
}
