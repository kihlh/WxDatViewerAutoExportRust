#![allow(warnings, unused)]

use fltk::enums::FrameType;
use fltk::{prelude::*, *};
use fltk_theme::{color_themes, ColorTheme, SchemeType, ThemeType, WidgetScheme, WidgetTheme};
use FrameType::*;

pub mod img;
pub mod text;
pub mod hotspot;
pub mod variable_tag_control;
pub(crate) mod message;
pub mod resize_debug;
pub(crate) mod border;
mod lib;

pub use hotspot::{*};
pub use text::{*};
pub use img::{*};
pub use message::{*};
pub use border::{*};

#[macro_export]
macro_rules! inject_fltk_theme {
    () => {
        use fltk_theme::{
            color_themes, ColorTheme, SchemeType, ThemeType, WidgetScheme, WidgetTheme,
        };

        // 设置主题
        let theme = ColorTheme::new(color_themes::BLACK_THEME);
        let widget_theme = WidgetTheme::new(ThemeType::HighContrast);
        widget_theme.apply();
        let widget_scheme = WidgetScheme::new(SchemeType::Aqua);
        widget_scheme.apply();
        theme.apply();

    };
}

#[macro_export]
macro_rules! set_item_id {
    ($win:expr,$id:expr) => {
        $win.set_xclass($id);
        $win.set_id($id);
    };
}

// 用来解决小概率窗口显示不完整
pub fn redraw_win(win: &fltk::window::DoubleWindow) {
    fltk::app::add_timeout3(0.05, {
        let mut win = win.clone();
        move |cb| {
            win.resize(win.x(), win.y(), win.w(), win.h() - 1);
            win.redraw();
            win.redraw_label();
            win.resize(win.x(), win.y(), win.w(), win.h() + 1);
        }
    });
}

