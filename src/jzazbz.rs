use std::slice;

use colorutils_rs::{Jzazbz, Rgb, TransferFunction};

use crate::image_configuration::ImageConfiguration;

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

    let mut src_offset = 0usize;
    let mut jzazbz_offset = 0usize;
    let mut color_planes_offset = 0usize;
    let color_planes_stride = if image_configuration.has_alpha() {
        width as usize * 3usize
    } else {
        width as usize * 2usize
    };
    for _ in 0..height as usize {
        let dst_ptr = unsafe { (dst.as_mut_ptr() as *mut u8).add(jzazbz_offset) as *mut u16 };
        let new_slice = unsafe { slice::from_raw_parts_mut(dst_ptr, width as usize * channels) };
        for x in 0..width as usize {
            let px = x * channels;
            let cx = x * 3;

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
            let jzazbz = Jzazbz::from_rgb(rgb, TransferFunction::Srgb);
            unsafe {
                *new_slice.get_unchecked_mut(x) =
                    (jzazbz.jz * full_scale).round().min(scale) as u16;
                // Just for storing in u16 adding 500 to subtract 500 after to keep values in positive range
                *color_planes.get_unchecked_mut(color_planes_offset + cx + 0) = jzazbz.az;
                *color_planes.get_unchecked_mut(color_planes_offset + cx + 1) = jzazbz.bz;
                if image_configuration.has_alpha() {
                    let a = *src.get_unchecked(
                        src_offset + px + image_configuration.get_a_channel_offset(),
                    );
                    *color_planes.get_unchecked_mut(color_planes_offset + cx + 2) = a as f32;
                }
            }
        }
        src_offset += src_stride as usize;
        jzazbz_offset += dst_stride as usize;
        color_planes_offset += color_planes_stride;
    }
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

    let mut src_offset = 0usize;
    let mut src_color_planes_offset = 0usize;
    let mut dst_offset = 0usize;
    for _ in 0..height as usize {
        let src_ptr = unsafe { (src.as_ptr() as *const u8).add(src_offset) as *const u16 };
        let source_slice = unsafe { slice::from_raw_parts(src_ptr, width as usize * channels) };
        for x in 0..width as usize {
            let px = x * channels;
            let cx = x * 3;

            let l = unsafe { *source_slice.get_unchecked(x) } as f32 * full_scale;

            let a = unsafe { *color_planes.get_unchecked(src_color_planes_offset + cx + 0) };
            let b = unsafe { *color_planes.get_unchecked(src_color_planes_offset + cx + 1) };

            let rgb = Jzazbz::new(l, a, b);
            let rgb = rgb.to_rgb(TransferFunction::Srgb);
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
                    let a = *color_planes.get_unchecked(src_color_planes_offset + cx + 2);
                    *dst.get_unchecked_mut(
                        dst_offset + px + image_configuration.get_a_channel_offset(),
                    ) = a as u8;
                }
            }
        }
        src_offset += src_stride as usize;
        dst_offset += dst_stride as usize;
        src_color_planes_offset += color_planes_stride;
    }
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
