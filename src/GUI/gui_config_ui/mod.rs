#![allow(warnings, unused)]

use crate::config::CONFIG_KEY;
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
    // gui_util::TextControl::new(262, 448, 75, 21, 16, "保存配置", [122, 120, 120]);

    gui_util::create_hotspot(24, 94, 543, 30);

    gui_util::create_hotspot(23, 148, 477, 34);

    gui_util::create_hotspot(23, 303, 428, 35);
    gui_util::create_hotspot(24, 351, 411, 29);

    let common_decline = 65 + 15;

    macro_rules! add_check {
        ($name:expr,$CONFIG_KEY:expr) => {{
            let mut button = button::CheckButton::default().with_label($name);
            button.set_checked(config::get_config_bool($CONFIG_KEY));
            button.set_callback(|win| {
                config::set_config($CONFIG_KEY, win.is_checked());
                config::store_config();
            });
            button
        }};
    }

    let mut flex = group::Flex::default()
        .with_size(490, 30)
        .row()
        .center_of_parent();

    flex.set_pos(25, 165-7 - common_decline);

    let mut scan_adding = add_check!("立即扫描(任务创建)", CONFIG_KEY::ScanAdding);
    add_check!("实时更新(任务创建)", CONFIG_KEY::ScanLogAdding);
    add_check!("授权联网(更新/检测哈希)", CONFIG_KEY::Networking);

    flex.end();

    let mut flex = group::Flex::default()
        .with_size(490, 30)
        .row()
        .center_of_parent();

    flex.set_pos(25, 220-15 - common_decline);

    add_check!("任务连续创建", CONFIG_KEY::CreateCont);
    add_check!("保留任务配置", CONFIG_KEY::PreserveConfig);
    add_check!("保留对象列表(缓冲)", CONFIG_KEY::PreserveList);

    flex.end();

    let mut flex = group::Flex::default()
        .with_size(490, 30)
        .row()
        .center_of_parent();

    flex.set_pos(25, 220+23 - common_decline);
    let mut global_scan = add_check!("全局扫描", CONFIG_KEY::GlobalScan);
    add_check!("立即预览(选定对象)", CONFIG_KEY::QuickPreview);
    add_check!("隐藏入口( 设置/同步/关于 )", CONFIG_KEY::ScanAdding);
    global_scan.set_checked(config::get_config_bool(config::CONFIG_KEY::AutoAction));
    global_scan.set_callback({
        let mut scan_adding = scan_adding.clone();
        
        move |win|{
            if !win.is_checked() {
                scan_adding.set_checked(false);
                scan_adding.deactivate();
            }else{
                scan_adding.activate();
            }
            config::set_config(config::CONFIG_KEY::AutoAction, win.is_checked());
            config::store_config();

        }
    });
    flex.end();

    let mut flex = group::Flex::default()
        .with_size(490, 30)
        .row()
        .center_of_parent();

    flex.set_pos(25, 375 - common_decline);

    add_check!("日志输出", CONFIG_KEY::ConsoleLog).deactivate();

    if config::APP_ENABLE_DEVELOPER {
        add_check!("开发者模式", CONFIG_KEY::Developer);
    } else {
        add_check!("开发者模式", CONFIG_KEY::Developer).deactivate();
    }

    let mut show_mask = add_check!("数据消敏", CONFIG_KEY::ShowMask);

    flex.end();

    let mut flex = group::Flex::default()
        .with_size(510, 30)
        .row()
        .center_of_parent();

    flex.set_pos(25, 430 - 5 - common_decline);

    // button::CheckButton::default().with_label("演示模式");
    // button::CheckButton::default().with_label("日志输出到文件");
    let mut dome_preview = button::CheckButton::default().with_label("演示模式");
    dome_preview.set_checked(config::is_show_dome());

    dome_preview.set_callback({
        let mut show_mask = show_mask.clone();
        move |win| {
            if win.is_checked() {
                show_mask.deactivate();
                show_mask.set_checked(true);
            } else {
                show_mask.activate();
            }
            config::set_config(CONFIG_KEY::DomePreview, win.is_checked());
            config::store_config();
        }
    });

    add_check!("不拦重复启动", CONFIG_KEY::LogOutputFile).deactivate();
    add_check!("日志文件输出", CONFIG_KEY::LogOutputFile).deactivate();
    flex.end();

    UiControl { btn_close }
}

pub fn main_init() -> Option<fltk::window::DoubleWindow> {
    main_init_check!();
    let hwnd: i128 = 0;
    let mut win: DoubleWindow =
        fltk::window::DoubleWindow::new(0, 0, 600, 430, "WxAutoExIm 配置设置").center_screen();
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
