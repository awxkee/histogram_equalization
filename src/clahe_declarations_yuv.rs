use yuvutils_rs::{
    bgra_to_ycgco444, rgb_to_ycgco444, rgba_to_ycgco444, ycgco444_to_rgb,
    ycgco444_with_alpha_to_bgra, ycgco444_with_alpha_to_rgba, YuvRange,
};

use crate::clahe_yuv_impl::clahe_yuv_impl;
use crate::ClaheGridSize;
use crate::hist_support::AheImplementation;

pub(crate) fn ycgco444_skip_alpha_to_rgb(
    y_plane: &[u8],
    y_stride: u32,
    cg_plane: &[u8],
    cg_stride: u32,
    co_plane: &[u8],
    co_stride: u32,
    _: &[u8],
    _: u32,
    rgba: &mut [u8],
    rgba_stride: u32,
    width: u32,
    height: u32,
    range: YuvRange,
    _: bool,
) {
    ycgco444_to_rgb(
        y_plane,
        y_stride,
        cg_plane,
        cg_stride,
        co_plane,
        co_stride,
        rgba,
        rgba_stride,
        width,
        height,
        range,
    );
}

/// Converts image to YUV, performs CLAHE and reverts back into RGB.
///
/// For optimization purposes YUV histogram bins always 256
///
/// # Arguments
///
/// * `threshold` - Level of clipping histogram ~[0, 10]
/// * `grid_size` - Grid for constructing histograms - default is (8,8)
///
/// # Panics
///
/// This function panics if the lengths of the planes or the input data are not valid based
/// on the specified width, height, and strides
pub fn clahe_yuv_rgb(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    threshold: f32,
    grid_size: ClaheGridSize,
) {
    clahe_yuv_impl::<3, { AheImplementation::Clahe as u8 }>(
        src,
        src_stride,
        dst,
        dst_stride,
        width,
        height,
        threshold,
        grid_size,
        rgb_to_ycgco444,
        ycgco444_skip_alpha_to_rgb,
    );
}

/// Converts image to YUV, performs AHE and reverts back into RGB
///
/// For optimization purposes YUV histogram bins always 256
///
/// # Arguments
///
/// * `grid_size` - Grid for constructing histograms - default is (8,8)
///
/// # Panics
///
/// This function panics if the lengths of the planes or the input data are not valid based
/// on the specified width, height, and strides
pub fn ahe_yuv_rgb(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    grid_size: ClaheGridSize,
) {
    clahe_yuv_impl::<3, { AheImplementation::Ahe as u8 }>(
        src,
        src_stride,
        dst,
        dst_stride,
        width,
        height,
        0f32,
        grid_size,
        rgb_to_ycgco444,
        ycgco444_skip_alpha_to_rgb,
    );
}

/// Converts image to YUV, performs CLAHE and reverts back into RGBA
///
/// For optimization purposes YUV histogram bins always 256
///
/// # Arguments
///
/// * `threshold` - Level of clipping histogram ~[0, 10]
/// * `grid_size` - Grid for constructing histograms - default is (8,8)
///
/// # Panics
///
/// This function panics if the lengths of the planes or the input data are not valid based
/// on the specified width, height, and strides
pub fn clahe_yuv_rgba(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    threshold: f32,
    grid_size: ClaheGridSize,
) {
    clahe_yuv_impl::<4, { AheImplementation::Clahe as u8 }>(
        src,
        src_stride,
        dst,
        dst_stride,
        width,
        height,
        threshold,
        grid_size,
        rgba_to_ycgco444,
        ycgco444_with_alpha_to_rgba,
    );
}

/// Converts image to YUV, performs AHE and reverts back into RGB
///
/// For optimization purposes YUV histogram bins always 256
///
/// # Arguments
///
/// * `grid_size` - Grid for constructing histograms - default is (8,8)
///
/// # Panics
///
/// This function panics if the lengths of the planes or the input data are not valid based
/// on the specified width, height, and strides
pub fn ahe_yuv_rgba(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    grid_size: ClaheGridSize,
) {
    clahe_yuv_impl::<4, { AheImplementation::Ahe as u8 }>(
        src,
        src_stride,
        dst,
        dst_stride,
        width,
        height,
        0f32,
        grid_size,
        rgba_to_ycgco444,
        ycgco444_with_alpha_to_rgba,
    );
}

/// Converts image to YUV, performs CLAHE and reverts back into BGRA
///
/// For optimization purposes YUV histogram bins always 256
///
/// # Arguments
///
/// * `threshold` - Level of clipping histogram ~[0, 10]
/// * `grid_size` - Grid for constructing histograms - default is (8,8)
///
/// # Panics
///
/// This function panics if the lengths of the planes or the input data are not valid based
/// on the specified width, height, and strides
pub fn clahe_yuv_bgra(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    threshold: f32,
    grid_size: ClaheGridSize,
) {
    clahe_yuv_impl::<4, { AheImplementation::Clahe as u8 }>(
        src,
        src_stride,
        dst,
        dst_stride,
        width,
        height,
        threshold,
        grid_size,
        bgra_to_ycgco444,
        ycgco444_with_alpha_to_bgra,
    );
}

/// Converts image to YUV, performs AHE and reverts back into RGB. For optimization purposes YUV histogram bins always 256
///
/// For optimization purposes YUV histogram bins always 256
///
/// # Arguments
///
/// * `grid_size` - Grid for constructing histograms - default is (8,8)
///
/// # Panics
///
/// This function panics if the lengths of the planes or the input data are not valid based
/// on the specified width, height, and strides
pub fn ahe_yuv_bgra(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    grid_size: ClaheGridSize,
) {
    clahe_yuv_impl::<4, { AheImplementation::Ahe as u8 }>(
        src,
        src_stride,
        dst,
        dst_stride,
        width,
        height,
        0f32,
        grid_size,
        bgra_to_ycgco444,
        ycgco444_with_alpha_to_bgra,
    );
}
