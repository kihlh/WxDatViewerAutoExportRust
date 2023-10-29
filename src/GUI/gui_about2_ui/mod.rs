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

pub(crate) const THE_WIN_CLASS_NAME: &str = "wx_auto_ex_im::gui_util::main::about_app<242363>";

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

fn win_about() -> DoubleWindow {
    let mut win = fltk::window::Window::new(1, 1, 600, 375, "");
    win.set_color(Color::from_rgb(227, 237, 249));



    gui_util::ImgPreview::new(0, 0, 600, 395, "").from_data(include_bytes!("./src/page/main.png").to_vec(),0,0,600,450);
    gui_util::ImgPreview::new_border(46,238,510,96, include_str!("./src/dev.svg"));
    
    gui_util::TextControl::new(439,133,135,21,13,"到永远无赞赏关于",[148, 180, 209])
    .add_cursor_hand_callback(&win, {
        move |win, frame| {
           fs::write("./WxAutoExIm_not_about", "Hello WxAutoExIm");
           sub_message_auto(get_the_hwnd!(),gui_util::IconType::Success, "执行成功  关闭页面后将定向新关于",3500u64);
        }
    });
    
    win.end();
    win
}

fn win_zhifubao() -> DoubleWindow {
    let mut win = fltk::window::Window::new(1, 1, 600, 375, "");
    win.set_color(Color::from_rgb(227, 237, 249));

    gui_util::ImgPreview::new(0, 0, 600, 395, "").from_data(include_bytes!("./src/page/zhifubao.png").to_vec(),0,0,600,450);
    
    win.end();
    win
}

fn win_wechat() -> DoubleWindow {
    let mut win = fltk::window::Window::new(1, 1, 600, 375, "");
    win.set_color(Color::from_rgb(227, 237, 249));

    gui_util::ImgPreview::new(0, 0, 600, 395, "").from_data(include_bytes!("./src/page/wechat.png").to_vec(),0,0,600,450);
    
    win.end();
    win
}

