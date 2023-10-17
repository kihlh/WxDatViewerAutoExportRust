#![allow(unused_variables)]
#![allow(clippy::many_single_char_names)]

use crate::activated_color;
use fltk::{
    app,
    draw::*,
    enums::{Color, FrameType},
    misc::Tooltip,
};

pub(crate) mod aero;
pub(crate) mod aqua_classic;
pub(crate) mod blue;
pub(crate) mod classic;
pub(crate) mod dark;
pub(crate) mod greybird;
pub(crate) mod high_contrast;
pub(crate) mod metro;

pub const OS_BUTTON_UP_BOX: FrameType = FrameType::GtkUpBox;
pub const OS_CHECK_DOWN_BOX: FrameType = FrameType::GtkDownBox;
pub const OS_BUTTON_UP_FRAME: FrameType = FrameType::GtkUpFrame;
pub const OS_CHECK_DOWN_FRAME: FrameType = FrameType::GtkDownFrame;
pub const OS_PANEL_THIN_UP_BOX: FrameType = FrameType::GtkThinUpBox;
pub const OS_SPACER_THIN_DOWN_BOX: FrameType = FrameType::GtkThinDownBox;
pub const OS_PANEL_THIN_UP_FRAME: FrameType = FrameType::GtkThinUpFrame;
pub const OS_SPACER_THIN_DOWN_FRAME: FrameType = FrameType::GtkThinDownFrame;
pub const OS_RADIO_ROUND_DOWN_BOX: FrameType = FrameType::GtkRoundDownBox;
pub const OS_HOVERED_UP_BOX: FrameType = FrameType::PlasticUpBox;
pub const OS_DEPRESSED_DOWN_BOX: FrameType = FrameType::PlasticDownBox;
pub const OS_HOVERED_UP_FRAME: FrameType = FrameType::PlasticUpFrame;
pub const OS_DEPRESSED_DOWN_FRAME: FrameType = FrameType::PlasticDownFrame;
pub const OS_INPUT_THIN_DOWN_BOX: FrameType = FrameType::PlasticThinDownBox;
pub const OS_INPUT_THIN_DOWN_FRAME: FrameType = FrameType::PlasticRoundDownBox;
pub const OS_MINI_BUTTON_UP_BOX: FrameType = FrameType::GleamUpBox;
pub const OS_MINI_DEPRESSED_DOWN_BOX: FrameType = FrameType::GleamDownBox;
pub const OS_MINI_BUTTON_UP_FRAME: FrameType = FrameType::GleamUpFrame;
pub const OS_MINI_DEPRESSED_DOWN_FRAME: FrameType = FrameType::GleamDownFrame;
pub const OS_DEFAULT_BUTTON_UP_BOX: FrameType = FrameType::DiamondUpBox;
pub const OS_DEFAULT_HOVERED_UP_BOX: FrameType = FrameType::PlasticThinUpBox;
pub const OS_DEFAULT_DEPRESSED_DOWN_BOX: FrameType = FrameType::DiamondDownBox;
pub const OS_TOOLBAR_BUTTON_HOVER_BOX: FrameType = FrameType::GleamRoundUpBox;
pub const OS_TABS_BOX: FrameType = FrameType::EmbossedBox;
pub const OS_SWATCH_BOX: FrameType = FrameType::EngravedBox;
pub const OS_SWATCH_FRAME: FrameType = FrameType::EngravedFrame;
pub const OS_BG_BOX: FrameType = FrameType::FreeBoxType;

pub const OS_FONT_SIZE: i32 = if cfg!(target_os = "window") { 12 } else { 13 };

pub(crate) fn use_native_settings() {
    app::set_visible_focus(false);
    app::set_scrollbar_size(15);
    Tooltip::set_font_size(OS_FONT_SIZE);
    Tooltip::set_delay(0.5);
}

pub(crate) fn vertical_gradient(x1: i32, y1: i32, x2: i32, y2: i32, c1: Color, c2: Color) {
    let imax = y2 - y1;
    let d = if imax > 0 { imax } else { 1 };
    if app::draw_frame_active() {
        for i in 0..=imax {
            let w = 1.0 - i as f32 / d as f32;
            set_draw_color(Color::color_average(c1, c2, w));
            draw_xyline(x1, y1 + i, x2);
        }
    } else {
        for i in 0..=imax {
            let w = 1.0 - i as f32 / d as f32;
            set_draw_color(Color::inactive(&Color::color_average(c1, c2, w)));
            draw_xyline(x1, y1 + i, x2);
        }
    }
}

pub(crate) fn horizontal_gradient(x1: i32, y1: i32, x2: i32, y2: i32, c1: Color, c2: Color) {
    let imax = x2 - x1;
    let d = if imax > 0 { imax } else { 1 };
    if app::draw_frame_active() {
        for i in 0..=imax {
            let w = 1.0 - i as f32 / d as f32;
            set_draw_color(Color::color_average(c1, c2, w));
            draw_yxline(x1 + i, y1, y2);
        }
    } else {
        for i in 0..=imax {
            let w = 1.0 - i as f32 / d as f32;
            set_draw_color(Color::inactive(&Color::color_average(c1, c2, w)));
            draw_yxline(x1 + i, y1, y2);
        }
    }
}

pub(crate) fn devalued(c: Color, w: f32) -> Color {
    Color::color_average(Color::Black, c, w)
}
