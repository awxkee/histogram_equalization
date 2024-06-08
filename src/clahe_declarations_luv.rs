use crate::clahe_impl::clahe_impl_u16;
use crate::hist_support::AheImplementation;
use crate::luv::{bgra_to_luv, luv_to_bgra, luv_to_rgb, luv_to_rgba, rgb_to_luv, rgba_to_luv};
use crate::ClaheGridSize;

/// Converts image to LUV, performs CLAHE and reverts back into RGB
///
/// # Arguments
///
/// * `threshold` - Level of clipping histogram ~[0, 10]
/// * `grid_size` - Grid for constructing histograms - default is (8,8)
/// * `bins_count` - Histogram bins, default is 128
///
/// # Panics
///
/// This function panics if the lengths of the planes or the input data are not valid based
/// on the specified width, height, and strides
pub fn clahe_luv_rgb(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    threshold: f32,
    grid_size: ClaheGridSize,
    bins_count: usize,
) {
    clahe_impl_u16::<3, 0, { AheImplementation::Clahe as u8 }>(
        src, src_stride, dst, dst_stride, width, height, threshold, grid_size, bins_count,
        rgb_to_luv, luv_to_rgb,
    );
}

/// Converts image to LUV, performs AHE and reverts back into RGB
///
/// # Arguments
///
/// * `grid_size` - Grid for constructing histograms - default is (8,8)
/// * `bins_count` - Histogram bins, default is 128
///
/// # Panics
///
/// This function panics if the lengths of the planes or the input data are not valid based
/// on the specified width, height, and strides
pub fn ahe_luv_rgb(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    grid_size: ClaheGridSize,
    bins_count: usize,
) {
    clahe_impl_u16::<3, 0, { AheImplementation::Ahe as u8 }>(
        src, src_stride, dst, dst_stride, width, height, 0f32, grid_size, bins_count, rgb_to_luv,
        luv_to_rgb,
    );
}

/// Converts image to LUV, performs CLAHE and reverts back into RGBA
///
/// # Arguments
///
/// * `threshold` - Level of clipping histogram ~[0, 10]
/// * `grid_size` - Grid for constructing histograms - default is (8,8)
/// * `bins_count` - Histogram bins, default is 128
///
/// # Panics
///
/// This function panics if the lengths of the planes or the input data are not valid based
/// on the specified width, height, and strides
pub fn clahe_luv_rgba(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    threshold: f32,
    grid_size: ClaheGridSize,
    bins_count: usize,
) {
    clahe_impl_u16::<4, 0, { AheImplementation::Clahe as u8 }>(
        src,
        src_stride,
        dst,
        dst_stride,
        width,
        height,
        threshold,
        grid_size,
        bins_count,
        rgba_to_luv,
        luv_to_rgba,
    );
}

/// Converts image to LUV, performs AHE and reverts back into RGB
///
/// # Arguments
///
/// * `grid_size` - Grid for constructing histograms - default is (8,8)
/// * `bins_count` - Histogram bins, default is 128
///
/// # Panics
///
/// This function panics if the lengths of the planes or the input data are not valid based
/// on the specified width, height, and strides
pub fn ahe_luv_rgba(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    grid_size: ClaheGridSize,
    bins_count: usize,
) {
    clahe_impl_u16::<4, 0, { AheImplementation::Ahe as u8 }>(
        src,
        src_stride,
        dst,
        dst_stride,
        width,
        height,
        0f32,
        grid_size,
        bins_count,
        rgba_to_luv,
        luv_to_rgba,
    );
}

/// Converts image to LUV, performs CLAHE and reverts back into BGRA
///
/// # Arguments
///
/// * `threshold` - Level of clipping histogram ~[0, 10]
/// * `grid_size` - Grid for constructing histograms - default is (8,8)
/// * `bins_count` - Histogram bins, default is 128
///
/// # Panics
///
/// This function panics if the lengths of the planes or the input data are not valid based
/// on the specified width, height, and strides
pub fn clahe_luv_bgra(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    threshold: f32,
    grid_size: ClaheGridSize,
    bins_count: usize,
) {
    clahe_impl_u16::<4, 0, { AheImplementation::Clahe as u8 }>(
        src,
        src_stride,
        dst,
        dst_stride,
        width,
        height,
        threshold,
        grid_size,
        bins_count,
        bgra_to_luv,
        luv_to_bgra,
    );
}

/// Converts image to LUV, performs AHE and reverts back into RGB
///
/// # Arguments
///
/// * `grid_size` - Grid for constructing histograms - default is (8,8)
/// * `bins_count` - Histogram bins, default is 128
///
/// # Panics
///
/// This function panics if the lengths of the planes or the input data are not valid based
/// on the specified width, height, and strides
pub fn ahe_luv_bgra(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    grid_size: ClaheGridSize,
    bins_count: usize,
) {
    clahe_impl_u16::<4, 0, { AheImplementation::Ahe as u8 }>(
        src,
        src_stride,
        dst,
        dst_stride,
        width,
        height,
        0f32,
        grid_size,
        bins_count,
        bgra_to_luv,
        luv_to_bgra,
    );
}
