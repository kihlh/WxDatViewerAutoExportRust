#![allow(warnings, unused)]

use crate::{atomic_util, global_var, gui_hotspot, gui_imge, handle_dat, libWxIkunPlus, util::{str_eq_str, Sleep}, wh_mod};
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
use crate::wh_mod::get_walk_attach_file;
use fltk::draw::{height, width};
use fltk::image::PngImage;
use lazy_static::lazy_static;
use std::hint;
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicUsize, Ordering};
use std::sync::OnceLock;
use winapi::um::winnt::LPWSTR;
use winapi::um::winuser::{CloseWindow, SetActiveWindow};
// 是否正在执行
static WATCH_PUPPET_ING: AtomicBool = AtomicBool::new(false);
static INITIALIZED_PUPPET: AtomicBool = AtomicBool::new(false);

static WINDOW_STATE_AVAILABLE: AtomicBool = AtomicBool::new(false);
static WINDOW_HWND: AtomicI64 = AtomicI64::new(0);

fn setInterfaceBackgroundImage(appMainWin: &mut window::DoubleWindow) -> Frame {
    let background_image =
        // image::SvgImage::from_data(include_str!("./assets/select_user_base/main.svg"))
        image::PngImage::from_data(include_bytes!("./assets/select_user_base/drag_scan/scan_user.png"))
            .expect("set main icon error");
    let mut frame = Frame::default().with_size(351, 0).center_of(appMainWin);
    frame.set_frame(FrameType::EngravedBox);
    frame.set_image(Some(background_image));

    return frame;
}

