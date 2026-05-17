use crate::image_configuration::ImageConfiguration;
use colorutils_rs::{ColorError, ImageBuffer, ImageBufferMut, Lab, Rgb};
use rayon::iter::{IndexedParallelIterator, ParallelIterator};
use rayon::prelude::{ParallelSlice, ParallelSliceMut};

pub(crate) fn generic_image_to_lab<const IMAGE: u8>(
    src: &ImageBuffer<'_, u8>,
    dst: &mut ImageBufferMut<'_, u16>,
    scale: f32,
) -> Result<(), ColorError> {
    src.validate()?;
    dst.validate()?;
    dst.try_match_immutable(src)?;
    let image_configuration: ImageConfiguration = IMAGE.into();
    let channels = image_configuration.get_channels_count();

    let full_scale = scale / 100f32;

    let dst_stride = dst.stride();
    let width = src.width as usize;
    dst.data
        .borrow_mut()
        .par_chunks_exact_mut(dst_stride)
        .zip(src.data.as_ref().par_chunks_exact(src.stride()))
        .for_each(|(dst, src)| {
            let dst = &mut dst[..width * channels];
            let src = &src[..width * channels];
            if image_configuration.has_alpha() {
                for (dst, src) in dst
                    .as_chunks_mut::<4>()
                    .0
                    .iter_mut()
                    .zip(src.as_chunks::<4>().0.iter())
                {
                    let rgb = Rgb::<u8>::new(
                        src[image_configuration.get_r_channel_offset()],
                        src[image_configuration.get_g_channel_offset()],
                        src[image_configuration.get_b_channel_offset()],
                    );
                    let luv = rgb.to_lab();

                    dst[0] = (luv.l * full_scale).round().min(scale) as u16;
                    dst[1] = (luv.a * 100f32 + 100f32).round() as u16;
                    dst[2] = (luv.b * 100f32 + 100f32).round() as u16;
                    dst[3] = src[image_configuration.get_a_channel_offset()] as u16;
                }
            } else {
                for (dst, src) in dst
                    .as_chunks_mut::<3>()
                    .0
                    .iter_mut()
                    .zip(src.as_chunks::<3>().0.iter())
                {
                    let rgb = Rgb::<u8>::new(
                        src[image_configuration.get_r_channel_offset()],
                        src[image_configuration.get_g_channel_offset()],
                        src[image_configuration.get_b_channel_offset()],
                    );
                    let luv = rgb.to_lab();

                    dst[0] = (luv.l * full_scale).round().min(scale) as u16;
                    dst[1] = (luv.a * 100f32 + 100f32).round() as u16;
                    dst[2] = (luv.b * 100f32 + 100f32).round() as u16;
                }
            }
        });
    Ok(())
}

pub(crate) fn lab_to_generic_image<const IMAGE: u8>(
    src: &ImageBuffer<'_, u16>,
    dst: &mut ImageBufferMut<'_, u8>,
    scale: f32,
) -> Result<(), ColorError> {
    let image_configuration: ImageConfiguration = IMAGE.into();
    let channels = image_configuration.get_channels_count();

    let full_scale = 100f32 / scale;

    let dst_stride = dst.stride();
    let width = src.width as usize;
    dst.data
        .borrow_mut()
        .par_chunks_exact_mut(dst_stride)
        .zip(src.data.as_ref().par_chunks_exact(src.stride()))
        .for_each(|(dst, src)| {
            let dst = &mut dst[..width * channels];
            let src = &src[..width * channels];
            if image_configuration.has_alpha() {
                for (dst, src) in dst
                    .as_chunks_mut::<4>()
                    .0
                    .iter_mut()
                    .zip(src.as_chunks::<4>().0.iter())
                {
                    let l = src[0] as f32 * full_scale;

                    let a = (src[1] as f32 - 100f32) * (1. / 100.);
                    let b = (src[2] as f32 - 100f32) * (1. / 100.);

                    let rgb = Lab::new(l, a, b);
                    let rgb = rgb.to_rgb();

                    dst[image_configuration.get_r_channel_offset()] = rgb.r;
                    dst[image_configuration.get_g_channel_offset()] = rgb.g;
                    dst[image_configuration.get_b_channel_offset()] = rgb.b;
                    dst[image_configuration.get_a_channel_offset()] = src[3] as u8;
                }
            } else {
                for (dst, src) in dst
                    .as_chunks_mut::<3>()
                    .0
                    .iter_mut()
                    .zip(src.as_chunks::<3>().0.iter())
                {
                    let l = src[0] as f32 * full_scale;

                    let a = (src[1] as f32 - 100f32) * (1. / 100.);
                    let b = (src[2] as f32 - 100f32) * (1. / 100.);

                    let rgb = Lab::new(l, a, b);
                    let rgb = rgb.to_rgb();

                    dst[image_configuration.get_r_channel_offset()] = rgb.r;
                    dst[image_configuration.get_g_channel_offset()] = rgb.g;
                    dst[image_configuration.get_b_channel_offset()] = rgb.b;
                }
            }
        });
    Ok(())
}

pub(crate) fn rgb_to_lab(
    src: &ImageBuffer<'_, u8>,
    dst: &mut ImageBufferMut<'_, u16>,
    scale: f32,
) -> Result<(), ColorError> {
    generic_image_to_lab::<{ ImageConfiguration::Rgb as u8 }>(src, dst, scale)
}

pub(crate) fn bgra_to_lab(
    src: &ImageBuffer<'_, u8>,
    dst: &mut ImageBufferMut<'_, u16>,
    scale: f32,
) -> Result<(), ColorError> {
    generic_image_to_lab::<{ ImageConfiguration::Bgra as u8 }>(src, dst, scale)
}

pub(crate) fn rgba_to_lab(
    src: &ImageBuffer<'_, u8>,
    dst: &mut ImageBufferMut<'_, u16>,
    scale: f32,
) -> Result<(), ColorError> {
    generic_image_to_lab::<{ ImageConfiguration::Rgba as u8 }>(src, dst, scale)
}

pub(crate) fn lab_to_rgb(
    src: &ImageBuffer<'_, u16>,
    dst: &mut ImageBufferMut<'_, u8>,
    scale: f32,
) -> Result<(), ColorError> {
    lab_to_generic_image::<{ ImageConfiguration::Rgb as u8 }>(src, dst, scale)
}

pub(crate) fn lab_to_bgra(
    src: &ImageBuffer<'_, u16>,
    dst: &mut ImageBufferMut<'_, u8>,
    scale: f32,
) -> Result<(), ColorError> {
    lab_to_generic_image::<{ ImageConfiguration::Bgra as u8 }>(src, dst, scale)
}

pub(crate) fn lab_to_rgba(
    src: &ImageBuffer<'_, u16>,
    dst: &mut ImageBufferMut<'_, u8>,
    scale: f32,
) -> Result<(), ColorError> {
    lab_to_generic_image::<{ ImageConfiguration::Rgba as u8 }>(src, dst, scale)
}