fn win_update() -> DoubleWindow {
    let mut win = fltk::window::Window::new(1, 1, 600, 375, "");
    win.set_color(Color::from_rgb(227, 237, 249));

    gui_util::ImgPreview::new(0, 0, 600, 395, "").from_data(include_bytes!("./src/page/update.png").to_vec(),0,0,600,450);
    
    let mut location = gui_util::border::BorderPreview::new(110,48,5,5,90,(239, 126, 126, 1.0),(204, 201, 228, 1.0),0);
    if config::is_build_52pojie(){
        location.preview.set_pos(181,217);
    }
    else if config::is_developer(){
        location.preview.set_pos(385,217);
    }
    else {
        location.preview.set_pos(291,217);
    }
    
    let version_info = gui_main_ui::lib::get_app_version_info();
    let version = (version_info["package"]["version"]).as_str().unwrap();
    let build_time = (version_info["package"]["build_time"]).as_str().unwrap();
        gui_util::TextControl::new(245, 350, 83, 21, 13, "果核/莫理(教程)", [153, 184, 212])
        .add_cursor_hand_callback(&win, {
            move |win, frame| {
                if config::is_build_52pojie() {
                    sub_message_auto(
                        get_the_hwnd!(),
                        gui_util::IconType::Failure,
                        "因平台规则不受站外支持",
                        3500u64,
                    );
                    return;
                }
                 sub_message_auto(get_the_hwnd!(),gui_util::IconType::Success, "正在唤起 果核剥壳 网址",3500u64);
            util::open_link_in_browser("https://www.ghxi.com/wx20230927.html?from_app_name=WxAutoExIm");
            }
        });
    gui_util::TextControl::new(344, 350, 68, 21, 13, "52pojie", [153, 184, 212])
        .add_cursor_hand_callback(&win, {
            move |win, frame| {
           
                 sub_message_auto(get_the_hwnd!(),gui_util::IconType::Success, "正在唤起 52pojie 网址",3500u64);
            util::open_link_in_browser("https://52pojie.cn?from_app_name=WxAutoExIm");
            }
        });
    gui_util::TextControl::new(411, 351, 68, 21, 13, "所有引用", [153, 184, 212])
        .add_cursor_hand_callback(&win, {
            move |win, frame| {
                if config::is_build_52pojie() {
                    sub_message_auto(
                        get_the_hwnd!(),
                        gui_util::IconType::Failure,
                        "因平台规则不受站外支持",
                        3500u64,
                    );
                    return;
                }
                
            }
        });
    
    gui_util::TextControl::new(85,350,135,21,13,"MoAlyousef (fltk-rs)",[153, 184, 212],)
    .add_cursor_hand_callback(&win, {
        move |win, frame| {
            if config::is_build_52pojie() {
               sub_message_auto(get_the_hwnd!(),gui_util::IconType::Failure, "因平台规则不受站外支持",3500u64);
                return;
            }
            sub_message_auto(get_the_hwnd!(),gui_util::IconType::Success, "正在唤起 github 网址",3500u64);
            util::open_link_in_browser("https://github.com/fltk-rs/fltk-rs");
        }
    });

    // qq 群 复制
    gui_util::border::BorderPreview::new(475,247,52,25,6,(222, 230, 245, 1.0),(190, 180, 215, 1.0),2,)
    .add_cursor_hand_callback(&win, {
        move |win, frame| {
            if config::is_build_52pojie() {
               sub_message_auto(get_the_hwnd!(),gui_util::IconType::Failure, "因平台规则不受站外支持",3500u64);
                return;
            }
            util::clip_copy_str("649020539");
            sub_message_auto(get_the_hwnd!(),gui_util::IconType::Success, "已复制",3500u64);
        }
    });
    
    // 检测更新
    gui_util::border::BorderPreview::new(126,86,82,25,6,(222, 230, 245, 1.0),(190, 180, 215, 1.0),2,)
    .add_cursor_hand_callback(&win, {
        move |win, frame| {
            if config::is_build_52pojie() {
               sub_message_auto(get_the_hwnd!(),gui_util::IconType::Failure, "因平台规则不受站外支持",3500u64);
                return;
            }
            sub_message_auto(get_the_hwnd!(),gui_util::IconType::Success, "正在检测新版本中",3500u64);
            util::update_app(true);
        }
    });
    
    // 邮箱复制
    gui_util::border::BorderPreview::new(476,281,51,25,6,(222, 230, 245, 1.0),(190, 180, 215, 1.0),2,)
    .add_cursor_hand_callback(&win, {
        move |win, frame| {
            if config::is_build_52pojie() {
               sub_message_auto(get_the_hwnd!(),gui_util::IconType::Failure, "因平台规则不受站外支持",3500u64);
                return;
            }
            util::clip_copy_str("E5DM6@outlook.com");
            sub_message_auto(get_the_hwnd!(),gui_util::IconType::Success, "已复制",3500u64);
        }
    });
   
    // github 复制
    gui_util::border::BorderPreview::new(475,314,51,25,6,(222, 230, 245, 1.0),(190, 180, 215, 1.0),2,)
    .add_cursor_hand_callback(&win, {
        move |win, frame| {
            if config::is_build_52pojie() {
               sub_message_auto(get_the_hwnd!(),gui_util::IconType::Failure, "因平台规则不受站外支持",3500u64);
                return;
            }
                 util::clip_copy_str("https://github.com/kihlh/WxDatViewerAutoExportRust");
            sub_message_auto(get_the_hwnd!(),gui_util::IconType::Success, "已复制",3500u64);

        }
    });
    
    
    // 唤起github
    gui_util::border::BorderPreview::new(535,314,52,25,6,(222, 230, 245, 1.0),(190, 180, 215, 1.0),2,)
    .add_cursor_hand_callback(&win, {
        move |win, frame| {
            if config::is_build_52pojie() {
               sub_message_auto(get_the_hwnd!(),gui_util::IconType::Failure, "因平台规则不受站外支持",3500u64);
                return;
            }
            sub_message_auto(get_the_hwnd!(),gui_util::IconType::Success, "正在唤起 github 网址",3500u64);
            util::open_link_in_browser("https://github.com/kihlh/WxDatViewerAutoExportRust");
        }
    });
    
    // qq群唤起
    gui_util::border::BorderPreview::new( 535, 247,52,25,6,(222, 230, 245, 1.0),(190, 180, 215, 1.0),2,)
    .add_cursor_hand_callback(&win, {
        move |win, frame| {
            if config::is_build_52pojie() {
               sub_message_auto(get_the_hwnd!(),gui_util::IconType::Failure, "因平台规则不受站外支持",3500u64);
                return;
            }
            sub_message_auto(get_the_hwnd!(),gui_util::IconType::Success, "正在唤起 qq群 网址",3500u64);
            util::open_link_in_browser("http://qm.qq.com/cgi-bin/qm/qr?_wv=1027&k=1O3SxDJNYzosdMENt_uQ3tFRkGKpEfz7&authKey=SgwpdTjJU7JESwDTtQ3VkuMUt%2Bm8AJCKxBnieAcMH0oanft8%2Bcr53cC37pEnOzXR&noverify=0&group_code=649020539");
        }
    });

    // 所有引用
    gui_util::border::BorderPreview::new(491,347,100,25,6,(222, 230, 245, 1.0),(190, 180, 215, 1.0),2,)
    .add_cursor_hand_callback(&win, {
        move |win, frame| {
            if config::is_build_52pojie() {
                sub_message_auto(get_the_hwnd!(),gui_util::IconType::Failure, "因平台规则不受站外支持",3500u64);
                return;
            }
            sub_message_auto(get_the_hwnd!(),gui_util::IconType::Success, "正在开发中",3500u64);

        }
    });

    gui_util::TextControl::new(209,89,172,19,15,format!("{} ({}-{})",version,APP_VERSION,build_time),[153, 184, 212]).add_cursor_hand(&win); 
    gui_util::TextControl::new(389,90,56,19,15,if config::is_build_52pojie(){"(不受支持)"}else{"(开源通道)"},[153, 184, 212]);//.add_cursor_hand(&win); 

    gui_util::TextPreview::new(81,248,117,16,13,if config::is_build_52pojie(){"特供版不提供"}else{"649020539"},[153, 184, 212]).set_back_color(227, 237, 249);
    gui_util::TextPreview::new(79,283,149,18,13,if config::is_build_52pojie(){"特供版不提供支持"}else{"E5DM6@outlook.com"},[153, 184, 212]).set_back_color(227, 237, 249);

    gui_util::TextPreview::new(103,316,340,21,13,if config::is_build_52pojie(){"特供版不提供支持"}else{"https://github.com/kihlh/WxDatViewerAutoExportRust"},[153, 184, 212]).set_back_color(227, 237, 249);
    gui_util::TextControl::new(523,252,78,15,13,"唤起",[153, 184, 212]).add_cursor_hand(&win); ; 
    gui_util::TextControl::new(523,320,78,15,13,"唤起",[153, 184, 212]).add_cursor_hand(&win); ; 
    gui_util::TextControl::new(462,320,78,15,13,"复制",[153, 184, 212]).add_cursor_hand(&win); ; 
    gui_util::TextControl::new(462,287,78,15,13,"复制",[153, 184, 212]).add_cursor_hand(&win); ; 
    gui_util::TextControl::new(462,252,78,15,13,"复制",[153, 184, 212]).add_cursor_hand(&win); ; 

    gui_util::TextControl::new(425,345,234,29,12,"查看全部 (All)",[153, 184, 212]).add_cursor_hand(&win); 
    gui_util::TextControl::new(128,86,79,23,12,if config::is_build_52pojie(){"更新已移除"}else{"检测更新"},[153, 184, 212]).add_cursor_hand(&win); 

    win.end();
    win
}

