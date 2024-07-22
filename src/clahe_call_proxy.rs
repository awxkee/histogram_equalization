use crate::hist_support::{
    blerp, cdf, clip_hist_clahe, make_histogram_region, minmax, AheImplementation, ClaheGridSize,
    ImageHistogram,
};

#[allow(dead_code)]
pub(crate) fn clahe_impl_u16_proxy<const CHANNELS: usize, const IMPLEMENTATION: u8>(
    src: &[u8],
    src_stride: u32,
    dst: &mut [u8],
    dst_stride: u32,
    width: u32,
    height: u32,
    threshold: f32,
    clahe_grid_size: ClaheGridSize,
    bins_count: usize,
    destructuring: fn(&[u8], u32, &mut [u16], u32, &mut [f32], u32, u32, f32),
    structuring: fn(&[u16], u32, &[f32], &mut [u8], u32, u32, u32, f32),
) {
    if bins_count <= 1 {
        panic!("Bins count must be more than one");
    }
    if clahe_grid_size.w == 0 || clahe_grid_size.h == 0 {
        panic!("zero sized grid is not accepted");
    }
    let implementation: AheImplementation = IMPLEMENTATION.into();
    let horizontal_tile_size = width / clahe_grid_size.w;
    let vertical_tile_size = height / clahe_grid_size.h;
    let tiles_horizontal = width / horizontal_tile_size;
    let tiles_vertical = height / vertical_tile_size;

    let mut hsv_image: Vec<u16> = vec![0u16; width as usize * height as usize];
    let hsv_stride = width as usize;

    let mut color_planes: Vec<f32> = vec![0.; width as usize * height as usize * CHANNELS];

    destructuring(
        src,
        src_stride,
        &mut hsv_image,
        hsv_stride as u32 * std::mem::size_of::<u16>() as u32,
        &mut color_planes,
        width,
        height,
        (bins_count - 1) as f32,
    );

    let mut histograms: Vec<Vec<ImageHistogram>> = vec![];

    let max_bins = bins_count - 1;

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

            let mut region_hist = make_histogram_region::<0, 1, u16>(
                &hsv_image,
                hsv_stride as u32,
                start_x,
                end_x,
                start_y,
                end_y,
                bins_count,
            );

            let mut bins = region_hist.bins;
            match implementation {
                AheImplementation::Clahe => {
                    clip_hist_clahe(
                        &mut bins,
                        threshold,
                        (end_x - start_x) as usize,
                        (end_y - start_y) as usize,
                    );
                }
                _ => {}
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

    let mut hsv_offset = 0usize;

    let max_bins = bins_count - 1;

    for y in 0usize..height as usize {
        for x in 0usize..width as usize {
            let c_x_f =
                (x as f32 - horizontal_tile_size as f32 / 2f32) / horizontal_tile_size as f32;
            let r_y_f = (y as f32 - vertical_tile_size as f32 / 2f32) / vertical_tile_size as f32;

            let x1 = (x as f32 - ((c_x_f as i64) as f32 + 0.5f32) * horizontal_tile_size as f32)
                / horizontal_tile_size as f32;
            let y1 = (y as f32 - ((r_y_f as i64) as f32 + 0.5f32) * vertical_tile_size as f32)
                / vertical_tile_size as f32;

            let px = x * 1;

            let value = unsafe { *hsv_image.get_unchecked(hsv_offset + px + 0) }
                .min(max_bins as u16)
                .max(0u16) as usize;

            let r_y = r_y_f.max(0f32) as i64;
            let c_x = c_x_f.max(0f32) as i64;

            let r = (r_y as usize).min(tiles_vertical as usize - 1usize);
            let c = (c_x as usize).min(tiles_horizontal as usize - 1usize);
            let bin1 = histograms[r][c].bins[value] as f32;
            let bin2 =
                histograms[r][(c + 1).min(tiles_horizontal as usize - 1usize)].bins[value] as f32;
            let bin3 =
                histograms[(r + 1).min(tiles_vertical as usize - 1usize)][c].bins[value] as f32;
            let bin4 = histograms[(r + 1).min(tiles_vertical as usize - 1usize)]
                [(c + 1).min(tiles_horizontal as usize - 1usize)]
            .bins[value] as f32;
            let interpolated = blerp(bin1, bin2, bin3, bin4, x1, y1);
            unsafe {
                *hsv_image.get_unchecked_mut(hsv_offset + px + 0) =
                    interpolated.min(max_bins as f32).max(0f32) as u16;
            }
        }

        hsv_offset += hsv_stride;
    }

    structuring(
        &hsv_image,
        hsv_stride as u32 * std::mem::size_of::<u16>() as u32,
        &color_planes,
        dst,
        dst_stride,
        width,
        height,
        (bins_count - 1) as f32,
    );
}
