use std::slice;

use crate::image_configuration::ImageConfiguration;
use colorutils_rs::{Jzazbz, Rgb, TransferFunction};
use rayon::iter::{IndexedParallelIterator, ParallelIterator};
use rayon::prelude::{ParallelSlice, ParallelSliceMut};

#[inline]
pub(crate) fn generic_image_to_jzazbz<const IMAGE: u8>(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u16],
    dst_stride: u32,
    color_planes: &mut [f32],
    width: u32,
    height: u32,
    scale: f32,
) {
    let image_configuration: ImageConfiguration = IMAGE.into();
    let channels = image_configuration.get_channels_count();

    let full_scale = scale;

    let color_planes_stride = if image_configuration.has_alpha() {
        width as usize * 3usize
    } else {
        width as usize * 2usize
    };
    let dst_slice_safe_align = unsafe {
        slice::from_raw_parts_mut(
            dst.as_mut_ptr() as *mut u8,
            dst_stride as usize * height as usize,
        )
    };

    let color_planes_channels = if image_configuration.has_alpha() {
        3usize
    } else {
        2usize
    };

    dst_slice_safe_align
        .par_chunks_exact_mut(dst_stride as usize)
        .zip(color_planes.par_chunks_exact_mut(color_planes_stride))
        .zip(src.par_chunks_exact(src_stride as usize))
        .for_each(|((dst, color), src)| unsafe {
            let dst_ptr = dst.as_mut_ptr() as *mut u16;
            for x in 0..width as usize {
                let px = x * channels;
                let cx = x * color_planes_channels;

                let rgb = Rgb::<u8>::new(
                    *src.get_unchecked(px + image_configuration.get_r_channel_offset()),
                    *src.get_unchecked(px + image_configuration.get_g_channel_offset()),
                    *src.get_unchecked(px + image_configuration.get_b_channel_offset()),
                );
                let jzazbz = Jzazbz::from_rgb(rgb, TransferFunction::Srgb);
                dst_ptr
                    .add(x)
                    .write_unaligned((jzazbz.jz * full_scale).round().min(scale) as u16);
                *color.get_unchecked_mut(cx) = jzazbz.az;
                *color.get_unchecked_mut(cx + 1) = jzazbz.bz;
                if image_configuration.has_alpha() {
                    let a = *src.get_unchecked(px + image_configuration.get_a_channel_offset());
                    *color.get_unchecked_mut(cx + 2) = a as f32;
                }
            }
        });
}

#[inline]
pub(crate) fn jzazbz_to_generic_image<const IMAGE: u8>(
    src: &[u16],
    src_stride: u32,
    color_planes: &[f32],
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    scale: f32,
) {
    let image_configuration: ImageConfiguration = IMAGE.into();
    let channels = image_configuration.get_channels_count();

    let full_scale = 1. / scale;

    let color_planes_stride = if image_configuration.has_alpha() {
        width as usize * 3usize
    } else {
        width as usize * 2usize
    };

    let src_slice_safe_align = unsafe {
        slice::from_raw_parts(
            src.as_ptr() as *const u8,
            dst_stride as usize * height as usize,
        )
    };

    let color_planes_channels = if image_configuration.has_alpha() {
        3usize
    } else {
        2usize
    };

    dst.par_chunks_exact_mut(dst_stride as usize)
        .zip(color_planes.par_chunks_exact(color_planes_stride))
        .zip(src_slice_safe_align.par_chunks_exact(src_stride as usize))
        .for_each(|((dst, color), src)| unsafe {
            let src_ptr = src.as_ptr() as *const u16;
            for x in 0..width as usize {
                let px = x * channels;
                let cx = x * color_planes_channels;

                let l = src_ptr.add(x).read_unaligned() as f32 * full_scale;

                let a = *color.get_unchecked(cx);
                let b = *color.get_unchecked(cx + 1);

                let rgb = Jzazbz::new(l, a, b);
                let rgb = rgb.to_rgb(TransferFunction::Srgb);
                *dst.get_unchecked_mut(px + image_configuration.get_r_channel_offset()) = rgb.r;
                *dst.get_unchecked_mut(px + image_configuration.get_g_channel_offset()) = rgb.g;
                *dst.get_unchecked_mut(px + image_configuration.get_b_channel_offset()) = rgb.b;
                if image_configuration.has_alpha() {
                    let a = *color.get_unchecked(cx + 2);
                    *dst.get_unchecked_mut(px + image_configuration.get_a_channel_offset()) =
                        a as u8;
                }
            }
        });
}

pub(crate) fn rgb_to_jzazbz(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u16],
    dst_stride: u32,
    color_planes: &mut [f32],
    width: u32,
    height: u32,
    scale: f32,
) {
    generic_image_to_jzazbz::<{ ImageConfiguration::Rgb as u8 }>(
        src,
        src_stride,
        dst,
        dst_stride,
        color_planes,
        width,
        height,
        scale,
    );
}

pub(crate) fn bgra_to_jzazbz(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u16],
    dst_stride: u32,
    color_planes: &mut [f32],
    width: u32,
    height: u32,
    scale: f32,
) {
    generic_image_to_jzazbz::<{ ImageConfiguration::Bgra as u8 }>(
        src,
        src_stride,
        dst,
        dst_stride,
        color_planes,
        width,
        height,
        scale,
    );
}

pub(crate) fn rgba_to_jzazbz(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u16],
    dst_stride: u32,
    color_planes: &mut [f32],
    width: u32,
    height: u32,
    scale: f32,
) {
    generic_image_to_jzazbz::<{ ImageConfiguration::Rgba as u8 }>(
        src,
        src_stride,
        dst,
        dst_stride,
        color_planes,
        width,
        height,
        scale,
    );
}

pub(crate) fn jzazbz_to_rgb(
    src: &[u16],
    src_stride: u32,
    color_planes: &[f32],
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    scale: f32,
) {
    jzazbz_to_generic_image::<{ ImageConfiguration::Rgb as u8 }>(
        src,
        src_stride,
        color_planes,
        dst,
        dst_stride,
        width,
        height,
        scale,
    );
}

pub(crate) fn jzazbz_to_bgra(
    src: &[u16],
    src_stride: u32,
    color_planes: &[f32],
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    scale: f32,
) {
    jzazbz_to_generic_image::<{ ImageConfiguration::Bgra as u8 }>(
        src,
        src_stride,
        color_planes,
        dst,
        dst_stride,
        width,
        height,
        scale,
    );
}

pub(crate) fn jzazbz_to_rgba(
    src: &[u16],
    src_stride: u32,
    color_planes: &[f32],
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    scale: f32,
) {
    jzazbz_to_generic_image::<{ ImageConfiguration::Rgba as u8 }>(
        src,
        src_stride,
        color_planes,
        dst,
        dst_stride,
        width,
        height,
        scale,
    );
}