macro_rules! next_gui {
    () => {
        let mut preview = ImgPreview::new(0, 0, 351, 367, "gui::preview_main::index::user_select");
        preview.from_data(
            include_bytes!("./assets/select_user_base/drag_scan/scan.png").to_vec(),
            0,
            0,
            351,
            367,
        );

        let mut text_recent_pictures = Frame::new(137, 152, 75, 20, "扫描图源用户");
        text_recent_pictures.set_label_size(12);

        let mut text_recent_pictures =
            Frame::new(63, 183, 222, 20, "你可以从聊天窗口拖拽一张图片到本窗口");
        text_recent_pictures.set_label_size(11);

        let mut text_recent_pictures = Frame::new(124, 215, 104, 20, "用时： 约万张/1秒");
        text_recent_pictures.set_label_size(11);
        text_recent_pictures.set_label_color(Color::from_rgb(51, 51, 51));
    };
}
macro_rules! set_theme {
    () => {
        // 设置主题
        let theme = ColorTheme::new(color_themes::BLACK_THEME);
        let widget_theme = WidgetTheme::new(ThemeType::HighContrast);
        widget_theme.apply();
        let widget_scheme = WidgetScheme::new(SchemeType::Aqua);
        widget_scheme.apply();
        theme.apply();
    };
}
fn initialize_watch_attach_puppet(img_path: String) {
    atomic_util::set_bool(&INITIALIZED_PUPPET,true);
    thread::spawn(move || {
        atomic_util::set_bool(&WATCH_PUPPET_ING,true);
        let file_name = Path::new(img_path.as_str())
            .file_name()
            .unwrap_or_else(|| "".as_ref());
        // let
        let split_name = file_name.to_string_lossy().to_string();
        let chars_mach = "0123456789qwertyuioplkjhgfdsazxcvbnmQWERTYUIOPLKJHGFDSAZXCVBNM";
        let mut id = String::new();
        for name in split_name.chars() {
            if (chars_mach.contains(name)) {
                id.push(name);
            } else {
                break;
            }
        }

        let mut text_recent_pictures: Frame =
            app::widget_from_id("gui::preview_main::text_recent_pictures_info").unwrap();

        let history_attach_list = get_walk_attach_file();
        if (history_attach_list.len() != 0) {
            text_recent_pictures.set_label("扫描历史查找...");
        }

        println!("id -> {}", id.clone());

        for (key, path_list) in history_attach_list.clone() {
            for path in path_list {
                let resolve_path = path.to_string_lossy();
                if resolve_path.contains(id.as_str()) {
                    // println!("是 {} 吗", resolve_path.clone());
                    // text_recent_pictures
                    //     .set_label(format!("是 {} 吗", path.to_string_lossy()).as_str());
                    global_var::set_str("user::config::walk_drag_path",path.to_string_lossy().to_string());
                    atomic_util::set_bool(&WATCH_PUPPET_ING,false);
                    return;
                }
            }
        }
        drop(history_attach_list);

        let mut walk_next = true;
        let (tx, rx) = std::sync::mpsc::channel();

        thread::spawn(move || {
            // 启动扫描线程
            let mut set_map = HashSet::new();
            let input_select_dir = global_var::get_str("user::config::input_select_dir");
            let user_select_wxid = global_var::get_str("user::config::user_select_wxid");
            // 优先考虑当前已选用户
            set_map.insert(wh_mod::resolve_path(format!(
                "{}\\{}\\FileStorage\\MsgAttach",
                input_select_dir.clone(),
                user_select_wxid.clone()
            )));

            // 当前用户没找到扩展到全局
            let wx_read_root_wxid = wh_mod::wx_read_root_wxid(Path::new(input_select_dir.as_str()));

            for wx_read_wxid in wx_read_root_wxid {
                let path2str = wx_read_wxid.attach.to_string_lossy().to_string();
                set_map.insert(wh_mod::resolve_path(path2str));
            }

            // 启动扫描线程
            for path_for in set_map {
                if (walk_next) {
                    wh_mod::walk_file(Path::new(path_for.as_str()), tx.clone(), "".to_string());
                }
            }
        });

        // println!("{:?}",wh_mod::wx_read_root_wxid(Path::new(global_var::get_str("user::config::input_select_dir").as_str())));

        let mut walk_next_not_message: usize = 0;

        while walk_next {
            if let Result::Ok((attach_key, paths)) = rx.recv() {
                // println!("attach_key-> {} paths->{:?}",attach_key,paths.len());
                for path in paths {
                    let resolve_path = path.to_string_lossy();
                    if resolve_path.contains(id.as_str()) {
                        // println!("是 {} 吗", resolve_path.clone());
                        // text_recent_pictures
                        //     .set_label(format!("是 {} 吗", path.to_string_lossy()).as_str());
                        global_var::set_str("user::config::walk_drag_path",path.to_string_lossy().to_string());
                        walk_next = false;
                    }
                }
            } else {
                walk_next_not_message = walk_next_not_message + 1;
                if (walk_next_not_message > 50) {
                    walk_next = false;
                }
                // println!("没有消息");
            }
        }
        atomic_util::set_bool(&WATCH_PUPPET_ING,false);
        // if let Some(mut next_scan_ing_gui) = app::widget_from_id("gui::gui_drag_scan::next_scan_ing_gui") as Option<DoubleWindow>{
        //     next_scan_ing_gui.hide();
        //
        // }
        // if let Some(mut next_scan_not_gui) = app::widget_from_id("gui::gui_drag_scan::next_scan_not_gui") as Option<DoubleWindow>{
        //     next_scan_not_gui.show();
        //
        // }
        println!("拖拽检测线程已经退出");
    });
}

