use std::slice;

use crate::image_configuration::ImageConfiguration;
use colorutils_rs::{Lab, Rgb};
use rayon::iter::{IndexedParallelIterator, ParallelIterator};
use rayon::prelude::{ParallelSlice, ParallelSliceMut};

#[inline]
pub(crate) fn generic_image_to_lab<const IMAGE: u8>(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u16],
    dst_stride: u32,
    width: u32,
    height: u32,
    scale: f32,
) {
    let image_configuration: ImageConfiguration = IMAGE.into();
    let channels = image_configuration.get_channels_count();

    let full_scale = scale / 100f32;

    let dst_slice_safe_align = unsafe {
        slice::from_raw_parts_mut(
            dst.as_mut_ptr() as *mut u8,
            dst_stride as usize * height as usize,
        )
    };

    dst_slice_safe_align
        .par_chunks_exact_mut(dst_stride as usize)
        .zip(src.par_chunks_exact(src_stride as usize))
        .for_each(|(dst, src)| unsafe {
            for x in 0..width as usize {
                let px = x * channels;

                let rgb = Rgb::<u8>::new(
                    *src.get_unchecked(px + image_configuration.get_r_channel_offset()),
                    *src.get_unchecked(px + image_configuration.get_g_channel_offset()),
                    *src.get_unchecked(px + image_configuration.get_b_channel_offset()),
                );
                let luv = rgb.to_lab();

                let dst_ptr = (dst.as_mut_ptr() as *mut u16).add(px);
                dst_ptr.write_unaligned((luv.l * full_scale).round().min(scale) as u16);
                dst_ptr
                    .add(1)
                    .write_unaligned((luv.a * 100f32 + 100f32) as u16);
                dst_ptr
                    .add(2)
                    .write_unaligned((luv.b * 100f32 + 100f32) as u16);

                if image_configuration.has_alpha() {
                    let a = *src.get_unchecked(px + image_configuration.get_a_channel_offset());
                    dst_ptr.add(3).write_unaligned(a as u16);
                }
            }
        });
}

#[inline]
pub(crate) fn lab_to_generic_image<const IMAGE: u8>(
    src: &[u16],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    scale: f32,
) {
    let image_configuration: ImageConfiguration = IMAGE.into();
    let channels = image_configuration.get_channels_count();

    let full_scale = 100f32 / scale;

    let src_slice_safe_align = unsafe {
        slice::from_raw_parts(
            src.as_ptr() as *const u8,
            src_stride as usize * height as usize,
        )
    };

    dst.par_chunks_exact_mut(dst_stride as usize)
        .zip(src_slice_safe_align.par_chunks_exact(src_stride as usize))
        .for_each(|(dst, src)| unsafe {
            for x in 0..width as usize {
                let px = x * channels;

                let src_ptr = (src.as_ptr() as *const u16).add(px);

                let l = src_ptr.read_unaligned() as f32 * full_scale;

                let a = (src_ptr.add(1).read_unaligned() as f32 - 100f32) / 100.;
                let b = (src_ptr.add(2).read_unaligned() as f32 - 100f32) / 100.;

                let rgb = Lab::new(l, a, b);
                let rgb = rgb.to_rgb();
                *dst.get_unchecked_mut(px + image_configuration.get_r_channel_offset()) = rgb.r;
                *dst.get_unchecked_mut(px + image_configuration.get_g_channel_offset()) = rgb.g;
                *dst.get_unchecked_mut(px + image_configuration.get_b_channel_offset()) = rgb.b;
                if image_configuration.has_alpha() {
                    let a = src_ptr.add(3).read_unaligned();
                    *dst.get_unchecked_mut(px + image_configuration.get_a_channel_offset()) =
                        a as u8;
                }
            }
        });
}

pub(crate) fn rgb_to_lab(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u16],
    dst_stride: u32,
    width: u32,
    height: u32,
    scale: f32,
) {
    generic_image_to_lab::<{ ImageConfiguration::Rgb as u8 }>(
        src, src_stride, dst, dst_stride, width, height, scale,
    );
}

pub(crate) fn bgra_to_lab(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u16],
    dst_stride: u32,
    width: u32,
    height: u32,
    scale: f32,
) {
    generic_image_to_lab::<{ ImageConfiguration::Bgra as u8 }>(
        src, src_stride, dst, dst_stride, width, height, scale,
    );
}

pub(crate) fn rgba_to_lab(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u16],
    dst_stride: u32,
    width: u32,
    height: u32,
    scale: f32,
) {
    generic_image_to_lab::<{ ImageConfiguration::Rgba as u8 }>(
        src, src_stride, dst, dst_stride, width, height, scale,
    );
}

pub(crate) fn lab_to_rgb(
    src: &[u16],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    scale: f32,
) {
    lab_to_generic_image::<{ ImageConfiguration::Rgb as u8 }>(
        src, src_stride, dst, dst_stride, width, height, scale,
    );
}

pub(crate) fn lab_to_bgra(
    src: &[u16],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    scale: f32,
) {
    lab_to_generic_image::<{ ImageConfiguration::Bgra as u8 }>(
        src, src_stride, dst, dst_stride, width, height, scale,
    );
}

pub(crate) fn lab_to_rgba(
    src: &[u16],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    scale: f32,
) {
    lab_to_generic_image::<{ ImageConfiguration::Rgba as u8 }>(
        src, src_stride, dst, dst_stride, width, height, scale,
    );
}
