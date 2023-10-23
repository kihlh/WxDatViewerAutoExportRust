#![allow(non_upper_case_globals)]

use fltk::enums::Color;
use fltk::utils::oncelock::Lazy;

pub static backgroundColor2: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((255, 255, 255, 255)));
pub static windowBackgroundColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((231, 231, 231, 255)));
pub static labelColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((0, 0, 0, 216)));
pub static controlBackgroundColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((255, 254, 254, 255)));
pub static secondaryLabelColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((0, 0, 0, 127)));
pub static tertiaryLabelColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((0, 0, 0, 66)));
pub static quaternaryLabelColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((0, 0, 0, 25)));
pub static textColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((0, 0, 0, 255)));
pub static placeholderTextColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((0, 0, 0, 63)));
pub static selectedTextColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((0, 0, 0, 255)));
pub static textBackgroundColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((255, 254, 254, 255)));
pub static selectedTextBackgroundColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((164, 204, 254, 255)));
pub static keyboardFocusIndicatorColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((7, 75, 240, 63)));
pub static unemphasizedSelectedTextColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((0, 0, 0, 255)));
pub static unemphasizedSelectedTextBackgroundColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((211, 211, 211, 255)));
pub static linkColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((8, 79, 209, 255)));
pub static separatorColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((0, 0, 0, 25)));
pub static selectedContentBackgroundColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((7, 73, 217, 255)));
pub static unemphasizedSelectedContentBackgroundColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((211, 211, 211, 255)));
pub static selectedMenuItemTextColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((255, 254, 254, 255)));
pub static gridColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((223, 223, 223, 255)));
pub static headerTextColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((0, 0, 0, 216)));
pub static controlAccentColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((10, 95, 254, 255)));
pub static controlColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((255, 254, 254, 255)));
pub static controlTextColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((0, 0, 0, 216)));
pub static disabledControlTextColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((0, 0, 0, 63)));
pub static selectedControlColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((164, 204, 254, 255)));
pub static selectedControlTextColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((0, 0, 0, 216)));
pub static alternateSelectedControlTextColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((255, 254, 254, 255)));
pub static scrubberTexturedBackgroundColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((255, 254, 254, 255)));
pub static windowFrameTextColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((0, 0, 0, 216)));
pub static underPageBackgroundColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((131, 131, 131, 229)));
pub static findHighlightColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((255, 255, 10, 255)));
pub static highlightColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((255, 254, 254, 255)));
pub static shadowColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((0, 0, 0, 255)));
pub static systemBrownColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((144, 113, 75, 255)));
pub static systemGrayColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((123, 123, 128, 255)));
pub static systemGreenColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((40, 199, 50, 255)));
pub static systemIndigoColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((69, 59, 204, 255)));
pub static systemOrangeColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((252, 129, 8, 255)));
pub static systemPinkColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((251, 12, 67, 255)));
pub static systemPurpleColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((157, 51, 213, 255)));
pub static systemRedColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((251, 32, 37, 255)));
pub static systemTealColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((71, 175, 235, 255)));
pub static systemYellowColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((253, 194, 9, 255)));
pub static systemBlueColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((10, 95, 254, 255)));
pub static systemCyanColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((85, 190, 240, 255)));
