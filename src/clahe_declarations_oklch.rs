use crate::clahe_call_proxy::clahe_impl_u16_proxy;
use crate::hist_support::AheImplementation;
use crate::oklch::{
    bgra_to_oklch, oklch_to_bgra, oklch_to_rgb, oklch_to_rgba, rgb_to_oklch, rgba_to_oklch,
};
use crate::ClaheGridSize;

/// Converts image to oklch, performs CLAHE and reverts back into RGB
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
pub fn clahe_oklch_rgb(
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
    clahe_impl_u16_proxy::<3, { AheImplementation::Clahe as u8 }>(
        src,
        src_stride,
        dst,
        dst_stride,
        width,
        height,
        threshold,
        grid_size,
        bins_count,
        rgb_to_oklch,
        oklch_to_rgb,
    );
}

/// Converts image to oklch, performs AHE and reverts back into RGB
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
pub fn ahe_oklch_rgb(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    grid_size: ClaheGridSize,
    bins_count: usize,
) {
    clahe_impl_u16_proxy::<3, { AheImplementation::Ahe as u8 }>(
        src,
        src_stride,
        dst,
        dst_stride,
        width,
        height,
        0f32,
        grid_size,
        bins_count,
        rgb_to_oklch,
        oklch_to_rgb,
    );
}

/// Converts image to oklch, performs CLAHE and reverts back into RGBA
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
pub fn clahe_oklch_rgba(
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
    clahe_impl_u16_proxy::<4, { AheImplementation::Clahe as u8 }>(
        src,
        src_stride,
        dst,
        dst_stride,
        width,
        height,
        threshold,
        grid_size,
        bins_count,
        rgba_to_oklch,
        oklch_to_rgba,
    );
}

/// Converts image to oklch, performs AHE and reverts back into RGBA
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
pub fn ahe_oklch_rgba(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    grid_size: ClaheGridSize,
    bins_count: usize,
) {
    clahe_impl_u16_proxy::<4, { AheImplementation::Ahe as u8 }>(
        src,
        src_stride,
        dst,
        dst_stride,
        width,
        height,
        0f32,
        grid_size,
        bins_count,
        rgba_to_oklch,
        oklch_to_rgba,
    );
}

/// Converts image to oklch, performs CLAHE and reverts back into BGRA
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
pub fn clahe_oklch_bgra(
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
    clahe_impl_u16_proxy::<4, { AheImplementation::Clahe as u8 }>(
        src,
        src_stride,
        dst,
        dst_stride,
        width,
        height,
        threshold,
        grid_size,
        bins_count,
        bgra_to_oklch,
        oklch_to_bgra,
    );
}

/// Converts image to oklch, performs AHE and reverts back into RGB
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
pub fn ahe_oklch_bgra(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    grid_size: ClaheGridSize,
    bins_count: usize,
) {
    clahe_impl_u16_proxy::<4, { AheImplementation::Ahe as u8 }>(
        src,
        src_stride,
        dst,
        dst_stride,
        width,
        height,
        0f32,
        grid_size,
        bins_count,
        bgra_to_oklch,
        oklch_to_bgra,
    );
}
