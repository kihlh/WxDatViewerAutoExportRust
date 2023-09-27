#![allow(warnings, unused)]

use crate::{global_var, gui_hotspot, gui_imge, gui_text_control, handle_dat, libWxIkunPlus, util::{str_eq_str, Sleep}, wh_mod};
use chrono::Local;
use fltk::{
    app::{self, handle},
    button::{self, Button},
    dialog,
    draw::font,
    enums::{self, Color, Cursor, Event, Font, FrameType, LabelType},
    frame,
    frame::Frame,
    group::{self, Flex, Group},
    image,
    input::{self, InputType, IntInput},
    prelude::*,
    text::TextDisplay,
    tree,
    window::{self, DoubleWindow, Window},
};
use fltk_table::{SmartTable, TableOpts};
use fltk_theme::{color_themes, ColorTheme, SchemeType, ThemeType, WidgetScheme, WidgetTheme};
use rusqlite::Connection;
use std::sync::{mpsc, MutexGuard};

use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::ops::Deref;
use std::ptr::null;
use std::sync::{Arc, Condvar, Mutex};
use std::{
    fs,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    process,
    process::{Command, Stdio},
    thread,
};

use crate::gui_imge::ImgPreview;
use crate::libWxIkunPlus::closeWindow;
use crate::watching::insert_watch_path_token;
use fltk::draw::{height, width};
use fltk::image::PngImage;
use lazy_static::lazy_static;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::OnceLock;
use winapi::um::winnt::LPWSTR;
use winapi::um::winuser::{CloseWindow, SetActiveWindow};

fn setInterfaceBackgroundImage(appMainWin: &mut window::DoubleWindow) -> Frame {
    let background_image =
        // image::SvgImage::from_data(include_str!("./assets/select_user_base/main.svg"))
        image::PngImage::from_data(include_bytes!("./assets/select_user_base/detect/Clip_20230921_064332.png"))
            .expect("set main icon error");
    let mut frame = Frame::default().with_size(351, 0).center_of(appMainWin);
    frame.set_frame(FrameType::EngravedBox);
    frame.set_image(Some(background_image));
    return frame;
}


