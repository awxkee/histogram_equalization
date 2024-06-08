use yuvutils_rs::{
    bgra_to_ycgco444, rgb_to_ycgco444, rgba_to_ycgco444,
    ycgco444_with_alpha_to_bgra, ycgco444_with_alpha_to_rgba,
};

use crate::clahe_declarations_yuv::ycgco444_skip_alpha_to_rgb;
use crate::hist_equal_yuv_impl::equalize_histogram_yuv_impl;

/// Converts image to YUV, performs histogram equalization and reverts back into RGB.
///
/// For optimization purposes YUV histogram bins always 256
///
/// # Arguments
///
/// # Panics
///
/// This function panics if the lengths of the planes or the input data are not valid based
/// on the specified width, height, and strides
pub fn hist_equal_yuv_rgb(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
) {
    equalize_histogram_yuv_impl::<3>(
        src,
        src_stride,
        dst,
        dst_stride,
        width,
        height,
        rgb_to_ycgco444,
        ycgco444_skip_alpha_to_rgb,
    );
}

/// Converts image to YUV, performs histogram equalization and reverts back into RGBA
///
/// For optimization purposes YUV histogram bins always 256
///
/// # Arguments
///
/// # Panics
///
/// This function panics if the lengths of the planes or the input data are not valid based
/// on the specified width, height, and strides
pub fn hist_equal_yuv_rgba(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
) {
    equalize_histogram_yuv_impl::<4>(
        src,
        src_stride,
        dst,
        dst_stride,
        width,
        height,
        rgba_to_ycgco444,
        ycgco444_with_alpha_to_rgba,
    );
}

/// Converts image to YUV, performs histogram equalization and reverts back into BGRA
///
/// For optimization purposes YUV histogram bins always 256
///
/// # Arguments
///
/// # Panics
///
/// This function panics if the lengths of the planes or the input data are not valid based
/// on the specified width, height, and strides
pub fn hist_equal_yuv_bgra(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
) {
    equalize_histogram_yuv_impl::<4>(
        src,
        src_stride,
        dst,
        dst_stride,
        width,
        height,
        bgra_to_ycgco444,
        ycgco444_with_alpha_to_bgra,
    );
}