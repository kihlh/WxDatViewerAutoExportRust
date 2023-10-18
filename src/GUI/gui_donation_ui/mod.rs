#![allow(warnings, unused)]

use crate::gui_util::*;
use crate::libWxIkunPlus;
use crate::{get_bool, global_var, gui_util, inject_fltk_theme, set_bool, set_item_id, wh_mod};
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
// use crate::{*};
use clipboard;

pub(crate) const THE_WIN_CLASS_NAME: &str = "wx_auto_ex_im::gui_util::main::donation<65263>";

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
                return;
            }
        }
    };
}

struct UiControl {}

fn add_ui_control() -> UiControl {
    UiControl {}
}

pub fn main_init() {
    main_init_check!();

    let mut win: DoubleWindow =
        fltk::window::DoubleWindow::new(0, 0, 660, 495, "通过捐赠支持").center_screen();
    inject_fltk_theme!();
    win.set_color(Color::from_rgb(227, 237, 249));

    set_item_id!(win, THE_WIN_CLASS_NAME);
    let mut hello =
        gui_util::img::ImgPreview::new2(0, 0, win.w(), win.h(), "", 0, 0, win.w(), win.h());
    hello.re_data(include_bytes!("./src/hello.png").to_vec());
    hello.preview.show();
    let mut hello_hotspot = gui_util::HotspotItmeControl::new(35, 425, 135, 50);

    let mut zhifubao =
        gui_util::img::ImgPreview::new2(0, 0, win.w(), win.h(), "", 0, 0, win.w(), win.h());
    zhifubao.re_data(include_bytes!("./src/zhifubao.png").to_vec());
    zhifubao.preview.hide();
    let mut zhifubao_hotspot = gui_util::HotspotItmeControl::new(190, 425, 135, 50);

    let mut wechat =
        gui_util::img::ImgPreview::new2(0, 0, win.w(), win.h(), "", 0, 0, win.w(), win.h());
    wechat.re_data(include_bytes!("./src/wechat.png").to_vec());
    wechat.preview.hide();
    let mut wechat_hotspot = gui_util::HotspotItmeControl::new(330, 425, 135, 50);

    let mut token =
        gui_util::img::ImgPreview::new2(0, 0, win.w(), win.h(), "", 0, 0, win.w(), win.h());
    token.re_data(include_bytes!("./src/utsd.png").to_vec());
    token.preview.hide();
    let mut token_hotspot = gui_util::HotspotItmeControl::new(490, 425, 135, 50);
    let mut token_copy_text =
        gui_util::TextControl::new(560 + 8, 325 - 8, 25, 15, 18, "复制", [132, 104, 172]);
    token_copy_text.text.hide();

    let mut win_control = add_ui_control();
    let mut close_hotspot = gui_util::HotspotItmeControl::new(590, 19, 36, 36);
    let mut token_copy_hotspot = gui_util::HotspotItmeControl::new(535, 300, 95, 50);

    win.handle({
        let mut x = 0;
        let mut y = 0;
        move |win, ev| match ev {
            enums::Event::Show => {
                libWxIkunPlus::setWinIcon(get_the_hwnd!());

                true
            }
            enums::Event::Close => {
                // gc_the_window!(win);
                false
            }
            enums::Event::Hide => {
                // gc_the_window!(win);
                false
            }
            enums::Event::Push => {
                if hello_hotspot.existPoint(x, y) {
                    hello.preview.show();
                    zhifubao.preview.hide();
                    token.preview.hide();
                    wechat.preview.hide();
                    token_copy_text.text.hide();
                }

                if zhifubao_hotspot.existPoint(x, y) {
                    hello.preview.hide();
                    zhifubao.preview.show();
                    token.preview.hide();
                    wechat.preview.hide();
                    token_copy_text.text.hide();
                }

                if token_hotspot.existPoint(x, y) {
                    if !wh_mod::config::is_show_token_donate() {
                        gui_util::sub_message_auto(
                            get_the_hwnd!(),
                            gui_util::message::IconType::Warning,
                            "当前版本/下载渠道不支持此方式",
                            3500u64,
                        );
                        return false;
                    }

                    hello.preview.hide();
                    zhifubao.preview.hide();
                    token.preview.show();
                    wechat.preview.hide();
                    token_copy_text.text.show();
                }

                if wechat_hotspot.existPoint(x, y) {
                    hello.preview.hide();
                    zhifubao.preview.hide();
                    token.preview.hide();
                    wechat.preview.show();
                    token_copy_text.text.hide();
                }

                if close_hotspot.existPoint(x, y) {
                    fltk::window::Window::delete(win.clone());
                }

                if token_copy_hotspot.existPoint(x, y) && token.preview.visible() {
                    use clipboard::ClipboardContext;
                    use clipboard::ClipboardProvider;
                    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                    
                    ctx.set_contents(String::from("TBd9BX4WmWJEtzVABxr1BJnWxhLcVUxXBW").to_owned()).unwrap();

                    gui_util::sub_message_auto(
                        get_the_hwnd!(),
                        gui_util::message::IconType::Success,
                        "已复制到剪贴板中",
                        3500u64,
                    );
                }

                true
            }
            enums::Event::Move => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;

                if hello_hotspot.existPoint(x, y)
                    || zhifubao_hotspot.existPoint(x, y)
                    || token_hotspot.existPoint(x, y)
                    || wechat_hotspot.existPoint(x, y)
                    || close_hotspot.existPoint(x, y)
                    || (token_copy_hotspot.existPoint(x, y) && token.preview.visible())
                {
                    win.set_cursor(fltk::enums::Cursor::Hand);
                } else {
                    win.set_cursor(fltk::enums::Cursor::Default);
                }

                true
            }
            enums::Event::Drag => {
                if y < 69 {
                    win.clone()
                        .set_pos(app::event_x_root() - x, app::event_y_root() - y);
                }

                true
            }
            _ => false,
        }
    });

    win.end();
    win.show();
}
