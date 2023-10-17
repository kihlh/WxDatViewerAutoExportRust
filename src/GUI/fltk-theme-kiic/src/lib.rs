#![doc = include_str!("../README.md")]
#![allow(clippy::needless_doctest_main)]

use fltk::{app, enums::Color};
pub mod color_themes;
pub mod colors;
pub mod widget_schemes;
pub mod widget_themes;

/// Color map struct. (index, r, g, b)
#[derive(Default, Clone, Debug)]
pub struct ColorMap {
    pub index: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[macro_export]
macro_rules! cmap {
    ($i:tt, $r:tt, $g:tt, $b:tt) => {
        ColorMap {
            index: $i,
            r: $r,
            g: $g,
            b: $b,
        }
    };
}

/// A theme is just a Vec of colormaps
#[derive(Debug, Clone)]
pub struct ColorTheme(pub Vec<ColorMap>);

impl ColorTheme {
    /// Load from a color map
    pub fn from_colormap(map: &[ColorMap]) -> ColorTheme {
        ColorTheme(map.to_vec())
    }

    /// Load from a color map
    pub fn new(map: &[ColorMap]) -> ColorTheme {
        ColorTheme(map.to_vec())
    }

    /// apply() the theme
    pub fn apply(&self) {
        for elem in &self.0 {
            app::set_color(Color::by_index(elem.index), elem.r, elem.g, elem.b);
        }
        app::redraw();
    }
}

pub(crate) fn activated_color(c: Color) -> Color {
    if fltk::app::draw_frame_active() {
        c
    } else {
        c.inactive()
    }
}

/// Lists supported themes
#[derive(Debug, Clone, Copy)]
pub enum ThemeType {
    /// Windows classic
    Classic,
    /// Windows 7
    Aero,
    /// Windows 8
    Metro,
    /// Classic MacOS
    AquaClassic,
    /// Xfce
    Greybird,
    /// Windows 2000
    Blue,
    /// Dark
    Dark,
    /// High Contrast
    HighContrast,
}

/// A widget theme is a scheme + a set of default colors
#[derive(Debug, Clone, Copy)]
pub struct WidgetTheme {
    theme: ThemeType,
}

impl WidgetTheme {
    /// Create a Widget theme object
    pub fn new(theme: ThemeType) -> Self {
        Self { theme }
    }

    /// Apply the widget theme
    pub fn apply(&self) {
        match self.theme {
            ThemeType::Classic => widget_themes::classic::use_classic_theme(),
            ThemeType::Aero => widget_themes::aero::use_aero_theme(),
            ThemeType::AquaClassic => widget_themes::aqua_classic::use_aqua_classic_theme(),
            ThemeType::Dark => widget_themes::dark::use_dark_theme(),
            ThemeType::HighContrast => widget_themes::high_contrast::use_high_contrast_theme(),
            ThemeType::Blue => widget_themes::blue::use_blue_theme(),
            ThemeType::Metro => widget_themes::metro::use_metro_theme(),
            ThemeType::Greybird => widget_themes::greybird::use_greybird_theme(),
        }
    }
}

/// Lists supported schemes
#[derive(Debug, Clone, Copy)]
pub enum SchemeType {
    /// A scheme mimicking modern Aqua
    Aqua,
    /// Taken from the NTK fork
    Clean,
    /// Taken from the NTK fork
    Crystal,
    /// Windows 10
    Fluent,
    /// Taken from the NTK fork, a modification of the FLTK Gleam scheme
    Gleam,
    /**
    Draws the following FrameTypes using scalable vector graphics:
    - RoundedFrame
    - RoundedBox
    - RFlatBox
    - OvalBox
    - OvalFrame
    - OFlatFrame
    */
    SvgBased,
}

/// A widget scheme sets the style of drawing a widget without interfering with coloring
#[derive(Debug, Clone, Copy)]
pub struct WidgetScheme {
    scheme: SchemeType,
}

impl WidgetScheme {
    /// Create a Widget theme object
    pub fn new(scheme: SchemeType) -> Self {
        Self { scheme }
    }

    /// Apply the widget theme
    pub fn apply(&self) {
        match self.scheme {
            SchemeType::Aqua => widget_schemes::aqua::use_aqua_scheme(),
            SchemeType::Clean => widget_schemes::clean::use_clean_scheme(),
            SchemeType::Crystal => widget_schemes::crystal::use_crystal_scheme(),
            SchemeType::Fluent => widget_schemes::fluent::use_fluent_scheme(),
            SchemeType::Gleam => widget_schemes::gleam::use_gleam_scheme(),
            SchemeType::SvgBased => widget_schemes::svg_based::use_svg_based_scheme(),
        }
    }
}
