use crate::clahe_impl::clahe_impl_u16;
use crate::hist_support::AheImplementation;
use crate::ClaheGridSize;
use colorutils_rs::{
    bgra_to_hsl, bgra_to_hsv, hsl_to_bgra, hsl_to_rgb, hsl_to_rgba, hsv_to_bgra, hsv_to_rgb,
    hsv_to_rgba, rgb_to_hsl, rgb_to_hsv, rgba_to_hsl, rgba_to_hsv,
};

/// Converts image to HSV, performs CLAHE and reverts back into RGB
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
pub fn clahe_hsv_rgb(
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
    clahe_impl_u16::<3, 2, { AheImplementation::Clahe as u8 }>(
        src, src_stride, dst, dst_stride, width, height, threshold, grid_size, bins_count,
        rgb_to_hsv, hsv_to_rgb,
    );
}

/// Converts image to HSV, performs AHE and reverts back into RGB
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
pub fn ahe_hsv_rgb(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    grid_size: ClaheGridSize,
    bins_count: usize,
) {
    clahe_impl_u16::<3, 2, { AheImplementation::Ahe as u8 }>(
        src, src_stride, dst, dst_stride, width, height, 0f32, grid_size, bins_count, rgb_to_hsv,
        hsv_to_rgb,
    );
}

/// Converts image to HSL, performs CLAHE and reverts back into RGB
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
pub fn clahe_hsl_rgb(
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
    clahe_impl_u16::<3, 2, { AheImplementation::Clahe as u8 }>(
        src, src_stride, dst, dst_stride, width, height, threshold, grid_size, bins_count,
        rgb_to_hsl, hsl_to_rgb,
    );
}

/// Converts image to HSV, performs AHE and reverts back into RGB
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
pub fn ahe_hsl_rgb(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    grid_size: ClaheGridSize,
    bins_count: usize,
) {
    clahe_impl_u16::<3, 2, { AheImplementation::Ahe as u8 }>(
        src, src_stride, dst, dst_stride, width, height, 0f32, grid_size, bins_count, rgb_to_hsl,
        hsl_to_rgb,
    );
}

/// Converts image to HSV, performs CLAHE and reverts back into RGBA
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
pub fn clahe_hsv_rgba(
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
    clahe_impl_u16::<4, 2, { AheImplementation::Clahe as u8 }>(
        src,
        src_stride,
        dst,
        dst_stride,
        width,
        height,
        threshold,
        grid_size,
        bins_count,
        rgba_to_hsv,
        hsv_to_rgba,
    );
}

/// Converts image to HSV, performs AHE and reverts back into RGBA
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
pub fn ahe_hsv_rgba(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    grid_size: ClaheGridSize,
    bins_count: usize,
) {
    clahe_impl_u16::<4, 2, { AheImplementation::Ahe as u8 }>(
        src,
        src_stride,
        dst,
        dst_stride,
        width,
        height,
        0f32,
        grid_size,
        bins_count,
        rgba_to_hsv,
        hsv_to_rgba,
    );
}

/// Converts image to HSL, performs CLAHE and reverts back into RGBA
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
pub fn clahe_hsl_rgba(
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
    clahe_impl_u16::<4, 2, { AheImplementation::Clahe as u8 }>(
        src,
        src_stride,
        dst,
        dst_stride,
        width,
        height,
        threshold,
        grid_size,
        bins_count,
        rgba_to_hsl,
        hsl_to_rgba,
    );
}

/// Converts image to HSL, performs CLAHE and reverts back into RGBA
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
pub fn ahe_hsl_rgba(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    grid_size: ClaheGridSize,
    bins_count: usize,
) {
    clahe_impl_u16::<4, 2, { AheImplementation::Ahe as u8 }>(
        src,
        src_stride,
        dst,
        dst_stride,
        width,
        height,
        0f32,
        grid_size,
        bins_count,
        rgba_to_hsl,
        hsl_to_rgba,
    );
}

/// Converts image to HSV, performs CLAHE and reverts back into BGRA
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
pub fn clahe_hsv_bgra(
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
    clahe_impl_u16::<4, 2, { AheImplementation::Clahe as u8 }>(
        src,
        src_stride,
        dst,
        dst_stride,
        width,
        height,
        threshold,
        grid_size,
        bins_count,
        bgra_to_hsv,
        hsv_to_bgra,
    );
}

/// Converts image to HSV, performs AHE and reverts back into BGRA
pub fn ahe_hsv_bgra(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    grid_size: ClaheGridSize,
    bins_count: usize,
) {
    clahe_impl_u16::<4, 2, { AheImplementation::Ahe as u8 }>(
        src,
        src_stride,
        dst,
        dst_stride,
        width,
        height,
        0f32,
        grid_size,
        bins_count,
        bgra_to_hsv,
        hsv_to_bgra,
    );
}

/// Converts image to HSL, performs CLAHE and reverts back into BGRA
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
pub fn clahe_hsl_bgra(
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
    clahe_impl_u16::<4, 2, { AheImplementation::Clahe as u8 }>(
        src,
        src_stride,
        dst,
        dst_stride,
        width,
        height,
        threshold,
        grid_size,
        bins_count,
        bgra_to_hsl,
        hsl_to_bgra,
    );
}

/// Converts image to HSL, performs AHE and reverts back into BGRA
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
pub fn ahe_hsl_bgra(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    grid_size: ClaheGridSize,
    bins_count: usize,
) {
    clahe_impl_u16::<4, 2, { AheImplementation::Ahe as u8 }>(
        src,
        src_stride,
        dst,
        dst_stride,
        width,
        height,
        0f32,
        grid_size,
        bins_count,
        bgra_to_hsl,
        hsl_to_bgra,
    );
}
