#![allow(unused_imports)]
#![allow(non_upper_case_globals)]

use cocoa_colors::*;
use fltk::utils::oncelock::Lazy;

fn convert_colors(colors: (f64, f64, f64, f64)) -> (u8, u8, u8, u8) {
    let r = (colors.0 * 255.0) as u8;
    let g = (colors.1 * 255.0) as u8;
    let b = (colors.2 * 255.0) as u8;
    let a = (colors.3 * 255.0) as u8;
    (r, g, b, a)
}

macro_rules! get_colors {
    ($s:ident) => {{
        let mut r = 1.0;
        let mut g = 1.0;
        let mut b = 1.0;
        let mut a = 1.0;
        unsafe {
            $s(&mut r, &mut g, &mut b, &mut a);
        }
        convert_colors((r, g, b, a))
    }};
}

pub static windowBackgroundColor: Lazy<(u8, u8, u8, u8)> =
    Lazy::new(|| get_colors!(get_windowBackgroundColor));
pub static labelColor: Lazy<(u8, u8, u8, u8)> = Lazy::new(|| get_colors!(get_labelColor));
pub static controlBackgroundColor: Lazy<(u8, u8, u8, u8)> =
    Lazy::new(|| get_colors!(get_controlBackgroundColor));
pub static secondaryLabelColor: Lazy<(u8, u8, u8, u8)> =
    Lazy::new(|| get_colors!(get_secondaryLabelColor));
pub static tertiaryLabelColor: Lazy<(u8, u8, u8, u8)> =
    Lazy::new(|| get_colors!(get_tertiaryLabelColor));
pub static quaternaryLabelColor: Lazy<(u8, u8, u8, u8)> =
    Lazy::new(|| get_colors!(get_quaternaryLabelColor));
pub static textColor: Lazy<(u8, u8, u8, u8)> = Lazy::new(|| get_colors!(get_textColor));
pub static placeholderTextColor: Lazy<(u8, u8, u8, u8)> =
    Lazy::new(|| get_colors!(get_placeholderTextColor));
pub static selectedTextColor: Lazy<(u8, u8, u8, u8)> =
    Lazy::new(|| get_colors!(get_selectedTextColor));
pub static textBackgroundColor: Lazy<(u8, u8, u8, u8)> =
    Lazy::new(|| get_colors!(get_textBackgroundColor));
pub static selectedTextBackgroundColor: Lazy<(u8, u8, u8, u8)> =
    Lazy::new(|| get_colors!(get_selectedTextBackgroundColor));
pub static keyboardFocusIndicatorColor: Lazy<(u8, u8, u8, u8)> =
    Lazy::new(|| get_colors!(get_keyboardFocusIndicatorColor));
pub static unemphasizedSelectedTextColor: Lazy<(u8, u8, u8, u8)> =
    Lazy::new(|| get_colors!(get_unemphasizedSelectedTextColor));
pub static unemphasizedSelectedTextBackgroundColor: Lazy<(u8, u8, u8, u8)> =
    Lazy::new(|| get_colors!(get_unemphasizedSelectedTextBackgroundColor));
pub static linkColor: Lazy<(u8, u8, u8, u8)> = Lazy::new(|| get_colors!(get_linkColor));
pub static separatorColor: Lazy<(u8, u8, u8, u8)> = Lazy::new(|| get_colors!(get_separatorColor));
pub static selectedContentBackgroundColor: Lazy<(u8, u8, u8, u8)> =
    Lazy::new(|| get_colors!(get_selectedContentBackgroundColor));
pub static unemphasizedSelectedContentBackgroundColor: Lazy<(u8, u8, u8, u8)> =
    Lazy::new(|| get_colors!(get_unemphasizedSelectedContentBackgroundColor));
pub static selectedMenuItemTextColor: Lazy<(u8, u8, u8, u8)> =
    Lazy::new(|| get_colors!(get_selectedMenuItemTextColor));