pub fn main_window() {
    atomic_util::set_bool(&INITIALIZED_PUPPET,false);
    atomic_util::set_bool(&WINDOW_STATE_AVAILABLE,true);
    atomic_util::set_i64(&WINDOW_HWND,0);
    global_var::set_str("user::config::walk_drag_path",String::new());

    if (global_var::get_bool("gui::open::gui_drag_scan")) {
        if let Some(mut wins) =
            app::widget_from_id("gui::DoubleWindow::gui_drag_scan::main") as Option<DoubleWindow>
        {
            wins.show();
            wins.set_visible_focus();

            return;
        }
        return;
    }
    global_var::set_bool("gui::open::gui_drag_scan", true);

    set_theme!();
    let mut win: DoubleWindow =
        fltk::window::DoubleWindow::new(0, 0, 351, 367, "扫描图源用户").center_screen();
    win.set_color(Color::from_rgb(24, 24, 24));
    win.set_border(false);
    setInterfaceBackgroundImage(&mut win);
    win.set_id("gui::DoubleWindow::gui_drag_scan::main");

    let mut preview = ImgPreview::new(0, 0, 351, 367, "gui::preview_main::index::user_select");
    preview.from_data(
        include_bytes!("./assets/select_user_base/drag_scan/scan.png").to_vec(),
        0,
        0,
        351,
        367,
    );

    let mut page_02_state_title = Frame::new(137, 152, 75, 20, "扫描图源用户");
    page_02_state_title.set_label_size(12);

    let mut btn_text_next = Frame::new(137 + 5, 317, 65, 20, "朕不找了");
    btn_text_next.set_label_size(13);
    let mut next_btn = gui_hotspot::create_hotspot(109, 308, 126, 39);

    let mut page_02_state_info_01 =
        Frame::new(63, 183, 222, 20, "你可以从聊天窗口拖拽一张图片到本窗口");
    page_02_state_info_01.set_label_size(11);

    let mut page_02_state_info_02 = Frame::new(124, 215, 104, 20, "用时： 约万张/1秒");
    page_02_state_info_02.set_label_size(11);
    page_02_state_info_02.set_label_color(Color::from_rgb(51, 51, 51));

    let mut next_scan_ing_gui = fltk::window::Window::new(0, 0, 351, 269, None);
    next_scan_ing_gui.set_id("gui::gui_drag_scan::next_scan_ing_gui");
    let mut preview = ImgPreview::new(0, 0, 351, 367, "gui::preview_main::index::user_select");
    preview.from_data(
        include_bytes!("./assets/select_user_base/drag_scan/scan.png").to_vec(),
        0,
        0,
        351,
        367,
    );
    let mut text_recent_pictures = Frame::new(130, 175, 80, 20, "正在扫描中");
    text_recent_pictures.set_label_size(15);

    let mut text_recent_pictures_info = Frame::new(124, 215, 104, 20, "正在初始化...");
    text_recent_pictures_info.set_label_size(11);
    text_recent_pictures_info.set_label_color(Color::from_rgb(51, 51, 51));
    text_recent_pictures_info.set_id("gui::preview_main::text_recent_pictures_info");

    next_scan_ing_gui.end();
    next_scan_ing_gui.hide();

    let mut next_scan_not_gui: DoubleWindow = fltk::window::Window::new(0, 0, 351, 269, None);
    next_scan_not_gui.set_id("gui::gui_drag_scan::next_scan_not_gui");
    let mut preview_scan_not =
        ImgPreview::new(0, 0, 351, 367, "gui::preview_main::index::user_select");
    preview_scan_not.from_data(
        include_bytes!("./assets/select_user_base/drag_scan/scan_not.png").to_vec(),
        0,
        0,
        351,
        367,
    );
    let mut text_recent_pictures_scan_not = Frame::new(130 + 5, 175 + 35, 80, 20, "找不到此用户");
    text_recent_pictures_scan_not.set_label_size(13);

    next_scan_not_gui.end();
    next_scan_not_gui.hide();

    // 小标题
    let mut state_title = text_recent_pictures.clone();
    // 大标题
    let mut state_info = text_recent_pictures_info.clone();
    let mut btn_text = btn_text_next.clone();

    thread::spawn(move||{
        let mut window_state_available =  atomic_util::get_bool(&WINDOW_STATE_AVAILABLE);

        while window_state_available {

            Sleep(300);

            if atomic_util::get_bool(&INITIALIZED_PUPPET) {

                if atomic_util::get_bool(&WATCH_PUPPET_ING) {
                    println!("开始扫描了-> ");
                    state_info.set_label("正在扫描中...");
                    btn_text.set_label("取消任务");
                }else{
                    let walk_drag_path = global_var::get_str("user::config::walk_drag_path");

                    state_title.set_label("扫描已结束");
                    println!("walk_drag_path-> {}",walk_drag_path.as_str());
                    if(walk_drag_path.len()<3){
                        state_info.set_label("未扫描到此文件");
                        btn_text.set_label("关闭窗口");
                    }else{
                        state_info.set_label("已经获取到此文件");
                        btn_text.set_label("完成选定");
                        // atomic_util::set_bool(&WINDOW_STATE_AVAILABLE,false);pr
                        // println!("user_select_path-> {}",global_var::get_str("user::config::user_select_path"));

                       global_var::set_str("user::config::user_select_path",wh_mod::wx_parse_path(walk_drag_path.clone()).attach_id);
                       global_var::set_i32("user::config::select_user_thumbnail_obj",-2);

                    }
                    return;
                }

            }
            window_state_available =  atomic_util::get_bool(&WINDOW_STATE_AVAILABLE);
        }
        println!("gui_drag_scan 窗口关闭了-> ");
    });

    macro_rules! is_closeWindow {
        ()=>{
               if !atomic_util::get_bool(&WINDOW_STATE_AVAILABLE) {
                    closeWindow(atomic_util::get_i64(&WINDOW_HWND) as i128, true);
                    global_var::set_bool("gui::open::gui_drag_scan", false);
                }
        }
    }
    win.handle({
        let mut x = 0;
        let mut y = 0;
        let mut released = true;
        let mut dnd = true;
        let mut drag_path = std::path::PathBuf::new();

        // let mut move_
        move |win, ev| match ev {

            enums::Event::Show => {
                win.set_visible_focus();
                let hwnd = win.raw_handle() as i128;
                libWxIkunPlus::setwinVisible(hwnd.clone(), true);
                atomic_util::set_i64(&WINDOW_HWND,hwnd as i64);
                println!("walk_drag_page hwnd -> :  {}", hwnd.clone());
                true
            }
            enums::Event::Hide=>{
                is_closeWindow!();
                false
            }
            enums::Event::Push => {
                if (next_btn.existPoint(x, y))
                    {
                        atomic_util::set_bool(&WINDOW_STATE_AVAILABLE,false);
                        closeWindow(win.raw_handle() as i128, true);
                        global_var::set_bool("gui::open::gui_drag_scan", false);
                        atomic_util::set_i64(&WINDOW_HWND,0);
                }
                true
            }

            enums::Event::Move => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;
                if (next_btn.existPoint(x, y)) {
                    win.set_cursor(Cursor::Hand);
                } else {
                    win.set_cursor(Cursor::Default);
                }
                true
            }

            Event::DndEnter => {
                dnd = true;
                true
            }
            Event::DndDrag => true,
            Event::DndRelease => {
                released = true;
                true
            }
            Event::Unfocus=>{
                true
            }
            Event::Leave=>{
                true
            }
            Event::NoEvent=>{
                true
            }
            Event::Paste => {
                if dnd && released {
                    let mut path_list = Vec::new();
                    let path = app::event_text()
                        .replace("\n\r", "\n")
                        .replace("\r\n", "\n")
                        .replace("\r", "\n")
                        .replace("file://", "\n");

                    let lines: Vec<&str> = path.split('\n').collect();
                    for line in lines {
                        let line_f = format!("{}", line);
                        if (line_f.is_empty()) {
                            continue;
                        }
                        path_list.push(line_f);
                    }

                    if(!atomic_util::get_bool(&INITIALIZED_PUPPET)){
                        if !path_list.is_empty() {
                            let path = Path::new(path_list.first().unwrap()).to_path_buf();
                            drag_path = path.clone();

                            next_scan_ing_gui.show();
                            btn_text_next.set_label("结束任务");
                            initialize_watch_attach_puppet(drag_path.to_string_lossy().to_string());
                        }

                    }

                    dnd = false;
                    released = false;

                    true
                } else {
                    false
                }
            }
            Event::DndLeave => {
                dnd = false;
                released = false;
                true
            }
            enums::Event::Drag => {
                if y < 69 {
                    win.clone()
                        .set_pos(app::event_x_root() - x, app::event_y_root() - y);
                }

                true
            }
            _ => {
                false },
        }
    });

    win.end();
    win.show();


}
