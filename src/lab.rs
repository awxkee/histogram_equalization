use std::slice;

use colorutils_rs::{Lab, Rgb};

use crate::image_configuration::ImageConfiguration;

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

    let mut src_offset = 0usize;
    let mut lab_offset = 0usize;
    for _ in 0..height as usize {
        let dst_ptr = unsafe { (dst.as_mut_ptr() as *mut u8).add(lab_offset) as *mut u16 };
        let new_slice = unsafe { slice::from_raw_parts_mut(dst_ptr, width as usize * channels) };
        for x in 0..width as usize {
            let px = x * channels;

            let rgb = Rgb::<u8>::new(
                unsafe {
                    *src.get_unchecked(src_offset + px + image_configuration.get_r_channel_offset())
                },
                unsafe {
                    *src.get_unchecked(src_offset + px + image_configuration.get_g_channel_offset())
                },
                unsafe {
                    *src.get_unchecked(src_offset + px + image_configuration.get_b_channel_offset())
                },
            );
            let luv = rgb.to_lab();
            unsafe {
                *new_slice.get_unchecked_mut(px) = (luv.l * full_scale).round().min(scale) as u16;
                // Just for storing in u16 adding 500 to subtract 500 after to keep values in positive range
                *new_slice.get_unchecked_mut(px + 1) = (luv.a * 100f32 + 100f32) as u16;
                *new_slice.get_unchecked_mut(px + 2) = (luv.b * 100f32 + 100f32) as u16;
                if image_configuration.has_alpha() {
                    let a = *src.get_unchecked(
                        src_offset + px + image_configuration.get_a_channel_offset(),
                    );
                    *new_slice.get_unchecked_mut(px + 3) = a as u16;
                }
            }
        }
        src_offset += src_stride as usize;
        lab_offset += dst_stride as usize;
    }
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

    let mut src_offset = 0usize;
    let mut dst_offset = 0usize;
    for _ in 0..height as usize {
        let src_ptr = unsafe { (src.as_ptr() as *const u8).add(src_offset) as *const u16 };
        let source_slice = unsafe { slice::from_raw_parts(src_ptr, width as usize * channels) };
        for x in 0..width as usize {
            let px = x * channels;

            let l = unsafe { *source_slice.get_unchecked(px) } as f32 * full_scale;

            let a = (unsafe { *source_slice.get_unchecked(px + 1) } as f32 - 100f32) / 100.;
            let b = (unsafe { *source_slice.get_unchecked(px + 2) } as f32 - 100f32) / 100.;

            let rgb = Lab::new(l, a, b);
            let rgb = rgb.to_rgb();
            unsafe {
                *dst.get_unchecked_mut(
                    dst_offset + px + image_configuration.get_r_channel_offset(),
                ) = rgb.r;
                *dst.get_unchecked_mut(
                    dst_offset + px + image_configuration.get_g_channel_offset(),
                ) = rgb.g;
                *dst.get_unchecked_mut(
                    dst_offset + px + image_configuration.get_b_channel_offset(),
                ) = rgb.b;
                if image_configuration.has_alpha() {
                    let a = *source_slice.get_unchecked(px + 3);
                    *dst.get_unchecked_mut(
                        dst_offset + px + image_configuration.get_a_channel_offset(),
                    ) = a as u8;
                }
            }
        }
        src_offset += src_stride as usize;
        dst_offset += dst_stride as usize;
    }
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
