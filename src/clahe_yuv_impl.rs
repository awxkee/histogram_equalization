use crate::hist_support::{
    blerp, cdf, clip_hist_clahe, make_histogram_region, minmax, AheImplementation,
};
use crate::{ClaheGridSize, ImageHistogram};
use rayon::iter::{IndexedParallelIterator, ParallelIterator};
use rayon::prelude::ParallelSliceMut;
use yuvutils_rs::{BufferStoreMut, YuvError, YuvPlanarImageMut, YuvPlanarImageWithAlpha, YuvRange};

#[allow(dead_code)]
pub(crate) fn clahe_yuv_impl<const CHANNELS: usize, const IMPLEMENTATION: u8>(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    threshold: f32,
    clahe_grid_size: ClaheGridSize,
    destructuring: fn(&mut YuvPlanarImageMut<u8>, &[u8], u32, YuvRange) -> Result<(), YuvError>,
    structuring: fn(&YuvPlanarImageWithAlpha<u8>, &mut [u8], u32, YuvRange) -> Result<(), YuvError>,
) {
    const CHANNEL_POSITION: usize = 0;
    if clahe_grid_size.w == 0 || clahe_grid_size.h == 0 {
        panic!("zero sized grid is not accepted");
    }
    let implementation: AheImplementation = IMPLEMENTATION.into();
    let bins_count = 256;

    let mut a_plane = if CHANNELS == 4 {
        vec![0u8; width as usize * height as usize]
    } else {
        Vec::new()
    };

    let mut planar_image_mut = YuvPlanarImageMut {
        y_plane: BufferStoreMut::Owned(vec![0u8; width as usize * height as usize]),
        y_stride: width,
        u_plane: BufferStoreMut::Owned(vec![0u8; width as usize * height as usize]),
        u_stride: width,
        v_plane: BufferStoreMut::Owned(vec![0u8; width as usize * height as usize]),
        v_stride: width,
        width,
        height,
    };

    destructuring(&mut planar_image_mut, src, src_stride, YuvRange::Full).unwrap();
    if CHANNELS == 4 {
        a_plane
            .chunks_exact_mut(width as usize)
            .zip(src.chunks_exact(src_stride as usize))
            .for_each(|(a_row, src)| {
                for (dst, src) in a_row.iter_mut().zip(src.chunks_exact(4)) {
                    *dst = src[3];
                }
            });
    }

    let mut histograms: Vec<Vec<ImageHistogram>> = vec![];

    let max_bins = bins_count - 1;

    let horizontal_tile_size = width / clahe_grid_size.w;
    let vertical_tile_size = height / clahe_grid_size.h;
    let tiles_horizontal = width / horizontal_tile_size;
    let tiles_vertical = height / vertical_tile_size;
    for h in 0..tiles_vertical {
        let mut regions_hist: Vec<ImageHistogram> = vec![];
        for w in 0..tiles_horizontal {
            let start_x = w * horizontal_tile_size;
            let start_y = h * vertical_tile_size;
            let mut end_x = (w + 1) * horizontal_tile_size;
            if w + 1 == tiles_horizontal {
                end_x = width;
            }
            let mut end_y = (h + 1) * vertical_tile_size;
            if h + 1 == tiles_vertical {
                end_y = height;
            }

            let mut region_hist = make_histogram_region::<CHANNEL_POSITION, 1, u8>(
                planar_image_mut.y_plane.borrow(),
                width,
                start_x,
                end_x,
                start_y,
                end_y,
                bins_count,
            );

            let mut bins = region_hist.bins;
            if implementation == AheImplementation::Clahe {
                clip_hist_clahe(
                    &mut bins,
                    threshold,
                    (end_x - start_x) as usize,
                    (end_y - start_y) as usize,
                );
            }
            cdf(&mut bins);

            let (min_bin, _) = minmax(&bins);

            let distance_r =
                1f64 / ((end_y - start_y) as f64 * (end_x - start_x) as f64 - min_bin as f64);

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

            region_hist.bins = bins;

            regions_hist.push(region_hist);
        }
        histograms.push(regions_hist);
    }

    let max_bins = bins_count - 1;

    planar_image_mut
        .y_plane
        .borrow_mut()
        .par_chunks_exact_mut(width as usize)
        .enumerate()
        .for_each(|(y, y_plane)| {
            for (x, y_data) in y_plane.iter_mut().enumerate() {
                let c_x_f =
                    (x as f32 - horizontal_tile_size as f32 / 2f32) / horizontal_tile_size as f32;
                let r_y_f =
                    (y as f32 - vertical_tile_size as f32 / 2f32) / vertical_tile_size as f32;

                let x1 = (x as f32
                    - ((c_x_f as i64) as f32 + 0.5f32) * horizontal_tile_size as f32)
                    / horizontal_tile_size as f32;
                let y1 = (y as f32 - ((r_y_f as i64) as f32 + 0.5f32) * vertical_tile_size as f32)
                    / vertical_tile_size as f32;

                let value = (*y_data).min(max_bins as u8) as usize;

                let r_y = r_y_f.max(0f32) as i64;
                let c_x = c_x_f.max(0f32) as i64;

                let r = (r_y as usize).min(tiles_vertical as usize - 1usize);
                let c = (c_x as usize).min(tiles_horizontal as usize - 1usize);
                let bin1 = histograms[r][c].bins[value] as f32;
                let bin2 = histograms[r][(c + 1).min(tiles_horizontal as usize - 1usize)].bins
                    [value] as f32;
                let bin3 =
                    histograms[(r + 1).min(tiles_vertical as usize - 1usize)][c].bins[value] as f32;
                let bin4 = histograms[(r + 1).min(tiles_vertical as usize - 1usize)]
                    [(c + 1).min(tiles_horizontal as usize - 1usize)]
                .bins[value] as f32;
                let interpolated = blerp(bin1, bin2, bin3, bin4, x1, y1);
                *y_data = interpolated.min(max_bins as f32).max(0f32) as u8;
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