struct BottomNav{
        win:DoubleWindow,
        hotspot_about:gui_util::HotspotItmeControl,
        hotspot_wechat:gui_util::HotspotItmeControl,
        hotspot_zhifubao:gui_util::HotspotItmeControl,
        hotspot_update:gui_util::HotspotItmeControl,
        location: gui_util::BorderPreview,
}

fn win_bottom_nav() -> BottomNav {
    let mut win = fltk::window::Window::new(0, 375, 600, 75, "");
    gui_util::ImgPreview::new_border(23,9,550,56, include_str!("./src/bottom_nav.svg"));
    
    let mut location = gui_util::border::BorderPreview::new(110,48,5,5,90,(204, 201, 228, 1.0),(204, 201, 228, 1.0),0);
    
    gui_util::TextControl::new(72,17,80,39,13,"欢迎页面",[138, 113, 177]);

    gui_util::TextControl::new(213,17,80,39,13,"支付宝赞赏",[138, 113, 177]);

    gui_util::TextControl::new(341,17,92,39,13,"微信赞赏码",[138, 113, 177]);

    gui_util::TextControl::new(477,17,80,39,13,"反馈/更新",[138, 113, 177]);

    let mut hotspot_about = gui_util::create_hotspot(43,12,126,49);
    let mut hotspot_wechat = gui_util::create_hotspot(310,12,120,49);
    let mut hotspot_zhifubao = gui_util::create_hotspot(180,12,120,49);
    let mut hotspot_update = gui_util::create_hotspot(448,12,108,49);


    win.set_color(Color::from_rgb(227, 237, 249));
    win.end();
    
    BottomNav{
        win,
        hotspot_about,
        hotspot_wechat,
        hotspot_zhifubao,
        hotspot_update,
        location
    }
}