macro_rules! set_theme{
    () => {
            // 设置主题
    let theme = ColorTheme::new(color_themes::BLACK_THEME);
    let widget_theme = WidgetTheme::new(ThemeType::HighContrast);
    widget_theme.apply();
    let widget_scheme = WidgetScheme::new(SchemeType::Aqua);
    widget_scheme.apply();
    theme.apply();
    }
}
pub fn main_window() {

    if( global_var::get_bool("gui::open::gui_detect_config")){
        if let Some (mut wins) = app::widget_from_id("gui::DoubleWindow::gui_detect_config::main") as Option<DoubleWindow> {
            wins.show();
            wins.set_visible_focus();

            return;
        }
        return;
    }



    // if( global_var::get_bool("gui::open::gui_detect_config")){
    //     return;
    // }

    set_theme!();
    let mut win: DoubleWindow = fltk::window::DoubleWindow::new(0, 0, 450, 460, "用户配置检测").center_screen();
    win.set_color(Color::from_rgb(24, 24, 24));
    win.set_border(false);
    setInterfaceBackgroundImage(&mut win);
    win.set_id("gui::DoubleWindow::gui_detect_config::main");
    global_var::set_bool("gui::open::gui_detect_config",true);

    let mut main_window_back = ImgPreview::new(0, 0, 450, 453, "gui::ImgPreview::main_window::back");
    main_window_back.from_data(include_bytes!("./assets/select_user_base/detect/detect.png").to_vec(), 0,0,win.width(),win.height());

    let mut next_btn = gui_hotspot::create_hotspot(203, 570, 233, 72);
    let mut gui_text_form01_state =gui_text_control::TextControl::new(320,53,0,0,11,String::from("已经完成"),[96, 139, 153]);
    let mut gui_text_form02_state =gui_text_control::TextControl::new(320,168,0,0,11,String::from("已经完成"),[96, 139, 153]);
    let mut gui_text_form03_state =gui_text_control::TextControl::new(320, 285,0,0, 11, String::from("已经完成"),[96, 139, 153]);

    let mut gui_imag_from01_state = ImgPreview::new(43,58,50,50,"gui_imag_from01_state");
    let mut gui_imag_from02_state = ImgPreview::new(43,175,50,50,"gui_imag_from02_state");
    let mut gui_imag_from03_state = ImgPreview::new(43,296,50,50,"gui_imag_from03_state");
    gui_imag_from01_state.from_data(include_bytes!("./assets/select_user_base/detect/success.png").to_vec(),0,0,50,50);
    gui_imag_from02_state.from_data(include_bytes!("./assets/select_user_base/detect/success.png").to_vec(),0,0,50,50);
    gui_imag_from03_state.from_data(include_bytes!("./assets/select_user_base/detect/success.png").to_vec(),0,0,50,50);

    let mut gui_text_form01_title =gui_text_control::TextControl::new(111-60-10,55,0,0,12,String::from("选择WX存储位置"),[207, 207, 207]);
    let mut gui_text_form02_title =gui_text_control::TextControl::new(111-78-10,175,0,0,12,String::from("选择被保存的对象"),[207, 207, 207]);
    let mut gui_text_form03_title =gui_text_control::TextControl::new(111-65-10, 288,0,0, 12, String::from("选择存储的选项"),[207, 207, 207]);

    let mut gui_text_form01_cont =gui_text_control::TextControl::new(100,80,300,0,11,String::from("此路径在您的 WX 中的  设置  /  文件管理  / 文件管理"),[78, 78, 78]);
    let mut gui_text_form01_cont_2 =gui_text_control::TextControl::new(-8,96,300,0,11,String::from("找到此路径"),[78, 78, 78]);

    let mut gui_text_form02_cont =gui_text_control::TextControl::new(100-3,80+120,300,0,11,String::from("您需要选择需要同步的对象， 在选择最近对象      "),[78, 78, 78]);
    let mut gui_text_form02_cont_2 =gui_text_control::TextControl::new(100,96+120,300,0,11,String::from("如果不存在的话 您可以向找的人 随意发送一张图片"),[78, 78, 78]);

    let mut gui_text_form03_cont =gui_text_control::TextControl::new(110,80+120+110,300,0,11,   String::from("1.保存缩略  就是很小的图片 显示在聊天的 所有图片都有"),[78, 78, 78]);
    let mut gui_text_form03_cont_2 =gui_text_control::TextControl::new(110,96+120+110,300,0,11,   String::from("2.保存原图  当您打开了图片 就会下载大图 此图片为原图"),[78, 78, 78]);
    let mut gui_text_form03_cont_3 =gui_text_control::TextControl::new(110,96+16+120+110,300,0,11,String::from("3.保存本月  当此项打开 只会保存本月开始 之前将被忽略"),[78, 78, 78]);

    let mut gui_text_btn_name =gui_text_control::TextControl::new(173, 405+3+1, 103,22,13, String::from("朕知道了"),[121, 121, 121]);

    let mut next_btn = gui_hotspot::create_hotspot(140, 395, 162, 51);

    // global_var::set_bool("user::config::check_button_the_month",false);
    // global_var::set_bool("user::config::check_button_source",false);
    // global_var::set_bool("user::config::check_button_thumbnail",false);
    // global_var::set_str("user::config::input_select_dir","".to_string());
    // global_var::set_i32("user::config::select_user_thumbnail_obj",-1);
    macro_rules! update_gui_state {
                    () => {
               if(!global_var::get_bool("user::config::check_button_the_month")&&!global_var::get_bool("user::config::check_button_source")&&!global_var::get_bool("user::config::check_button_thumbnail")){
        gui_text_form03_state.set_label("尚未选择".to_string());
        gui_text_form03_state.set_color(215, 97, 97);
        gui_imag_from03_state.from_data(include_bytes!("./assets/select_user_base/detect/fail.png").to_vec(),0,0,50,50);
    }else{
        gui_text_form03_state.set_label("已经完成".to_string());
        gui_text_form03_state.set_color(96, 139, 153);
        gui_imag_from03_state.from_data(include_bytes!("./assets/select_user_base/detect/success.png").to_vec(),0,0,50,50);
    }

    if(global_var::get_i32("user::config::select_user_thumbnail_obj")==-1){
        gui_text_form02_state.set_label("尚未选择".to_string());
        gui_text_form02_state.set_color(215, 97, 97);
        gui_imag_from02_state.from_data(include_bytes!("./assets/select_user_base/detect/fail.png").to_vec(),0,0,50,50);

    }else{
        gui_text_form02_state.set_label("已经完成".to_string());
        gui_text_form02_state.set_color(96, 139, 153);
        gui_imag_from02_state.from_data(include_bytes!("./assets/select_user_base/detect/success.png").to_vec(),0,0,50,50);
    }

    if(global_var::get_str("user::config::input_select_dir").is_empty()){
        gui_text_form01_state.set_label("尚未选择".to_string());
        gui_text_form01_state.set_color(215, 97, 97);
        gui_imag_from01_state.from_data(include_bytes!("./assets/select_user_base/detect/fail.png").to_vec(),0,0,50,50);
    }else{
        gui_text_form01_state.set_label("已经完成".to_string());
        gui_text_form01_state.set_color(96, 139, 153);
        gui_imag_from01_state.from_data(include_bytes!("./assets/select_user_base/detect/success.png").to_vec(),0,0,50,50);
    }
                    };
                }

    thread::spawn(move||{
        let mut is_open_win  = global_var::get_bool("gui::open::gui_detect_config");
        loop{
            if !is_open_win {

                return ;
            }
            Sleep(500);
            update_gui_state!();
        }
    });

    win.handle({
        let mut x = 0;
        let mut y = 0;
        let mut released = true;
        let mut dnd =true;
        let mut drag_path = std::path::PathBuf::new();

        // let mut move_
        move |win, ev| match ev {
            enums::Event::Show => {
                true
            }

            enums::Event::Push => {
                if next_btn.existPoint(x,y){
                    libWxIkunPlus::closeWindow(win.raw_handle() as i128, true);
                    global_var::set_bool("gui::open::gui_detect_config",false);
                }

                true
            }

            enums::Event::Move => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;

                if(next_btn.existPoint(x,y)){
                    win.set_cursor(Cursor::Hand);
                }else{
                    win.set_cursor(Cursor::Default);
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