use yuvutils_rs::{
    bgra_to_ycgco444, rgb_to_ycgco444, rgba_to_ycgco444, ycgco444_to_rgb,
    ycgco444_with_alpha_to_bgra, ycgco444_with_alpha_to_rgba, YuvError, YuvPlanarImage,
    YuvPlanarImageWithAlpha, YuvRange,
};

use crate::clahe_yuv_impl::clahe_yuv_impl;
use crate::hist_support::AheImplementation;
use crate::ClaheGridSize;

pub(crate) fn ycgco444_skip_alpha_to_rgb(
    image_with_alpha: &YuvPlanarImageWithAlpha<u8>,
    rgba: &mut [u8],
    rgba_stride: u32,
    range: YuvRange,
    _: bool,
) -> Result<(), YuvError> {
    let image = YuvPlanarImage {
        y_plane: image_with_alpha.y_plane,
        y_stride: image_with_alpha.y_stride,
        u_plane: image_with_alpha.u_plane,
        u_stride: image_with_alpha.u_stride,
        v_plane: image_with_alpha.v_plane,
        v_stride: image_with_alpha.v_stride,
        width: image_with_alpha.width,
        height: image_with_alpha.height,
    };
    ycgco444_to_rgb(&image, rgba, rgba_stride, range)
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
