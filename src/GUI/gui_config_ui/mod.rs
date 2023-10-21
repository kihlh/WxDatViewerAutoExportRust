#![allow(warnings, unused)]

use crate::gui_util::*;
use crate::libWxIkunPlus;
use crate::*;
use crate::{get_bool, global_var, gui_util, inject_fltk_theme, set_bool, set_item_id};
use fltk::enums::{Color, FrameType};
use fltk::examples::tile;
use fltk::window::DoubleWindow;
use fltk::{prelude::*, *};
use fltk_theme::{color_themes, ColorTheme, SchemeType, ThemeType, WidgetScheme, WidgetTheme};
use std::collections::{HashMap, HashSet};
use std::path;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicI64, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, MutexGuard, OnceLock, RwLock};
use std::thread::sleep;
mod lib;

pub(crate) const THE_WIN_CLASS_NAME: &str = "wx_auto_ex_im::gui_util::main::config<12156>";
pub(crate) const THE_WIN_UI_BORDER: &str = include_str!("./src/contour.svg");

macro_rules! gc_the_window {
    ($win:expr) => {
        fltk::window::Window::delete($win.clone());
    };
}

macro_rules! get_the_hwnd {
    ($class_id:expr) => {{
        let mut _hwnd = 0;
        for _ in 0..8 {
            _hwnd = libWxIkunPlus::findWindow($class_id, "");
            if !libWxIkunPlus::isWindow(_hwnd) {
                _hwnd = 0;
            } else {
                break;
            }
            fltk::app::sleep(0.020);
        }
        _hwnd as i128
    }};
    () => {
        get_the_hwnd!(THE_WIN_CLASS_NAME)
    };
}

macro_rules! main_init_check {
    () => {
        // 禁止创建多个窗口
        if let hwnd = get_the_hwnd!() {
            if hwnd != 0 && libWxIkunPlus::isWindow(hwnd) {
                if let Some(mut win) =
                    app::widget_from_id(THE_WIN_CLASS_NAME) as Option<DoubleWindow>
                {
                    win.show();
                    win.set_visible_focus();
                }
                libWxIkunPlus::setWindowShake(hwnd);
                return Option::None;
            }
        }
    };
}

struct UiControl {
    btn_close: HotspotItmeControl,
}

fn add_ui_control() -> UiControl {
    let btn_close = gui_util::hotspot::create_hotspot(556, 26, 25, 25);
    gui_util::TextControl::new(-10, 25, 130, 20, 15, "常规设置", [122, 120, 120]);
    gui_util::TextControl::new(0, 25 + 220, 130, 20, 15, "开发者配置", [122, 120, 120]);
    gui_util::TextControl::new(262, 448, 75, 21, 16, "保存配置", [122, 120, 120]);

    gui_util::create_hotspot(24,94,543,30);

    gui_util::create_hotspot(23,148,477,34);

    gui_util::create_hotspot(23,303,428,35);
    gui_util::create_hotspot(24,351,411,29);

    let common_decline = 65+15;

    let mut flex = group::Flex::default()
        .with_size(490-15, 50)
        .row()
        .center_of_parent();

    flex.set_pos(25, 165-common_decline);

    let mut check_button_sync = button::CheckButton::default().with_label("实时响应");
    let mut check_button_video = button::CheckButton::default().with_label("全局扫描");
    let mut check_button_video = button::CheckButton::default().with_label("添加后立即扫描");
    let mut check_button_thumbnail =
        button::CheckButton::default().with_label("授权联网(更新/检测哈希)");
    flex.end();

    let mut flex = group::Flex::default()
        .with_size(490, 50)
        .row()
        .center_of_parent();

    flex.set_pos(25, 220-common_decline);

    let mut check_button_thumbnail = button::CheckButton::default().with_label("配置不存储");
    let mut check_button_thumbnail = button::CheckButton::default().with_label("任务连续创建");
    let mut check_button_thumbnail = button::CheckButton::default().with_label("记录聊天对象列表");

    flex.end();

    let mut flex = group::Flex::default()
        .with_size(490, 50)
        .row()
        .center_of_parent();

    flex.set_pos(25, 375-common_decline);

    let mut check_button_thumbnail = button::CheckButton::default().with_label("日志输出");
    let mut check_button_thumbnail = button::CheckButton::default().with_label("开发者模式");
    let mut check_button_thumbnail = button::CheckButton::default().with_label("数据消敏");
    flex.end();

    let mut flex = group::Flex::default()
        .with_size(510, 50)
        .row()
        .center_of_parent();

    flex.set_pos(25, 430-10-common_decline);

    let mut check_button_thumbnail = button::CheckButton::default().with_label("演示模式");
    let mut check_button_thumbnail = button::CheckButton::default().with_label("日志输出到文件");
    flex.end();

    UiControl { btn_close }
}

pub fn main_init() -> Option<fltk::window::DoubleWindow> {
    main_init_check!();
    let hwnd: i128 = 0;
    let mut win: DoubleWindow =
        fltk::window::DoubleWindow::new(0, 0, 600, 500, "WxAutoExIm").center_screen();
    inject_fltk_theme!();
    win.set_color(Color::from_rgb(24, 24, 24));
    set_item_id!(win, THE_WIN_CLASS_NAME);
    gui_util::img::ImgPreview::new_border(0, 0, win.w(), win.h(), THE_WIN_UI_BORDER);
    let mut win_control = add_ui_control();

    win.handle({
        let mut x = 0;
        let mut y = 0;
        move |win, ev| match ev {
            enums::Event::Show => {
                libWxIkunPlus::setWinIcon(get_the_hwnd!());

                true
            }
            enums::Event::Close => {
                gc_the_window!(win);
                false
            }
            enums::Event::Hide => {
                gc_the_window!(win);
                fltk::window::Window::delete(win.clone());
                false
            }
            enums::Event::Push => {
                // 关闭
                if win_control.btn_close.existPoint(x, y) {
                    gc_the_window!(win);
                    libWxIkunPlus::setwinVisible(get_the_hwnd!(), false);
                }

                true
            }

            enums::Event::Move => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;

                if win_control.btn_close.existPoint(x, y) {
                    win.set_cursor(fltk::enums::Cursor::Hand);
                } else {
                    win.set_cursor(fltk::enums::Cursor::Default);
                }

                true
            }

            _ => false,
        }
    });

    win.end();
    win.show();

    Some(win)
}
