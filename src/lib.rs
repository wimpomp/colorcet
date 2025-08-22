mod colormaps;

use std::ops::Sub;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ColorcetError {
    #[error("no colormap with name {0} known")]
    ColormapNotFound(String),
}

pub trait Int: Sub + Sized {
    const MIN: Self;
    const MAX: Self;
}

macro_rules! impl_int {
    ($($t:tt),+ $(,)?) => {
        $(
            impl Int for $t {
                const MIN: Self = $t::MIN;
                const MAX: Self = $t::MAX;
            }
        )*
    };
}

impl_int!(
    u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, isize, usize
);

pub struct ColorMap([[f64; 3]; 256]);

impl ColorMap {
    /// get a vector of rgb values, scaled between 0.0 and 1.0
    pub fn get_colors_f64(&self) -> Vec<Vec<f64>> {
        self.0.iter().map(|row| row.to_vec()).collect()
    }

    /// get a vector of rgb values, scaled between T::MIN and T::MAX
    pub fn get_colors_int<T>(&self) -> Vec<Vec<T>>
    where
        T: Int + From<f64>,
        f64: From<T>,
    {
        let a: f64 = T::MIN.into();
        let b: f64 = T::MAX.into();
        let c = b - a;
        self.0
            .iter()
            .map(|row| row.iter().map(|&i| ((i - a) / c).round().into()).collect())
            .collect()
    }
}

/// find a colorcet colormap by name, add _r to reverse the colormap
pub fn get_named_colormap(name: &str) -> Result<ColorMap, ColorcetError> {
    let (name0, reverse) = if let Some(name) = name.strip_suffix("_r") {
        (name, true)
    } else {
        (name, false)
    };

    if let Some(&alias) = colormaps::ALIASES.get(name0)
        && let Some(cmap) = colormaps::COLOR_MAPS.get(alias)
    {
        let mut cmap = cmap.to_owned();
        if reverse {
            cmap.reverse();
        }
        Ok(ColorMap(cmap))
    } else {
        Err(ColorcetError::ColormapNotFound(name.into()))
    }
}