pub fn main_init() -> Option<fltk::window::DoubleWindow> {
    main_init_check!();
    let mut win: DoubleWindow = fltk::window::DoubleWindow::new(0, 0, 600, 450, "关于WxAutoExIm").center_screen();
    win.set_color(Color::from_rgb(227, 237, 249));
    set_item_id!(win,THE_WIN_CLASS_NAME);
    let mut win_bottom_nav = win_bottom_nav();
    let mut win_about = win_about();
    let mut win_update = win_update();
    let mut win_wechat = win_wechat();
    let mut win_zhifubao = win_zhifubao();

    win_bottom_nav.win.handle({
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
                macro_rules! show_from_window{
                  ($win:expr,$pos:expr)=>{
                    win_about.hide();
                    win_update.hide();
                    win_wechat.hide();
                    win_zhifubao.hide();
                    $win.show();
                    win_bottom_nav.location.preview.hide();
                    win_bottom_nav.location.preview.set_pos($pos.0, $pos.1);
                    win_bottom_nav.location.preview.show();
                 }
                }

                if win_bottom_nav.hotspot_about.existPoint(x, y) {
                   show_from_window!(win_about,(110,49));
                }
                
                if win_bottom_nav.hotspot_zhifubao.existPoint(x, y) {
                   show_from_window!(win_zhifubao,(240,49));
                }

                if win_bottom_nav.hotspot_wechat.existPoint(x, y) {
                   show_from_window!(win_wechat,(372,49));
                }
                 
                if win_bottom_nav.hotspot_update.existPoint(x, y) {
                   show_from_window!(win_update,(510,49));
                }

                true},

            enums::Event::Move => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;

                if 
                win_bottom_nav.hotspot_about.existPoint(x, y) ||
                win_bottom_nav.hotspot_zhifubao.existPoint(x, y)||
                win_bottom_nav.hotspot_wechat.existPoint(x, y)||
                win_bottom_nav.hotspot_update.existPoint(x, y)
                {
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
    win_bottom_nav.win.show();

    gui_util::redraw_win(&win);
    
    libWxIkunPlus::setWinIcon(get_the_hwnd!());

    Some(win)
}
