use crate::hist_support::{cdf, make_histogram_region, minmax};
use colorutils_rs::{BufferStore, ColorError, ImageBuffer, ImageBufferMut};

#[allow(dead_code)]
pub(crate) fn equalize_histogram_impl<const CHANNELS: usize, const CHANNEL_POSITION: usize>(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    bins_count: usize,
    destructuring: fn(
        &ImageBuffer<'_, u8>,
        &mut ImageBufferMut<'_, u16>,
        f32,
    ) -> Result<(), ColorError>,
    structuring: fn(
        &ImageBuffer<'_, u16>,
        &mut ImageBufferMut<'_, u8>,
        f32,
    ) -> Result<(), ColorError>,
) {
    if bins_count <= 1 {
        panic!("Bins count must be more than one");
    }

    let mut hsv_image: Vec<u16> = vec![0u16; width as usize * height as usize * CHANNELS];
    let hsv_stride = width as usize * CHANNELS;

    let src_image = ImageBuffer {
        data: std::borrow::Cow::Borrowed(src),
        stride: src_stride,
        width,
        height,
        channels: CHANNELS as u32,
    };
    let mut dst_image_hsv = ImageBufferMut {
        data: BufferStore::Borrowed(&mut hsv_image),
        stride: hsv_stride as u32,
        width,
        height,
        channels: 3,
    };

    _ = destructuring(&src_image, &mut dst_image_hsv, (bins_count - 1) as f32);
    let histogram = make_histogram_region::<CHANNEL_POSITION, CHANNELS, u16>(
        &hsv_image,
        hsv_stride as u32,
        0,
        width,
        0,
        height,
        bins_count,
    );
    let mut bins = histogram.bins;

    cdf(&mut bins);

    let pixels_count = width * height;

    let (min_bin, _) = minmax(&bins);

    let distance_r = 1f64 / (pixels_count as f64 - min_bin as f64);

    let max_bins = bins_count - 1;

    if distance_r != 0f64 {
        for i in 0..bins_count {
            unsafe {
                *bins.get_unchecked_mut(i) = (max_bins as f64
                    * (*bins.get_unchecked(i) as f64 - min_bin as f64)
                    * distance_r)
                    .round()
                    .min(max_bins as f64)
                    .max(0f64) as u64;
            }
        }
    }

    let mut y_shift = 0usize;
    for _ in 0usize..height as usize {
        for x in 0usize..width as usize {
            let px = x * CHANNELS;
            unsafe {
                let value = *hsv_image.get_unchecked(y_shift + px + CHANNEL_POSITION) as usize;
                let bin_value = *bins.get_unchecked(value);
                *hsv_image.get_unchecked_mut(y_shift + px + CHANNEL_POSITION) = bin_value as u16;
            }
        }
        y_shift += hsv_stride;
    }

    let mut dst_image = ImageBufferMut {
        data: BufferStore::Borrowed(dst),
        stride: dst_stride,
        width,
        height,
        channels: CHANNELS as u32,
    };
    let src_image_hsv = ImageBuffer {
        data: std::borrow::Cow::Borrowed(&hsv_image),
        stride: hsv_stride as u32,
        width,
        height,
        channels: 3,
    };

    _ = structuring(&src_image_hsv, &mut dst_image, (bins_count - 1) as f32);
}
