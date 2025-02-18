use crate::hist_support::{cdf, make_histogram_region, minmax};
use yuvutils_rs::{BufferStoreMut, YuvError, YuvPlanarImageMut, YuvPlanarImageWithAlpha, YuvRange};

#[allow(dead_code)]
pub(crate) fn equalize_histogram_yuv_impl<const CHANNELS: u8>(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    destructuring: fn(&mut YuvPlanarImageMut<u8>, &[u8], u32, YuvRange) -> Result<(), YuvError>,
    structuring: fn(&YuvPlanarImageWithAlpha<u8>, &mut [u8], u32, YuvRange) -> Result<(), YuvError>,
) {
    let bins_count = 256;

    let y_plane: Vec<u8> = vec![0u8; width as usize * height as usize];

    let u_plane: Vec<u8> = vec![0u8; width as usize * height as usize];

    let v_plane: Vec<u8> = vec![0u8; width as usize * height as usize];
    let mut a_plane = if CHANNELS == 4 {
        vec![0u8; width as usize * height as usize]
    } else {
        Vec::new()
    };

    let mut planar_image_mut = YuvPlanarImageMut {
        y_plane: BufferStoreMut::Owned(y_plane),
        y_stride: width,
        u_plane: BufferStoreMut::Owned(u_plane),
        u_stride: width,
        v_plane: BufferStoreMut::Owned(v_plane),
        v_stride: width,
        width,
        height,
    };

    destructuring(&mut planar_image_mut, src, src_stride, YuvRange::Full).unwrap();

    if CHANNELS == 4 {
        let mut a_shift = 0usize;
        let mut y_shift = 0usize;
        for _ in 0usize..height as usize {
            for x in 0usize..width as usize {
                unsafe {
                    *a_plane.get_unchecked_mut(a_shift + x) =
                        *src.get_unchecked(y_shift + x * 4 + 3);
                }
            }
            y_shift += src_stride as usize;
            a_shift += width as usize;
        }
    }

    let histogram = make_histogram_region::<0, 1, u8>(
        planar_image_mut.y_plane.borrow(),
        width,
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

    if distance_r != 0f64 {
        for i in 0..256usize {
            unsafe {
                *bins.get_unchecked_mut(i) =
                    (255f64 * (*bins.get_unchecked(i) as f64 - min_bin as f64) * distance_r)
                        .round()
                        .min(255f64) as u64;
            }
        }
    }

    planar_image_mut
        .y_plane
        .borrow_mut()
        .chunks_exact_mut(width as usize)
        .for_each(|row| {
            for dst in row.iter_mut() {
                let value = *dst;
                let bin_value = unsafe { *bins.get_unchecked(value as usize) };
                *dst = bin_value as u8;
            }
        });

    let planar_image = YuvPlanarImageWithAlpha {
        y_plane: planar_image_mut.y_plane.borrow(),
        y_stride: width,
        u_plane: planar_image_mut.u_plane.borrow(),
        u_stride: width,
        v_plane: planar_image_mut.v_plane.borrow(),
        v_stride: width,
        a_plane: &a_plane,
        a_stride: width,
        width,
        height,
    };

    structuring(&planar_image, dst, dst_stride, YuvRange::Full).unwrap();
}
