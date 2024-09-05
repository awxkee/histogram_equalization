use num_traits::FromPrimitive;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) enum AheImplementation {
    Ahe = 1,
    Clahe = 2,
}

impl From<u8> for AheImplementation {
    fn from(value: u8) -> Self {
        match value {
            1 => AheImplementation::Ahe,
            2 => AheImplementation::Clahe,
            _ => {
                panic!("{} not implemented in AHE", value)
            }
        }
    }
}

#[inline]
pub(crate) fn lerp(a: f32, b: f32, f: f32) -> f32 {
    a * (1f32 - f) + (b * f)
}

#[inline]
pub(crate) fn blerp(c00: f32, c10: f32, c01: f32, c11: f32, tx: f32, ty: f32) -> f32 {
    lerp(lerp(c00, c10, tx), lerp(c01, c11, tx), ty)
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct ClaheGridSize {
    pub w: u32,
    pub h: u32,
}

impl ClaheGridSize {
    pub fn new(w: u32, h: u32) -> ClaheGridSize {
        ClaheGridSize { w, h }
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct ImageHistogram {
    pub bins: Vec<u64>,
}

pub(crate) fn cdf(arr: &mut [u64]) {
    let mut sum: u64 = 0u64;
    for i in 0..arr.len() {
        sum += unsafe { *arr.get_unchecked(i) };
        unsafe {
            *arr.get_unchecked_mut(i) = sum;
        }
    }
}

pub(crate) fn clip_hist_clahe(bins: &mut [u64], level: f32, width: usize, height: usize) {
    let threshold_value: f64 = level as f64 * width as f64 * height as f64 / bins.len() as f64;
    let clip_limit = threshold_value as u64;
    let mut excess = 0u64;

    for i in 0..bins.len() {
        let value = unsafe { *bins.get_unchecked(i) };
        if value > clip_limit {
            excess += value;
            unsafe {
                *bins.get_unchecked_mut(i) = clip_limit;
            }
        }
    }

    let mean_excess = (excess as f64 / bins.len() as f64) as u64;

    for i in 0..bins.len() {
        unsafe {
            *bins.get_unchecked_mut(i) = *bins.get_unchecked(i) + mean_excess;
        }
    }
}

pub(crate) fn minmax(slice: &[u64]) -> (u64, u64) {
    let mut min_value = u64::MAX;
    let mut max_value = u64::MIN;
    let mut i = 0usize;
    let count = slice.len();

    while i < count {
        let value = unsafe { *slice.get_unchecked(i) };
        if value < min_value {
            min_value = value;
        }
        if value > max_value {
            max_value = value;
        }
        i += 1;
    }
    (min_value, max_value)
}

pub(crate) fn make_histogram_region<
    const CHANNEL: usize,
    const CHANNELS: usize,
    T: Copy + Ord + FromPrimitive + Default + Into<usize>,
>(
    in_place: &[T],
    stride: u32,
    start_x: u32,
    end_x: u32,
    start_y: u32,
    end_y: u32,
    bins_count: usize,
) -> ImageHistogram {
    let mut bins = vec![0u64; bins_count];
    let mut y_shift = (stride * start_y) as usize;

    let min_v = T::from_u16(bins_count as u16 - 1).unwrap_or_default();

    for _ in start_y as usize..end_y as usize {
        for x in start_x as usize..end_x as usize {
            let px = x * CHANNELS;
            let value: T = unsafe { *in_place.get_unchecked(y_shift + px + CHANNEL) }
                .min(min_v)
                .max(T::from_u16(0u16).unwrap_or_default());
            let iter: usize = value.into();
            unsafe {
                *bins.get_unchecked_mut(iter) += 1u64;
            }
        }
        y_shift += stride as usize;
    }

    let hist = ImageHistogram { bins };
    hist
}