pub static gridColor: Lazy<(u8, u8, u8, u8)> = Lazy::new(|| get_colors!(get_gridColor));
pub static headerTextColor: Lazy<(u8, u8, u8, u8)> = Lazy::new(|| get_colors!(get_headerTextColor));
pub static controlAccentColor: Lazy<(u8, u8, u8, u8)> =
    Lazy::new(|| get_colors!(get_controlAccentColor));
pub static controlColor: Lazy<(u8, u8, u8, u8)> = Lazy::new(|| get_colors!(get_controlColor));
pub static controlTextColor: Lazy<(u8, u8, u8, u8)> =
    Lazy::new(|| get_colors!(get_controlTextColor));
pub static disabledControlTextColor: Lazy<(u8, u8, u8, u8)> =
    Lazy::new(|| get_colors!(get_disabledControlTextColor));
pub static selectedControlColor: Lazy<(u8, u8, u8, u8)> =
    Lazy::new(|| get_colors!(get_selectedControlColor));
pub static selectedControlTextColor: Lazy<(u8, u8, u8, u8)> =
    Lazy::new(|| get_colors!(get_selectedControlTextColor));
pub static alternateSelectedControlTextColor: Lazy<(u8, u8, u8, u8)> =
    Lazy::new(|| get_colors!(get_alternateSelectedControlTextColor));
pub static scrubberTexturedBackgroundColor: Lazy<(u8, u8, u8, u8)> =
    Lazy::new(|| get_colors!(get_scrubberTexturedBackgroundColor));
pub static windowFrameTextColor: Lazy<(u8, u8, u8, u8)> =
    Lazy::new(|| get_colors!(get_windowFrameTextColor));
pub static underPageBackgroundColor: Lazy<(u8, u8, u8, u8)> =
    Lazy::new(|| get_colors!(get_underPageBackgroundColor));
pub static findHighlightColor: Lazy<(u8, u8, u8, u8)> =
    Lazy::new(|| get_colors!(get_findHighlightColor));
pub static highlightColor: Lazy<(u8, u8, u8, u8)> = Lazy::new(|| get_colors!(get_highlightColor));
pub static shadowColor: Lazy<(u8, u8, u8, u8)> = Lazy::new(|| get_colors!(get_shadowColor));
pub static systemBrownColor: Lazy<(u8, u8, u8, u8)> =
    Lazy::new(|| get_colors!(get_systemBrownColor));
pub static systemGrayColor: Lazy<(u8, u8, u8, u8)> = Lazy::new(|| get_colors!(get_systemGrayColor));
pub static systemGreenColor: Lazy<(u8, u8, u8, u8)> =
    Lazy::new(|| get_colors!(get_systemGreenColor));
pub static systemIndigoColor: Lazy<(u8, u8, u8, u8)> =
    Lazy::new(|| get_colors!(get_systemIndigoColor));
pub static systemOrangeColor: Lazy<(u8, u8, u8, u8)> =
    Lazy::new(|| get_colors!(get_systemOrangeColor));
pub static systemPinkColor: Lazy<(u8, u8, u8, u8)> = Lazy::new(|| get_colors!(get_systemPinkColor));
pub static systemPurpleColor: Lazy<(u8, u8, u8, u8)> =
    Lazy::new(|| get_colors!(get_systemPurpleColor));
pub static systemRedColor: Lazy<(u8, u8, u8, u8)> = Lazy::new(|| get_colors!(get_systemRedColor));
pub static systemTealColor: Lazy<(u8, u8, u8, u8)> = Lazy::new(|| get_colors!(get_systemTealColor));
pub static systemYellowColor: Lazy<(u8, u8, u8, u8)> =
    Lazy::new(|| get_colors!(get_systemYellowColor));
pub static systemBlueColor: Lazy<(u8, u8, u8, u8)> = Lazy::new(|| get_colors!(get_systemBlueColor));
// pub static systemCyanColor: Lazy<(u8, u8, u8, u8)> = Lazy::new(|| get_colors!(get_systemCyanColor)); // beta
