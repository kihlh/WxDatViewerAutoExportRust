#![allow(non_upper_case_globals)]

use fltk::enums::Color;
use fltk::utils::oncelock::Lazy;

pub static backgroundColor2: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((0, 0, 0, 255)));
pub static windowBackgroundColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((37, 37, 37, 255)));
pub static labelColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((255, 254, 254, 216)));
pub static controlBackgroundColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((22, 22, 22, 255)));
pub static secondaryLabelColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((255, 254, 254, 140)));
pub static tertiaryLabelColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((255, 254, 254, 63)));
pub static quaternaryLabelColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((255, 254, 254, 25)));
pub static textColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((255, 254, 254, 255)));
pub static placeholderTextColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((255, 254, 254, 63)));
pub static selectedTextColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((255, 255, 255, 255)));
pub static textBackgroundColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((22, 22, 22, 255)));
pub static selectedTextBackgroundColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((48, 79, 120, 255)));
pub static keyboardFocusIndicatorColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((27, 149, 254, 76)));
pub static unemphasizedSelectedTextColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((255, 255, 255, 255)));
pub static unemphasizedSelectedTextBackgroundColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((54, 54, 54, 255)));
pub static linkColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((52, 134, 254, 255)));
pub static separatorColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((255, 254, 254, 25)));
pub static selectedContentBackgroundColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((5, 63, 197, 255)));
pub static unemphasizedSelectedContentBackgroundColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((54, 54, 54, 255)));
pub static selectedMenuItemTextColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((255, 254, 254, 255)));
pub static gridColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((20, 20, 20, 255)));
pub static headerTextColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((255, 254, 254, 255)));
pub static controlAccentColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((10, 95, 254, 255)));
pub static controlColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((255, 254, 254, 63)));
pub static controlTextColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((255, 254, 254, 216)));
pub static disabledControlTextColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((255, 254, 254, 63)));
pub static selectedControlColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((48, 79, 120, 255)));
pub static selectedControlTextColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((255, 254, 254, 216)));
pub static alternateSelectedControlTextColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((255, 254, 254, 255)));
pub static scrubberTexturedBackgroundColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((255, 254, 254, 255)));
pub static windowFrameTextColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((255, 254, 254, 216)));
pub static underPageBackgroundColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((29, 29, 29, 255)));
pub static findHighlightColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((255, 255, 10, 255)));
pub static highlightColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((164, 164, 164, 255)));
pub static shadowColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((0, 0, 0, 255)));
pub static systemBrownColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((155, 123, 85, 255)));
pub static systemGrayColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((133, 133, 139, 255)));
pub static systemGreenColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((48, 211, 58, 255)));
pub static systemIndigoColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((74, 64, 223, 255)));
pub static systemOrangeColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((252, 141, 13, 255)));
pub static systemPinkColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((251, 25, 76, 255)));
pub static systemPurpleColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((175, 56, 238, 255)));
pub static systemRedColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((251, 43, 44, 255)));
pub static systemTealColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((76, 187, 242, 255)));
pub static systemYellowColor: Lazy<Color> =
    Lazy::new(|| Color::from_rgba_tuple((254, 207, 14, 255)));
pub static systemBlueColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((16, 106, 254, 255)));
pub static systemCyanColor: Lazy<Color> = Lazy::new(|| Color::from_rgba_tuple((90, 200, 245, 255)));
