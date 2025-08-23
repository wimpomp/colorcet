//! Colorcet: a list of [colorcet](https://colorcet.com/),
//! [matplotlib](https://pypi.org/project/matplotlib/) and simple
//! gradient colormaps.

mod colormaps;

use colorgrad::{
    BasisGradient, CatmullRomGradient, Color, Gradient, GradientBuilder, GradientBuilderError,
    LinearGradient,
};
use num_traits::{Bounded, Float, FromPrimitive, NumCast, PrimInt, ToPrimitive};
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ColorcetError {
    #[error("no colormap with name {0} known")]
    ColormapNotFound(String),
}

/// Struct holding f64 values defining the colormap.
///
/// # Examples
/// ```
/// use colorcet::ColorMap;
/// use colorgrad::{LinearGradient, Color};
///
/// let colormap: ColorMap = "glasbey".parse().unwrap();
/// let vec_color: Vec<Color> = colormap.clone().try_into().unwrap();
/// let vec_css: Vec<String> = colormap.clone().try_into().unwrap();
/// let linear_gradient: LinearGradient = colormap.clone().try_into().unwrap();
/// let vec_float: Vec<[f64; 3]> = colormap.clone().get_rgb_float();
/// let vec_int: Vec<[u8; 3]> = colormap.get_rgb_int();
/// ```
#[derive(Clone, Debug)]
pub struct ColorMap(Vec<[f64; 3]>);

impl ColorMap {
    /// get a vector of rgb values, scaled between 0.0 and 1.0
    pub fn get_rgb_float<T>(&self) -> Vec<[T; 3]>
    where
        T: Float + NumCast,
    {
        self.0
            .iter()
            .map(|color| {
                [
                    T::from(color[0]).unwrap(),
                    T::from(color[1]).unwrap(),
                    T::from(color[2]).unwrap(),
                ]
            })
            .collect()
    }

    /// get a vector of rgb values, scaled between T::MIN and T::MAX
    pub fn get_rgb_int<T>(&self) -> Vec<[T; 3]>
    where
        T: Bounded + PrimInt + FromPrimitive + ToPrimitive,
    {
        let a: f64 = T::min_value().to_f64().unwrap();
        let b: f64 = T::max_value().to_f64().unwrap();
        let c = b - a;
        self.0
            .iter()
            .map(|color| {
                [
                    T::from_f64(((color[0] - a) / c).round()).unwrap(),
                    T::from_f64(((color[1] - a) / c).round()).unwrap(),
                    T::from_f64(((color[2] - a) / c).round()).unwrap(),
                ]
            })
            .collect()
    }

    /// get names of all colormaps defined in this crate
    pub fn all_colormap_names() -> Vec<String> {
        colormaps::ALIASES
            .keys()
            .chain(colormaps::COLOR_MAPS.keys())
            .map(|k| k.to_string())
            .collect()
    }
}

impl FromStr for ColorMap {
    type Err = ColorcetError;

    /// find a colorcet colormap by name, add _r to reverse the colormap
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name = s.to_lowercase();
        let name = name.as_str();
        let (name0, reverse) = if let Some(name) = name.strip_suffix("_r") {
            (name, true)
        } else {
            (name, false)
        };

        if let Some(&alias) = colormaps::ALIASES.get(name0)
            && let Some(cmap) = colormaps::COLOR_MAPS.get(alias)
        {
            let mut cmap = cmap.to_vec();
            if reverse {
                cmap.reverse();
            }
            Ok(ColorMap(cmap))
        } else if let Ok(cmap) = GradientBuilder::new()
            .html_colors(&["#000000", name0])
            .build::<LinearGradient>()
        {
            let mut cmap: Vec<_> = cmap
                .colors(256)
                .into_iter()
                .map(|c| [c.r as f64, c.g as f64, c.b as f64])
                .collect();
            if reverse {
                cmap.reverse();
            }
            Ok(ColorMap(cmap))
        } else {
            Err(ColorcetError::ColormapNotFound(name.into()))
        }
    }
}

impl From<ColorMap> for Vec<Color> {
    fn from(value: ColorMap) -> Self {
        value
            .get_rgb_float::<f32>()
            .into_iter()
            .map(|c| Color::from((c[0], c[1], c[2])))
            .collect()
    }
}

impl From<ColorMap> for Vec<String> {
    /// convert ColorMap into a Vec of css color strings
    fn from(value: ColorMap) -> Self {
        value
            .get_rgb_float::<f32>()
            .into_iter()
            .map(|c| Color::from((c[0], c[1], c[2])).to_css_hex())
            .collect()
    }
}

macro_rules! impl_to_gradient {
    ($($t:ty $(,)?)*) => {
        $(
            impl TryFrom<ColorMap> for $t {
                type Error = GradientBuilderError;

                fn try_from(value: ColorMap) -> Result<Self, Self::Error> {
                    let colors: Vec<Color> = value.into();
                    GradientBuilder::new().colors(&colors).build()
                }
            }
        )*
    };
}

impl_to_gradient!(LinearGradient, CatmullRomGradient, BasisGradient);
