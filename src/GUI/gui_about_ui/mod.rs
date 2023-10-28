#![allow(warnings, unused)]

use crate::config::CONFIG_KEY;
use crate::gui_util::*;
use crate::libWxIkunPlus;
use crate::*;
use crate::{get_bool, global_var, gui_util, inject_fltk_theme, set_bool, set_item_id};
use clipboard::ClipboardProvider;
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

pub(crate) const THE_WIN_CLASS_NAME: &str = "wx_auto_ex_im::gui_util::main::about<262163>";
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

pub fn main_init() -> Option<fltk::window::DoubleWindow> {
    main_init_check!();
    if config::is_build_52pojie() {
        libWxIkunPlus::error("软件接口被调试", "当前发布版本的关于页面不可用，请勿使用调试手段调用接口");
        process::exit(0);
    }
    let hwnd: i128 = 0;
    let mut win: DoubleWindow =
        fltk::window::DoubleWindow::new(0, 0, 600, 450, "关于WxAutoExIm").center_screen();
    inject_fltk_theme!();
    win.set_color(Color::from_rgb(227, 237, 249));
    set_item_id!(win, THE_WIN_CLASS_NAME);
    gui_util::img::ImgPreview::new_border(0, 0, win.w(), win.h(), THE_WIN_UI_BORDER);
    // let mut win_control = add_ui_control();

    let mut gui_about_txt = gui_util::TextPreview::new(291,370,124,27,19, "649020539", [153, 184, 212]);
    gui_about_txt.resize_debug();
    gui_about_txt.preview.set_color(fltk::enums::Color::from_rgb(227, 237, 249));


    let mut gui_about_txt_mail = gui_util::TextPreview::new(101,400,170,21,15,"E5DM6@outlook.com",[153, 184, 212]); 
    gui_about_txt_mail.preview.set_color(fltk::enums::Color::from_rgb(227, 237, 249));

    let mut gui_about_github = gui_util::TextPreview::new(345,402,210,24,13, "kihlh/WxDatViewerAutoExportRust", [153, 184, 212]);
    // gui_about_github.resize_debug();
    gui_about_github.preview.set_color(fltk::enums::Color::from_rgb(227, 237, 249));
    
    // 检测更新按钮
    if config::get_config_bool(config::CONFIG_KEY::Networking) {
        let mut gui_about_update = gui_util::border::BorderPreview::new(426,370,92,25,10,(227, 237, 249, 1.0),(192, 180, 212, 1.0),2).resize_debug();
        let mut gui_about_update_str = gui_util::TextControl::new(428,371,89,23,11,"检测更新",[192, 180, 212]);
    
        gui_about_update_str.add_cursor_hand(&win);
        gui_about_update.add_cursor_hand_callback(&win,{move|win,frame|{
            gui_util::sub_message_auto(get_the_hwnd!(), gui_util::IconType::Success, "当前已是最新版本", 3500u64);
        }});
    }


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
                if gui_about_github.existPoint(x, y) {
                    let mut ctx = clipboard::ClipboardContext::new().unwrap();
                    let the_string = "https://github.com/kihlh/WxDatViewerAutoExportRust";
                    ctx.set_contents(the_string.to_owned()).unwrap();
                    gui_util::sub_message_auto(get_the_hwnd!(), gui_util::IconType::Success, "链接已复制", 3500u64);
                }
                
                if gui_about_txt.existPoint(x, y) {
                    let mut ctx = clipboard::ClipboardContext::new().unwrap();
                    let the_string = "649020539";
                    ctx.set_contents(the_string.to_owned()).unwrap();
                    gui_util::sub_message_auto(get_the_hwnd!(), gui_util::IconType::Success, "QQ群号已复制", 3500u64);
                }
                
                if gui_about_txt_mail.existPoint(x, y) {
                    let mut ctx = clipboard::ClipboardContext::new().unwrap();
                    let the_string = "E5DM6@outlook.com";
                    ctx.set_contents(the_string.to_owned()).unwrap();
                    gui_util::sub_message_auto(get_the_hwnd!(), gui_util::IconType::Success, "反馈邮箱已复制", 3500u64);
                }

                true
            }

            enums::Event::Move => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;

                true
            }

            _ => false,
        }
    });

    win.end();
    win.show();
    gui_util::redraw_win(&win);

    Some(win)
}
