

// 判断当前是否处于开发者模式
pub fn is_developer() -> bool {

    // std::env::var("_debug").is_ok()
    !false
}

// 编译版本是 52破解专版
pub fn is_build_52pojie() -> bool {
    false
}

// 是否对显示的数据进行消敏
pub fn is_show_mask() -> bool {
    is_show_dome()||true
}

// 是否在选择对象后自动显示最近十张照片
pub fn is_click_open_preview() -> bool {
    false
}

// 演示模式
pub fn is_show_dome() -> bool {
    true
}#![allow(warnings, unused)]

use std::collections::hash_map::DefaultHasher;
use std::io::Read;
use std::mem::transmute;
use std::{
    env,
    ffi::{c_int, c_long, OsStr},
    fs,
    hash::{Hash, Hasher},
    io,
    path::{Path, PathBuf},
    process,
    process::{Command, Stdio},
    thread,
    time::Duration,
};

use libc::getenv;
use std::string::FromUtf8Error;

use crate::{libWxIkunPlus, wh_mod};

// 判断文件头(魔术数字)
pub fn detect_image_format(data: &[u8]) -> Option<&'static str> {
    if data.len() < 4 {
        return None; // 数据不够长，无法识别
    }

    match &data[0..4] {
        [0x89, 0x50, 0x4E, 0x47] => Some("png"),
        [0xFF, 0xD8, _, _] => Some("jpg"),
        [0x47, 0x49, 0x46, 0x38] => Some("gif"),
        [0x52, 0x49, 0x46, 0x46]
            if data.len() >= 12 && &data[8..12] == [0x57, 0x45, 0x42, 0x50] =>
        {
            Some("webp")
        }
        _ => None, // 未知格式
    }
}

// !请注意  异或解密算法是不开源的部分
// 对指定的文件进行异或算法 以此获取解码的文件
// 对指定的文件进行异或算法 以此获取解码的文件
pub fn convert_bat_images(input_path: PathBuf, mut output_path: PathBuf) -> io::Result<String> {

    let buff =  convert_dat_images_buff(output_path.clone())?;
    match detect_image_format(&buff.clone()) {
        Some(format) => {
            output_path.set_extension(format);
            let mut out_dir = Path::new(&output_path).join("..");

            if !out_dir.exists() {
                fs::create_dir_all(out_dir)?;
            }

            fs::write(&output_path, &buff)?;
        }
        None => println!("Unknown image format."),
    }

    Ok(output_path.display().to_string())
}


// !请注意  异或解密算法是不开源的部分
// 对指定的文件进行异或算法 以此获取解码的文件
pub fn convert_dat_images_buff(input_path: PathBuf) -> io::Result<Vec<u8>> {
    let content = fs::read(&input_path)?;

    // let ** = content[0];
    // let ** = content[1];

    // let ** = ** ^ 0x66666;
    // let ** = ** ^ 0x66666;
    // let ** = ** ^ 0x66666;
    // let ** = ** ^ 0x66666;
    // let ** = ** ^ 0x66666;
    // let ** = ** ^ 0x66666;

    // let mut ** = ** ^ 0x66666;

    // if ** == ** {
    //     vi = **;
    // } else if ** == ** {
    //     vi = **;
    // } else if ** == ** {
    //     vi = **;
    // }

    // let buff: Vec<u8> = content.iter().map(|br| br ^ vi).collect();
   
    let buff: Vec<u8> = Vec::new();
    Ok(buff)
}

// 按照换行分割文本
pub fn split_line(input_path: String) -> Vec<String> {
    let mut path2arr: Vec<String> = Vec::new();
    let mut str: String = String::new();
    // 按照 \\ / 分割路径
    for char in input_path.chars() {
        if char.to_string().bytes().eq("\n".to_string().bytes())
            || char.to_string().bytes().eq("\r".to_string().bytes())
        {
            path2arr.push(str.clone());
            str.clear();
            continue;
        }
        str.push(char);
    }

    // 最后一个文本
    if str.len() != 0 {
        path2arr.push(str.clone());
    }

    let mut path2arr_filter = Vec::new();

    for path2 in path2arr {
        if (!path2.is_empty()) {
            path2arr_filter.push(path2);
        }
    }

    path2arr_filter
}

// 获取用户数据根文件夹
pub fn get_user_data_path() -> Option<String> {
    // ! 这部分也是不开源的部分
    Option::None
}

// 获取用户名
#[derive(Debug)]
pub struct WxidNameAccinfo {
    // 头像url
    pub avatar: String,
    // 名称
    pub name: String,
    // wxid
    pub wx_id: String,
}

// 获取指定数据的用户名
pub fn get_wxid_name(user_data_path: String, wxid: String) -> Option<WxidNameAccinfo> {
    // ! 这部分也是不开源的部分
    Option::None
}

// 开发者用户id
pub fn get_user_id1() -> String {
    // ! 这部分也是不开源的部分
    return String::new();
}

// 开发者用户id
pub fn get_user_id2() -> String {
    // ! 这部分也是不开源的部分
    return String::new();
}

// 获取活动用户信息
#[derive(Debug)]
pub struct WxActiveUser {
    // 获取用户名
    pub accinfo: Option<WxidNameAccinfo>,
    // wxid
    pub user_wxid: String,
    // 根数据目录 D:\...\WeChat Files
    pub user_root: String,
    // 数据目录 D:\...\WeChat Files\%wxid%
    pub user_data: String
}

// 获取活动用户的路径
pub fn get_active_user(user_root: &str) -> Vec<WxActiveUser> {
    let mut active_users = Vec::new();
    let mut get_size = libWxIkunPlus::findAllWindow("WeChatMainWndForPC", "").len();
    let mut read_dir_list: Vec<fs::DirEntry> = Vec::new();

    // 读取包含 wxid_ 的文件夹
    if let Ok(read_dir) = fs::read_dir(user_root) {
        for dir in read_dir {
            if let Ok(dir) = dir {
                let string_lossy = format!("{:?}", dir.file_name());

                if string_lossy.contains("wxid_") {
                    read_dir_list.push(dir);
                }
            }
        }
    }
    
    let mut vec_wxid_list:Vec<PathBuf> = Vec::new();
    
    for value in read_dir_list {
        // 通过高更新率的文件判断出最后修改时间
        let config_path = value.path().join("config");

        let mut read_file_list = Vec::new();

        if let Ok(read_dir) = fs::read_dir(config_path) {
            for dir in read_dir {
                if let Ok(read_file) = dir {
                    read_file_list.push(read_file);
                }
            }
        }

        //按照修改时间排序
        read_file_list.sort_by(|a, b| {
            let mut a_created = std::time::UNIX_EPOCH;
            let mut b_created = std::time::UNIX_EPOCH;

            if let Ok(metadata) = a.metadata() {
                if let Result::Ok(create) = metadata.modified() {
                    a_created = create;
                }
            }

            if let Ok(metadata) = b.metadata() {
                if let Result::Ok(create) = metadata.modified() {
                    b_created = create;
                }
            }

            a_created.cmp(&b_created)
        });
        read_file_list.reverse();

        if !read_file_list.is_empty() {
            vec_wxid_list.push(read_file_list[0].path());
        }   

    }

    // 排序出根目录的路径
    vec_wxid_list.sort_by(|a, b| {
        let mut a_created = std::time::UNIX_EPOCH;
        let mut b_created = std::time::UNIX_EPOCH;

        if let Ok(metadata) = a.metadata() {
            if let Result::Ok(create) = metadata.modified() {
                a_created = create;
            }
        }

        if let Ok(metadata) = b.metadata() {
            if let Result::Ok(create) = metadata.modified() {
                b_created = create;
            }
        }

        a_created.cmp(&b_created)
    });

    vec_wxid_list.reverse();

    for value in vec_wxid_list.to_vec() {
        if active_users.len() >= get_size &&!is_developer(){
            break;
        }
     
       let parse_path = wh_mod::wx_parse_path(format!("{}",value.to_str().unwrap()));
        
        if !parse_path.user_data.is_empty()&&!parse_path.wxid.is_empty() {
            active_users.push(WxActiveUser{
                accinfo: get_wxid_name(parse_path.user_data.clone(),parse_path.wxid.clone()),
                user_wxid: parse_path.wxid.clone(),
                user_root: parse_path.user_data.clone(),
                user_data: format!("{}\\{}",parse_path.user_data.clone(),parse_path.wxid.clone()),
            });  
        }
        
      

    //    println!("parse_path-> {:?}",&parse_path);
    }
   
    // println!("read_file_list ->  {:?}",&vec_wxid_list);

    active_users
}

#![allow(warnings, unused)]

use crate::{atomic_util, global_var, gui_hotspot, gui_imge, handle_dat, inject_fltk_theme, libWxIkunPlus, util::{str_eq_str, Sleep}, wh_mod};
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

fn initialize_watch_attach_puppet(img_path: String) {
    atomic_util::set_bool(&INITIALIZED_PUPPET, true);
    thread::spawn(move || {
        atomic_util::set_bool(&WATCH_PUPPET_ING, true);
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

        let history_attach_list = wh_mod::get_walk_attach_file_history();
        if (history_attach_list.len() != 0) {
            println!("扫描历史查找... 共-> {} 条",history_attach_list.len());
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
                    global_var::set_string(
                        "user::config::walk_drag_path",
                        path.to_string_lossy().to_string(),
                    );
                    atomic_util::set_bool(&WATCH_PUPPET_ING, false);
              
              
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
            let input_select_dir = global_var::get_string_default("user::config::input_select_dir");
            let user_select_wxid = global_var::get_string_default("user::config::user_select_wxid");
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
                        global_var::set_string(
                            "user::config::walk_drag_path",
                            path.to_string_lossy().to_string(),
                        );
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
        atomic_util::set_bool(&WATCH_PUPPET_ING, false);
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
    atomic_util::set_bool(&INITIALIZED_PUPPET, false);
    atomic_util::set_bool(&WINDOW_STATE_AVAILABLE, true);
    atomic_util::set_i64(&WINDOW_HWND, 0);
    global_var::set_string("user::config::walk_drag_path", String::new());

    if (global_var::get_bool_default("gui::open::gui_drag_scan")) {
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

    inject_fltk_theme!();
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

    thread::spawn(move || {
        let mut window_state_available = atomic_util::get_bool(&WINDOW_STATE_AVAILABLE);

        while window_state_available {
            Sleep(300);

            if atomic_util::get_bool(&INITIALIZED_PUPPET) {
                if atomic_util::get_bool(&WATCH_PUPPET_ING) {
                    println!("开始扫描了-> ");
                    state_info.set_label("正在扫描中...");
                    btn_text.set_label("取消任务");
                } else {
                    let walk_drag_path =
                        global_var::get_string_default("user::config::walk_drag_path");

                    state_title.set_label("扫描已结束");
                    println!("walk_drag_path-> {}", walk_drag_path.as_str());
                    if (walk_drag_path.len() < 3) {
                        state_info.set_label("未扫描到此文件");
                        btn_text.set_label("关闭窗口");
                    } else {
                        state_info.set_label("已经获取到此文件");
                        btn_text.set_label("完成选定");
                        // atomic_util::set_bool(&WINDOW_STATE_AVAILABLE,false);pr
                        // println!("user_select_path-> {}",global_var::get_str("user::config::user_select_path"));

                        global_var::set_string(
                            "user::config::user_select_path",
                            wh_mod::wx_parse_path(walk_drag_path.clone()).attach_id,
                        );
                        global_var::set_i32("user::config::select_user_thumbnail_obj", -2);
                        
                        // atomic_util::set_bool(&WINDOW_STATE_AVAILABLE, false);
                        // global_var::set_bool("gui::open::gui_drag_scan", false);
                        // atomic_util::set_i64(&WINDOW_HWND, 0);
                        // if let Some(mut win) = fltk::app::widget_from_id("gui::DoubleWindow::gui_drag_scan::main") as Option<DoubleWindow> {
                        //     win.hide();
                        //     win.clear();
                            
                        //     fltk::window::Window::delete(win.clone());
                        // }

                    }
                    return;
                }
            }
            window_state_available = atomic_util::get_bool(&WINDOW_STATE_AVAILABLE);
        }
        println!("gui_drag_scan 窗口关闭了-> ");
    });

    macro_rules! is_closeWindow {
        () => {
            if !atomic_util::get_bool(&WINDOW_STATE_AVAILABLE) {
                closeWindow(atomic_util::get_i64(&WINDOW_HWND) as i128, true);
                global_var::set_bool("gui::open::gui_drag_scan", false);
            }
        };
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
                atomic_util::set_i64(&WINDOW_HWND, hwnd as i64);
                println!("walk_drag_page hwnd -> :  {}", hwnd.clone());
                true
            }
            enums::Event::Hide => {
                is_closeWindow!();
                false
            }
            enums::Event::Push => {
                if (next_btn.existPoint(x, y)) {
                    atomic_util::set_bool(&WINDOW_STATE_AVAILABLE, false);
                    // closeWindow(win.raw_handle() as i128, true);
                    fltk::window::Window::delete(win.clone());
                    global_var::set_bool("gui::open::gui_drag_scan", false);
                    atomic_util::set_i64(&WINDOW_HWND, 0);
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
            Event::Unfocus => true,
            Event::Leave => true,
            Event::NoEvent => true,
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

                    if (!atomic_util::get_bool(&INITIALIZED_PUPPET)) {
                        if !path_list.is_empty() {
                            path_list.reverse();
                            for for_path in path_list {
                                // if wh_mod::convert::is_developer()||(for_path.contains("wxid_")&&(for_path.contains("FileStorage\\MsgAttach")||for_path.contains("FileStorage/MsgAttach"))){

                                if PathBuf::from(for_path.clone()).exists() {
                                    let path = Path::new(&for_path.clone()).to_path_buf();
                                    drag_path = path.clone();

                                    next_scan_ing_gui.show();
                                    btn_text_next.set_label("结束任务");
                                    initialize_watch_attach_puppet(
                                        drag_path.to_string_lossy().to_string(),
                                    );
                                    break;
                                }
                                
                            }
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
            enums::Event::Close=>{
                atomic_util::set_bool(&WINDOW_STATE_AVAILABLE, false);
                global_var::set_bool("gui::open::gui_drag_scan", false);
                atomic_util::set_i64(&WINDOW_HWND, 0);
                // fltk::window::Window::delete(win.clone());
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
#![allow(warnings, unused)]

pub struct HotspotItmeControl {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl HotspotItmeControl {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        let mut check_item_control = HotspotItmeControl {
            x,
            y,
            width,
            height,
        };

        check_item_control
    }

    // 判断鼠标是否在当前元素
    pub fn existPoint(&self, x: i32, y: i32) -> bool {
        return x > self.x.clone()
            && x < self.x.clone() + self.width.clone()
            && y > self.y.clone()
            && y < self.y.clone() + self.height.clone();
    }
}

pub fn create_hotspot(x: i32, y: i32, width: i32, height: i32) -> HotspotItmeControl {
    HotspotItmeControl::new(x, y, width, height)
}
#![allow(warnings, unused)]

use fltk::{
    app::{self, handle},
    button::{self, Button},
    draw::font,
    enums::{self, Color, Cursor, Event, Font, FrameType, LabelType},
    frame::{self, Frame},
    group::{self, Flex, Group},
    image::{self, Image, PngImage},
    input::{InputType, IntInput},
    prelude::*,
    text::TextDisplay,
    tree,
    window::{self, DoubleWindow, Window},
};
use fltk_table::{SmartTable, TableOpts};
use std::{
    fs,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    process,
    process::{Command, Stdio},
    thread,
};

pub struct ImagesItmeControl {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub id: String,
}

impl ImagesItmeControl {
    pub fn new(
        appMainWin: &mut window::DoubleWindow,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        images: PngImage,
        id: String,
    ) -> Self {
        let mut check_itme_control = ImagesItmeControl {
            x,
            y,
            width,
            height,
            id: id.clone(),
        };
        let mut frame = Frame::default()
            .with_size(width, height)
            .center_of(appMainWin);

        frame.set_frame(FrameType::NoBox);
        frame.set_color(Color::from_u32(0));
        frame.set_id(id.as_str());

        frame.set_image(Some(images));
        frame.set_pos(x, y);
        frame.show();

        check_itme_control
    }

    /**
     * 获取主窗口
     */
    pub fn get_main(&self) -> Frame {
        let mut frame: Frame = app::widget_from_id(self.id.as_str()).unwrap();
        frame
    }

    // 判断鼠标是否在当前元素
    pub fn existPoint(&self, x: i32, y: i32) -> bool {
        return x > self.x.clone()
            && x < self.x.clone() + self.width.clone()
            && y > self.y.clone()
            && y < self.y.clone() + self.height.clone();
    }
}

pub fn create_Images(
    appMainWin: &mut window::DoubleWindow,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    images: PngImage,
    id: String,
) -> ImagesItmeControl {
    ImagesItmeControl::new(appMainWin, x, y, width, height, images, id)
}

pub struct ImgPreview {
    pub preview: frame::Frame,
    x:i32,
    y:i32,
    width: i32, 
    height: i32
}
impl Clone for ImgPreview {
    fn clone(&self) -> Self {
        ImgPreview {
            preview:self.preview.clone(),
            x: self.x.clone(),
            y: self.y.clone(),
            width: self.width.clone(),
            height: self.height.clone(),
        }
    }
}

impl ImgPreview {
    pub fn detect_image_format(data: &[u8]) -> Option<&'static str> {
        if data.len() < 4 {
            return None; // 数据不够长，无法识别
        }

        match &data[0..4] {
            [0x89, 0x50, 0x4E, 0x47] => Some("png"),
            [0xFF, 0xD8, _, _] => Some("jpg"),
            [0x47, 0x49, 0x46, 0x38] => Some("gif"),
            [0x52, 0x49, 0x46, 0x46]
                if data.len() >= 12 && &data[8..12] == [0x57, 0x45, 0x42, 0x50] =>
            {
                Some("webp")
            }
            _ => None, // 未知格式
        }
    }

    pub fn new(x: i32, y: i32, width: i32, height: i32, id: &'static str) -> Self {
        let mut preview = frame::Frame::new(x, y, width, height, "");
        preview.set_frame(enums::FrameType::FlatBox);
        preview.set_color(enums::Color::from_rgb(80, 80, 80));
        preview.set_id(id);
        Self { preview, x, y, width, height }
    }

    // 判断鼠标是否在当前元素
    pub fn existPoint(&self, x: i32, y: i32) -> bool {
        return x > self.x.clone()
            && x < self.x.clone() + self.width.clone()
            && y > self.y.clone()
            && y < self.y.clone() + self.height.clone();
    }

    pub fn load(&mut self, path: String, x: i32, y: i32, width: i32, height: i32) -> bool {
        let mut res = false;
        // macro_rules! re_imag {
        //     ($imag:expr) => {
        //         $imag.scale(width, height, false, true);
        //         self.preview.draw(move |cb| {
        //             let cbx: i32 = cb.x();
        //             let cby: i32 = cb.y();
        //             let cbh: i32 = cb.h();
        //             let cbw: i32 = cb.w();
        //             let cx: i32 = x;
        //             let cy: i32 = y;
        //             $imag.draw_ext(cbx, cby, cbw, cbh, cx, cy);
        //         });
        //         self.preview.redraw();
        //         self.preview.redraw_label();
        //         res = true;
        //     };
        // }

        if let Result::Ok(data) = fs::read(path) {
            res = self.from_data(data, x, y, width, height);
        }
        res
    }

    pub fn from_data(&mut self, data: Vec<u8>, x: i32, y: i32, width: i32, height: i32) -> bool {
        let mut res = false;
        macro_rules! re_imag {
            ($imag:expr) => {
                $imag.scale(width, height, false, true);
                self.preview.draw(move |cb| {
                    let cbx: i32 = cb.x();
                    let cby: i32 = cb.y();
                    let cbh: i32 = cb.h();
                    let cbw: i32 = cb.w();
                    let cx: i32 = x;
                    let cy: i32 = y;
                    $imag.draw_ext(cbx, cby, cbw, cbh, cx, cy);
                });
                self.preview.redraw();
                self.preview.redraw_label();
                res = true;
            };
        }

        if let Some(ext) = ImgPreview::detect_image_format(&data) {
            if (ext.as_bytes().eq("png".as_bytes())) {
                if let Result::Ok(mut img) = image::PngImage::from_data(&*data) {
                    re_imag!(img);
                }
            } else if (ext.as_bytes().eq("jpg".as_bytes())) {
                if let Result::Ok(mut img) = image::JpegImage::from_data(&*data) {
                    re_imag!(img);
                }
            } else if (ext.as_bytes().eq("gif".as_bytes())) {
                if let Result::Ok(mut img) = image::GifImage::from_data(&*data) {
                    re_imag!(img);
                }
            }
        }

        res
    }

    // pub fn from_imag<T : fltk::prelude::ImageExt >(&mut self, mut data:  T, x: i32, y: i32, width: i32, height: i32){
    //     let mut res = false;
    //     self.preview.draw(move |cb| {
    //         let cbx: i32 = cb.x();
    //         let cby: i32 = cb.y();
    //         let cbh: i32 = cb.h();
    //         let cbw: i32 = cb.w();
    //         let cx: i32 = x;
    //         let cy: i32 = y;
    //         data.draw_ext(cbx, cby, cbw, cbh, cx, cy);
    //     });
    //     self.preview.redraw();
    //     self.preview.redraw_label();
    //     res = true;
    //
    // }
}
#![allow(warnings, unused)]


use crate::{
    global_var, handle_dat, libWxIkunPlus,
    util::{str_eq_str, Sleep}, global_var_util,
};
use fltk::{
    app::{self, handle},
    button::{self, Button},
    draw::font,
    enums::{self, Color, Cursor, Event, Font, FrameType, LabelType},
    frame::Frame,
    group::{self, Flex, Group},
    image,
    input::{InputType, IntInput},
    prelude::*,
    text::TextDisplay,
    tree,
    window::{self, DoubleWindow, Window},
};
use fltk_table::{SmartTable, TableOpts};
use rusqlite::Connection;
use std::{
    fs,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    process,
    process::{Command, Stdio},
    thread,
};
use winapi::um::winuser::{CloseWindow, SetActiveWindow};
use crate::console_log;
// 按钮基本都是以坐标判断的
struct PointExistHasmap {
    // 关闭按钮
    quit: bool,
    // 下个
    next: bool,
    // 上个
    backup: bool,
    // 卡片 1
    card_01: bool,
    // 卡片 2
    card_02: bool,
    // 卡片 3
    card_03: bool,

    // 卡片 1
    rm_card_01: bool,
    // 卡片 2
    rm_card_02: bool,
    // 卡片 3
    rm_card_03: bool,

    // 所有按钮
    existCursor: bool,
}

// 判断鼠标坐标是否在此元素内
fn getFormPointSpace(x: i32, y: i32) -> PointExistHasmap {
    // 输入: x,y,w,y
    macro_rules! check_point_in_space {
        ($xx:expr, $yy:expr, $width:expr, $height:expr) => {
            x > $xx && x < $xx + $width && y > $yy && y < $yy + $height
        };
    }

    let mut point_exist_hasmap = PointExistHasmap {
        quit: false,
        next: false,
        backup: false,
        card_01: false,
        card_02: false,
        card_03: false,
        rm_card_01: false,
        rm_card_02: false,
        rm_card_03: false,
        existCursor: false,
    };

    point_exist_hasmap.quit = check_point_in_space!(273, 17, 40, 40);
    point_exist_hasmap.backup = check_point_in_space!(41, 492, 121, 44);
    point_exist_hasmap.next = check_point_in_space!(171, 492, 121, 44);
    point_exist_hasmap.card_01 = check_point_in_space!(20, 90, 286, 110);
    point_exist_hasmap.card_02 = check_point_in_space!(20, 216, 286, 110);
    point_exist_hasmap.card_03 = check_point_in_space!(20, 345, 286, 110);

    // 移除
    point_exist_hasmap.rm_card_01 = check_point_in_space!(230, 164, 65, 22);
    point_exist_hasmap.rm_card_02 = check_point_in_space!(230, 291, 65, 22);
    point_exist_hasmap.rm_card_03 = check_point_in_space!(230, 419, 65, 22);

    let mut win_coords_cursor_list = vec![
        point_exist_hasmap.quit,
        point_exist_hasmap.backup,
        point_exist_hasmap.next,
        point_exist_hasmap.rm_card_01,
        point_exist_hasmap.rm_card_02,
        point_exist_hasmap.rm_card_03,
    ];

    let mut existCursor = false;
    for value in win_coords_cursor_list.iter() {
        // 关闭按钮
        if *(value) {
            existCursor = true;
        }
    }

    point_exist_hasmap.existCursor = existCursor;

    return point_exist_hasmap;
}

// 设置背景为图片（主视图）
fn setInterfaceBackgroundImage(appMainWin: &mut window::DoubleWindow) -> Frame {
    let background_image = image::PngImage::from_data(include_bytes!("../assets/manage_itme.png"))
        .expect("set main icon error");
    let mut frame = Frame::default().with_size(326, 0).center_of(appMainWin);
    frame.set_frame(FrameType::EngravedBox);
    frame.set_image(Some(background_image));
    return frame;
}

// 剪裁路径 字数不超过XXX
pub fn get_form_slice_path(path: String, max_size: usize) -> Result<String, String> {
    let mut new_path: String = String::new();
    let mut size = 0;

    for ch in path.chars() {
        new_path.push(ch);
        size = size + 1;
        if (size > max_size) {
            break;
        }
        // println!("{}", ch);
    }

    // let mut size = path.len();

    // if size > max_size {
    // size = max_size;
    // return Err("Input string exceeds maximum size".to_string());
    // }

    // let new_path: String = path[..size].to_string();
    Ok(new_path)
}

// 卡片会回传为这个参数 用来控制
struct CardItme {
    background: Frame,
    path: Frame,
    ouput: Frame,
    name: Frame,
    status_err: Frame,
    status_ok: Frame,
    main: DoubleWindow,
    nameStr: String,
    pathStr: String,
    ouputStr: String,
    remove: bool,
}

// 任务卡片 （不会动态创建 而是引用同一个）
fn create_card(x: i32, y: i32, name: String, path: String, ouput: String) -> CardItme {
    let mut card_win = window::Window::new(x, y, 289, 113, None);
    card_win.set_color(Color::from_rgb(24, 24, 24));

    // 背景
    let background_image = image::PngImage::from_data(include_bytes!("../assets/card.png"))
        .expect("set main icon error");
    let mut frame = Frame::default().with_size(289, 0).center_of(&card_win);
    frame.set_frame(FrameType::EngravedBox);
    frame.set_image(Some(background_image));

    // 文本-> path
    let mut frame_info_path = Frame::new(17, 18, 200, 18, None); //.with_size(200, 18).left_of(&flex,1);
                                                                 // let slice_path =path.get(0..35).unwrap();

    frame_info_path.set_label(&get_form_slice_path(path.clone(), 20).unwrap_or(path.clone()));
    frame_info_path.set_label_size(12);
    frame_info_path.set_label_color(Color::from_rgb(186, 186, 186));
    frame_info_path.resize(32, 18, 200, 18);

    // 文本-> ouput
    let mut frame_info_ouput = Frame::new(17, 18, 200, 18, None); //::default().with_size(200, 18).left_of(&flex,1);
    frame_info_ouput.set_label(&get_form_slice_path(ouput.clone(), 20).unwrap_or(ouput.clone()));
    frame_info_ouput.set_label_size(12);
    frame_info_ouput.set_label_color(Color::from_rgb(186, 186, 186));
    frame_info_ouput.resize(32, 44, 200, 18);

    // 文本-> name
    let mut frame_info_name = Frame::default().with_size(200, 22);
    frame_info_name.set_label(&get_form_slice_path(name.clone(), 7).unwrap_or(name.clone()));
    frame_info_name.set_label_size(15);
    frame_info_name.set_label_color(Color::from_rgb(255, 255, 255));
    frame_info_name.resize(32, 80, 200, 22);

    let card_ok = image::PngImage::from_data(include_bytes!("../assets/card_ok.png"))
        .expect("set main icon error");

    let card_error = image::PngImage::from_data(include_bytes!("../assets/card_error.png"))
        .expect("set main icon error");

    let mut card_status_ok = Frame::default().with_size(26, 26).center_of(&card_win);
    card_status_ok.set_frame(FrameType::EngravedBox);
    card_status_ok.set_image(Some(card_ok));
    card_status_ok.set_pos(244, 11);

    let mut card_status_error = Frame::default().with_size(26, 26).center_of(&card_win);
    card_status_error.set_frame(FrameType::EngravedBox);
    card_status_error.set_image(Some(card_error));
    card_status_error.set_pos(244, 11);
    card_status_error.hide();

    card_win.end();

    let move_frame_info_path = frame_info_path.clone();
    let mut move_card_status_error = card_status_error.clone();
    let mut move_card_status_ok = card_status_ok.clone();

    return CardItme {
        main: card_win,
        background: frame,
        path: frame_info_path,
        ouput: frame_info_ouput,
        name: frame_info_name,
        status_err: card_status_error,
        status_ok: card_status_ok,
        ouputStr: ouput,
        nameStr: name,
        pathStr: path,
        remove: false,
    };
}

// 主窗口
pub fn ManageItmeMain() {
    if (global_var::get_bool_default("gui::open::manage_item")) {
        if let Some(mut wins) =
            app::widget_from_id("gui::DoubleWindow::manage_item::main") as Option<DoubleWindow>
        {
            wins.show();
            wins.set_visible_focus();
        }

        return ;
    }

    global_var::set_bool("gui::open::manage_item", true);

    let mut win: DoubleWindow = DoubleWindow::new(0, 0, 326, 554, "管理分组");
    win.set_color(Color::from_rgb(24, 24, 24));
    win.set_border(false);
    win.set_id ("gui::DoubleWindow::manage_item::main");

    let mut export_dir_path_list: Vec<global_var_util::ExportDirItme> =
    global_var_util::get_export_dir_itme_list();

    // 偏移量
    let mut offset = 0;

    fltk::app::set_scrollbar_size(3);
    fltk::app::set_selection_color(24, 24, 24);
    setInterfaceBackgroundImage(&mut win);

    let mut default_itme_text = "没有数据...";

    macro_rules! create_card_itme {
        ($card_id:expr) => {
            create_card(
                18,
                (if ($card_id == 1) {
                    88
                } else if ($card_id == 2) {
                    215
                } else {
                    342
                }),
                default_itme_text.to_owned(),
                default_itme_text.to_owned(),
                default_itme_text.to_owned(),
            )
        };

        ($card_id:expr,$itme_id:expr) => {
            create_card(
                18,
                (if ($card_id == 1) {
                    88
                } else if ($card_id == 2) {
                    215
                } else {
                    342
                }),
                export_dir_path_list[$itme_id].name.clone().to_owned(),
                export_dir_path_list[$itme_id].path.clone().to_owned(),
                export_dir_path_list[$itme_id].ouput.clone().to_owned(),
            )
        };

        ($card_id:expr,$name:expr,$path:expr,$ouput:expr ) => {
            create_card(
                18,
                (if ($card_id == 1) {
                    88
                } else if ($card_id == 2) {
                    215
                } else {
                    342
                }),
                $name.to_owned(),
                $path.to_owned(),
                $ouput.to_owned(),
            )
        };
    }

    let mut card_01 = if export_dir_path_list.len() >= 1 {
        create_card_itme!(1, 0)
    } else {
        create_card_itme!(1)
    };
    let mut card_02 = if export_dir_path_list.len() >= 2 {
        create_card_itme!(2, 1)
    } else {
        create_card_itme!(2)
    };
    let mut card_03 = if export_dir_path_list.len() >= 3 {
        create_card_itme!(3, 2)
    } else {
        create_card_itme!(3)
    };

    if export_dir_path_list.len() < 2 {
        card_02.main.hide();
    } else {
        // offset = 1;
    }

    if export_dir_path_list.len() < 3 {
        card_03.main.hide();
    } else {
        // offset = 2;
    }

    // card_01
    macro_rules! push_card_itme {
        ($card_itme:expr,$name:expr,$path:expr,$ouput:expr) => {
            $card_itme
                .path
                .set_label(&get_form_slice_path($path.clone(), 20).unwrap_or($path.clone()));
            $card_itme.path.set_label_size(12);
            $card_itme
                .path
                .set_label_color(Color::from_rgb(186, 186, 186));
            $card_itme.path.resize(32, 18, 200, 18);
            $card_itme.pathStr = $path;

            $card_itme
                .ouput
                .set_label(&get_form_slice_path($ouput.clone(), 20).unwrap_or($ouput.clone()));
            $card_itme.ouput.set_label_size(12);
            $card_itme
                .ouput
                .set_label_color(Color::from_rgb(186, 186, 186));
            $card_itme.ouput.resize(32, 44, 200, 18);
            $card_itme.ouputStr = $ouput;

            $card_itme
                .name
                .set_label(&get_form_slice_path($name.clone(), 7).unwrap_or($name.clone()));
            $card_itme.name.set_label_size(15);
            $card_itme
                .name
                .set_label_color(Color::from_rgb(255, 255, 255));
            $card_itme.name.resize(32, 80, 200, 22);
            $card_itme.nameStr = $name;

            $card_itme.status_ok.show();
            $card_itme.status_err.hide();
            $card_itme.remove = false;
        };

        ($card_id:expr,$itme_id:expr) => {
            if $card_id == 1 {
                push_card_itme!(
                    card_01,
                    export_dir_path_list[$itme_id].name.clone().to_owned(),
                    export_dir_path_list[$itme_id].path.clone().to_owned(),
                    export_dir_path_list[$itme_id].ouput.clone().to_owned()
                );
            }
            if $card_id == 2 {
                push_card_itme!(
                    card_02,
                    export_dir_path_list[$itme_id].name.clone().to_owned(),
                    export_dir_path_list[$itme_id].path.clone().to_owned(),
                    export_dir_path_list[$itme_id].ouput.clone().to_owned()
                );
            }
            if $card_id == 3 {
                push_card_itme!(
                    card_03,
                    export_dir_path_list[$itme_id].name.clone().to_owned(),
                    export_dir_path_list[$itme_id].path.clone().to_owned(),
                    export_dir_path_list[$itme_id].ouput.clone().to_owned()
                );
            }
        };

        ($card_id:expr,$itme_list:expr,$itme_id:expr) => {
            if $card_id == 1 {
                push_card_itme!(
                    card_01,
                    $itme_list[$itme_id].name.clone().to_owned(),
                    $itme_list[$itme_id].path.clone().to_owned(),
                    $itme_list[$itme_id].ouput.clone().to_owned()
                );
            }
            if $card_id == 2 {
                push_card_itme!(
                    card_02,
                    $itme_list[$itme_id].name.clone().to_owned(),
                    $itme_list[$itme_id].path.clone().to_owned(),
                    $itme_list[$itme_id].ouput.clone().to_owned()
                );
            }
            if $card_id == 3 {
                push_card_itme!(
                    card_03,
                    $itme_list[$itme_id].name.clone().to_owned(),
                    $itme_list[$itme_id].path.clone().to_owned(),
                    $itme_list[$itme_id].ouput.clone().to_owned()
                );
            }
        };
    }

    win.clone().center_screen();
    win.end();
    win.show();
    let mut num_items_to_take = 3;

    win.handle({
        let mut x = 0;
        let mut y = 0;
        let mut point_exist_hasmap = getFormPointSpace(x, y);
        let mut has_show = false;
        // let mut move_
        move |win, ev| match ev {
            enums::Event::Show => {
                win.set_visible_focus();
                libWxIkunPlus::setwinVisible(win.raw_handle() as i128 , true);
                // libWxIkunPlus::setWinIcon(win.raw_handle() as i128);

                println!("hwnd -> :  {}",win.raw_handle() as i128);
                true
            }

            enums::Event::Push => {
                // 关闭按钮
                if (point_exist_hasmap.quit) {
                    let mut hwnd = win.raw_handle() as i128;
                    win.clear();
                    global_var::set_bool("gui::open::manage_item", false);
                    libWxIkunPlus::setwinVisible(hwnd, false);                  
                    libWxIkunPlus::closeWindow(hwnd, true);
                    fltk::window::Window::delete(win.clone());
                    // fltk::window::Window::
                    println!("closeWindow hwnd -> :  {}",hwnd);
                }

                macro_rules! handle_point_card {
                    ($rm_card_id:expr, $card_itme:expr) => {
                        if ($rm_card_id && !$card_itme.remove) {
                          
                      
                            let mut path_string = $card_itme.pathStr.clone();
                            let conn: Connection =
                                Connection::open("ikun_user_data.db").expect("无法 创建/打开 数据库");
                            handle_dat::initialize_table(&conn);
                            match conn.execute(
                                "DELETE FROM export_dir_path WHERE path = ?1",
                                [path_string.clone()],
                            ) {
                                Ok(updated) => {
                                    $card_itme.status_err.show();
                                    $card_itme.status_ok.hide();
                                    // println!(
                                    //     "[移除] 移除的文件夹是: 名称：{}  路径：{}",
                                    //     $card_itme.nameStr,
                                    //     $card_itme.path.label()
                                    // );
                                    $card_itme.remove = true;
                                    console_log!(format!(
                                        "[移除] 移除的文件夹是: 名称：{}  路径：{}",
                                        $card_itme.nameStr,
                                        $card_itme.path.label()
                                    ));

                                    export_dir_path_list.clear();
                                    for cat in global_var_util::update_export_dir_itme_list() {
                                        export_dir_path_list.push(cat);
                                    }

                                }
                                Err(err) => {
                                    // $card_itme.status_err.show();
                                    // $card_itme.status_ok.hide();
                                    // println!(
                                        // "[移除失败] 移除的文件夹是: 名称：{}  路径：{}",
                                        // $card_itme.nameStr,
                                        // $card_itme.path.label()
                                    // );
                                    $card_itme.remove = true;
                                    // handle_dat::push_console_message(format!(
                                    //     "[移除失败] 移除的文件夹是: 名称：{}  路径：{}",
                                    //     $card_itme.nameStr,
                                    //     $card_itme.path.label()
                                    // ));
                                    console_log!(format!(
                                        "[移除失败] 移除的文件夹是: 名称：{}  路径：{}",
                                        $card_itme.nameStr,
                                        $card_itme.path.label()
                                    ));

                                }
                            }
            
                            let _ = conn.close();

                            
                        }
                    };
                }
                
                handle_point_card!(point_exist_hasmap.rm_card_01, card_01);
                handle_point_card!(point_exist_hasmap.rm_card_02, card_02);
                handle_point_card!(point_exist_hasmap.rm_card_03, card_03);
            
                if (point_exist_hasmap.next){
                    let mut start_index = 0;
                  
                    let mut end_index = offset+3;

                    // 偏移量不能超出数组
                    if(end_index >= export_dir_path_list.len()){
                        end_index = export_dir_path_list.len();
                    }
                    start_index = offset;
                    offset= end_index ;

                    if(start_index==end_index){
                        start_index= end_index-3;
                        if(start_index>9999999){
                            start_index=0;
                        }
                    }
                    let mut index = 0;
                    loop{
                        let mut indexof = start_index+index;
                        if(indexof>export_dir_path_list.len()-1){
                            break;
                        }

                        push_card_itme!(index+1,indexof);

                        if(index>=3){
                            card_03.main.clone().show();
                            break;
                        }
                        
                        index = index+1;
                       
                    }

                    println!("next-> {} -> {}",start_index,end_index);

                }
                if (point_exist_hasmap.backup){
                    // let start_index = if offset >= 3 { offset - 3 } else { 0 };
                    // let end_index = if offset + 3 < get_item.len() { offset + 3 } else { get_item.len() };
                
                    let mut start_index = 0;
            
                    
                    let mut end_index = offset-3;
                    
                    // 这里有个usize溢出问题 负数会巨大
                    if(end_index<999999){
                        end_index = 3;
                    }
                    
                    // end 不低于三个 除非没有那么多
                    if(end_index<3){
                        end_index=if export_dir_path_list.len()<=3 {export_dir_path_list.len()} else {3};
                    }

                    // 偏移量不能超出数组
                    if(end_index >= export_dir_path_list.len()){
                        end_index = export_dir_path_list.len();
                    }

                    if(end_index-start_index>3){
                        start_index= start_index-3;
                    }

                    if(start_index<999999){
                        start_index=0;
                    }
                    
                    let mut index = 0;
                    loop{
                        let mut indexof = start_index+index;
                            if(indexof>export_dir_path_list.len()-1){
                            break;
                        }
                        push_card_itme!(index+1,indexof);
                        if(index>=3){
                            card_03.main.clone().show();
                            break;
                        }
                        index = index+1;
                    }
                    
                    offset= end_index ;

                    println!("backup-> {} -> {}",start_index,end_index);

                }

                true
            }

            enums::Event::Move => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;
                point_exist_hasmap = getFormPointSpace(x, y);
                // -处理鼠标图标的逻辑

                if (point_exist_hasmap.existCursor) {
                    win.clone().set_cursor(Cursor::Hand);
                } else {
                    win.clone().set_cursor(Cursor::Default);
                }

                true
            }

            // enums::Event:
            enums::Event::Drag => {
                if (y < 69) {
                    win.clone()
                        .set_pos(app::event_x_root() - x, app::event_y_root() - y);
                }

                true
            }
            _ => false,
        }
    });

    // libWxIkunPlus::setWinIcon(win.raw_handle() as i128);
    // libWxIkunPlus::setWinTop(win.raw_handle() as i128, true);
}
#![allow(warnings, unused)]

use crate::{
    get_arc_bind_variable, global_var, gui_detect_config, gui_drag_scan, gui_hotspot, gui_imge,
    gui_text_control, handle_dat, libWxIkunPlus::{self, getFocusTopWindow}, read_rw_lazy_lock, read_rw_lock,
    set_arc_bind_variable, set_arc_bind_variable_insert,
    util::{str_eq_str, Sleep},
    wh_mod::{self, AttachThumbnail},
    write_rw_lock, write_rw_lock_insert,
};
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
use once_cell::sync::Lazy;
use rusqlite::Connection;
use std::sync::{mpsc, MutexGuard};
use std::time::{SystemTime, UNIX_EPOCH};

use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::ops::Deref;
use std::ptr::null;
use std::sync::{Arc, Condvar, Mutex, RwLock};
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
use arc_swap::ArcSwap;
use fltk::draw::{height, width};
use fltk::image::PngImage;
// use lazy_static::lazy_static;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::OnceLock;
use winapi::um::winnt::LPWSTR;
use winapi::um::winuser::{CloseWindow, SetActiveWindow};

static REQUEST_RECV: AtomicUsize = AtomicUsize::new(0);
// static mut static_var:Vec<ImgPreview> = Vec::new();
// static mut static_atomic :AtomicUsize = AtomicUsize::new(0);

// 使用原子锁代替线程锁 避免死锁

// 图片预览 全局变量
static mut IMG_PREVIEW_LIST: Vec<ImgPreview> = Vec::new();
static IMG_PREVIEW_LIST_BIND: AtomicUsize = AtomicUsize::new(0);

// 缩略图 全局变量
static mut THUMBNAIL_LIST: Vec<wh_mod::AttachThumbnail> = Vec::new();
static THUMBNAIL_LIST_BIND: AtomicUsize = AtomicUsize::new(0);

// wxid 全局变量
static mut WX_ID: String = String::new();
static WX_ID_BIND: AtomicUsize = AtomicUsize::new(0);

// 用户wx存储位置 全局变量
static mut USER_PATH: String=String::new();
static USER_PATH_BIND: AtomicUsize = AtomicUsize::new(0);

// 使用原子锁代替线程锁 避免死锁
// fn set_IMG_PREVIEW_LIST(value:Vec<ImgPreview>){

//     set_arc_bind_variable!(THUMBNAIL_LIST,THUMBNAIL_LIST_BIND,value);

//     get_arc_bind_variable!(IMG_PREVIEW_LIST,IMG_PREVIEW_LIST_BIND);

// }

// thread_local! {
// // static WX_ID_ARC: RwLock<Arc<String>> = RwLock::new(Arc::new(String::new()));
// static USER_PATH_ARC: RwLock<Arc<String>> = RwLock::new(Arc::new(String::new()));
// // static THUMBNAIL_LIST_ARC: RwLock<Arc<Vec<wh_mod::AttachThumbnail>>> = RwLock::new(Arc::new(Vec::new()));
// // static IMG_PREVIEW_LIST_ARC: RwLock<Arc<Vec<ImgPreview>>> = RwLock::new(Arc::new(Vec::new()));


// }

// thread_local! {
//         static IMG_PREVIEW_LIST_ARCLAZY: ArcSwap< Vec<ImgPreview> > = ArcSwap::from_pointee(Vec::new().into());
// }

// 设置背景为图片（主视图）
fn setInterfaceBackgroundImage(appMainWin: &mut window::DoubleWindow) -> Frame {
    let background_image =
        // image::SvgImage::from_data(include_str!("./assets/select_user_base/main.svg"))
        image::PngImage::from_data(include_bytes!("./assets/select_user_base/main.png"))
            .expect("set main icon error");
    let mut frame = Frame::default().with_size(600, 0).center_of(appMainWin);
    frame.set_frame(FrameType::EngravedBox);
    frame.set_image(Some(background_image));

    return frame;
}

// 按钮基本都是以坐标判断的
struct PointExistHasmap {
    // 显示手型
    existCursor: bool,
}

// 判断鼠标坐标是否在此元素内
fn getFormPointSpace(x: i32, y: i32) -> PointExistHasmap {
    // 输入: x,y,w,y
    macro_rules! check_point_in_space {
        ($xx:expr, $yy:expr, $width:expr, $height:expr) => {
            x > $xx && x < $xx + $width && y > $yy && y < $yy + $height
        };
    }
    let existCursor = false;
    // IMG_PREVIEW_LIST_KEEP;
    let mut point_exist_hasmap = PointExistHasmap { existCursor: false };

    point_exist_hasmap.existCursor = existCursor;

    return point_exist_hasmap;
}

pub struct SelectUserBaseMain {
    // 选择的路径
    pub select_path: Vec<String>,
    // pub win: DoubleWindow,
    pub thumbnail: bool,
    // 是否导出原图
    pub source: bool,
    // 是否仅限本月
    pub the_month: bool,
    // 数据库位置
    pub database: String,
}

impl Clone for SelectUserBaseMain {
    fn clone(&self) -> Self {
        SelectUserBaseMain {
            select_path: self.select_path.clone(),
            // win:self.win.clone(),
            thumbnail: self.thumbnail.clone(),
            source: self.source.clone(),
            the_month: self.the_month.clone(),
            database: self.database.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}

struct UpdateJpgItme {
    index: usize,
    jpg: image::JpegImage,
    path: String,
}

struct PicturePreviewItem {
    // pub main_id:String,
    pub picture_id: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl PicturePreviewItem {
    pub fn get_picture(&self) -> Frame {
        let mut frame: Frame = app::widget_from_id(self.picture_id.as_str()).unwrap();
        frame
    }

    // 判断鼠标是否在当前元素
    pub fn existPoint(&self, x: i32, y: i32) -> bool {
        return x > self.x.clone()
            && x < self.x.clone() + self.width.clone()
            && y > self.y.clone()
            && y < self.y.clone() + self.height.clone();
    }
}

fn get_next_id() -> usize {
    let mut id: usize = 0;
    let mutex = Arc::new(Mutex::new(&REQUEST_RECV));
    mutex.lock();
    id = REQUEST_RECV.fetch_add(1, Ordering::Relaxed);
    drop(mutex);
    println!("id-> {}", id);
    id
}

fn push_wx_user_table(select_path: String, user_name: String) {
    // let mut lazy_value = USER_PATH.lock().unwrap();

    // if lazy_value.contains(select_path.as_str()) {
    //     return;
    // }

    // *lazy_value = select_path.clone();
    // drop(lazy_value);
    // if read_rw_lock!(USER_PATH_ARC, String::new()).contains(select_path.as_str()) {
    //     return;
    // }
    if(get_arc_bind_variable!(USER_PATH,USER_PATH_BIND).contains(&select_path)){
        return;
    }
    // write_rw_lock!(USER_PATH_ARC, select_path.clone());
    set_arc_bind_variable!(USER_PATH,USER_PATH_BIND,select_path.clone());

    
    thread::spawn(|| {
        let conn: Connection = Connection::open("ikun_user_data.db").unwrap();
        handle_dat::initialize_table(&conn);
        match conn.execute(
            "DELETE FROM user_wx_root_history WHERE path = ?1",
            [select_path.clone()],
        ) {
            Ok(updated) => {}
            Err(err) => {}
        }

        match conn.execute(
            "INSERT INTO user_wx_root_history (time,path,name) values (?1, ?2, ?3)",
            [
                Local::now().format("%Y-%m-%d").to_string(),
                select_path,
                user_name,
            ],
        ) {
            Ok(_) => {}
            Err(err) => {}
        }

        conn.close();
    });
}
struct UserWxRootHistory {
    time: String,
    path: String,
    name: String,
}
fn get_wx_user_history_path() -> Result<UserWxRootHistory, rusqlite::Error> {
    let mut user_wx_root_history = UserWxRootHistory {
        time: "".to_string(),
        path: "".to_string(),
        name: "".to_string(),
    };

    let conn: Connection = Connection::open("ikun_user_data.db").unwrap();
    handle_dat::initialize_table(&conn);
    if let Ok(mut stmt) =
        conn.prepare("SELECT time,name,path  FROM user_wx_root_history ORDER BY time DESC LIMIT 1")
    {
        let cats = stmt.query_map([], |row| {
            Ok(UserWxRootHistory {
                time: row.get(0)?,
                name: row.get(1)?,
                path: row.get(2)?,
            })
        })?;

        for cat in cats {
            let cat = cat?;
            user_wx_root_history.path = cat.path;
            user_wx_root_history.name = cat.name;
            user_wx_root_history.time = cat.time;
        }
    }

    conn.close();
    Ok(user_wx_root_history)
}

fn update_preview_main() {
    // println!(
    //     "[preview_main] IMG_PREVIEW_LIST_ARC-> {}",
    //     read_rw_lock!(IMG_PREVIEW_LIST_ARC, Vec::new()).len()
    // );
    // // println!(
    // //     "[preview_main] IMG_PREVIEW_LIST_ARCSWAP-> {}",
    // //     IMG_PREVIEW_LIST_ARCLAZY.with().len()
    // // );

    // println!(
    //     "[preview_main] IMG_PREVIEW_LIST_ARCSWAP-> {}",
    //     read_rw_lazy_lock!(IMG_PREVIEW_LIST_ARCLAZY).len()
    // );

    // println!(
    //     "[preview_main]THUMBNAIL_LIST-> {}",
    //     get_arc_bind_variable!(IMG_PREVIEW_LIST,IMG_PREVIEW_LIST_BIND).len()
    // );

    println!(
        "[preview-main]IMG_PREVIEW_LIST-> {}",
        get_arc_bind_variable!(IMG_PREVIEW_LIST, IMG_PREVIEW_LIST_BIND).len()
    );
    println!(
        "[preview-main]THUMBNAIL_LIST-> {}",
        get_arc_bind_variable!(THUMBNAIL_LIST, THUMBNAIL_LIST_BIND).len()
    );

    // 取出缩略图列表 并将其缩减到5条以内
    let mut thumbnail_list = {
        // let mut thumbnail_list = read_rw_lock!(THUMBNAIL_LIST_ARC, Vec::new()).to_vec();
        let mut thumbnail_list =
            get_arc_bind_variable!(THUMBNAIL_LIST, THUMBNAIL_LIST_BIND).to_vec();
        let mut atid_list: HashMap<String, AttachThumbnail> = HashMap::new();

        for value in thumbnail_list {
            let key = value.attach_id.clone();
            let mut oid_created = UNIX_EPOCH;
            let mut new_created = UNIX_EPOCH;

            // oid create time
            if let Some(thumbnail) = atid_list.get(&key) {
                if let Ok(metadata) = fs::metadata(thumbnail.thumbnail_path.clone()) {
                    if let Result::Ok(create) = metadata.created() {
                        oid_created = create;
                    }
                }
            }

            // new create time
            if let Ok(metadata) = fs::metadata(value.thumbnail_path.clone()) {
                if let Result::Ok(create) = metadata.created() {
                    new_created = create;
                }
            }

            // 按照创建时间判断是否更新视图
            if (new_created > oid_created) {
                atid_list.insert(value.attach_id.clone(), value);
            }
        }

        println!("atid_list size -> {}", atid_list.len());

        let mut thumbnail_list: Vec<AttachThumbnail> = Vec::new();

        for (key, value) in atid_list {
            thumbnail_list.push(value);
        }

        thumbnail_list.sort_by(|a, b| {
            let mut a_created = UNIX_EPOCH;
            let mut b_created = UNIX_EPOCH;

            if let Ok(metadata) = fs::metadata(a.thumbnail_path.clone()) {
                if let Result::Ok(create) = metadata.created() {
                    a_created = create;
                }
            }

            if let Ok(metadata) = fs::metadata(b.thumbnail_path.clone()) {
                if let Result::Ok(create) = metadata.created() {
                    b_created = create;
                }
            }

            a_created.cmp(&b_created)
        });

        let mut new_thumbnail_list = Vec::new();

        thumbnail_list.reverse();
        for value in thumbnail_list {
            if (new_thumbnail_list.len() > 5 - 1) {
                break;
            }
            new_thumbnail_list.push(value);
        }

        new_thumbnail_list
    };

    // println!("[328] thumbnail_list-> {}",thumbnail_list.len());

    // write_rw_lock!(THUMBNAIL_LIST_ARC, thumbnail_list.to_vec());
    set_arc_bind_variable!(THUMBNAIL_LIST, THUMBNAIL_LIST_BIND, thumbnail_list.to_vec());

    // println!(
    //     "[preview_main_end] IMG_PREVIEW_LIST_ARC-> {}",
    //     read_rw_lock!(IMG_PREVIEW_LIST_ARC, Vec::new()).len()
    // );
    // println!(
    //     "[preview_main_end] IMG_PREVIEW_LIST_ARCSWAP-> {}",
    //     read_rw_lazy_lock!(IMG_PREVIEW_LIST_ARCLAZY).len()
    // );
    // println!(
    //     "[preview_main_end]THUMBNAIL_LIST-> {}",
    //     get_arc_bind_variable!(IMG_PREVIEW_LIST,IMG_PREVIEW_LIST_BIND).len()
    // );

    // 更新到视图中
    let thumbnail_list = get_arc_bind_variable!(THUMBNAIL_LIST, THUMBNAIL_LIST_BIND);

    // 锁定缩略图更新
    let mutex = Arc::new(Mutex::new(&THUMBNAIL_LIST_BIND));
    mutex.lock();

    let img_preview_list = get_arc_bind_variable!(IMG_PREVIEW_LIST, IMG_PREVIEW_LIST_BIND);

    let (width, height) = (75, 75);

    for index in 0..img_preview_list.len() {
        if let Some(mut img_preview) = img_preview_list.get(index) {
            if let Some(thumbnail) = thumbnail_list.get(index) {
                img_preview.clone().from_data(
                    thumbnail.thumbnail.clone(),
                    -1,
                    -1,
                    width - 2,
                    height - 2,
                );
            } else {
            }
        }
    }
    drop(mutex);
}

// 开始获取更新
fn initialize_watch_path_puppet(path: String) {
    let mut frame: Frame = app::widget_from_id("watch_path_user_name").unwrap();
    let mut btn_next: Frame = app::widget_from_id("gui::btn_text_recent_pictures").unwrap();
    let copy_path = path.clone();
    let watch_path = path.clone();
    // println!("initialize_watch_path_puppet-> {}",path.clone());

    let path_list = wh_mod::sort_modified_dir(path.clone().as_ref());
    let mut has_wx = false;
    let mut match_wxid_len = 0;
    let mut wxid = String::new();
    for path in path_list.clone() {
        if (path.to_string_lossy().contains("wxid_")) {
            match_wxid_len = match_wxid_len + 1;
        }

        if (path.to_string_lossy().contains("wxid_")
            || path.to_string_lossy().contains("Applet")
            || path.to_string_lossy().contains("All Users"))
        {
            has_wx = true;
        }
    }

    if (!has_wx) {
        dialog::alert_default("此路径不是有效的wx的根目录");
        return;
    }

    if (match_wxid_len >= 1) {
        for for_path in path_list.clone() {
            let file_name = for_path
                .file_name()
                .unwrap_or_else(|| "".as_ref())
                .to_string_lossy()
                .to_string();

            if file_name.contains("wxid_") {
                wxid = file_name.clone();
                // let mut lazy_value = WX_ID.lock().unwrap();
                // *lazy_value = file_name.clone();
                // drop(lazy_value);

                // write_rw_lock!(WX_ID_ARC, file_name.clone());
                set_arc_bind_variable!(WX_ID,WX_ID_BIND,file_name.clone());

                // let get_wxid_acc = wh_mod::convert::get_wxid_name(format!("{}",for_path.to_str()),wxid.clone());
                if let Some(get_wxid_acc) = wh_mod::convert::get_wxid_name(
                    format!("{}", for_path.to_str().unwrap_or_else(|| &"")),
                    wxid.clone(),
                ) {
                    frame.set_label(&format!("{}  [ {} ]", file_name, get_wxid_acc.name));
                } else {
                    frame.set_label(
                        format!("{}  [ {} ]", file_name, wh_mod::wx_account_id(for_path).id)
                            .as_str(),
                    );
                };
                // 显示到ui
                // frame.set_label(
                //     format!("{}  [ {} ]", file_name,get_wxid_acc /*wh_mod::wx_account_id(for_path).id).as_str()*/,
                // );
                frame.redraw();
                btn_next.set_label("检测");
                // btn_next.redraw();
                // let attach_path = Path::new(copy_path.as_str());
                push_wx_user_table(path.clone(), file_name);
                global_var::set_string("user::config::user_select_wxid", wxid.clone());

                let copy_path = format!("{}/{}/FileStorage/MsgAttach", copy_path.as_str(), wxid);
                let copy_path_wake = format!("{}", watch_path);

                // 取得缩略图
                thread::spawn(move || {
                    // 扫描最近文件夹
                    let path = Path::new(copy_path.as_str());
                    let imag = wh_mod::read_attach_buff_thumbnail_list(path, 5, 1);

                    let mut data_list = Vec::new();

                    //
                    for imag in imag {
                        println!("{}", imag.thumbnail_path.clone());
                        data_list.push(imag);
                    }
                    // *lazy_value = data_list.clone();
                    // drop(lazy_value);

                    // write_rw_lock_insert!(THUMBNAIL_LIST_ARC, data_list.to_vec());
                    // let mut oid_thumbnail_list = get_arc_bind_variable!(THUMBNAIL_LIST,THUMBNAIL_LIST_BIND).to_vec();

                    // for value in data_list {
                    //     oid_thumbnail_list.push(value);
                    // }

                    set_arc_bind_variable_insert!(
                        THUMBNAIL_LIST,
                        THUMBNAIL_LIST_BIND,
                        data_list.to_vec()
                    );

                    // println!("read_rw_lock!(THUMBNAIL_LIST_ARC).unwrap().len() ->  {}",read_rw_lock!(THUMBNAIL_LIST_ARC).unwrap().len());

                    if (data_list.len() > 0) {
                        update_preview_main();
                    }

                    // 释放 并已更新

                    // 启动日志检测模式
                    let (tx, rx) = std::sync::mpsc::channel();

                    let wh_id = wh_mod::watch_path::watch_path_puppet(copy_path_wake.clone(), tx);
                    println!("copy_path_wake-> {}", copy_path_wake.clone());
                    while wh_id == wh_mod::watch_path::get_the_id() {
                        if let Result::Ok(data) = rx.recv() {
                            let path = data.join("..").join("..").join("..");
                            let data_list = wh_mod::read_attach_buff_thumbnail_data(&path, 1);
                            // write_rw_lock_insert!(THUMBNAIL_LIST_ARC, data_list.to_vec());
                            set_arc_bind_variable_insert!(
                                THUMBNAIL_LIST,
                                THUMBNAIL_LIST_BIND,
                                data_list.to_vec()
                            );

                            if (data_list.len() > 0) {
                                update_preview_main();
                            }
                        }
                    }
                    // wh_mod::watch_path
                });

                break;
            }
        }
    } else {
        frame.set_label("存在多个用户 请手动发送图片确认...");
        frame.redraw();
        btn_next.set_label("刷新");
        btn_next.redraw();
    }

    // frame.set_label("开始扫描中...");
}

fn wx_ready_initialize_open_preview_main_up(path: String) {
    let path_list = wh_mod::sort_modified_dir(path.clone().as_ref());

    for path in path_list {
        println!("{:?}", path);
        // let path_list = sort_modified_dir(path.as_ref());
        //
        // for path in path_list {
        //
        // }
    }
}

// 创建选择的窗口
pub fn mian_window() -> SelectUserBaseMain {
    if (global_var::get_bool_default("gui::open::handle_dat")) {
        if let Some(mut wins) =
            app::widget_from_id("gui::DoubleWindow::handle_dat::main") as Option<DoubleWindow>
        {
            wins.show();
            wins.set_visible_focus();
        }
        let mut select_user_base: SelectUserBaseMain = SelectUserBaseMain {
            // win:DoubleWindow::new(0, 0, 600, 531, "Ikun导出"),
            select_path: Vec::new(),
            thumbnail: false,
            source: false,
            the_month: false,
            database: String::new(),
        };

        return select_user_base;
    }

    global_var::set_bool("gui::open::handle_dat", true);

    // 设置主题
    let theme = ColorTheme::new(color_themes::BLACK_THEME);
    let widget_theme = WidgetTheme::new(ThemeType::HighContrast);
    widget_theme.apply();
    let widget_scheme = WidgetScheme::new(SchemeType::Aqua);
    widget_scheme.apply();
    theme.apply();

    let mut select_user_base: SelectUserBaseMain = SelectUserBaseMain {
        // win:DoubleWindow::new(0, 0, 600, 531, "Ikun导出"),
        select_path: Vec::new(),
        thumbnail: false,
        source: false,
        the_month: false,
        database: String::new(),
    };

    let mut win: DoubleWindow = DoubleWindow::new(0, 0, 600, 470, "任务创建向导").center_screen();
    win.set_color(Color::from_rgb(24, 24, 24));
    win.set_border(false);
    win.set_id("gui::DoubleWindow::handle_dat::main");

    // fltk::app::set_scrollbar_size(3);
    // fltk::app::set_selection_color(24, 24, 24);
    setInterfaceBackgroundImage(&mut win);
    // 背景已经绘制

    let mut btn_exit_imag = gui_imge::create_Images(
        &mut win,
        540,
        15,
        37,
        37,
        image::PngImage::from_data(include_bytes!("./assets/select_user_base/exit.png"))
            .expect("set exit_imag_btn icon error"),
        "btn_exit_select".to_owned(),
    );

    let mut flex = group::Flex::default()
        .with_size(395 + 60, 30)
        .row()
        .center_of_parent();

    flex.set_pos(75, 396);

    macro_rules! show_cursor {
        ($show_cursor_itme:expr) => {
            let mut new_win = win.clone();
            $show_cursor_itme.handle({
                move |win, ev| match ev {
                    enums::Event::Move => {
                        new_win.set_cursor(Cursor::Hand);
                        true
                    }
                    enums::Event::Leave => {
                        new_win.set_cursor(Cursor::Default);
                        true
                    }
                    _ => false,
                }
            });
        };
    }

    let mut check_button_thumbnail = button::CheckButton::default().with_label("保存缩略图");
    let mut check_button_source = button::CheckButton::default().with_label("保存原图(如果有)");
    let mut check_button_the_month = button::CheckButton::default().with_label("只保存本月图片");

    check_button_thumbnail.set_callback(|win| {
        global_var::set_bool("user::config::check_button_thumbnail", win.is_checked());
    });

    check_button_source.set_callback(|win| {
        global_var::set_bool("user::config::check_button_source", win.is_checked());
    });

    check_button_the_month.set_callback(|win| {
        global_var::set_bool("user::config::check_button_the_month", win.is_checked());
    });

    show_cursor!(check_button_thumbnail);
    show_cursor!(check_button_source);
    show_cursor!(check_button_the_month);

    flex.end();

    let mut user_name = Frame::new(59, 207, 269, 37, "尚未找到更新图片的用户");
    user_name.set_id("watch_path_user_name");
    user_name.set_label_size(12);

    let mut text_fall_pictures = Frame::new(364, 218, 85, 15, "通过拽入获取");
    text_fall_pictures.set_label_size(12);

    let mut text_recent_pictures = Frame::new(495, 218, 30, 15, "扫描");
    text_recent_pictures.set_label_size(12);
    text_recent_pictures.set_id("gui::btn_text_recent_pictures");

    let mut text_title01 = Frame::new(30, 96, 190, 21, "请选择 WX文件的保存位置 *");
    text_title01.set_label_size(12);
    text_title01.set_label_color(Color::from_rgb(105, 105, 105));

    let mut text_title02 = Frame::new(
        6,
        96 + 70,
        490,
        21,
        "选择用户*    [ 涉及用户数据安全   仅限已登录中的用户 并且能够收发的用户 ]",
    );
    text_title02.set_label_size(12);
    text_title02.set_label_color(Color::from_rgb(105, 105, 105));

    let mut text_title03 = Frame::new(
        6 + 50,
        96 + 70 + 90,
        490,
        21,
        "选择最近对象*  （ 如果不存在请随意发送一张的图片给对方 [不能是表情]  更新后约5秒显示 ） ",
    );
    text_title03.set_label_size(12);
    text_title03.set_label_color(Color::from_rgb(105, 105, 105));
    text_title03.set_id("gui::gui_select_user_base::text_title03");

    thread::spawn(move || {
        let mut title = text_title03.clone();
        while global_var::get_bool_default("gui::open::handle_dat") {
            // if let Some (mut title) = app::widget_from_id("gui::gui_select_user_base::text_title03") as  Option<Frame> {
            let data = global_var::get_string_default("user::config::user_select_path");
            let id = global_var::get_i32_default("user::config::select_user_thumbnail_obj");

            if (data.is_empty()) {
                title.set_label("选择最近对象*  （ 如果不存在请随意发送一张的图片给对方 [不能是表情]  更新后约5秒显示 ） ");
            } else {
                title.set_label(
                    format!(
                        "已选定 ：[ {} ] {}  [再次点击取消]",
                        (if id == -2 {
                            "拖拽".to_string()
                        } else {
                            id.to_string()
                        }),
                        data
                    )
                    .as_str(),
                );
            }

            title.resize(6 + 50, 96 + 70 + 90, 490, 21);

            // }
            Sleep(500);
        }
        println!("gui_select_user_base 线程退出");
    });
    // 刷新最近图片
    let mut btn_recent_pictures = gui_hotspot::create_hotspot(480, 204, 62, 39);
    // 通过拖拽确认用户
    let mut btn_fall_pictures_path = gui_hotspot::create_hotspot(334, 203, 125, 42);
    // 打开微信文件所在位置
    let mut btn_open_select_dir = gui_hotspot::create_hotspot(515, 123, 39, 35);
    // 输入文件夹路径
    let mut input_select_dir = input::Input::new(50, 127, 450 - 5, 27, "");
    input_select_dir.set_id("gui::input_select_dir");

    if let Ok(history) = get_wx_user_history_path() {
        let paths = history.path;
        input_select_dir.set_value(paths.as_str());
        global_var::set_string("user::config::input_select_dir", paths);

        // initialize_watch_path_puppet(paths);
    }
    if (input_select_dir.value().is_empty()) {
        if let Some(paths) = wh_mod::convert::get_user_data_path() {
            input_select_dir.set_value(paths.as_str());
            global_var::set_string("user::config::input_select_dir", paths);
        }
    }

    // 输入文件夹路径 的热区
    let mut input_select_dir_hotspot = gui_hotspot::create_hotspot(50, 127, 450 - 5, 27);

    // initialize_preview_main
    let mut preview_main_point_list = [
        [71, 296, 75, 75],
        [167, 296, 75, 75],
        [263, 296, 75, 75],
        [359, 296, 75, 75],
        [455, 296, 75, 75],
    ];
    let mut preview_main_hotspot_01 = gui_hotspot::create_hotspot(76, 303, 73, 73);
    let mut preview_main_hotspot_02 = gui_hotspot::create_hotspot(169, 303, 73, 73);
    let mut preview_main_hotspot_03 = gui_hotspot::create_hotspot(264, 303, 73, 73);
    let mut preview_main_hotspot_04 = gui_hotspot::create_hotspot(355, 303, 73, 73);
    let mut preview_main_hotspot_05 = gui_hotspot::create_hotspot(447, 303, 73, 73);

    let mut preview_main = Vec::new();
    for index in 0..preview_main_point_list.len() {
        let point = preview_main_point_list[index];
        let [x, y, width, height] = point;
        // let id = format!("gui::preview_main::index::{}",index);
        // let idc = id.as_str();
        let mut preview = ImgPreview::new(x, y, width, height, "gui::preview_main::index::");
        // -1,-1,width-2,height-2 每个边 向内缩进1像素 使其有1px描边的效果
        preview.from_data(
            include_bytes!("./assets/select_user_base/not.png").to_vec(),
            -1,
            -1,
            width - 2,
            height - 2,
        );
        preview_main.push(preview);
    }
    let mut preview = ImgPreview::new(455, 296, 75, 75, "gui::preview_main::index::user_select");
    preview.preview.hide();
    preview_main.push(preview);

    println!("preview_main size-> {}", preview_main.len());

    // // 写入到全局变量中
    // write_rw_lock!(IMG_PREVIEW_LIST_ARC, preview_main.to_vec());
    // // write_rw_lock!(IMG_PREVIEW_LIST_ARCLAZY,preview_main.to_vec().into());

    // // *IMG_PREVIEW_LIST_ARCLAZY.write().unwrap() = preview_main.to_vec().into();
    // // IMG_PREVIEW_LIST_ARCLAZY.store(preview_main.to_vec().into());

    set_arc_bind_variable!(
        IMG_PREVIEW_LIST,
        IMG_PREVIEW_LIST_BIND,
        preview_main.to_vec()
    );

    // // get_arc_bind_variable!(IMG_PREVIEW_LIST,IMG_PREVIEW_LIST_BIND);
    // println!(
    //     "[win-main]IMG_PREVIEW_LIST_ARC size-> {}",
    //     read_rw_lock!(IMG_PREVIEW_LIST_ARC, Vec::new()).len()
    // );
    // println!(
    //     "[win-main]IMG_PREVIEW_LIST_ARCSWAP-> {}",
    //     read_rw_lazy_lock!(IMG_PREVIEW_LIST_ARCLAZY).len()
    // );
    // println!(
    //     "[win-main]THUMBNAIL_LIST-> {}",
    //     get_arc_bind_variable!(IMG_PREVIEW_LIST,IMG_PREVIEW_LIST_BIND).len()
    // );

    println!(
        "[win-main]IMG_PREVIEW_LIST-> {}",
        get_arc_bind_variable!(IMG_PREVIEW_LIST, IMG_PREVIEW_LIST_BIND).len()
    );
    println!(
        "[win-main]THUMBNAIL_LIST-> {}",
        get_arc_bind_variable!(THUMBNAIL_LIST, THUMBNAIL_LIST_BIND).len()
    );

    let mut preview_tips = ImgPreview::new(165, 18, 27, 27, "gui::preview_main::next::tips");
    preview_tips.from_data(
        include_bytes!("./assets/select_user_base/tips.png").to_vec(),
        0,
        0,
        27,
        27,
    );

    let mut title_tips =
        gui_text_control::TextControl::new(46, 18, 330, 27, 12, "帮助".to_string(), [84, 84, 84]);
    // end

    win.end();
    win.show();

    win.handle({
        let mut x = 0;
        let mut y = 0;
        let mut point_exist_hasmap = getFormPointSpace(x, y);
        let mut has_show = false;
        let mut hwnd = 0;
        // let mut move_
        move |win, ev| match ev {
            enums::Event::Show => {
                win.set_visible_focus();
                hwnd = win.raw_handle() as i128;
                libWxIkunPlus::setwinVisible(hwnd.clone(), true);
                println!("hwnd -> :  {}", &hwnd);
                true
            }

            enums::Event::Push => {
                // 处理 check 组件
                macro_rules! check_select_click {
                    ($check_select:expr) => {
                        $check_select.existPoint(x, y)
                        // check_select_source.set_value(global_var);
                    };
                }

                //  打开文件夹选择器
                if check_select_click!(btn_open_select_dir) {

                    let the_win = libWxIkunPlus::getFocusWindow();
                    
                    let user_select_path = libWxIkunPlus::openSelectFolder2();


                    if user_select_path.len() > 1 {
                        initialize_watch_path_puppet(user_select_path.clone());
                        user_name.set_label("开始扫描...");
                        input_select_dir
                            .clone()
                            .set_value(user_select_path.clone().as_str());
                        global_var::set_string(
                            "user::config::input_select_dir",
                            user_select_path.clone(),
                        );
                    }
                }

                if btn_recent_pictures.existPoint(x, y) {}

                // 刷新按钮
                if btn_recent_pictures.existPoint(x, y) {
                    if text_recent_pictures.label().contains("检测") {
                        if (check_button_the_month.is_checked()
                            || check_button_source.is_checked()
                            || check_button_thumbnail.is_checked())
                            && global_var::get_i32_or("user::config::select_user_thumbnail_obj",-1) != -1
                        {
                        } else {
                            gui_detect_config::main_window();
                            // title_tips.set_label("帮助:[未选存储内容]".to_string());
                        }
                    }
                }

                // 通过拖拽获取
                if (btn_fall_pictures_path.existPoint(x, y)) {
                    let value = input_select_dir.value();

                    if (value.len() > 3) {
                        match fs::metadata(Path::new(value.as_str())) {
                            Ok(meta) => {
                                user_name.set_label("开始扫描...");
                                initialize_watch_path_puppet(value.clone());
                            }
                            Err(err) => {
                                let choice = dialog::alert_default(&*format!(
                                    "输入的路径存在错误 错误-> {:?}",
                                    err
                                ));
                            }
                        }
                        gui_drag_scan::main_window();
                    } else {
                        gui_detect_config::main_window();
                        let choice = dialog::alert_default(&*format!(
                            "输入的路径存在错误 错误-> 没有选择WX路径"
                        ));
                    }
                }

                // 选择 或者刷新
                if (btn_recent_pictures.existPoint(x, y)
                    && !text_recent_pictures.label().contains("检测"))
                {
                    let value = input_select_dir.value();
                    global_var::set_string("user::config::input_select_dir", value.clone());
                    if (value.len() > 3) {
                        match fs::metadata(Path::new(value.as_str())) {
                            Ok(meta) => {
                                user_name.set_label("开始扫描...");
                                initialize_watch_path_puppet(value.clone());
                            }
                            Err(err) => {
                                let choice = dialog::alert_default(&*format!(
                                    "输入的路径存在错误 错误-> {:?}",
                                    err
                                ));
                            }
                        }
                    } else {
                        let choice = dialog::alert_default(&*format!("输入的路径过短或者不存在"));
                    }
                }

                // 关闭
                if (btn_exit_imag.existPoint(x, y)) {
                    // libWxIkunPlus::closeWindow(win.raw_handle() as i128, true);
                    // write_rw_lock!(THUMBNAIL_LIST_ARC, Vec::new());
                    // write_rw_lock!(IMG_PREVIEW_LIST_ARC,Vec::new());
                    set_arc_bind_variable!(THUMBNAIL_LIST, THUMBNAIL_LIST_BIND, Vec::new());
                    set_arc_bind_variable!(IMG_PREVIEW_LIST, IMG_PREVIEW_LIST_BIND, Vec::new());

                    // 终止更新检测
                    wh_mod::watch_path::un_next_exits();
                    global_var::set_i32("user::config::select_user_thumbnail_obj", -1);
                    global_var::set_string("user::config::user_select_path", String::new());
                    global_var::set_string("user::config::user_select_wxid", String::new());
                    global_var::set_bool("gui::open::handle_dat", false);
                    fltk::window::Window::delete(win.clone());

                }

                if preview_tips.existPoint(x, y) {
                    gui_detect_config::main_window();
                }

                macro_rules! select_user_preview {
                    ($select_user_preview:expr,$id:expr) => {
                        if ($select_user_preview.existPoint(x, y)) {
                            let select_id =
                                global_var::get_i32_or("user::config::select_user_thumbnail_obj",-1);

                            if (select_id == $id) {
                                global_var::set_string(
                                    "user::config::user_select_path",
                                    "".to_string(),
                                );
                                global_var::set_i32("user::config::select_user_thumbnail_obj", -1);
                            }

                            if (select_id != $id) {
                                let mut str_path = String::new();
                                let thumbnail_list = get_arc_bind_variable!(THUMBNAIL_LIST,THUMBNAIL_LIST_BIND);
                                
                                if let Some(item) = thumbnail_list.get($id - 1) {
                                    str_path = item.attach_id.clone();

                                    println!("[select_user_preview] -> {} [{}] ",&str_path,&$id);

                                    global_var::set_string("user::config::user_select_path", str_path);
                                    global_var::set_i32("user::config::select_user_thumbnail_obj", $id);
                                }
                            }
                        }
                    };
                }
                // app::grab().expect("msg")
                select_user_preview!(preview_main_hotspot_01, 1);
                select_user_preview!(preview_main_hotspot_02, 2);
                select_user_preview!(preview_main_hotspot_03, 3);
                select_user_preview!(preview_main_hotspot_04, 4);
                select_user_preview!(preview_main_hotspot_05, 5);

                true
            }

            enums::Event::Move => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;

                macro_rules! move_show_cursor {
                    ($check_select:expr) => {
                        $check_select.existPoint(x, y)
                    };
                }

                if move_show_cursor!(btn_exit_imag)
                    || move_show_cursor!(preview_main_hotspot_01)
                    || move_show_cursor!(preview_main_hotspot_02)
                    || move_show_cursor!(preview_main_hotspot_03)
                    || move_show_cursor!(preview_main_hotspot_04)
                    || move_show_cursor!(preview_main_hotspot_05)
                    || move_show_cursor!(btn_recent_pictures)
                    || move_show_cursor!(btn_fall_pictures_path)
                    || move_show_cursor!(btn_open_select_dir)
                    || preview_tips.existPoint(x, y)
                {
                    win.clone().set_cursor(Cursor::Hand);
                } else {
                    win.clone().set_cursor(Cursor::Default);
                }

                true
            }

            // enums::Event:
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

    win.set_visible_focus();

    select_user_base
}
use fltk::enums::Color;
use fltk::frame;
use fltk::prelude::{WidgetBase, WidgetExt};

pub struct TextControl {
    pub text: frame::Frame,
}

impl TextControl {
    // 创建
    pub fn new(x:i32, y:i32, width: i32, height: i32, size:i32, input:String, rgb: [u8; 3]) -> Self {
        let mut text = frame::Frame::new(x, y, (if width==0 {input.len()as i32 * size}else {width}), (if height==0 {size+2}else {height}), "");
        text.set_label(input.as_str());
        text.set_label_size(size);
        text.set_label_color(Color::from_rgb(rgb[0],rgb[1],rgb[2]));

        Self { text  }
    }
    // 获取文本内容
    pub fn get_label(&self)->String {
        return self.text.label();
    }
    // 设置文本内容
    pub fn set_label(&mut self,input:String){
        self.text.set_label(input.as_str());
        self.text.redraw_label();
        self.text.redraw();
    }
    // 设置颜色
    pub fn set_color(&mut self, r: u8, g: u8, b: u8)  {
        self.text.set_label_color(Color::from_rgb(r,g,b));

    }
    // 判断鼠标是否在当前元素
    pub fn existPoint(&self, x: i32, y: i32) -> bool {
        return x > self.text.x()
            && x < self.text.x() + self.text.width()
            && y > self.text.y()
            && y < self.text.y() + self.text.height();
    }

}
impl Clone for TextControl {
    fn clone(&self) -> Self {
        TextControl {
            text:self.text.clone()
        }
    }
}
#![allow(warnings, unused)]

use fltk::enums::FrameType;
use fltk::prelude::{WidgetBase, WidgetExt};

pub struct HotspotItmeControl {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}
impl HotspotItmeControl {
    pub(crate) fn clone(&self) -> Self {
        HotspotItmeControl{
            x: self.x.clone(),
            y: self.y.clone(),
            width: self.width.clone(),
            height: self.height.clone(),
        }
    }
}
impl HotspotItmeControl {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        let mut check_item_control = HotspotItmeControl {
            x,
            y,
            width,
            height,
        };

        check_item_control
    }

    // 判断鼠标是否在当前元素
    pub fn existPoint(&self, x: i32, y: i32) -> bool {
        return x > self.x.clone()
            && x < self.x.clone() + self.width.clone()
            && y > self.y.clone()
            && y < self.y.clone() + self.height.clone();
    }

    pub fn set_callback (&mut self, cb: fn()){
        let mut main = fltk::frame::Frame::new(self.x,self.y,self.width,self.height, "");
        main.set_frame(FrameType::NoBox);
        main.handle(move |txt, event| match event {
            fltk::enums::Event::Push=>{
                cb();
                true
            }
           fltk::enums::Event::Move=>{
                // cb();
                true
            }
            fltk::enums::Event::Leave=>{
                // cb();
                true
            }
            _ => false,
        });
    }

}

pub fn create_hotspot(x: i32, y: i32, width: i32, height: i32) -> HotspotItmeControl {
    HotspotItmeControl::new(x, y, width, height)
}
use fltk::{
    app::{self, handle},
    button::{self, Button},
    draw::font,
    enums::{self, Color, Cursor, Event, Font, FrameType, LabelType},
    frame::{self, Frame},
    group::{self, Flex, Group},
    image::{self, Image, PngImage},
    input::{InputType, IntInput},
    prelude::*,
    text::TextDisplay,
    tree,
    window::{self, DoubleWindow, Window},
};
use std::{
    fs,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    process,
    process::{Command, Stdio},
    thread,
};

pub struct ImgPreview {
    pub preview: frame::Frame,
    pub x:i32,
    pub y:i32,
    pub width: i32,
    pub height: i32,
    pub(crate) data: Vec<u8>,
    pub img_type:ImgPreviewDataType,
    pub data_x:i32,
    pub data_y:i32,
    pub data_width: i32,
    pub data_height: i32,
}
pub enum ImgPreviewDataType {
    NoneS,
    Svg,
    Jpeg,
    Png,
    Gif
}

impl Clone for ImgPreview {
    fn clone(&self) -> Self {
        ImgPreview {
            preview:self.preview.clone(),
            x: self.x.clone(),
            y: self.y.clone(),
            width: self.width.clone(),
            height: self.height.clone(),
            data:self.data.clone(),
            img_type: ImgPreviewDataType::Svg,
            data_x: self.data_x.clone(),
            data_y: self.data_y.clone(),
            data_width: self.data_width.clone(),
            data_height: self.data_height.clone(),
        }
    }
}

impl ImgPreview {
    pub fn detect_image_format(data: &[u8]) -> Option<&'static str> {
        if data.len() < 4 {
            return None; // 数据不够长，无法识别
        }

        match &data[0..4] {
            [0x89, 0x50, 0x4E, 0x47] => Some("png"),
            [0xFF, 0xD8, _, _] => Some("jpg"),
            [0x47, 0x49, 0x46, 0x38] => Some("gif"),
            [0x52, 0x49, 0x46, 0x46]
            if data.len() >= 12 && &data[8..12] == [0x57, 0x45, 0x42, 0x50] =>
                {
                    Some("webp")
                }
            _ => None, // 未知格式
        }
    }

    pub fn new(x: i32, y: i32, width: i32, height: i32, id: &'static str) -> Self {
        let mut preview = frame::Frame::new(x, y, width, height, "");
        preview.set_frame(enums::FrameType::FlatBox);
        preview.set_color(enums::Color::from_rgb(80, 80, 80));
        preview.set_id(id);
        Self { preview, x, y, width, height ,data:Vec::new(), img_type: ImgPreviewDataType::NoneS, data_x: 0, data_y: 0, data_width: 0, data_height: 0 }
    }

    pub fn new2(x: i32, y: i32, width: i32, height: i32, id: &'static str, data_x: i32, data_y: i32, data_width: i32, data_height: i32) -> Self {
        let mut preview = frame::Frame::new(x, y, width, height, "");
        preview.set_frame(enums::FrameType::FlatBox);
        preview.set_color(enums::Color::from_rgb(80, 80, 80));
        preview.set_id(id);
        Self { preview, x, y, width, height ,data:Vec::new(), img_type: ImgPreviewDataType::NoneS, data_x , data_y, data_width, data_height}
    }

    pub fn new_border(x: i32, y: i32, width: i32, height: i32,svg_data:&str) -> Self {
        let mut preview = frame::Frame::new(x, y, width, height, "");
        preview.set_frame(enums::FrameType::NoBox);
        preview.set_color(enums::Color::from_rgb(80, 80, 80));
        let mut self_data =  Self { preview, x, y, width, height ,data:svg_data.as_bytes().to_vec(), img_type: ImgPreviewDataType::Svg, data_x:0 , data_y:0, data_width:width, data_height:height};
        self_data.from_svg(svg_data,0,0,width,height);
        self_data
    }

    // 判断鼠标是否在当前元素
    pub fn existPoint(&self, x: i32, y: i32) -> bool {
        return x > self.x.clone()
            && x < self.x.clone() + self.width.clone()
            && y > self.y.clone()
            && y < self.y.clone() + self.height.clone();
    }

    pub fn load(&mut self, path: String, x: i32, y: i32, width: i32, height: i32) -> bool {
        let mut res = false;
        if let Result::Ok(data) = fs::read(path) {
            res = self.from_data(data, x, y, width, height);
        }
        res
    }

    pub fn from_data(&mut self, data: Vec<u8>, x: i32, y: i32, width: i32, height: i32) -> bool {
        let mut res = false;
        macro_rules! re_imag {
            ($imag:expr) => {
                $imag.scale(width, height, false, true);
                self.preview.draw(move |cb| {
                    let cbx: i32 = cb.x();
                    let cby: i32 = cb.y();
                    let cbh: i32 = cb.h();
                    let cbw: i32 = cb.w();
                    let cx: i32 = x;
                    let cy: i32 = y;
                    $imag.draw_ext(cbx, cby, cbw, cbh, cx, cy);
                });
                self.data_height = height;
                self.data_width = width;
                self.data_x = x;
                self.data_y = y;
                self.preview.redraw();
                self.preview.redraw_label();
                res = true;
            };
        }

        if let Some(ext) = ImgPreview::detect_image_format(&data) {
            self.data = data.to_vec();

            if (ext.as_bytes().eq("png".as_bytes())) {
                if let Result::Ok(mut img) = image::PngImage::from_data(&*data) {
                    re_imag!(img);
                    self.img_type = ImgPreviewDataType::Png
                }
            } else if (ext.as_bytes().eq("jpg".as_bytes())) {
                if let Result::Ok(mut img) = image::JpegImage::from_data(&*data) {
                    self.img_type = ImgPreviewDataType::Jpeg;
                    re_imag!(img);
                }
            } else if (ext.as_bytes().eq("gif".as_bytes())) {
                if let Result::Ok(mut img) = image::GifImage::from_data(&*data) {
                    self.img_type = ImgPreviewDataType::Gif;
                    re_imag!(img);
                }
            }
        }

        res
    }

    pub fn re_data(&mut self, data: Vec<u8>){
        self.from_data(data,self.data_x,self.data_y,self.data_width,self.data_height);
    }

    pub fn from_svg(&mut self, data: &str, x: i32, y: i32, width: i32, height: i32) -> bool {
        let mut res = false;
        macro_rules! re_imag {
            ($imag:expr) => {
                $imag.scale(width, height, false, true);
                self.preview.draw(move |cb| {
                    let cbx: i32 = cb.x();
                    let cby: i32 = cb.y();
                    let cbh: i32 = cb.h();
                    let cbw: i32 = cb.w();
                    let cx: i32 = x;
                    let cy: i32 = y;
                    $imag.draw_ext(cbx, cby, cbw, cbh, cx, cy);
                });
                self.preview.redraw();
                self.preview.redraw_label();
                res = true;
            };
        }
        if let Result::Ok(mut img) = image::SvgImage::from_data(data) {
            self.img_type = ImgPreviewDataType::Svg;
            self.data = data.as_bytes().to_vec();

            re_imag!(img);
        }
        res
    }

    pub fn get_data (&self) -> Vec<u8> {
        self.data.to_vec()
    }

    pub fn as_mut (&mut self) -> &mut ImgPreview {
         self
    }

}
use chrono::Local;
use rusqlite::Connection;

use crate::{atomic_util, get_arc_bind_variable, global_var, handle_dat, libWxIkunPlus::getFocusTopWindow, read_rw_lazy_lock, read_rw_lock, set_arc_bind_variable, set_arc_bind_variable_insert, set_arc_bind_variable_vec_clear, set_arc_bind_variable_vec_replace_data, util::{str_eq_str, Sleep}, wh_mod::{self, AttachThumbnail}, write_rw_lock, write_rw_lock_insert, gui_util, libWxIkunPlus};

use std::{
    sync::atomic::{AtomicUsize, Ordering},
    sync::{Arc, Condvar, Mutex, RwLock},
    thread, time::UNIX_EPOCH, collections::HashMap, path::{Path, PathBuf},
};
use std::sync::atomic::AtomicBool;
use crate::gui_select_user_ui::{ASSETS_DEMO_DATA, ASSETS_DEMO_NOT_DATA, THE_WINDOW_CLASS_NAME};

static HAS_SELECT_USER_WINDOW_NORMAL: AtomicBool = AtomicBool::new(false);

// 图片预览 全局变量
static mut IMG_PREVIEW_LIST: Vec<gui_util::img::ImgPreview> = Vec::new();
static IMG_PREVIEW_LIST_BIND: AtomicUsize = AtomicUsize::new(0);

// 缩略图 全局变量
static mut THUMBNAIL_LIST: Vec<wh_mod::AttachThumbnail> = Vec::new();
static THUMBNAIL_LIST_BIND: AtomicUsize = AtomicUsize::new(0);


// wxid 全局变量
static mut WX_ID: String = String::new();
static WX_ID_BIND: AtomicUsize = AtomicUsize::new(0);

// 用户wx存储位置 全局变量
static mut USER_PATH: String = String::new();
static USER_PATH_BIND: AtomicUsize = AtomicUsize::new(0);

// 用户列表绑定
static mut ACTIVE_USER_LIST: Vec<wh_mod::convert::WxActiveUser> = Vec::new();
static ACTIVE_USER_LIST_BIND: AtomicUsize = AtomicUsize::new(0);

pub struct UserWxRootHistory {
    pub time: String,
    pub path: String,
    pub name: String,
}

macro_rules! get_the_hwnd {
    ($class_id:expr) => {
        {
        let mut _hwnd = 0 ;
        for _ in 0..8 {
          _hwnd = libWxIkunPlus::findWindow($class_id,"");
            if !libWxIkunPlus::isWindow(_hwnd) {
                 _hwnd=0;
            }else {
              break;
          }
            fltk::app::sleep(0.020);
        }
        _hwnd as i128}
    };
    ()=>{
        get_the_hwnd!(THE_WINDOW_CLASS_NAME)
    }
}

// 从数据库读取历史记录
pub fn get_wx_user_history_path() -> Result<UserWxRootHistory, rusqlite::Error> {
    let mut user_wx_root_history = UserWxRootHistory {
        time: "".to_string(),
        path: "".to_string(),
        name: "".to_string(),
    };

    let conn: Connection = Connection::open("ikun_user_data.db").unwrap();
    handle_dat::initialize_table(&conn);
    if let Ok(mut stmt) =
        conn.prepare("SELECT time,name,path  FROM user_wx_root_history ORDER BY time DESC LIMIT 1")
    {
        let cats = stmt.query_map([], |row| {
            Ok(UserWxRootHistory {
                time: row.get(0)?,
                name: row.get(1)?,
                path: row.get(2)?,
            })
        })?;

        for cat in cats {
            let cat = cat?;
            user_wx_root_history.path = cat.path;
            user_wx_root_history.name = cat.name;
            user_wx_root_history.time = cat.time;
        }
    }

    conn.close();
    Ok(user_wx_root_history)
}

// 保存读取历史
pub fn store_wx_user_path_history(select_path: String, user_name: String) {
    thread::spawn(move || {
        let conn: Connection = Connection::open("ikun_user_data.db").unwrap();
        handle_dat::initialize_table(&conn);

        match conn.execute(
            "DELETE FROM user_wx_root_history WHERE path = ?1",
            [select_path.clone()],
        ) {
            Ok(updated) => {}
            Err(err) => {}
        }

        match conn.execute(
            "INSERT INTO user_wx_root_history (time,path,name) values (?1, ?2, ?3)",
            [
                Local::now().format("%Y-%m-%d").to_string(),
                select_path,
                user_name,
            ],
        ) {
            Ok(_) => {}
            Err(err) => {}
        }

        conn.close();
    });
}

// 保存备注
pub fn set_store_user_remark(wxid: String, attach_id: String, remark_name: String) {
    // thread::spawn(move || {
        let conn: Connection = Connection::open("ikun_user_data.db").unwrap();
        handle_dat::initialize_table(&conn);

        match conn.execute(
            "DELETE FROM user_remark WHERE wxid = ?1 AND attach_id = ?2",
            [wxid.clone(), attach_id.clone()],
        ) {
            Ok(updated) => {
            }
            Err(err) => {}
        }

        match conn.execute(
            "INSERT INTO user_remark (time,wxid,attach_id,remark_name) values (?1, ?2, ?3, ?4)",
            [
                Local::now().format("%Y-%m-%d").to_string(),
                wxid.clone(),
                attach_id.clone(),
                remark_name.clone(),
            ],
        ) {
            Ok(_) => {
                gui_util::message::sub_message(get_the_hwnd!(),gui_util::message::IconType::Success,"当前别名备注已经更新",5000u64);

            }
            Err(err) => {
                gui_util::message::sub_message(get_the_hwnd!(),gui_util::message::IconType::Failure,format!("别名更新失败 因为-> {:?}",err).as_str(),5000u64);

            }
        }

        conn.close();
    // });
}

struct UserRemark {
    time: String,
    wxid: String,
    attach_id: String,
    remark_name: String,
}

// 获取备注
pub fn get_store_user_remark(wxid: String, attach_id: String) -> Option<String> {
    let mut res_data = Option::None;

    let conn: Connection = Connection::open("ikun_user_data.db").unwrap();
    handle_dat::initialize_table(&conn);
    if let Ok(mut stmt) =
        conn.prepare("SELECT time,wxid,attach_id,remark_name  FROM user_remark  WHERE wxid = ?1 AND attach_id = ?2")
    {
        let cats = stmt.query_map([wxid.clone(),attach_id.clone()], |row| {
            let mut row_data = UserRemark {
                time: String::new(),
                wxid: String::new(),
                attach_id: String::new(),
                remark_name: String::new()
             };

             if let Ok(item) = row.get(0) as Result<String,_> {
                row_data.time = item.clone();
             }
             
             if let Ok(item) = row.get(1) as Result<String,_> {
                row_data.wxid = item.clone();
             }
             
             if let Ok(item) = row.get(2) as Result<String,_> {
                row_data.attach_id = item.clone();
             }

             if let Ok(item) = row.get(3) as Result<String,_> {
                row_data.remark_name = item.clone();
             }

            Ok(row_data)
        });

        if let Ok(cats) = cats {
            for cat in cats {
                if let Ok(cat) = cat {
                    res_data.replace(cat.remark_name);
                }
          
        } 
        }
    }

    conn.close();

    res_data
}

// 添加active_user_list到全局变量
pub fn set_active_user_list(active_user_list: Vec<wh_mod::convert::WxActiveUser>) {
    set_arc_bind_variable!(ACTIVE_USER_LIST, ACTIVE_USER_LIST_BIND, active_user_list);
}

// 获取active_user_list到全局变量
pub fn get_active_user_list() -> Vec<wh_mod::convert::WxActiveUser> {
    let active_user_list = get_arc_bind_variable!(ACTIVE_USER_LIST, ACTIVE_USER_LIST_BIND);

    active_user_list.clone()
}

// 添加 active_user_list到全局变量
pub fn push_active_user_list(active_user: wh_mod::convert::WxActiveUser) {
    let mutex = Arc::new(Mutex::new(&ACTIVE_USER_LIST_BIND));
    mutex.lock();
    let the_value: usize = ACTIVE_USER_LIST_BIND.load(Ordering::SeqCst);

    unsafe {
        ACTIVE_USER_LIST.push(active_user);
    }

    ACTIVE_USER_LIST_BIND.store(the_value + 1, Ordering::SeqCst);
    drop(mutex);
}

// 更新进视图
pub fn update_thumbnail_preview_list()  {
        // 取出缩略图列表 并将其缩减到5条以内
        let mut thumbnail_list = {

            let mut thumbnail_list =
                get_arc_bind_variable!(THUMBNAIL_LIST, THUMBNAIL_LIST_BIND).to_vec();
            let mut atid_list: HashMap<String, AttachThumbnail> = HashMap::new();
    
            for value in thumbnail_list {
                let key = value.attach_id.clone();
                let mut oid_created = UNIX_EPOCH;
                let mut new_created = UNIX_EPOCH;
    
                // oid create time
                if let Some(thumbnail) = atid_list.get(&key) {
                    if let Ok(metadata) = std::fs::metadata(thumbnail.thumbnail_path.clone()) {
                        if let Result::Ok(create) = metadata.created() {
                            oid_created = create;
                        }
                    }
                }
    
                // new create time
                if let Ok(metadata) = std::fs::metadata(value.thumbnail_path.clone()) {
                    if let Result::Ok(create) = metadata.created() {
                        new_created = create;
                    }
                }
    
                // 按照创建时间判断是否更新视图
                if (new_created > oid_created) {
                    atid_list.insert(value.attach_id.clone(), value);
                }
            }
    
    
            let mut thumbnail_list: Vec<AttachThumbnail> = Vec::new();
    
            for (key, value) in atid_list {
                thumbnail_list.push(value);
            }
    
            // 排序
            thumbnail_list.sort_by(|a, b| {
                let mut a_created = UNIX_EPOCH;
                let mut b_created = UNIX_EPOCH;
    
                if let Ok(metadata) = std::fs::metadata(a.thumbnail_path.clone()) {
                    if let Result::Ok(create) = metadata.created() {
                        a_created = create;
                    }
                }
    
                if let Ok(metadata) = std::fs::metadata(b.thumbnail_path.clone()) {
                    if let Result::Ok(create) = metadata.created() {
                        b_created = create;
                    }
                }
    
                a_created.cmp(&b_created)
            });
    
            let mut new_thumbnail_list = Vec::new();
    
            thumbnail_list.reverse();
            for value in thumbnail_list {
                if (new_thumbnail_list.len() > 5 - 1) {
                    break;
                }
                new_thumbnail_list.push(value);
            }
    
            new_thumbnail_list
        };

        set_arc_bind_variable!(THUMBNAIL_LIST, THUMBNAIL_LIST_BIND, thumbnail_list.to_vec());
    
    
        // 更新到视图中
        let thumbnail_list = get_arc_bind_variable!(THUMBNAIL_LIST, THUMBNAIL_LIST_BIND);
    
        // 锁定缩略图更新
        let mutex = Arc::new(Mutex::new(&THUMBNAIL_LIST_BIND));
        mutex.lock();
    
        let img_preview_list = get_arc_bind_variable!(IMG_PREVIEW_LIST, IMG_PREVIEW_LIST_BIND);
    
        let (width, height) = (75, 75);

        if thumbnail_list.is_empty(){
            gui_util::message::sub_message(get_the_hwnd!(),gui_util::message::IconType::Warning,"没有发现图片列表 可以找开发者反馈",5000u64);
        }

        // 更新到视图中  
        for index in 0..img_preview_list.len() {
            if let Some(mut img_preview) = img_preview_list.get(index) {

                if let Some(thumbnail) = thumbnail_list.get(index) {
                    let pre: Vec<u8> = if wh_mod::config::is_show_dome() { ASSETS_DEMO_DATA() } else{ thumbnail.thumbnail.clone() };

                    img_preview.clone().from_data(
                        pre,
                        -1,
                        -1,
                        width - 2,
                        height - 2,
                    );
                } else {
                    img_preview.clone().from_data(
                        ASSETS_DEMO_NOT_DATA(),
                        -1,
                        -1,
                        width - 2,
                        height - 2,
                    );
                }
            }
        }

        drop(mutex);

}

// 初始化五张图片到视图
pub fn initialize_thumbnail_preview(user_root: &str,wxid: &str){
    
    let msg_attach_dir = PathBuf::from(format!("{}\\{}\\FileStorage\\MsgAttach",user_root,wxid).as_str());

    println!("msg_attach_dir-> {:?}",msg_attach_dir);

    let mut read_imag_list = wh_mod::read_attach_buff_thumbnail_list_v2(&msg_attach_dir.as_path(), 5, 1);

    // v2 没有内容就是说明这个库可能修改时间有误 尝试用v1 获取
    if read_imag_list.is_empty() {
        read_imag_list = wh_mod::read_attach_buff_thumbnail_list(&msg_attach_dir.as_path(), 5, 1);
    }

    set_arc_bind_variable_vec_replace_data!(THUMBNAIL_LIST,THUMBNAIL_LIST_BIND,read_imag_list);
    update_thumbnail_preview_list();

}

// 开始获取更新
pub fn initialize_watch_path_puppet(path: String) {
    std::thread::spawn(move ||{
        // 启动日志检测模式
        let (tx, rx) = std::sync::mpsc::channel();

        let wh_id = wh_mod::watch_path::watch_path_puppet(path.clone(), tx);
        println!("copy_path_wake-> {}", path.clone());
        while wh_id == wh_mod::watch_path::get_the_id() {
            if let Result::Ok(data) = rx.recv() {
                let path = data.join("..").join("..").join("..");
                let data_list = wh_mod::read_attach_buff_thumbnail_data(&path, 1);
                // write_rw_lock_insert!(THUMBNAIL_LIST_ARC, data_list.to_vec());
                set_arc_bind_variable_insert!(
                                THUMBNAIL_LIST,
                                THUMBNAIL_LIST_BIND,
                                data_list.to_vec()
                            );

                if (data_list.len() > 0) {
                   update_thumbnail_preview_list();
                }
            }
        }

    });
}

macro_rules! gc_select_user_ui {
   ()=>{
        if !atomic_util::get_bool(&HAS_SELECT_USER_WINDOW_NORMAL){
                println!("[gc] initialize_gc_select_user_ui");
                wh_mod::watch_path::un_next_exits();
                global_var::set_string("user::config::user_select_path", String::new());
                global_var::set_string("user::config::user_select_wxid", String::new());
                set_arc_bind_variable_vec_clear!(THUMBNAIL_LIST,THUMBNAIL_LIST_BIND);
                set_arc_bind_variable_vec_clear!(IMG_PREVIEW_LIST,IMG_PREVIEW_LIST_BIND);
                println!("[gc] initialize_gc_select_user_ui ok! ");
                return;
            }
   }
}

// 自动在窗口销毁时候自动清理
pub fn initialize_gc_select_user_ui(hwnd:i128){

    if atomic_util::get_bool(&HAS_SELECT_USER_WINDOW_NORMAL){
        return;
    }

    atomic_util::set_bool(&HAS_SELECT_USER_WINDOW_NORMAL, true);
    
    thread::spawn(move ||{
        loop {
            if !atomic_util::get_bool(&HAS_SELECT_USER_WINDOW_NORMAL)||!libWxIkunPlus::isWindow(hwnd){
                println!("[gc] initialize_gc_select_user_ui");
                atomic_util::set_bool(&HAS_SELECT_USER_WINDOW_NORMAL,false);
                wh_mod::watch_path::un_next_exits();
                global_var::set_string("user::config::user_select_path", String::new());
                global_var::set_string("user::config::user_select_wxid", String::new());
                set_arc_bind_variable_vec_clear!(THUMBNAIL_LIST,THUMBNAIL_LIST_BIND);
                set_arc_bind_variable_vec_clear!(IMG_PREVIEW_LIST,IMG_PREVIEW_LIST_BIND);
                println!("[gc] initialize_gc_select_user_ui ok! ");
                return;
            }
            Sleep(150);
        }
    });
}

// 绑定视图5张的显示控件
pub fn initialize_img_preview_list (img_preview_list:&Vec<gui_util::img::ImgPreview>){
    use std::sync::{Arc, Condvar, Mutex, RwLock};
    let mutex = Arc::new(Mutex::new(&IMG_PREVIEW_LIST_BIND));
    mutex.lock();
    let the_value: usize = IMG_PREVIEW_LIST_BIND.load(Ordering::SeqCst);
    unsafe {
        IMG_PREVIEW_LIST.clear();
        for value in img_preview_list {
            IMG_PREVIEW_LIST.push(value.clone())

        }
    }
    IMG_PREVIEW_LIST_BIND.store(the_value + 1, Ordering::SeqCst);

    drop(mutex);

    // set_arc_bind_variable_vec_replace_data!(IMG_PREVIEW_LIST,IMG_PREVIEW_LIST_BIND,img_preview_list);
}

// GC掉大部分高内存的存储
pub fn gc_select_user_ui(){
    atomic_util::set_bool(&HAS_SELECT_USER_WINDOW_NORMAL,false);
    wh_mod::gc_walk_attach_file_list();
    gc_select_user_ui!();
}

// 获取缩略图绑定列表
pub fn get_thumbnail_list() -> Vec<AttachThumbnail> {
    get_arc_bind_variable!(THUMBNAIL_LIST,THUMBNAIL_LIST_BIND).clone()
}

pub fn bool_to_str (b:bool) -> &'static str {
    if b {"是"} else { "否" }
}
use crate::util::str_eq_ostr;
use crate::{console_log, global_var_util, gui_util, handle_dat, libWxIkunPlus, wh_mod};
use chrono::Local;
use glob::glob;
use rusqlite::{params, Connection, Result};
use serde_json::json;
use serde_json::Value as Json;
use toml::Value as Toml;
use crate::global_var;
use crate::gui_main_ui::THE_WIN_CLASS_NAME;
pub struct AppVersionInfo {}

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

fn toml2json(toml: Toml) -> Json {
    match toml {
        Toml::String(s) => Json::String(s),
        Toml::Integer(i) => Json::Number(i.into()),
        Toml::Float(f) => {
            let n = serde_json::Number::from_f64(f).expect("float infinite and nan not allowed");
            Json::Number(n)
        }
        Toml::Boolean(b) => Json::Bool(b),
        Toml::Array(arr) => Json::Array(arr.into_iter().map(toml2json).collect()),
        Toml::Table(table) => {
            Json::Object(table.into_iter().map(|(k, v)| (k, toml2json(v))).collect())
        }
        Toml::Datetime(dt) => Json::String(dt.to_string()),
    }
}

pub fn get_app_version_info() -> Json {
    const APP_VERSION: &str = include_str!("../../../Cargo.toml");
    // println!("toml2json-> {:?}",toml2json(APP_VERSION));

    match APP_VERSION.parse() {
        Ok(toml) => {
            let json = toml2json(toml);
            return json;
        }
        Err(error) => println!("failed to parse TOML: {}", error),
    }

    json!("")
}

// 获取初始化文本
pub fn get_init_text() -> String {
    let mut result = String::new();
    let mut sync_type = String::new();
    let mut build_name = if wh_mod::config::is_build_52pojie() {
        "52破解专版"
    } else {
        "开源版"
    };
    let version_info = get_app_version_info();
    let version = (version_info["package"]["version"]).as_str().unwrap();

    if !wh_mod::config::is_developer() {
        result.push_str(
            format!(
                r#"作者 @Ikun 软件开源协议 GPL 3.0 (但是并不包含解码算法)
        当前版本：{} ({})
        本软件 是免费软件 如果付费请维权退款
        本软件只供节约自己另存为图片时间，禁止用于其他用途
        "#,
                version, build_name
            )
            .replace("  ", "")
            .as_str(),
        );
    } else {
        result.push_str(("初始化成功 [开发者模式]"));
    }

    if libWxIkunPlus::has_auto_sync() {
        result.push_str(format!("\n[用户] 自动同步开启").as_str());
    } else if wh_mod::config::is_developer() {
        result.push_str("\n[同步] 自动同步已启用 因为开发者模式有效");
    } else {
        result.push_str("\n[同步] 自动同步关闭");
    }

    result
}

// 添加进数据库
pub fn push_sql_export_dir_path(name: &str, export_dir: &str, task_command: &str) {
    if !eq_next() {
        libWxIkunPlus::stop(
            "错误".to_owned(),
            "当前未发现wx进程或者未登录 拒绝提供添加".to_owned(),
        );
        return;
    }
    if name.is_empty() {
        console_log!(format!("\n[错误] 没有名称"));
        gui_util::sub_message(get_the_hwnd!(), gui_util::IconType::Failure, "[错误] 没有名称", 3500u64);
        return;
    }

    if export_dir.is_empty() {
        console_log!(format!("\n[错误] 没有设置导出到的路径"));
        gui_util::sub_message(get_the_hwnd!(), gui_util::IconType::Failure, "[错误] 没有设置导出到的路径", 3500u64);
        return;
    }

    if task_command.is_empty() {
        console_log!(format!("\n[错误] 没有任务命令"));
        gui_util::sub_message(get_the_hwnd!(), gui_util::IconType::Failure, "[错误] 没有任务命令", 3500u64);
        return;
    }

    let conn: Connection = Connection::open("ikun_user_data.db").unwrap();

    handle_dat::initialize_table(&conn);
    match conn.execute(
        "INSERT INTO export_dir_path (name,time,path,ouput) values (?1, ?2, ?3, ?4)",
        [
            name,
            Local::now().format("%Y-%m-%d").to_string().as_str(),
            export_dir,
            task_command,
        ],
    ) {
        Ok(_) => {
            console_log!(format!("\n[存储] 添加成功"));
            gui_util::sub_message(
                get_the_hwnd!(),
                gui_util::IconType::Success,
                "添加成功",
                3500u64,
            );
        }
        Err(err) => {
            if (str_eq_ostr(
                err.to_string(),
                "UNIQUE constraint failed: export_dir_path.path",
            )) {
                console_log!(format!("\n[错误] 添加失败 因为-> {}", "当前任务已经存在"));
                gui_util::sub_message(
                    get_the_hwnd!(),
                    gui_util::IconType::Warning,
                    "当前任务已经存在",
                    3500u64,
                );
            } else {
                gui_util::sub_message(
                    get_the_hwnd!(),
                    gui_util::IconType::Failure,
                    "任务添加失败",
                    3500u64,
                );
            }
        }
    }

    conn.close();
    global_var_util::update_export_dir_itme_list();
}

pub fn eq_next() -> bool {
    (wh_mod::config::is_developer()
        || (libWxIkunPlus::hasWeChat() && libWxIkunPlus::hasWeChatWin()))
}

// 测试
pub fn test_task(name: &str, export_dir: &str, task_command: &str) {
    let mut path_dir = wh_mod::parse_dat2var_path(format!("{}", task_command));

    if name.is_empty() {
        console_log!(format!("\n[警告] 没有名称"));
    }

    if export_dir.is_empty() {
        console_log!(format!("\n[警告] 没有设置导出到的路径"));
    }

    if task_command.is_empty() {
        console_log!(format!("\n[警告] 没有任务命令"));
    }

    if let Ok(metadata) = std::fs::metadata(path_dir.attach_dir.clone()) {
        if (!metadata.is_dir()) {
            console_log!(format!("\n[错误] dat目录文件夹 不是文件夹"));
            gui_util::sub_message(
                get_the_hwnd!(),
                gui_util::IconType::Failure,
                "dat目录文件夹 不是文件夹",
                3500u64,
            );
            return;
        }

        console_log!(format!("\n[测试] 正在扫描当前文件夹存在的dat图片"));
        console_log!(format!(
            "\n[测试] 处理范围: 仅本月:{}   缩略图:{}   原图:{}   视频:{}   同步:{}   全部:{}   ",
            bool_to_str(path_dir.is_the_month),
            bool_to_str(path_dir.is_thumbnail),
            bool_to_str(path_dir.is_source),
            bool_to_str(path_dir.is_video),
            bool_to_str(path_dir.is_sync),
            bool_to_str(path_dir.is_all)
        ));

        let pattern = format!(
            "{}",
            std::path::Path::new(&path_dir.attach_dir.clone())
                .join("**/*.dat")
                .display()
                .to_string()
        );

        let mut index = 0;

        console_log!(format!("\n[测试] 开始扫描 “{}” 中的dat文件", pattern));

        for entry in glob(&pattern).unwrap() {
            index = index + 1;
        }

        console_log!(format!(
            "\n[测试] 在 “{}” \n中发现了 [{}] 个dat文件",
            pattern, index
        ));
        gui_util::sub_message(
            get_the_hwnd!(),
            gui_util::IconType::Success,
            "测试成功",
            3500u64,
        );

        return;
    }
    console_log!(format!(
        "\n[错误] dat目录文件夹 无法被读取",
    ));
    gui_util::sub_message(
        get_the_hwnd!(),
        gui_util::IconType::Failure,
        "dat目录文件夹 打开失败",
        3500u64,
    );
}

fn bool_to_str (b:bool) -> &'static str {
    if b {"是"} else { "否" }
}
#![allow(warnings, unused)]

use fltk::app::{event_key, sleep};
use fltk::enums::{Color, Cursor, Event, FrameType, Key};
use fltk::{prelude::*, *};
use fltk_theme::{color_themes, ColorTheme, SchemeType, ThemeType, WidgetScheme, WidgetTheme};
use crate::gui_util::hotspot::create_hotspot;
use crate::gui_util::img;
use crate::{gui_util, libWxIkunPlus};

use std::collections::{HashMap, HashSet};

use std::sync::atomic::{AtomicBool, AtomicI32, AtomicI64, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, MutexGuard, OnceLock, RwLock};
use crate::util::OverloadedAnyStr;

static mut MESS_HASH_MAP: Option< HashMap<String,i128> > = Option::None;
static MESS_HASH_MAP_BIND: AtomicUsize = AtomicUsize::new(0);

static mut WINDOW_CLASS_HASH_SET: Option< HashSet<String> > = Option::None;
static WINDOW_CLASS_HASH_SET_BIND: AtomicUsize = AtomicUsize::new(0);


// 已经初始化哈希表了
static VARIABLE_INITIALIZE: OnceLock<bool> = OnceLock::new();

// 初始化全部类型哈希表
fn initialize() {
    if *(VARIABLE_INITIALIZE.get().unwrap_or_else(|| &false)) {
        return;
    }

    unsafe {
        if MESS_HASH_MAP.is_none() {
            MESS_HASH_MAP.replace(HashMap::new());
        }
    }

    unsafe {
        if WINDOW_CLASS_HASH_SET.is_none() {
            WINDOW_CLASS_HASH_SET.replace(HashSet::new());
        }
    }

    VARIABLE_INITIALIZE.set(true);
}

fn has_hash_message(hwnd:i128, mess:&str) -> bool {
    let mutex = Arc::new(Mutex::new(&MESS_HASH_MAP_BIND));
    mutex.lock();
    let the_value: usize = MESS_HASH_MAP_BIND.load(Ordering::SeqCst);
    let mut result = false;

    unsafe {
        let mut mut_hash = MESS_HASH_MAP.as_mut().unwrap();
        if let Some(mut_hash_) = mut_hash.get_key_value(&*mess.to_string()) {
            result = mut_hash_.1.clone()==hwnd;
        }
    };

    MESS_HASH_MAP_BIND.store(the_value + 1, Ordering::SeqCst);
    drop(mutex);
    result
}

fn del_hash_message(mess:&str){
    let mutex = Arc::new(Mutex::new(&MESS_HASH_MAP_BIND));
    mutex.lock();
    let the_value: usize = MESS_HASH_MAP_BIND.load(Ordering::SeqCst);
    unsafe {
        let mut mut_hash = MESS_HASH_MAP.as_mut().unwrap();
        if let Some(mut_hash_) = mut_hash.get_key_value(&*mess.to_string()) {
            mut_hash.remove(&*mess.to_string());
        }
    };

    MESS_HASH_MAP_BIND.store(the_value + 1, Ordering::SeqCst);
    drop(mutex);
}

fn set_hash_message(hwnd:i128,mess:&str){
    let mutex = Arc::new(Mutex::new(&MESS_HASH_MAP_BIND));
    mutex.lock();
    let the_value: usize = MESS_HASH_MAP_BIND.load(Ordering::SeqCst);
    unsafe {
        let mut mut_hash = MESS_HASH_MAP.as_mut().unwrap();
        mut_hash.insert(mess.to_string(), hwnd.clone());
    };

    MESS_HASH_MAP_BIND.store(the_value + 1, Ordering::SeqCst);
    drop(mutex);
}


fn get_window_class_list() -> Vec<String> {
    let mutex = Arc::new(Mutex::new(&WINDOW_CLASS_HASH_SET_BIND));
    mutex.lock();
    let the_value: usize = WINDOW_CLASS_HASH_SET_BIND.load(Ordering::SeqCst);
    let mut result: Vec<String> = Vec::new();

    unsafe {
        let mut mut_hash = WINDOW_CLASS_HASH_SET.as_mut().unwrap();
        for mut_hash in mut_hash.iter() {
            result.push(mut_hash.clone());
        }
    };

    WINDOW_CLASS_HASH_SET_BIND.store(the_value + 1, Ordering::SeqCst);
    drop(mutex);
    result
}

fn del_window_class(class:&str){
    let mutex = Arc::new(Mutex::new(&WINDOW_CLASS_HASH_SET_BIND));
    mutex.lock();
    let the_value: usize = WINDOW_CLASS_HASH_SET_BIND.load(Ordering::SeqCst);
    unsafe {
        let mut mut_hash = WINDOW_CLASS_HASH_SET.as_mut().unwrap();
        mut_hash.remove(class);
    };

    WINDOW_CLASS_HASH_SET_BIND.store(the_value + 1, Ordering::SeqCst);
    drop(mutex);
}

fn set_window_class(class:&str){
    let mutex = Arc::new(Mutex::new(&WINDOW_CLASS_HASH_SET_BIND));
    mutex.lock();
    let the_value: usize = WINDOW_CLASS_HASH_SET_BIND.load(Ordering::SeqCst);
    unsafe {
        let mut mut_hash = WINDOW_CLASS_HASH_SET.as_mut().unwrap();
        mut_hash.insert(class.to_string());
    };

    WINDOW_CLASS_HASH_SET_BIND.store(the_value + 1, Ordering::SeqCst);
    drop(mutex);
}


pub enum IconType {
    Success,
    Failure,
    Warning,
    Info
}

macro_rules! set_item_id {
    ($win:expr,$id:expr) => {
        $win.set_xclass($id);
        $win.set_id($id);
    };
}

#[derive(Debug)]
pub struct TextSize{
    pub size:usize,
    pub utf8_len:usize,
    pub ansi_len:usize,
    pub all_len:usize,
    pub prediction_len:usize
}

pub fn text_size (data:&str) -> TextSize {
    let mut result = TextSize{
        size: 0,
        utf8_len: 0,
        ansi_len: 0,
        all_len: 0,
        prediction_len:0
    };
    
    let mut name_len = 0;

    for value in data.chars() {
        if value.len_utf8()>2 {
            result.utf8_len+=1;
            result.prediction_len+=2;

        }else {
            result.ansi_len+=1;
            result.prediction_len+=1;
        }

        result.all_len+=1;
    }


    result.size = data.len();

    result
}

pub fn message(x:i32, y:i32,icon: IconType, message: &str,close_sleep:u64) {
    let mut hwnd = 0;
    let mut max_top = 0;

    // 消息不叠加到同个位置
    for get_window_class in get_window_class_list() {
        let hwnd = libWxIkunPlus::findWindow(get_window_class.as_str(),"");
        let rect = libWxIkunPlus::getWindowRect(hwnd);
        // println!("hwnd->{}  rect-> {:?}",&hwnd,&rect);
        if rect.top >= max_top {
            max_top = rect.top+55;
        }
    }
    let mut new_y = if max_top!=0 {max_top} else {y};

    // println!("get_window_class_list()->{:?}  new_y => {} ",get_window_class_list(),new_y);

    let mut win = window::DoubleWindow::new(x,new_y, 350, 45, None);
    win.set_color(Color::from_rgb(25, 25, 25));
    win.set_border(false);

    let win_id = format!("hmc_message_id<{}>",libWxIkunPlus::randomNum());
    set_item_id!(win,win_id.as_str());
    let text_size_data = text_size(message);

    set_window_class(win_id.as_str());

    // println!("win_id->{}",&win_id);
    // println!("{:?}",text_size(message));

    win.set_pos(x, new_y);

    let mut back_border = img::ImgPreview::new(0,0,win.w(),win.h(),"");
    let mut text_size = 13;
    // 计算文本大小
    if text_size_data.prediction_len>37 {
        text_size-=1;
    }

    if text_size_data.prediction_len>42 {
        text_size-=1;
    }

    if text_size_data.prediction_len>45 {
        text_size-=1;
    }

    let mut text = gui_util::text::TextPreview::new(40,15,275,15,text_size,message,[255,255,255]);
    // TextSize { size: 48, utf8_len: 16, ansi_len: 0, all_len: 16 }
    // TextSize { size: 38, utf8_len: 0, ansi_len: 38, all_len: 38 }

    // 设置消息内容背景
    match icon {
        IconType::Success => {
            back_border.from_svg(include_str!("./src/success.svg"), 0, 0, win.w(),win.h());
            text.set_back_color(27, 175, 93);
        }
        IconType::Failure => {
            back_border.from_svg(include_str!("./src/error.svg"), 0, 0, win.w(),win.h());
            text.set_back_color(228, 88, 62);
        }
        IconType::Warning => {
            back_border.from_svg(include_str!("./src/warning.svg"), 0, 0, win.w(),win.h());
            text.set_back_color(198, 134, 88);
        }
        IconType::Info => {
            back_border.from_svg(include_str!("./src/meassge.svg"), 0, 0, win.w(),win.h());
            text.set_back_color(60, 60, 60);
        }
    }

    back_border.preview.set_frame(FrameType::NoBox);

    let close = create_hotspot(320,12,21,21);

    win.handle({
        let mut x = 0;
        let mut y = 0;
        // 是否显示手型
        let mut show_cursor = false;
        let win_id = win_id.clone();
        move |win, ev| match ev {
            enums::Event::Show=>{
                win.set_visible_focus();
                libWxIkunPlus::setWinTop(libWxIkunPlus::findWindow(win_id.as_str(),""),true);
                true
            }
            enums::Event::Drag => {
                win.clone()
                    .set_pos(app::event_x_root() - x, app::event_y_root() - y);
                true
            }
            enums::Event::Push =>{
                if close.existPoint(x,y){
                    fltk::window::Window::delete(win.clone());
                }

                true
            }
            enums::Event::Move => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;

                if close.existPoint(x,y){
                    win.set_cursor(Cursor::Hand);
                } else {
                    win.set_cursor(Cursor::Default);
                }

                true
            }
            enums::Event::Hide=>{
                fltk::window::Window::delete(win.clone());
                // println!(" fltk::window::Window::delete(win);");
                true
            }
            _ => {
                false },
        }
    });

    win.end();
    win.show();



    libWxIkunPlus::setWinTop(libWxIkunPlus::findWindow(win_id.as_str(),""),true);

    let win_id2 = win_id.clone();

    // 置顶
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(100u64));
        libWxIkunPlus::setWinTop(libWxIkunPlus::findWindow(win_id2.as_str(),""),true);

        std::thread::sleep(std::time::Duration::from_millis(500u64));
        libWxIkunPlus::setWinTop(libWxIkunPlus::findWindow(win_id2.as_str(),""),true);

        std::thread::sleep(std::time::Duration::from_millis(1500u64));
        libWxIkunPlus::setWinTop(libWxIkunPlus::findWindow(win_id2.as_str(),""),true);
    });


    let message_copy = format!("{}",message);
    let win_id3 = win_id.clone();

    // 释放
    std::thread::spawn(move || {
        let hwnd = libWxIkunPlus::findWindow(win_id3.as_str(),"");
        // Sleep(close_sleep);
        std::thread::sleep(std::time::Duration::from_millis(close_sleep));
        libWxIkunPlus::setwinVisible(hwnd,false);
        del_hash_message(message_copy.as_str());
        del_window_class(win_id3.as_str());
    });

}

pub fn sub_message(hwnd:i128,icon: IconType, _message: &str,close_sleep:u64){
    initialize();
    let mut rect =libWxIkunPlus::getWindowRect(hwnd);
    if !has_hash_message(hwnd,_message) {
        set_hash_message(hwnd,_message);
    }


    let [x,y] = [rect.left + (rect.width/2)-(350/2),rect.top+50];
    message(x,y,icon,_message,close_sleep);
}

pub fn message_the_win(icon: IconType, _message: &str,close_sleep:u64){
    initialize();
    let hwnd = libWxIkunPlus::getFocusWindow();
    let mut rect =libWxIkunPlus::getWindowRect(hwnd);
    let [x,y] = [rect.left + (rect.width/2)-(350/2),rect.top+50];
    message(x,y,icon,_message,close_sleep);
}


use image::{GenericImageView, ImageBuffer};
use imagesize;
const MOBILE_SCREENSHOT_SIZE: [[usize; 2]; 130] = [[1284,2778],[1170,2532],[1170,2532],[1080,2340],[1284,2778],[1170,2532],[1080,2340],[1125,2436],[1242,2688],[828,1792],[1242,2688],[1125,2436],[1242,2208],[750,1334],[640,1136],[480,854],[1080,2160],[1080,1920],[1440,2560],[1080,2160],[1080,2270],[1080,2160],[1080,2246],[720,1280],[1080,1920],[1080,1920],[1080,1920],[1440,2560],[1440,2560],[1080,2400],[1080,2340],[720,1560],[1080,2340],[1080,2280],[1440,3040],[1440,3040],[1440,2960],[1440,2960],[1440,2960],[1440,2960],[1440,3040],[1080,2280],[1440,2960],[1440,2560],[1440,2560],[1080,1920],[1440,2560],[1440,2560],[1080,1920],[1080,1920],[1440,2560],[1080,2242],[1080,2160],[1080,2160],[1080,2160],[1080,1920],[1440,3120],[1080,2340],[1080,1920],[1080,2340],[1080,1920],[1080,1920],[1080,1920],[1080,2340],[1080,2400],[1080,2340],[1080,2340],[1080,2340],[1080,2340],[1080,2340],[1080,2340],[720,1520],[1080,2340],[720,1520],[720,1280],[720,1280],[720,1280],[1080,2340],[1080,2400],[1080,2340],[720,1560],[1080,2340],[1080,2340],[1080,2160],[1080,2040],[1080,1920],[1080,1920],[1080,1920],[1080,1920],[1080,1920],[1080,1920],[1080,2340],[1080,2340],[1080,1920],[720,1280],[1080,2340],[1080,2340],[720,1544],[720,1520],[720,1520],[1080,2340],[1080,2340],[1080,2460],[1080,2340],[1080,2280],[1080,2280],[1080,2280],[1080,2340],[1080,2340],[1080,1920],[1080,1920],[1200,2640],[1200,2640],[1080,2310],[1080,2340],[1080,2312],[1080,2310],[1080,2340],[1176,2400],[1080,2340],[1440,3120],[1440,3120],[1080,2244],[1080,2244],[1080,2340],[1080,2340],[1080,2240],[1080,2244],[1080,1920],[1440,2560]];

pub fn has_mobile_screenshot(data:Vec<u8>)-> bool{
    match imagesize::blob_size(&data) {
        Ok(size) =>{
            for [width,height] in MOBILE_SCREENSHOT_SIZE {
                if size.width == width  && size.height == height {
                    return true;
                }
            }
            return true
        },
        Err(why) => {
            return  false
        },
    }
}#![allow(warnings, unused)]

use crate::{get_arc_bind_variable, get_bool, get_option_arc_bind_variable, get_option_arc_bind_variable_or, global_var, gui_util, inject_fltk_theme, libWxIkunPlus, set_arc_bind_variable, set_arc_bind_variable_string_replace_data, set_bool, set_item_id, set_option_arc_bind_variable, wh_mod};
use fltk::enums::{Color, FrameType};
use fltk::window::DoubleWindow;
use fltk::{prelude::*, *};
use fltk_theme::{color_themes, ColorTheme, SchemeType, ThemeType, WidgetScheme, WidgetTheme};
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Local};
use crate::gui_select_user_ui;
use std::collections::{HashMap, HashSet};
use std::path;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicI64, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, MutexGuard, OnceLock, RwLock};

const THE_WIN_UI_BORDER: &str =  include_str!("./src/ui.svg");
const THE_WIN_CLASS_NAME: &str = "wx_auto_ex_im::gui_util::drag_scan2_ui::main<56136>";

static mut STATUS_TIME:Option< std::time::Instant > = Option::None;
static STATUS_TIME_BIND: AtomicUsize = AtomicUsize::new(0);

static mut STATUS_PREVIEW_BUF:Option< fltk::text::TextBuffer > = Option::None;
static STATUS_PREVIEW_BUF_BIND: AtomicUsize = AtomicUsize::new(0);

static SCAN_SCAN_ING: AtomicBool = AtomicBool::new(false);
static PUSH_MESSAGE_ING: AtomicBool = AtomicBool::new(false);

macro_rules! gc_the_window {
    ($win:expr) => {
      fltk::window::Window::delete($win.clone());
      // wh_mod::gc_walk_attach_file_list();
      // set_arc_bind_variable_string_replace_data!(STATUS_PREVIEW_TEXT,STATUS_PREVIEW_TEXT_BIND,"");
      println!("[gc_window] [{}] [{}]",THE_WIN_CLASS_NAME,!has_window());
    };
}

macro_rules! get_the_hwnd {
    ($class_id:expr) => {
        {
        let mut _hwnd = 0 ;
        for _ in 0..8 {
          _hwnd = libWxIkunPlus::findWindow($class_id,"");
            if !libWxIkunPlus::isWindow(_hwnd) {
                 _hwnd=0;
            }else {
              break;
          }
            fltk::app::sleep(0.020);
        }
        _hwnd as i128}
    };
    ()=>{
        get_the_hwnd!(THE_WIN_CLASS_NAME)
    }
}

// 获取图片id
pub fn get_wx_temp_imag_id (img_path:&str) -> String {
    let file_name = std::path::Path::new(img_path)
        .file_name()
        .unwrap_or_else(|| "".as_ref());

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

    id
}

// 从历史记录中查找图片id
pub fn get_history_attach_name (temp_imag_id:&str) -> Option<String>{
    let mut walk_attach_file_history = wh_mod::get_walk_attach_file_history();

    for (key, path_list) in walk_attach_file_history {
        for path in path_list {
            let resolve_path = path.to_string_lossy();
            if resolve_path.contains(temp_imag_id) {
                return Some(resolve_path.to_string());
            }
        }
    }

    Option::None
}

fn push_message (message:&str,must_reach:bool) -> bool {

    if !must_reach && get_bool!(PUSH_MESSAGE_ING) {
        return false;
    }

    set_bool!(PUSH_MESSAGE_ING,true);

    let mutex = Arc::new(Mutex::new(&STATUS_PREVIEW_BUF_BIND));
    mutex.lock();
    let the_value:usize = STATUS_PREVIEW_BUF_BIND.load(Ordering::SeqCst);

    unsafe{

        let mut buf = STATUS_PREVIEW_BUF.as_mut().unwrap();
        buf.remove(0,buf.length());
        buf.append(message);

    };
    STATUS_PREVIEW_BUF_BIND.store(the_value+1, Ordering::SeqCst);

    drop(mutex);

    std::thread::spawn(||{
        std::thread::sleep(std::time::Duration::from_millis(500u64));
        set_bool!(PUSH_MESSAGE_ING,false);
    });

    true
}

fn initialize_watch_attach_puppet(imag_id: &str){
    set_bool!(SCAN_SCAN_ING,true);

    let mut user_select_wxid = global_var::get_string_default("user::config::user_select_wxid");
    let mut input_select_dir = global_var::get_string_default("user::config::user_select_path");
    let imag_id_copy2 = format!("{}",&imag_id);

    if input_select_dir.is_empty() {
        set_bool!(SCAN_SCAN_ING,false);
        let start = get_option_arc_bind_variable_or!(STATUS_TIME,STATUS_TIME_BIND,std::time::Instant::now());
        let mut input_data = format!("没有 WX根目录 已经结束扫描 用时: {:?}",start.elapsed());
        gui_util::message::sub_message(get_the_hwnd!(),gui_util::message::IconType::Failure,input_data.as_str(),3500u64);

        unsafe {
            let mut buf = STATUS_PREVIEW_BUF.as_mut().unwrap();
            buf.remove(0,buf.length());
            buf.append(input_data.as_str());
        }

        return;
    }

    let (tx, rx) = std::sync::mpsc::channel();

    // 开始启动扫描的线程
    std::thread::spawn(move || {
        // 启动扫描线程
        // 用户数据根目录
        let mut msg_attach_root = HashSet::new();


        // 优先考虑当前已选用户
        let msg_attach_dir =  format!("{}\\{}\\FileStorage\\MsgAttach", &input_select_dir,&user_select_wxid);
        msg_attach_root.insert(wh_mod::resolve_path(msg_attach_dir));

        // 扩展到全局
        let wx_read_root_wxid = wh_mod::wx_read_root_wxid(std::path::Path::new(input_select_dir.as_str()));
        for wx_read_wxid in wx_read_root_wxid {
            let path2str = wx_read_wxid.attach.to_string_lossy().to_string();
            msg_attach_root.insert(wh_mod::resolve_path(path2str));
        }

        // 启动扫描线程
        for path_for in msg_attach_root {
            if (has_window()) {
                wh_mod::walk_file(std::path::Path::new(path_for.as_str()), tx.clone(), "".to_string());
            }
        }
    });

    // 获取数据的线程
    std::thread::spawn(move || {
        let mut walk_next_not_message: usize = 0;

        loop {
            if let Result::Ok((attach_key, paths)) = rx.recv() {

                if let Some(file_name) = path::Path::new(attach_key.as_str()).file_name() {
                    push_message(format!("[扫描]当前：{:?}",wh_mod::get_show_mask_text(file_name)).as_str(),false);
                }

                for path in paths {
                    let resolve_path = path.to_string_lossy();
                    if resolve_path.contains(imag_id_copy2.as_str()) {
                        let start = get_option_arc_bind_variable_or!(STATUS_TIME,STATUS_TIME_BIND,std::time::Instant::now());
                        let att_info = wh_mod::wx_parse_path(resolve_path.to_string());

                        let mut input_data = format!("用户<{}> [已选定] 用时: {:?}", wh_mod::get_show_mask_text(&att_info.attach_id), start.elapsed());
                        global_var::set_string("user::config::walk_drag_path",resolve_path.clone().to_string());

                        set_bool!(SCAN_SCAN_ING,false);
                        println!("{}", &input_data);
                        push_message(input_data.as_str(),true);
                        break;
                    }
                }
            } else {
                walk_next_not_message += 1;

                if (walk_next_not_message > 50) {
                    set_bool!(SCAN_SCAN_ING,false);
                    // let start = get_option_arc_bind_variable_or!(STATUS_TIME,STATUS_TIME_BIND,std::time::Instant::now());
                    // let mut input_data = format!("扫描结束 用时: {:?}", start.elapsed());

                    // push_message(input_data.as_str(),true);

                    // println!("{}", &input_data);
                    break;
                }
            }
        }
    });


}

// 判断窗口是否有效
pub fn has_window()->bool{
    let hwnd = get_the_hwnd!(THE_WIN_CLASS_NAME);
    libWxIkunPlus::isWindow(hwnd)
}

struct  UiControl{
    btn_close: gui_util::hotspot::HotspotItmeControl,
    id_preview: gui_util::text::TextPreview,
    status_preview: gui_util::text::TextPreview,
    progress_bar:gui_util::img::ImgPreview
}

// 進度條
fn show_progress_bar_border(x: i32, y: i32) -> gui_util::img::ImgPreview {
    let mut progress_bar_border = gui_util::img::ImgPreview::new_border(x,y,521,15,"<svg width=\"520\" height=\"15\" viewBox=\"0 0 520 15\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\"> <rect x=\"0.5\" y=\"0.5\" width=\"520\" height=\"14\" rx=\"7\" fill=\"#181818\" stroke=\"#2C2C2C\"/> </svg> ");
    let width = progress_bar_border.width;
    let mut progress_bar_border_slider = gui_util::img::ImgPreview::new_border(x, y, 41, 15, "<svg width=\"40\" height=\"15\" viewBox=\"0 0 40 15\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\">\n<rect x=\"0.5\" y=\"0.5\" width=\"39\" height=\"14\" rx=\"7\" fill=\"#333333\" stroke=\"#2C2C2C\"/>\n</svg>");
    progress_bar_border_slider.preview.hide();
    progress_bar_border.preview.hide();

    progress_bar_border.preview.handle({
        let mut progress_bar_border_slider = progress_bar_border_slider.clone();
        let mut preview_main = progress_bar_border.preview.clone();

        move |win, ev| match ev {
            enums::Event::Show => {
                progress_bar_border_slider.preview.show();

                let mut preview = progress_bar_border_slider.preview.clone();
                let mut preview_main = preview_main.clone();

                app::add_timeout3(0.0086, move |handle| {
                    if !preview.visible()||!preview_main.visible() {
                        preview_main.hide();
                        preview.hide();
                        app::remove_timeout3(handle);
                        return;
                    }

                    let mut the_x = preview.x() + 2;
                    if the_x > width {
                        the_x = x + preview.width();
                        the_x -= preview.width();
                    }
                    preview.set_pos(the_x, preview.y());
                    preview.parent().unwrap().redraw();
                    app::repeat_timeout3(0.0086, handle);
                });
                true
            }
            enums::Event::Hide => {
                progress_bar_border_slider.preview.hide();
                true
            }
            _ => false,
        }
    });

    // progress_bar_border.preview.show();
    progress_bar_border
}

fn add_ui_control() -> UiControl{
    gui_util::text::TextControl::new(150,39,225,15, 12, "请从聊天窗口拖拽一张图片到本窗口  (部分PC需要拖拽两次)", [149, 149, 149]);
    gui_util::text::TextControl::new(35,258,75,15, 12, "当前图片ID:", [149, 149, 149]);
    gui_util::text::TextControl::new(35,281,75,15, 12, "当 前 状 态 :", [149, 149, 149]);
    gui_util::text::TextControl::new(265,180,70,15, 12, "拖拽到此处", [57, 57, 57]);
    let btn_close = gui_util::hotspot::create_hotspot(537,33,25,25);
    let mut id_preview = gui_util::text::TextPreview::new(35+75,258,450,15, 12, "null", [57, 57, 57]);
    let mut status_preview = gui_util::text::TextPreview::new(35+75,281,450,15, 12, "尚未拖拽入文件", [57, 57, 57]);
    let mut progress_bar = show_progress_bar_border(40,313);

    let mut  buf = status_preview.buf.clone();
    let mut progress_bar_preview =progress_bar.preview.clone();

    set_option_arc_bind_variable!(STATUS_PREVIEW_BUF,STATUS_PREVIEW_BUF_BIND,status_preview.buf.clone());

    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_millis(500u64));
        if !has_window() {
            return;
        }

        if !get_bool!(SCAN_SCAN_ING) {
            if has_window() { progress_bar_preview.hide();}

            let start = get_option_arc_bind_variable!(STATUS_TIME,STATUS_TIME_BIND);

            if let Some(start) = start {
                std::thread::sleep(std::time::Duration::from_millis(300u64));

                if !has_window() {
                    buf.remove(0,buf.length());
                    let walk_drag_path = global_var::get_string_default("user::config::walk_drag_path");
                    if walk_drag_path.is_empty() {
                        let mut input_data = format!("扫描结束 用时约为: {:?} ", start.elapsed());
                        buf.append(wh_mod::get_show_mask_text(&input_data).as_str());
                    }
                    else{
                        let att_info = wh_mod::wx_parse_path(walk_drag_path.to_string());
                        let mut input_data = format!("ID<{}> [已选定] 用时约为: {:?}",wh_mod::get_show_mask_text(att_info.attach_id) , start.elapsed());
                        buf.append(wh_mod::get_show_mask_text(&input_data).as_str());
                    }
                };


            }

            unsafe {
                let mutex = Arc::new(Mutex::new(&STATUS_TIME_BIND));
                mutex.lock();
                let the_value:usize = STATUS_TIME_BIND.load(Ordering::SeqCst);

                unsafe{ STATUS_TIME = Option::None;};

                STATUS_TIME_BIND.store(the_value+1, Ordering::SeqCst);

                drop(mutex);
            }

        }


    });

     UiControl{
        btn_close,
        id_preview,
        status_preview,
        progress_bar,
    }
}

// 初始化窗口
pub fn main_window(match_input:&str)->Option<DoubleWindow> {

    // 禁止创建多个窗口
    if let hwnd = get_the_hwnd!() {
        if hwnd!=0 && libWxIkunPlus::isWindow(hwnd) {
            if let Some(mut win) =app::widget_from_id(THE_WIN_CLASS_NAME) as Option<DoubleWindow>
            {
                win.show();
                win.set_visible_focus();
            }
            libWxIkunPlus::setWindowShake(hwnd);
            return Option::None;
        }
    }


    let mut win: DoubleWindow = fltk::window::DoubleWindow::new(0, 0, 600,360, "扫描图源用户").center_screen();
    let mut rect = libWxIkunPlus::getWindowRect(libWxIkunPlus::findWindow(gui_select_user_ui::THE_WINDOW_CLASS_NAME,""));
    win.set_pos(rect.left+8,rect.top+31);

    inject_fltk_theme!();
    win.set_color(Color::from_rgb(24, 24, 24));
    // win.set_border(false);
    set_item_id!(win,THE_WIN_CLASS_NAME);
    gui_util::img::ImgPreview::new_border(0,0,600,360,THE_WIN_UI_BORDER);
    let mut win_control = add_ui_control();
    let mut copy_progress_bar =win_control.progress_bar.preview.clone();


    win.handle({
        let mut x = 0;
        let mut y = 0;
        let mut released = true;
        let mut dnd = true;
        let mut drag_path = std::path::PathBuf::new();
        let mut id_preview = win_control.id_preview.clone();
        let mut status_preview = win_control.status_preview.clone();

        move |win, ev| match ev {
            enums::Event::Show => {
                win.set_visible_focus();
                true
            }
            enums::Event::Close => {
                gc_the_window!(win);
                false
            }
            enums::Event::Hide => {
                gc_the_window!(win);
                false
            }
            enums::Event::Push => {
                if win_control.btn_close.existPoint(x,y) {
                    gc_the_window!(win);
                }
                true
            }

            enums::Event::Move => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;
                if win_control.btn_close.existPoint(x,y) {
                    win.set_cursor(fltk::enums::Cursor::Hand);
                } else {
                    win.set_cursor(fltk::enums::Cursor::Default);
                }

                true
            }

            enums::Event::DndEnter => {
                dnd = true;
                true
            }
            enums::Event::DndDrag => true,
            enums::Event::DndRelease => {
                released = true;
                true
            }
            enums::Event::Paste => {
                if dnd && released {
                    if get_bool!(SCAN_SCAN_ING) {
                        gui_util::message::sub_message(get_the_hwnd!(THE_WIN_CLASS_NAME), gui_util::message::IconType::Failure, "请等待本次扫描完成！", 3500u64);
                        return false;
                    }
                    set_option_arc_bind_variable!(STATUS_TIME,STATUS_TIME_BIND,std::time::Instant::now());
                    // set_arc_bind_variable_string_replace_data!(SCAN_SCAN_PATH,SCAN_SCAN_PATH_BIND,"");
                    global_var::set_string("user::config::walk_drag_path",String::new());

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

                        if path_list.is_empty() {
                            gui_util::message::sub_message(get_the_hwnd!(THE_WIN_CLASS_NAME), gui_util::message::IconType::Warning, "拖拽的内容不存在文件路径", 3500u64)
                        }else{
                            drag_path.clear();
                            if let Some(pop) = path_list.pop() {

                                drag_path.push(pop.clone());

                                // 获取id 并开始处理
                                let temp_imag_id = get_wx_temp_imag_id(pop.as_str());

                                if !temp_imag_id.is_empty() {
                                    let mut  buf = id_preview.buf.clone();
                                    buf.remove(0,buf.length());
                                    buf.append(wh_mod::get_show_mask_text(&temp_imag_id).as_str());
                                    win_control.progress_bar.preview.show();

                                    let mut new_buf_str = String::new();
                                    if let Some(get_history_attach) = get_history_attach_name(temp_imag_id.as_str()) {
                                        let start = get_option_arc_bind_variable_or!(STATUS_TIME,STATUS_TIME_BIND,std::time::Instant::now());
                                        let att_info = wh_mod::wx_parse_path(get_history_attach.to_string());
                                        global_var::set_string("user::config::walk_drag_path",get_history_attach.clone().to_string());
                                        new_buf_str.push_str( format!("ID<{}> [已选定] 用时: {:?}",att_info.attach_id,start.elapsed()).as_str());
                                        gui_util::message::sub_message(get_the_hwnd!(THE_WIN_CLASS_NAME), gui_util::message::IconType::Success, new_buf_str.as_str(), 3500u64);
                                        set_bool!(SCAN_SCAN_ING,false);

                                    }else{
                                        new_buf_str.push_str("开始扫描拽入的文件");
                                        set_bool!(SCAN_SCAN_ING,true);
                                        initialize_watch_attach_puppet(temp_imag_id.as_str());
                                    }

                                    push_message(new_buf_str.as_str(),true);
                                }

                                else{
                                    gui_util::message::sub_message(get_the_hwnd!(THE_WIN_CLASS_NAME), gui_util::message::IconType::Failure, "未找到此文件中的有效图片id", 3500u64);
                                }

                                println!("path_list-> {:?}",&drag_path);
                            }

                        }

                        dnd = false;
                        released = false;
                    true
                } else {
                    false
                }
            }
            enums::Event::DndLeave => {
                dnd = false;
                released = false;
                true
            }

            _ => false,
        }
    });

    win.end();
    win.show();

    // 支持传值
    if !match_input.is_empty(){

        set_option_arc_bind_variable!(STATUS_TIME,STATUS_TIME_BIND,std::time::Instant::now());
        global_var::set_string("user::config::walk_drag_path",String::new());

        // 获取id 并开始处理
        let temp_imag_id = get_wx_temp_imag_id(match_input);

        if !temp_imag_id.is_empty() {
            let mut  buf = win_control.id_preview.buf.clone();
            buf.remove(0,buf.length());
            buf.append(wh_mod::get_show_mask_text(&temp_imag_id).as_str());
            copy_progress_bar.show();

            let mut new_buf_str = String::new();
            if let Some(get_history_attach) = get_history_attach_name(temp_imag_id.as_str()) {
                let start = get_option_arc_bind_variable_or!(STATUS_TIME,STATUS_TIME_BIND,std::time::Instant::now());
                let att_info = wh_mod::wx_parse_path(get_history_attach.to_string());
                global_var::set_string("user::config::walk_drag_path",get_history_attach.clone().to_string());
                new_buf_str.push_str( format!("ID<{}> [已选定] 用时: {:?}",wh_mod::get_show_mask_text(att_info.attach_id),start.elapsed()).as_str());
                gui_util::message::sub_message(get_the_hwnd!(THE_WIN_CLASS_NAME), gui_util::message::IconType::Success, new_buf_str.as_str(), 3500u64);
                set_bool!(SCAN_SCAN_ING,false);

            }else{
                new_buf_str.push_str("开始扫描拽入的文件");
                set_bool!(SCAN_SCAN_ING,true);
                initialize_watch_attach_puppet(temp_imag_id.as_str());
            }

            push_message(new_buf_str.as_str(),true);
        }

        else{
            gui_util::message::sub_message(get_the_hwnd!(THE_WIN_CLASS_NAME), gui_util::message::IconType::Failure, "未找到此文件中的有效图片id", 3500u64);
        }

    }

    libWxIkunPlus::setWinIcon(get_the_hwnd!());
    libWxIkunPlus::setWinTop(get_the_hwnd!(),true);

    Some(win)
}
#![allow(warnings, unused)]

use crate::{get_bool, global_var, gui_util, inject_fltk_theme, set_bool, set_item_id};
use fltk::enums::{Color, FrameType};
use fltk::window::DoubleWindow;
use fltk::{prelude::*, *};
use fltk_theme::{color_themes, ColorTheme, SchemeType, ThemeType, WidgetScheme, WidgetTheme};
use std::collections::{HashMap, HashSet};
use std::path;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicI64, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, MutexGuard, OnceLock, RwLock};
use std::thread::sleep;
use fltk::examples::tile;
use crate::libWxIkunPlus;
use crate::gui_util::{*};
use crate::{*};
mod lib;


pub(crate) const THE_WIN_CLASS_NAME: &str = "wx_auto_ex_im::gui_util::main::main<55216>";
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
    title: TextControl,
    text_control_list: Vec<TextControl>,
    check:FrameCheckButton,
    task_command: fltk::input::Input,
    name: fltk::input::Input,
    export: fltk::input::Input,
    console: FrameConsole,
}

struct FrameCheckButton{
    flex:  fltk::group::Flex,
    sync:  button::CheckButton,
    start_up:  button::CheckButton,
}

struct FrameConsole{
    buf: fltk::text::TextBuffer,
    txt: fltk::text::TextEditor,
}

fn add_ui_control() -> UiControl {
    let btn_close = gui_util::hotspot::create_hotspot(556,26,25,25);
    let title = gui_util::TextControl::new(60-30 , 24, 150, 20, 12, " WX 图片自动备份", [122, 120, 120]);
    let text_control_list =vec![
        TextControl::new(30-2, 80, 70, 15, 12, "创建新任务", [85,85,85]),
        TextControl::new(15, 125, 385, 15, 12, "dat 图片所在文件夹（点击右侧按钮在任务创建向导中选定）：", [85,85,85]),
        TextControl::new(-8, 200, 200, 15, 12, "导出到此文件夹：", [85,85,85]),
        TextControl::new(506, 160, 35, 15, 12, "向导", [85,85,85]),
        TextControl::new(10, 280+5, 115, 15, 13, "名称：", [85,85,85]),
        TextControl::new(356-1, 280+4, 35, 15, 13, "管理", [85,85,85]),
        TextControl::new(428-1, 280+4, 35, 15, 13, "测试", [85,85,85]),
        TextControl::new(500-1, 280+4, 35, 15, 13, "创建", [85,85,85]),
        TextControl::new(30-1, 333, 56, 15, 12, "执行日志", [85,85,85]),
    ];
    let mut flex = group::Flex::default()
        .with_size(200, 30)
        .row()
        .center_of_parent();

    flex.set_pos(30, 490-5);
    let mut check_button_sync = button::CheckButton::default().with_label("同步开关");
    let mut check_button_start_up = button::CheckButton::default().with_label("开机启动");
    flex.end();

    check_button_sync.set_checked(libWxIkunPlus::has_auto_sync());
    check_button_start_up.set_checked(libWxIkunPlus::hasStartup());

    // 同步
    check_button_sync.set_callback(|win|{
        libWxIkunPlus::set_auto_sync(win.is_checked());

        if libWxIkunPlus::has_auto_sync() {
            message::sub_message(get_the_hwnd!(),message::IconType::Success,"同步状态已被启用",3500u64);
        }else {
            message::sub_message(get_the_hwnd!(),message::IconType::Info,"同步状态已被禁用",3500u64);
        }
        // 用锁定线程来实现防抖
        app::sleep(0.300);
    });

    // 自启
    check_button_start_up.set_callback(|win|{
        if win.is_checked()!=libWxIkunPlus::hasStartup() {
            libWxIkunPlus::setStartup();
        }

        if libWxIkunPlus::hasStartup() {
            message::sub_message(get_the_hwnd!(),message::IconType::Success,"开机自启动已被启用",3500u64);
        }else {
            message::sub_message(get_the_hwnd!(),message::IconType::Info,"开机自启动已被禁用",3500u64);
        }
        // 用锁定线程来实现防抖
        app::sleep(0.300);
    });

    let mut task_command_input = input::Input::new(45, 152, 423, 30, "");
    let mut export_input = input::Input::new(45, 225, 450, 30, "");
    let mut name_input = input::Input::new(96, 276, 230, 30, "");
    task_command_input.set_readonly(!wh_mod::config::is_developer());

    let mut buf = fltk::text::TextBuffer::default();
    buf.append(lib::get_init_text().as_str());
    let mut txt = fltk::text::TextEditor::default()
        .with_size(530, 105)
        .center_of_parent();
    txt.set_buffer(buf.clone());
    txt.set_frame(FrameType::NoBox);
    txt.set_text_color(Color::from_rgb(120, 120, 120));
    txt.set_label_type(fltk::enums::LabelType::None);
    txt.set_text_size(12);
    txt.set_pos(33, 362);
    txt.set_scrollbar_size(6);
    txt.show();

    let mut move_buf = buf.clone();

    thread::spawn(move || loop {
        Sleep(150);
        let mut console_message = handle_dat::get_console_message().replace("\n\n", "\n");

        if console_message.starts_with('\n') {
            console_message = console_message.trim_start_matches('\n').to_string();
        }

        if (console_message.len() < 5) {
            continue;
        };

        let mut newline_count = 0;

        for line in move_buf.text().lines() {
            newline_count += 1
        }

        if (newline_count > 5) {
            move_buf.remove(0, move_buf.length());
            move_buf.set_text(&console_message);
        } else {
            move_buf.append(&format!("\n{}", &console_message));
        }
    });


    UiControl {
        btn_close,
        title,
        text_control_list,
        task_command:task_command_input,
        name:name_input,
        export:export_input,
        check:FrameCheckButton{
            flex,
            sync:check_button_sync,
            start_up:check_button_start_up
        },
        console:FrameConsole{
            buf,
            txt
        }
}
}


pub fn main_init() ->Option<fltk::window::DoubleWindow> {
    main_init_check!();
    let hwnd:i128 = 0 ;
    let mut win: DoubleWindow = fltk::window::DoubleWindow::new(0, 0, 600,530, "WxAutoExIm").center_screen();
    inject_fltk_theme!();
    win.set_color(Color::from_rgb(24, 24, 24));
    win.set_border(false);
    set_item_id!(win,THE_WIN_CLASS_NAME);
    gui_util::img::ImgPreview::new_border(0,0,win.w(),win.h(),THE_WIN_UI_BORDER);
    let mut win_control = add_ui_control();
    let mut button_wizard = gui_util::hotspot::create_hotspot(475i32, 150i32 , 72i32, 32i32);
    let mut button_open_export_dir = gui_util::hotspot::create_hotspot(513i32, 225i32 , 33i32, 32i32);
    let mut button_show_manage = gui_util::hotspot::create_hotspot(342i32, 275i32 , 60i32, 32i32);
    let mut button_test = gui_util::hotspot::create_hotspot(415i32, 275i32 , 60i32, 32i32);
    let mut button_create = gui_util::hotspot::create_hotspot(486i32, 275i32 , 60i32, 32i32);
    let mut bottom_check_hotspot = gui_util::hotspot::create_hotspot(30, 490-5,200, 30);


    win.handle({
        let mut x = 0;
        let mut y = 0;
        move |win, ev| match ev {
            enums::Event::Show => {
                env::set_var("ikunWinHwnd", format!("{}",get_the_hwnd!()).to_string());
                libWxIkunPlus::setWinIconMain(get_the_hwnd!());

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
                // 关闭
                if win_control.btn_close.existPoint(x,y){
                    // gc_the_window!(win);
                    libWxIkunPlus::setwinVisible(get_the_hwnd!(),false);
                }
                // 向导
                if button_wizard.existPoint(x,y) {
                    if(lib::eq_next()){
                        let mut token_id =  gui_select_user_ui::manage_tool_main();
                        println!("token_id-> {}",&token_id);
                        if(!token_id.is_empty()){

                            let mut task_command =  win_control.task_command.clone();
                            app::add_timeout3(0.3,move|handle|{
                                if !gui_select_user_ui::has_window(){

                                    let data = global_var::get_string_default(token_id.as_str());
                                    if !data.is_empty() {
                                        println!("{}",data);
                                        gui_util::sub_message(get_the_hwnd!(), gui_util::IconType::Success, "向导任务命令已赋值", 3500u64);
                                        task_command.set_value(wh_mod::get_show_mask_text(data.as_str()).as_str());
                                        global_var::set_string("user::config::task_command",data.clone());
                                    }else {
                                        gui_util::sub_message(get_the_hwnd!(), gui_util::IconType::Warning, "用户取消任务创建", 3500u64);
                                    }

                                    app::remove_timeout3(handle);
                                    return ;
                                }
                                app::repeat_timeout3(0.3, handle);
                            });

                        }else {
                            if gui_select_user_ui::has_window(){
                                gui_util::sub_message(get_the_hwnd!(), gui_util::IconType::Warning, "窗口重复创建", 3500u64);
                            }else {
                                gui_util::sub_message(get_the_hwnd!(), gui_util::IconType::Warning, "创建任务id失败", 3500u64);
                            }
                        }

                    }else{
                        libWxIkunPlus::stop("错误".to_owned(),"当前未发现wx进程或者未登录 拒绝提供选取方案".to_owned());
                    }
                }

                // 选择导出到
                if button_open_export_dir.existPoint(x,y) {
                    let mut open_path = libWxIkunPlus::openSelectFolder2();
                    // 非空
                    if !open_path.is_empty() {
                        win_control.export.set_value(open_path.as_str());
                    }
                    // 没有名称 则使用目录名
                    if win_control.name.value().is_empty() {
                        let path = path::Path::new(open_path.as_str());
                        if let Some(file_name) = path.file_name() {
                            if let Some(file_name) = file_name.to_str() {
                                win_control.name.set_value(file_name);
                            }
                        }
                    }
                }

                // 测试
                if button_test.existPoint(x,y) {
                    let mut task_command = global_var::get_string_default("user::config::task_command");
                    let [name,export]= [win_control.name.value(),win_control.export.value()];
                    lib::test_task(name.as_str(),export.as_str(),task_command.as_str());
                }
                // 管理
                if button_show_manage.existPoint(x,y) {
                    gui_manage_item::ManageItmeMain();
                }
                // 创建
                if button_create.existPoint(x,y) {
                    let mut task_command = global_var::get_string_default("user::config::task_command");
                    let [name,export]= [win_control.name.value(),win_control.export.value()];
                    lib::push_sql_export_dir_path(name.as_str(),export.as_str(),task_command.as_str());
                }
                true
            }
            enums::Event::Move => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;

                if win_control.btn_close.existPoint(x,y)
                    ||button_wizard.existPoint(x,y)
                    ||button_open_export_dir.existPoint(x,y)
                    ||button_show_manage.existPoint(x,y)
                    ||button_test.existPoint(x,y)
                    ||button_create.existPoint(x,y)
                    ||bottom_check_hotspot.existPoint(x,y)
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

    loop {
        Sleep(200);
        if (libWxIkunPlus::hasInitWindowIsDisplayed())
        {
            win.show();
            win.set_visible_focus();
            return  Some(win);
        }
    }

    Option::None
}
#![allow(warnings, unused)]

use fltk::enums::{Color, FrameType};
use fltk::window::DoubleWindow;
use fltk::{prelude::*, *};
use fltk_theme::{color_themes, ColorTheme, SchemeType, ThemeType, WidgetScheme, WidgetTheme};
use crate::{gui_util, set_item_id};


pub fn manage_tool_main() {
    let mut win = window::Window::default().with_size(600, 450).center_screen();
    win.set_label("用户任务管理");
    set_item_id!(win, "gui::manage_tool::main<win>");
    // win.set_border(false);
    // 退出窗口
    let exit_btn = gui::hotspot::create_hotspot(540, 15, 37, 37);

    let mut preview =
        gui::img::ImgPreview::new(0, 0, win.w(), win.h(), "gui::rename_tool::main<win>");
    preview.from_svg(
        include_str!("src/contour.svg"),
        0,
        0,
        preview.preview.w(),
        preview.preview.h(),
    );
    preview.preview.set_id("gui::rename_tool::main<contour>");

    gui::text::TextControl::new(60 - 25, 24, 150, 20, 15, "用户任务管理", [122, 120, 120]);





    win.handle({
        let mut x = 0;
        let mut y = 0;

        move |win, ev| match ev {
            enums::Event::Show => {
                win.set_visible_focus();

                true
            }
            enums::Event::KeyUp => true,

            enums::Event::Push => {
                if exit_btn.existPoint(x, y) {
                    fltk::window::Window::delete(win.clone());
                }

                true
            }

            enums::Event::Move => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;

                // 判断是否显示手型鼠标
                if exit_btn.existPoint(x, y) {
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

    win.show();
}
#![allow(warnings, unused)]

use crate::{global_var, gui_util, libWxIkunPlus, set_item_id, util};
use fltk::enums::{Color, FrameType};
use fltk::window::DoubleWindow;
use fltk::{prelude::*, *};
use fltk_theme::{color_themes, ColorTheme, SchemeType, ThemeType, WidgetScheme, WidgetTheme};
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Local};
pub(crate) const THE_WINDOW_CLASS_NAME: &'static str = "wx_auto_ex_im::gui_util::rename_tool::main<32626>";




macro_rules! the_token {
    ()=>{
       {
        let mut _the_token =format!("token<{}>@query",libWxIkunPlus::randomNum());
        loop{
            if global_var::has_string(_the_token.as_str()) {
                _the_token = format!("token<{}>@query",libWxIkunPlus::randomNum());
            }else{
                break;
            }
        }
            _the_token
        }
    }
}

pub fn rename_tool_main(input:&str) -> String {

    let mut the_token = the_token!();

    let mut win = window::Window::default().with_size(600, 550).center_screen();
    win.set_label("命名规则工具");
    set_item_id!(win, THE_WINDOW_CLASS_NAME);
    win.set_border(false);
    let mut rename_variable_input_oid_str = String::new();
    let time_info =util::get_time_info();
    
    let mut preview =
    gui_util::img::ImgPreview::new(0, 0, win.w(), win.h(), THE_WINDOW_CLASS_NAME);
    preview.from_svg(
        include_str!("./src/contour.svg"),
        0,
        0,
        preview.preview.w(),
        preview.preview.h(),
    );
    preview.preview.set_id("gui_util::rename_tool::main<contour>");

    let mut title =
    gui_util::text::TextControl::new(60 - 25, 24, 150, 20, 15, "命名规则工具", [122, 120, 120]);
    let mut title_box_01 = gui_util::text::TextControl::new(
        210 - 15, /* -15 Frame偏差*/
        76,
        206,
        18,
        12,
        "规则自变量 (自动)   单击可以输入",
        [122, 120, 120],
    );

    // 时间变量：（第一行）
    gui_util::text::TextControl::new(36, 113, 61, 18, 12, "时间变量:", [122, 120, 120]);
    //其他变量: （第二行）
    gui_util::text::TextControl::new(36, 156, 61, 18, 12, "其他变量:", [122, 120, 120]);
    //自增序列(必选): （第三行）
    gui_util::text::TextControl::new(36, 250, 95, 18, 12, "自增序列(必选):", [122, 120, 120]);

    //预览变量 (存储文件名)：
    gui_util::text::TextControl::new(
        36 - 6 + 32,
        318,
        150,
        18,
        12,
        "命名规则  变量公式 [ 在此可编辑 ]  ：",
        [122, 120, 120],
    );
    //预览变量 (存储文件名)：
    gui_util::text::TextControl::new(
        36,
        386 - 3, /* -3 Frame偏差*/
        150,
        18,
        12,
        "预览变量 (存储文件名)：",
        [122, 120, 120],
    );

    gui_util::text::TextControl::new(192, 505 - 7, 35, 18, 13, "取消", [121, 121, 121]);
    gui_util::text::TextControl::new(371 - 3, 505 - 7, 35, 18, 13, "确认", [122, 120, 120]);

    // 命名规则
    let mut rename_variable_input = input::Input::new(35 - 1, 346, 533, 30, "");
    rename_variable_input.set_id("gui_util::rename_variable_input");
    let rename_variable_input_hotspot = gui_util::hotspot::create_hotspot(35 - 1, 346, 533, 30);
    rename_variable_input.append(if input.is_empty() {"<创建月>/<任务名>/<类型>_<NN>"}else { input });

    // 预览
    let mut rename_preview_input = input::Input::new(35 - 1, 399 + 10, 533 /*490 - 12*/, 30, "");
    rename_preview_input.set_readonly(true);
    rename_preview_input.set_id("gui_util::rename_preview_input");

    // 后缀名
    // let rename_preview_ext =
    //     gui_util::text::TextControl::new(528 - 15, 399 + 16, 35, 20, 13, ".jpg", [122, 120, 120]);
    // rename_variable_input.set_id("gui_util::rename_preview_ext");

    // 退出窗口
    let exit_btn = gui_util::hotspot::create_hotspot(540, 15, 37, 37);

    // 确认/取消
    let mut cancel_btn = gui_util::hotspot::create_hotspot(147, 488 - 2, 125, 38);
    let mut confirm_btn = gui_util::hotspot::create_hotspot(139 + 8 + 170 + 5, 488 - 2, 125, 38);
    let mut variable_list = Vec::from([
        gui_util::variable_tag_control::varTagControl::new(
            106,
            108,
            128,
            31,
            "现在:",
            &time_info.time,
            &time_info.time,
        ),
        gui_util::variable_tag_control::varTagControl::new(245, 108, 72, 31, "年:", &time_info.years, &time_info.years),
        gui_util::variable_tag_control::varTagControl::new(328, 108, 72, 31, "月:", &time_info.month, &time_info.month),
        gui_util::variable_tag_control::varTagControl::new(388, 108, 72, 31, "日:", &time_info.day, &time_info.day),
        gui_util::variable_tag_control::varTagControl::new(450 - 2, 108, 72, 31, "时:", &time_info.hour, &time_info.hour),
        gui_util::variable_tag_control::varTagControl::new(508, 108, 72, 31, "分:", &time_info.minutes, &time_info.minutes),
        gui_util::variable_tag_control::varTagControl::new(
            106,
            150,
            178,
            31,
            "别名:",
            "软件内的用户备注名",
            "事妈老板",
        ),
        gui_util::variable_tag_control::varTagControl::new(
            293,
            150,
            182,
            31,
            "任务名:",
            "当前任务的任务名",
            "工作内容备份",
        ),
        gui_util::variable_tag_control::varTagControl::new(483, 150, 83, 31, "创建月:", "月", &time_info.time_years),
        gui_util::variable_tag_control::varTagControl::new(
            33,
            196,
            221,
            31,
            "类型:",
            "缩略图,视频,图片,手机截图",
            "图片",
        ),
        gui_util::variable_tag_control::varTagControl::new(
            265,
            196,
            100,
            31,
            "哈希:",
            "dat名称",
            "666a6b6666666abc999aa9b6bc99999a",
        ),
        gui_util::variable_tag_control::varTagControl::new(146, 231 + 13, 53, 31, "%N:", "1", "1"),
        gui_util::variable_tag_control::varTagControl::new(205, 231 + 13, 73, 31, "%NN:", "01", "01"),
        gui_util::variable_tag_control::varTagControl::new(289, 231 + 13, 92, 31, "%NNN:", "001", "001"),
        gui_util::variable_tag_control::varTagControl::new(
            392,
            231 + 13,
            115,
            31,
            "%NNNN:",
            "0001",
            "0001",
        ),
    ]);


    let mut rename_variable_input_copy = rename_variable_input.clone();
    let mut rename_preview_input_copy = rename_preview_input.clone();

    macro_rules! update_variable_input {
        () => {

            if !rename_variable_input_copy.value().as_bytes().eq(rename_variable_input_oid_str.as_bytes()){

            // 更新预览
            let mut label = rename_variable_input_copy.value();

            // 替换变量值
            for index in 1..variable_list.len() + 1 {
                if let Some(variable) = variable_list.get(variable_list.len() - index) {
                    label = label.replace(variable.get_var().as_str(), variable.data.as_str());
                }
            }
            rename_variable_input_oid_str = format!("{}{}",label.as_str(),".jpg");
            // 设置预览文本
            rename_preview_input_copy.set_value(rename_variable_input_oid_str.as_str());

            }
        };
    }

    update_variable_input!();

    win.handle({
        let mut the_token =the_token.clone();
        let mut x = 0;
        let mut y = 0;
        move |win, ev| match ev {
            enums::Event::Show => {
                win.set_visible_focus();

                true
            }
            enums::Event::KeyUp=>{
                update_variable_input!();
                true
            }

            enums::Event::Push => {
                if exit_btn.existPoint(x, y) {
                    fltk::window::Window::delete(win.clone());
                }

                for variable in variable_list.iter() {
                    if variable.existPoint(x, y) {
                        println!(
                            "click[variable]-> {}  var<  {}  >",
                            variable.id.clone(),
                            variable.get_var()
                        );
                        rename_variable_input_copy.append(variable.get_var().as_str());

                        break;
                    }
                }

                if cancel_btn.existPoint(x,y) {
                    global_var::set_string(the_token.as_str(),String::new());
                    fltk::window::Window::delete(win.clone());
                }

                if confirm_btn.existPoint(x,y){
                    global_var::set_string(the_token.as_str(),rename_variable_input_copy.value());
                    fltk::window::Window::delete(win.clone());
                }

                update_variable_input!();
                true
            }

            enums::Event::Move => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;

                if(rename_variable_input_hotspot.existPoint(x,y)){
                    update_variable_input!();
                }

                //  判断鼠标是否在变量标签上
                let mut is_variable_tag_control = false;
                for variable in variable_list.iter() {
                    is_variable_tag_control = variable.existPoint(x, y);
                    if (is_variable_tag_control) {
                        break;
                    }
                }

                // 判断是否显示手型鼠标
                if exit_btn.existPoint(x, y)
                    || confirm_btn.existPoint(x, y)
                    || cancel_btn.existPoint(x, y)
                    || is_variable_tag_control
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

    win.show();

    the_token.clone()
}
#![allow(warnings, unused)]

use crate::gui_rename_ui::rename_tool_main;
use crate::{gui_util, libWxIkunPlus, global_var, wh_mod, get_arc_bind_variable, atomic_util, inject_fltk_theme, gui_drag_scan2_ui, gui_detect_config_ui, util};
use crate::gui_util::img::ImgPreview;
use crate::gui_util::text::TextControl;
use crate::gui_util::variable_tag_control::varTagControl;
use fltk::enums::{Color, FrameType};
use fltk::window::DoubleWindow;
use fltk::{prelude::*, *};
use fltk_theme::{color_themes, ColorTheme, SchemeType, ThemeType, WidgetScheme, WidgetTheme};
use crate::libWxIkunPlus::findWindow;
use std::ptr::null_mut;
use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::{
    sync::atomic::Ordering,
    sync::Arc,
    sync::MutexGuard,
    sync::{atomic::AtomicUsize, OnceLock},
};
use std::sync::atomic::AtomicBool;
use crate::gui_drag_scan2_ui::{get_history_attach_name, get_wx_temp_imag_id};
use crate::util::Sleep;

mod lib;

pub(crate) const THE_WINDOW_CLASS_NAME : &'static str = "wx_auto_ex_im::gui_util::select_user_ui::main<win:56315:>";
pub(crate) const THE_SUB_WINDOW_CLASS_NAME_FRAME_THUMBNAIL_PREVIEW: &'static str = "wx_auto_ex_im::gui_util::select_user_ui::sub_main<6103>";
pub(crate) const THE_SUB_WINDOW_CLASS_NAME_SCAN: &'static str = "wx_auto_ex_im::gui_util::select_user_ui::sub_main<126126>";
const MAIN_CONTOUR: &str = include_str!("./src/contour.svg");

pub fn ASSETS_NOT_DATA() -> Vec<u8> {
    include_bytes!("./src/not_data.png").to_vec()
}

pub fn ASSETS_DEMO_DATA() -> Vec<u8> {
    include_bytes!("../../assets/icon/demo.png").to_vec()
}

pub fn ASSETS_DEMO_NOT_DATA() -> Vec<u8> {
    include_bytes!("../../assets/icon/demo_not.png").to_vec()
}
pub fn ASSETS_DEMO_NOT_SELECT() -> Vec<u8> {
    include_bytes!("../../assets/icon/demo_not_select.png").to_vec()
}

pub fn ASSETS_NOT_SELECT () -> Vec<u8> {
    include_bytes!("./src/not_select.png").to_vec()
}

macro_rules! set_item_id {
    ($win:expr,$id:expr) => {
        $win.set_xclass($id);
        $win.set_id($id);
    };
}

macro_rules! eq_wxid_dir{
    ($select_dir:expr)=>{
        {
            let mut is_wxid_dir = false;
            if !$select_dir.is_empty(){

                        if !$select_dir.contains("WeChat Files"){
                            // 没有 WeChat Files 则尝试为路径添加 WeChat Files
                            let mut to_path = std::path::Path::new($select_dir.as_str());
                            let mut join_path = to_path.join("WeChat Files");

                            if join_path.exists() && join_path.is_dir(){
                               $select_dir.push_str("\\WeChat Files");
                            }

                        }

                        // 判断路径下是否有 wxid_ 开头的文件夹
                        if let Ok(rd_dir) = std::fs::read_dir($select_dir.as_str()) {

                            for rd_dir in rd_dir {
                                if let Ok(dir) = rd_dir {
                                    is_wxid_dir= dir.file_name().to_string_lossy().contains("wxid_");
                                    if is_wxid_dir{
                                        break;
                                    }
                                }
                            }

                            if !is_wxid_dir{
                                // dialog::alert_default("此路径可能不是有效的WX目录 因为未发现有效的用户数据");
                              gui_util::message::sub_message(libWxIkunPlus::findWindow(THE_WINDOW_CLASS_NAME,""),gui_util::message::IconType::Warning,"此WX目录 未发现有效的用户数据目录",3500u64);
                            }

                        }else{
                            gui_util::message::sub_message(libWxIkunPlus::findWindow(THE_WINDOW_CLASS_NAME,""),gui_util::message::IconType::Failure,"目录无法被打开 请注意路径有效性",3500u64);
                            // dialog::alert_default("目录无法被打开 请注意路径有效性");
                        }

                    }
                   
            is_wxid_dir
        }
    }
}

macro_rules! get_the_hwnd {
    ($class_id:expr) => {
        {
        let mut _hwnd = 0 ;
        for _ in 0..8 {
          _hwnd = libWxIkunPlus::findWindow($class_id,"");
            if !libWxIkunPlus::isWindow(_hwnd) {
                 _hwnd=0;
            }else {
              break;
          }
            fltk::app::sleep(0.020);
        }
        _hwnd as i128}
    };
    ()=>{
        get_the_hwnd!(THE_WINDOW_CLASS_NAME)
    }
}

pub fn has_window() -> bool{
    let hwnd = get_the_hwnd!(THE_WINDOW_CLASS_NAME);
    libWxIkunPlus::isWindow(hwnd)
}

struct FrameText {
    选择: TextControl,
    文件管理: TextControl,
    选择用户: TextControl,
    通过拽入获取: TextControl,
    选择最近对象: TextControl,
    帮助: TextControl,
    别名备注: TextControl,
    用户目录: TextControl,
    命名规则: TextControl,
    编辑规则: TextControl,
    完成选定: TextControl,
    备注: TextControl,
    开始:TextControl
}

// 添加无热点的文本
fn set_frame_text() -> FrameText {

    let mut preview = gui_util::img::ImgPreview::new(
        490-2, 167,
        18, 18,
        "gui_util::select_user_ui::imag<help>",
    );

    preview.from_svg(
        include_str!("./src/help.svg"),
        0,
        0,
        preview.preview.w(),
        preview.preview.h(),
    );

    FrameText {
        选择: TextControl::new(
            50 - 15,
            46,
            180,
            18,
            12,
            "请选择WX文件的默认保存位置*",
            [85, 85, 85],
        ),
        文件管理: TextControl::new(
            273,
            46,
            239,
            18,
            12,
            "此路径在：  设置  /  文件管理   /  文件管理(输入框)",
            [49,49,49],
        ),
        选择用户: TextControl::new(
            50 - 3 - 37,
            118,
            475,
            18,
            12,
            "选择用户*    [ 涉及用户数据安全   仅限已登录中的用户 并且能够收发的用户 ]",
            [85, 85, 85],
        ),
        选择最近对象: TextControl::new(
            59 ,
            207+5,
            465,
            18,
            12,
            "选择最近对象*  （如果不存在请发送一张随意的图片给对方 [不能是表情] ） ",
            [85, 85, 85],
        ),
        通过拽入获取: TextControl::new(366, 166, 85, 18, 12, "通过扫描获取", [85, 85, 85]),
        帮助: TextControl::new(510-2, 167, 30, 18, 12, "帮助", [85, 85, 85]),
        别名备注: TextControl::new(139, 398, 65, 18, 13, "别名备注:", [85, 85, 85]),
        用户目录: TextControl::new(139, 439 + 2, 65, 18, 13, "用户目录:", [85, 85, 85]),
        命名规则: TextControl::new(42, 525, 65, 18, 13, "命名规则:", [85, 85, 85]),
        编辑规则: TextControl::new(495, 523, 56, 18, 12, "编辑规则", [85, 85, 85]),
        完成选定: TextControl::new(495, 437, 58, 18, 12, "完成选定", [255, 255, 255]),
        备注: TextControl::new(513, 395, 30, 18, 13, "备注", [85, 85, 85]),
        开始: TextControl::new(451+15, 73+7, 30, 18, 12, "开始", [85, 85, 85]),
    }

}

struct FrameCheck {
    sync: button::CheckButton,
    video: button::CheckButton,
    thumbnail: button::CheckButton,
    source: button::CheckButton,
    the_month: button::CheckButton,
}

impl FrameCheck {
    pub fn existPoint(&self, x: i32, y: i32) -> bool {
        let self_x = 38;
        let self_y = 475;
        let self_w = 520;
        let self_h = 33;

        return x > self_x
            && x < self_x + self_w
            && y > self_y
            && y < self_y + self_h;
    }
}

// 条件选定
fn add_check_button() -> FrameCheck {
    let mut flex = group::Flex::default()
        .with_size(530, 30)
        .row()
        .center_of_parent();

    flex.set_pos(43, 479);

    let mut check_button_sync = button::CheckButton::default().with_label("启用同步");
    let mut check_button_video = button::CheckButton::default().with_label("转存视频");
    let mut check_button_thumbnail = button::CheckButton::default().with_label("存缩略图");
    let mut check_button_source = button::CheckButton::default().with_label("保存原图");
    let mut check_button_the_month = button::CheckButton::default().with_label("只保存本月");

    check_button_video.deactivate();

    check_button_sync.set_callback(|win|{
        global_var::set_bool("user::config::check_button_sync",win.is_checked());
    });

    check_button_video.set_callback(|win|{
        global_var::set_bool("user::config::check_button_video",win.is_checked());
    });
    
    check_button_thumbnail.set_callback(|win|{
        global_var::set_bool("user::config::check_button_thumbnail",win.is_checked());
    });
    
    check_button_source.set_callback(|win|{
        global_var::set_bool("user::config::check_button_source",win.is_checked());
    });
    
    check_button_the_month.set_callback(|win|{
        global_var::set_bool("user::config::check_button_the_month",win.is_checked());
    });

    check_button_source.set_checked(true);
    check_button_sync.set_checked(true);
    
    global_var::set_bool("user::config::check_button_source",true);
    global_var::set_bool("user::config::check_button_sync",true);

    flex.end();

    FrameCheck {
        sync: check_button_sync,
        video: check_button_video,
        thumbnail: check_button_thumbnail,
        source: check_button_source,
        the_month: check_button_the_month,
    }
}

struct FrameThumbnailPreview{
    hotspot_list:Vec<gui_util::hotspot::HotspotItmeControl>,
    thumbnail_list:Vec<ImgPreview>
}

// 缩略图列表
fn add_frame_thumbnail_preview() ->FrameThumbnailPreview {
    let mut preview_main_point_list = [
        [71, 296, 75, 75],
        [167, 296, 75, 75],
        [263, 296, 75, 75],
        [359, 296, 75, 75],
        [455, 296, 75, 75],
    ];

    let mut preview_main = Vec::new();
    let mut hotspot_list =Vec::new();
    // let mut preview_main2 = Vec::new();

    for index in 0..preview_main_point_list.len() {
        let point = preview_main_point_list[index];
        let [x, y, width, height] = point;

        let mut preview = ImgPreview::new(x, y - 52, width, height, "gui::preview_main::index::");
        let pre: Vec<u8> = if wh_mod::config::is_show_dome() { ASSETS_DEMO_NOT_DATA() } else{ ASSETS_NOT_DATA() };

        preview.from_data(
            pre,
            -1,
            -1,
            width - 2,
            height - 2,
        );

        preview_main.push(preview);
        // preview_main2.push(preview.clone());
        hotspot_list.push(gui_util::hotspot::create_hotspot(x, y - 52, width, height));
    }

    lib::initialize_img_preview_list(&preview_main);

    FrameThumbnailPreview{
        hotspot_list:hotspot_list,
        thumbnail_list:preview_main,
    }
}

// 添加背景页
macro_rules! add_preview_contour {
    ($win:expr) => {{
        let mut preview = gui_util::img::ImgPreview::new(
            0,
            0,
            $win.w(),
            $win.h(),
            "gui_util::select_user_ui::main<win>",
        );
        preview.from_svg(
            MAIN_CONTOUR,
            0,
            0,
            preview.preview.w(),
            preview.preview.h(),
        );
        preview
            .preview
            .set_id("gui_util::select_user_ui::main<contour>");
        preview
    }};
}

// 用户选择
fn select_user_data_choice() -> menu::Choice {
    let mut choice = menu::Choice::default().with_size(277, 35).center_of_parent().with_label("");
    choice.set_pos(60,158);
    choice.add_choice("请点击 [开始] 获取在线用户列表");
    choice.set_value(0);
    choice.set_color(Color::from_rgb(23, 23, 23));

    choice
}

struct AttachThumbnailPreview{
    thumbnail_preview: ImgPreview,
    btn_remark: gui_util::hotspot::HotspotItmeControl,
    btn_rename: gui_util::hotspot::HotspotItmeControl,
    btn_select: gui_util::hotspot::HotspotItmeControl,
    input_rename: fltk::input::Input,
    input_remark: fltk::input::Input,
    input_attach: fltk::input::Input,
}

impl AttachThumbnailPreview {
    pub fn gc(&mut self) {
        let pre: Vec<u8> = if wh_mod::config::is_show_dome() { ASSETS_DEMO_NOT_SELECT() } else{ ASSETS_NOT_SELECT() };

        self.thumbnail_preview.from_data(
            pre,
            -1,
            -1,
            self.thumbnail_preview.preview.w()-2,
            self.thumbnail_preview.preview.h()-2,
        );

        self.input_remark.set_value("");
        self.input_attach.set_value("");
        global_var::set_string("user::config::user_select_attach",String::new());
        // self.input_rename.set_value("");

    }
    fn clone(&self) -> Self {
        AttachThumbnailPreview{
            thumbnail_preview: self.thumbnail_preview.clone(),
            btn_remark: self.btn_remark.clone(),
            btn_rename: self.btn_rename.clone(),
            btn_select: self.btn_select.clone(),
            input_rename: self.input_rename.clone(),
            input_remark: self.input_remark.clone(),
            input_attach: self.input_attach.clone(),
        }
    }
    fn redata(&mut self,thumbnail:wh_mod::AttachThumbnail){
        self.input_remark.set_value("");
        self.input_attach.set_value("");
        global_var::set_string("user::config::user_select_attach",String::new());

        let pre: Vec<u8> = if wh_mod::config::is_show_dome() { ASSETS_DEMO_DATA() } else{ thumbnail.thumbnail.to_vec() };

        // self.input_rename.set_value("");
        // 设置预览图
        self.thumbnail_preview.from_data(pre,-1,
                                                       -1,
                                         self.thumbnail_preview.preview.width()-2,
                                         self.thumbnail_preview.preview.height()-2,);
        // 绑定内容
        self.input_attach.set_value(wh_mod::get_show_mask_text( &thumbnail.attach_id).as_str());
        global_var::set_string("user::config::user_select_attach",thumbnail.attach_id.clone());

        let retrieval_struct = wh_mod::wx_parse_path(thumbnail.thumbnail_path.to_string());

        // 获取备注
        if let Some(user_remark) = lib::get_store_user_remark(retrieval_struct.wxid,thumbnail.attach_id.clone()) {
            self.input_remark.set_value(user_remark.as_str())
        }

    }
}

// 用户选定预览的卡片(底部)
fn add_select_attach_card() -> AttachThumbnailPreview {
    let mut preview = gui_util::img::ImgPreview::new(
        41+3, 385,
        82, 82,
        "gui_util::select_user_ui::imag<add_select_attach_card>",
    );
    let pre: Vec<u8> = if wh_mod::config::is_show_dome() { ASSETS_DEMO_NOT_SELECT() } else{ ASSETS_NOT_SELECT() };

    preview.from_data(pre,
        -1,
        -1,
        preview.preview.w()-2,
        preview.preview.h()-2,
    );

    let mut button_remark = gui_util::hotspot::create_hotspot(495, 387 , 66, 32);
    let mut button_select = gui_util::hotspot::create_hotspot(484, 430 , 77, 32);
    let mut button_rename = gui_util::hotspot::create_hotspot(484, 516 , 77, 32);

    // 命名规则
    let mut rename_input = input::Input::new(115, 518, 357, 30, "");

    // 备注输入框
    let mut remark_input = input::Input::new(211, 389, 275, 30, "");

    // 用户目录
    let mut user_data_preview = input::Input::new(213, 432, 263, 30, "");
    user_data_preview.set_readonly(true);

    AttachThumbnailPreview{
        thumbnail_preview:preview,
        btn_remark:button_remark,
        btn_rename:button_rename,
        btn_select:button_select,
        input_rename:rename_input,
        input_remark:remark_input,
        input_attach:user_data_preview
    }

}

macro_rules! set_select_user_base_input_default{
    ($input_select_dir:expr)=>{{

        let mut _path = String::new();
        if let Ok(history) = lib::get_wx_user_history_path() {

            let paths = history.path;
             _path = format!("{}",paths.as_str());
            $input_select_dir.set_value(wh_mod::get_show_mask_text(paths.as_str()).as_str());

        }
        if ($input_select_dir.value().is_empty()) {
            if let Some(paths) = wh_mod::convert::get_user_data_path() {
                _path = format!("{}",paths.as_str());
                $input_select_dir.set_value(wh_mod::get_show_mask_text(paths.as_str()).as_str());
            }
        }

        global_var::set_string("user::config::user_select_path",_path);

    }}
}

macro_rules! initialize_window_hwnd{
    ($hwnd:expr)=>{
        if $hwnd==0{
            $hwnd =  libWxIkunPlus::findWindow(THE_WINDOW_CLASS_NAME, "");
            libWxIkunPlus::setWinIcon($hwnd);
            lib::initialize_gc_select_user_ui($hwnd);
            println!("[initialize-window] {} -> {}",THE_WINDOW_CLASS_NAME, $hwnd);
        }
    }
}

struct PreviewData{
    preview_list:Vec<ImgPreview>,
    preview_main: ImgPreview,
}

impl PreviewData {

    pub fn gc_data(&mut self) {
        let pre: Vec<u8> = if wh_mod::config::is_show_dome() { ASSETS_DEMO_NOT_DATA() } else{ ASSETS_NOT_DATA() };

        let mut index = 0;
        for mut preview in self.preview_list.clone() {
            index+=1;
            let mut w = 90;
            if index==1 {
                w+=1;
            }
            if index==4 {
                w+=1;
            }
            if index==6 {
                w+=1;
            }
            preview.from_data(pre.clone(),-1,-1,w ,90 - 2,);
        }
        self.preview_main.from_data(pre , -1, -1, 230-2 , 230 - 2, );
    }

    fn clone(&self) -> Self {
        PreviewData{
            preview_list: self.preview_list.clone(),
            preview_main: self.preview_main.clone(),
        }
    }

    fn redata(&mut self,thumbnail_list:Vec<wh_mod::AttachThumbnail>){
        // self.gc_data();

        if let Some(main_thumbnail) = thumbnail_list.get(0) {
            let pre: Vec<u8> = if wh_mod::config::is_show_dome() { ASSETS_DEMO_DATA() } else{ main_thumbnail.thumbnail.to_vec() };
            self.preview_main.re_data(pre/*, -1, -1, 230-2 , 230 - 2, */);

        }

        let mut index = 0;
        for mut preview in self.preview_list.clone() {
            index+=1;
            // let mut w = 90;
            // // 第一列有1px的误差
            // if index==1 ||index==4||index==7{
            //     w+=1;
            // }

            if let Some(thumbnail) = thumbnail_list.get(index) {
                let pre: Vec<u8> = if wh_mod::config::is_show_dome() { ASSETS_DEMO_DATA() } else{ thumbnail.thumbnail.to_vec() };

                preview.re_data(pre/*,-1,-1,w ,90 - 2,*/);

            }else {
                let pre: Vec<u8> = if wh_mod::config::is_show_dome() { ASSETS_DEMO_NOT_DATA() } else{ ASSETS_NOT_DATA() };
                preview.re_data(pre/*,-1,-1,w ,90 - 2,*/);
            }

        }

    }

    pub fn existPoint(&self, x: i32, y: i32) -> bool {
        let preview_pint = [
            [35,280,85,85],[35,381,85,85],[35,482,85,85],
            [137,280,85,85],[137,381,85,85],[137,482,85,85],
            [239,280,85,85],[239,381,85,85],[239,482,85,85]
        ];

        for preview in preview_pint {
            if
            x > preview[1]
                && x < preview[1] + 85
                && y > preview[0]
                && y < preview[0] + 85
            {
                return  true
            }
        }
        false
    }

    pub fn existPointIndex(&self, x: i32, y: i32) -> usize {
        let mut index = 0 ;
        let preview_pint = [
            [35,280,85,85],[35,381,85,85],[35,482,85,85],
            [137,280,85,85],[137,381,85,85],[137,482,85,85],
            [239,280,85,85],[239,381,85,85],[239,482,85,85]
        ];

        for preview in preview_pint {
            index+=1;

            if
            x > preview[1]
                && x < preview[1] + 85
                && y > preview[0]
                && y < preview[0] + 85
            {
                return  index
            }

        }
        0
    }
}

// 九宫格
fn preview_main_list() ->PreviewData{

    let mut preview_main = gui_util::img::ImgPreview::new2(35,35,230,230,"",-1, -1, 230-2 , 230 - 2);

    // 九宫格位置预设
    let preview_pint = [
        [[35,280,85,85],[35,381,85,85],[35,482,85,85]],
        [[137,280,85,85],[137,381,85,85],[137,482,85,85]],
        [[239,280,85,85],[239,381,85,85],[239,482,85,85]]
    ];

    let mut flex = group::Flex::default()
        .with_size(287, 90)
        .row()
        .center_of_parent();

    flex.set_pos(280, 35);

    let mut preview_main_1 = gui_util::img::ImgPreview::new2(0,0,85,85,"",-1,-1,91 ,90 - 2);
    let mut preview_main_2 = gui_util::img::ImgPreview::new2(0,0,85,85,"",-1,-1,90 ,90 - 2);
    let mut preview_main_3 = gui_util::img::ImgPreview::new2(0,0,85,85,"",-1,-1,90 ,90 - 2);
    flex.end();

    let mut flex = group::Flex::default()
        .with_size(287, 90)
        .row()
        .center_of_parent();

    flex.set_pos(280, 137);

    let mut preview_main_4 = gui_util::img::ImgPreview::new2(0,0,85,85,"",-1,-1,91 ,90 - 2);
    let mut preview_main_5 = gui_util::img::ImgPreview::new2(0,0,85,85,"",-1,-1,90 ,90 - 2);
    let mut preview_main_6 = gui_util::img::ImgPreview::new2(0,0,85,85,"",-1,-1,90 ,90 - 2);
    flex.end();

    let mut flex = group::Flex::default()
        .with_size(287, 90)
        .row()
        .center_of_parent();

    flex.set_pos(280, 239);

    let mut preview_main_7 = gui_util::img::ImgPreview::new2(0,0,85,85,"",-1,-1,91 ,90 - 2);
    let mut preview_main_8 = gui_util::img::ImgPreview::new2(0,0,85,85,"",-1,-1,90 ,90 - 2);
    let mut preview_main_9 = gui_util::img::ImgPreview::new2(0,0,85,85,"",-1,-1,90 ,90 - 2);

    flex.end();

   let result = vec![preview_main_1,preview_main_2,preview_main_3,preview_main_4,preview_main_5,preview_main_6,preview_main_7,preview_main_8,preview_main_9];

   let mut  preview_data =  PreviewData{
        preview_list: result,
        preview_main
    };

    preview_data.gc_data();

    preview_data
}

struct ThumbnailPreviewMain{
    main:DoubleWindow,
    btn_close: gui_util::hotspot::HotspotItmeControl,
    preview_list: PreviewData,
}

// 图片预览窗口 九宫格
fn add_frame_thumbnail_preview_main (mut win: &DoubleWindow) -> ThumbnailPreviewMain {
    // 图片预览窗口
    let mut preview_win = fltk::window::Window::new(0,0,win.w(),359,"");
    preview_win.set_color(Color::from_rgb(23, 23, 23));
    set_item_id!(preview_win,THE_SUB_WINDOW_CLASS_NAME_FRAME_THUMBNAIL_PREVIEW);

    let mut preview_win_border = gui_util::img::ImgPreview::new(0,0,preview_win.w(),preview_win.h(),"");
    preview_win_border.from_svg(include_str!("./src/preview_win.svg"),0,0,preview_win.w(),preview_win.h());

    let mut preview_main_close_btn = gui_util::hotspot::create_hotspot(82,282,130,32);
    let mut preview_main_list = preview_main_list();
    gui_util::text::TextControl::new(82,283,130,32,13,"关闭预览",[121, 121, 121]);

    preview_win.end();
    preview_win.hide();

    ThumbnailPreviewMain{
        main:preview_win,
        btn_close:preview_main_close_btn,
        preview_list:preview_main_list,

    }
}

// 通过扫描获取的界面
struct ScanPreviewMain{
    main:fltk::window::DoubleWindow,
    btn_all_obj: gui_util::hotspot::HotspotItmeControl,
    btn_clip:  gui_util::hotspot::HotspotItmeControl,
    btn_close: gui_util::hotspot::HotspotItmeControl,
    text_bottom:Vec<gui_util::text::TextControl>,
    progress_bar: ImgPreview,
    all_text_list: Vec<TextControl>,
    btn_scan_drag: gui_util::hotspot::HotspotItmeControl,
}

impl ScanPreviewMain {
    pub fn existPoint(&self, x: i32, y: i32) -> bool {
        let self_x = self.main.x();
        let self_y = self.main.y();
        let self_w = self.main.w();
        let self_h = self.main.h();

        return x > self_x
            && x < self_x + self_w
            && y > self_y
            && y < self_y + self_h;
    }

    pub fn show_progress(& mut self){
        for (mut index) in 0..3 {
            if let Some(text_bottom) = self.text_bottom.get_mut(index) {
                text_bottom.text.hide();
            }
        }
        self.progress_bar.preview.show();
    }

    pub fn show_bottom_text(& mut self){
        for (mut index) in 0..3 {
            if let Some(text_bottom) = self.text_bottom.get_mut(index) {
                text_bottom.text.show();
            }
        }
        self.progress_bar.preview.hide();
    }
}

// 進度條
fn show_progress_bar_border(x: i32, y: i32) -> gui_util::img::ImgPreview {
    let mut progress_bar_border = gui_util::img::ImgPreview::new_border(x,y,520,15,"<svg width=\"520\" height=\"15\" viewBox=\"0 0 520 15\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\"> <rect x=\"0.5\" y=\"0.5\" width=\"520\" height=\"14\" rx=\"7\" fill=\"#181818\" stroke=\"#2C2C2C\"/> </svg> ");
    let width = progress_bar_border.width;
    let mut progress_bar_border_slider = gui_util::img::ImgPreview::new_border(x, y, 41, 15, "<svg width=\"40\" height=\"15\" viewBox=\"0 0 40 15\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\">\n<rect x=\"0.5\" y=\"0.5\" width=\"39\" height=\"14\" rx=\"7\" fill=\"#333333\" stroke=\"#2C2C2C\"/>\n</svg>");
    progress_bar_border_slider.preview.hide();
    progress_bar_border.preview.hide();

    progress_bar_border.preview.handle({
        let mut progress_bar_border_slider = progress_bar_border_slider.clone();
        let mut preview_main = progress_bar_border.preview.clone();

        move |win, ev| match ev {
            enums::Event::Show => {
                progress_bar_border_slider.preview.show();

                let mut preview = progress_bar_border_slider.preview.clone();
                let mut preview_main = preview_main.clone();

                app::add_timeout3(0.0086, move |handle| {
                    if !preview.visible()||!preview_main.visible() {
                        preview_main.hide();
                        preview.hide();
                        app::remove_timeout3(handle);
                        return;
                    }

                    let mut the_x = preview.x() + 2;
                    if the_x > width {
                        the_x = x + preview.width();
                        the_x -= preview.width();
                    }
                    preview.set_pos(the_x, preview.y());
                    preview.parent().unwrap().redraw();
                    app::repeat_timeout3(0.0086, handle);
                });
                true
            }
            enums::Event::Hide => {
                progress_bar_border_slider.preview.hide();
                true
            }
            _ => false,
        }
    });

    // progress_bar_border.preview.show();
    progress_bar_border
}

// 通过扫描获取的界面
fn add_scan_preview_window() -> ScanPreviewMain {
    // 图片预览窗口
    let mut preview_win = fltk::window::Window::new(0,0,600,359,"");
    preview_win.set_color(Color::from_rgb(23, 23, 23));
    set_item_id!(preview_win,THE_SUB_WINDOW_CLASS_NAME_SCAN);

    let mut preview_win_border = gui_util::img::ImgPreview::new(0,0,preview_win.w(),preview_win.h(),"");
    preview_win_border.from_svg(include_str!("./src/scan_cursor.svg"),0,0,preview_win.w(),preview_win.h());

    let mut show_all_user_obj = gui_util::hotspot::create_hotspot(83,231,87,40);
    let mut get_clip_data = gui_util::hotspot::create_hotspot(432,231,87,40);
    let mut scan_drag_data = gui_util::hotspot::create_hotspot(256,231,87,40);
    let mut btn_close = gui_util::hotspot::create_hotspot(537,33,25,25);

    let mut text_list = Vec::new();

    // 標題
    text_list.push(gui_util::text::TextControl::new(150-25,33,345,15,12,"请选择一种您喜欢的方式扫描聊天对象",[149, 149, 149]));
    text_list.push(gui_util::text::TextControl::new(130,58,345,15,12,"如果您有很多好友需要动态管理可以设置别名 在所有好友中可显示别名备注",[68, 68, 68]));

    //  卡片中上
    text_list.push(gui_util::text::TextControl::new(57,170+3 ,135,15,12,"所有存在图片的聊天对象",[149, 149, 149]));
    text_list.push(gui_util::text::TextControl::new(257,170+3 ,85,15,12,"拖拽聊天的图片",[149, 149, 149]));
    text_list.push(gui_util::text::TextControl::new(442,170+3 ,64,15,12,"剪贴板获取",[149, 149, 149]));

    // 卡片中下
    text_list.push(gui_util::text::TextControl::new(87,196+5,78,15,13,"查看所有人",[121, 121, 121]));
    text_list.push(gui_util::text::TextControl::new(255,196+5,91,15,13,"打开扫描窗口",[121, 121, 121]));
    text_list.push(gui_util::text::TextControl::new(410,198+5,135,15,12,"复制一张图片后点击开始",[121, 121, 121]));

    // 卡片下面
    text_list.push(gui_util::text::TextControl::new(111,239+5,31,15,13,"开始",[255, 255, 255]));
    text_list.push(gui_util::text::TextControl::new(256+(460-432),239+5,31,15,13,"开始",[255, 255, 255]));
    text_list.push(gui_util::text::TextControl::new(460,239+5,31,15,13,"开始",[255, 255, 255]));

    // 底部三言 / 進度條
    let mut text_01 = gui_util::text::TextControl::new(70,308,115,15,13,"聊天对象选择面板",[195, 195, 195]);
    let mut text_02 = gui_util::text::TextControl::new(260,308,85,15,13,"通过拖拽查找",[195, 195, 195]);
    let mut text_03 = gui_util::text::TextControl::new(435,308,85,15,13,"粘贴文件查找",[195, 195, 195]);

    let mut progress_bar_border = show_progress_bar_border(40,312);

    preview_win.end();
    preview_win.hide();

    preview_win.handle({
        let mut preview = progress_bar_border.preview.clone();
        move |win, ev| match ev {
            enums::Event::Show=>{
                true
            }
            enums::Event::Hide=>{
                preview.hide();
                true
            }
            _ => false
        }

    });

    ScanPreviewMain{
        main:preview_win,
        btn_clip:get_clip_data,
        btn_all_obj:show_all_user_obj,
        btn_scan_drag:scan_drag_data,
        btn_close,
        all_text_list:text_list,
        text_bottom: vec![text_01,text_02,text_03],
        progress_bar:progress_bar_border
    }
}

fn initialize_watch_walk_drag_path (mut preview1: AttachThumbnailPreview) {
    let mut oid_walk_drag_path = String::new();
    std::thread::spawn( move || loop{
        if !has_window(){
            lib::gc_select_user_ui();
            return;
        }

        let walk_drag_path = global_var::get_string_default("user::config::walk_drag_path");
        if(!oid_walk_drag_path.as_bytes().eq(walk_drag_path.as_bytes())){
            oid_walk_drag_path.clear();
            oid_walk_drag_path.push_str(walk_drag_path.as_str());
            let wx_parse = wh_mod::wx_parse_path(walk_drag_path.clone());
            // println!("wx_parse-> {:?}",&wx_parse);
            preview1.input_attach.set_value(wh_mod::get_show_mask_text(&wx_parse.attach_id).as_str());
            global_var::set_string("user::config::user_select_attach",format!("{}",wx_parse.attach_id.as_str()));

            if let Some(remark) = lib::get_store_user_remark(wx_parse.wxid,wx_parse.attach_id.clone()) {
                preview1.input_remark.set_value(remark.as_str());
            }else {
                preview1.input_remark.set_value("");
            }

            if let Ok(buff_thumbnail_data) = wh_mod::convert::convert_dat_images_buff(std::path::PathBuf::from(walk_drag_path.as_str())) {
                let pre: Vec<u8> = if wh_mod::config::is_show_dome() { ASSETS_DEMO_DATA() } else{ buff_thumbnail_data };

                preview1.thumbnail_preview.from_data(pre,-1, -1, 80, 80);
            }

            // println!("walk_drag_path->{}",&walk_drag_path);
        }

        if !gui_drag_scan2_ui::has_window() {
            return;
        }

        Sleep(500);
    });
}

macro_rules! the_token {
    ()=>{
       {
        let mut _the_token =format!("[select_user_ui]token<{}>@query",libWxIkunPlus::randomNum());
        loop{
            if global_var::has_string(_the_token.as_str()) {
                _the_token = format!("[select_user_ui]token<{}>@query",libWxIkunPlus::randomNum());
            }else{
                break;
            }
        }
            _the_token
        }
    }
}

pub fn manage_tool_main() -> String{
    let the_token = the_token!();
    // 禁止创建多个窗口
    if let hwnd = libWxIkunPlus::findWindow(THE_WINDOW_CLASS_NAME, "") {
        if hwnd!=0 && libWxIkunPlus::isWindow(hwnd) {
            if let Some(mut win) =app::widget_from_id(THE_WINDOW_CLASS_NAME) as Option<DoubleWindow>
             {
                 win.show();
                 win.set_visible_focus();
             }
             libWxIkunPlus::setWindowShake(hwnd);
            global_var::set_string(the_token.as_str(),String::new());
            return the_token;
        }
    }

    let mut hwnd :i128 = 0;
    let mut win = window::DoubleWindow::new(0, 0,600, 595,None).center_screen();
    win.set_color(Color::from_rgb(23, 23, 23));

    inject_fltk_theme!();
    win.set_label("任务创建向导");
    set_item_id!(win, THE_WINDOW_CLASS_NAME);
    
    let mut g_the_select_wxid = String::new();
    let mut g_the_select_attach_id = String::new();
    let mut preview_win =add_frame_thumbnail_preview_main(&win);
    let mut scan_preview_window = add_scan_preview_window();

    let mut preview = add_preview_contour!(win);

    // 固定文本
    let mut frame_text = set_frame_text();
    // 条件选定
    let mut frame_check = add_check_button();
    // 五张缩略图
    let mut frame_thumbnail_preview = add_frame_thumbnail_preview();
    // 用户选择数据库
    let mut select_user_data_choice = select_user_data_choice();
    // 预览卡片
    let mut select_attach_card  = add_select_attach_card();

    // 文件的默认保存位置(D:\...\...\WeChat Files)
    let mut user_select_database_dir_input = input::Input::new(45+3, 74, 395, 30, "");
    set_select_user_base_input_default!(user_select_database_dir_input);

    // 按钮 > 打开文件选择
    let mut button_open_dir = gui_util::hotspot::create_hotspot(516, 73 , 33, 32);
    // 按钮 > 扫描获取
    let mut button_show_drag = gui_util::hotspot::create_hotspot(346, 156 , 123, 38);
   
    // 按钮 > 帮助
    let mut button_show_help = gui_util::hotspot::create_hotspot(479, 156 , 66, 38);
    // 按钮 > 开始
    let mut button_start = gui_util::hotspot::create_hotspot(451, 73 , 57, 32);

    select_attach_card.input_rename.set_value("<创建月>/<任务名>/<类型>_<NN>");

    let mut move_select_attach_card = select_attach_card.clone();

    select_user_data_choice.set_callback(move |c| {
        if let Some(choice_value) = c.choice() {
            
            if let Some(item) = lib::get_active_user_list().get(c.value() as usize).clone() {
            move_select_attach_card.gc();
            c.deactivate();

            lib::store_wx_user_path_history(item.user_root.clone(),item.user_wxid.clone());
            
            println!("[{}] {}  , {}  ,  {} ",c.value(), choice_value.as_str() ,&item.user_root,&item.user_wxid);
            
            global_var::set_string("user::config::user_select_path", item.user_root.clone());
            global_var::set_string("user::config::user_select_wxid", item.user_wxid.clone());

            lib::initialize_thumbnail_preview(item.user_root.as_str(),item.user_wxid.as_str());
            lib::initialize_watch_path_puppet(format!("{}\\{}", &item.user_root, &item.user_wxid));

            c.activate();
            }
            
            
        }
    });

    let mut drag_path = std::path::PathBuf::new();



    win.end();
    win.show();
    // preview_win.show();

    win.handle({
        let mut x = 0;
        let mut y = 0;
        // 是否显示手型
        let mut show_cursor = false;
        let mut preview_win_show = false;
        let mut scan_win_show = false;

        let mut released = true;
        let mut dnd = true;
        let the_token =the_token.clone();
        let move_select_attach_card2 = select_attach_card.clone();

        move |win, ev| match ev {

            enums::Event::Focus=>{
                initialize_window_hwnd!(hwnd);
                true
            }

            enums::Event::Show => {
                initialize_window_hwnd!(hwnd);
                true
            }

            enums::Event::KeyUp => true,

            enums::Event::Close=>{
                lib::gc_select_user_ui();
                true
            }

            enums::Event::Push => {

                macro_rules! add_preview_win_show {
                    ()=>{
                        preview_win_show = true;
                        user_select_database_dir_input.hide();
                        select_user_data_choice.hide();
                        let path = PathBuf::from(global_var::get_string_default("user::config::user_select_path"))
                            .join(global_var::get_string_default("user::config::user_select_wxid"))
                            .join("FileStorage\\MsgAttach")
                            .join(g_the_select_attach_id.as_str())
                            ;
                        let dat  = wh_mod::read_attach_buff_thumbnail_data(&path,10);
                        println!("dat<{}> path:<{:?}>",dat.len(),&path);
                        preview_win.preview_list.redata(dat);
                        preview_win.main.show();
                    }
                }
                if preview_win_show {
                    if preview_win.btn_close.existPoint(x,y) {
                        preview_win_show =false;
                        preview_win.main.hide();
                        user_select_database_dir_input.show();
                        select_user_data_choice.show();
                        preview_win.preview_list.gc_data();
                    }

                }
                else if scan_win_show {

                    // 关闭扫描窗口
                    if scan_preview_window.btn_close.existPoint(x,y){
                        scan_preview_window.main.hide();
                        // user_select_database_dir_input.activate();
                        // select_attach_card.input_rename.activate();
                        // select_attach_card.input_remark.activate();
                        scan_preview_window.show_bottom_text();
                        scan_win_show = false;
                    }

                    if !scan_preview_window.progress_bar.preview.visible(){
                        if scan_preview_window.btn_all_obj.existPoint(x,y) {
                            gui_util::message::sub_message(hwnd,gui_util::message::IconType::Warning,"作者正在玩命的开发中。。。",3500u64);
                        }
                        if scan_preview_window.btn_scan_drag.existPoint(x,y) {
                            if global_var::get_string_default("user::config::user_select_wxid").is_empty() {
                                gui_util::message::sub_message(hwnd,gui_util::message::IconType::Warning,"尚未选择用户",3500u64);
                                return false;
                            }
                            if global_var::get_string_default("user::config::user_select_path").is_empty() {
                                gui_util::message::sub_message(hwnd,gui_util::message::IconType::Warning,"没有选择WX根目录",3500u64);
                                return false;
                            }
                            // gui_drag_scan::main_window();
                            gui_drag_scan2_ui::main_window("");
                            initialize_watch_walk_drag_path (move_select_attach_card2.clone());
                        }

                        if  scan_preview_window.btn_clip.existPoint(x,y) {
                            let clip_file_path_single = libWxIkunPlus::getClipFilePathSingle();
                            if clip_file_path_single.is_empty() {
                                gui_util::message::sub_message(hwnd,gui_util::message::IconType::Warning,"当前剪贴板无内容或者不是可识别格式",3500u64);
                            }else{

                                if global_var::get_string_default("user::config::user_select_wxid").is_empty() {
                                    gui_util::message::sub_message(hwnd,gui_util::message::IconType::Warning,"尚未选择用户",3500u64);
                                    return false;
                                }
                                if global_var::get_string_default("user::config::user_select_path").is_empty() {
                                    gui_util::message::sub_message(hwnd,gui_util::message::IconType::Warning,"没有选择WX根目录",3500u64);
                                    return false;
                                }

                                // scan_preview_window.show_progress();
                                drag_path.clear();
                                drag_path.push(clip_file_path_single.clone());
                                println!("getClipFilePathSingle-> {:?}",&drag_path);

                                // 获取id 并开始处理
                                let temp_imag_id = get_wx_temp_imag_id(clip_file_path_single.as_str());

                                if !temp_imag_id.is_empty() {
                                    gui_drag_scan2_ui::main_window(clip_file_path_single.as_str());
                                    initialize_watch_walk_drag_path (move_select_attach_card2.clone());
                                }

                            }

                        }

                    }else{
                        gui_util::message::sub_message(hwnd,gui_util::message::IconType::Failure,"当前正在扫描中 不能使用此功能",3500u64);
                    }
                }
                // 正常窗口
                else {
                    // 选择最近5个对象
                    {
                        let mut index = 0;
                        for hotspot in &frame_thumbnail_preview.hotspot_list {
                            index += 1;
                            let thumbnail_list = lib::get_thumbnail_list();
                            if hotspot.existPoint(x, y) {
                                select_attach_card.gc();

                                if let Some(thumbnail) = thumbnail_list.get(index - 1usize) {
                                    println!("[click] frame_thumbnail_preview -> {}", index);
                                    global_var::set_i32("user::config::select_user_thumbnail_obj", index as i32);
                                    select_attach_card.redata(thumbnail.clone());
                                    g_the_select_attach_id.clear();
                                    g_the_select_attach_id.push_str(thumbnail.attach_id.as_str());

                                    if wh_mod::config::is_click_open_preview() {
                                        add_preview_win_show!();
                                    }
                                    break;
                                } else {
                                    gui_util::message::sub_message(hwnd, gui_util::message::IconType::Warning, "没有选中聊天对象", 3500u64);
                                }
                            }
                        }
                    }

                    // 打开文件夹选择器
                    if button_open_dir.existPoint(x, y) {
                        let mut select_dir = libWxIkunPlus::openSelectFolder2();
                        let eq_wxid_dir = eq_wxid_dir!(select_dir);
                        if !select_dir.is_empty() {
                            user_select_database_dir_input.set_value(wh_mod::get_show_mask_text(select_dir.as_str()).as_str());

                            select_user_data_choice.clear();
                            select_user_data_choice.add_choice("请点击 [开始] 获取在线用户列表");
                            select_user_data_choice.set_value(0);
                            lib::set_active_user_list(Vec::new());
                            global_var::set_string("user::config::user_select_path", select_dir.clone());

                            println!("[click] existPoint {}  select_dir-> {} eq-> {}", "打开文件夹选择器", select_dir, eq_wxid_dir);
                        } else {
                            gui_util::message::sub_message(libWxIkunPlus::findWindow(THE_WINDOW_CLASS_NAME, ""), gui_util::message::IconType::Info, "用户取消", 3500u64);
                        }
                    }

                    // 显示帮助面板
                    if button_show_help.existPoint(x, y) {
                        println!("[click] existPoint {}", "");
                        gui_detect_config_ui::main_window();
                    }

                    // 图片预览大图
                    if select_attach_card.thumbnail_preview.existPoint(x, y) {

                        let attach_id = global_var::get_string_default("user::config::user_select_attach");//select_attach_card.input_attach.value();
                        if attach_id.is_empty() {
                            // dialog::alert_default("没有选择聊天对象 (attach ID)");
                            gui_util::message::sub_message(hwnd, gui_util::message::IconType::Warning, "没有选择聊天对象 (attach ID)", 3500u64);

                            return false;
                        }

                        add_preview_win_show!();

                    }

                    // 开始
                    if button_start.existPoint(x, y)||(button_show_drag.existPoint(x, y)&&
                        (
                            global_var::get_string_default("user::config::user_select_wxid").is_empty()||
                            user_select_database_dir_input.value().is_empty()
                        )
                    ) {
                        select_attach_card.gc();

                        if !user_select_database_dir_input.value().is_empty() {
                            if (!libWxIkunPlus::hasWeChat() || !libWxIkunPlus::hasWeChatWin()) {
                                // fltk::dialog::alert_default("尚未找到已经登录的WX进程 为避免滥用 扫描被拒绝 ");
                                gui_util::message::sub_message(hwnd, gui_util::message::IconType::Failure, "WX未登录 为避免滥用 面板开启请求被拒绝", 3500u64);
                                return false;
                            }

                            // 释放资源
                            select_user_data_choice.activate();
                            lib::set_active_user_list(Vec::new());

                            select_user_data_choice.clear();
                            select_user_data_choice.add_choice("【状态】  当前正在扫描用户列表... ");
                            select_user_data_choice.set_value(0);

                            let mut user_select_database = global_var::get_string_default("user::config::user_select_path");
                            let mut user_select_database_input = user_select_database_dir_input.value();

                            // 没有* 则引用input内容 并重新设置input消敏
                            if !user_select_database_input.contains("*") {
                                let mut new_path = PathBuf::from(user_select_database_input.as_str());

                                if !user_select_database_input.contains("WeChat Files") {
                                    let new_data = new_path.clone().join("WeChat Files");
                                    if new_data.exists() {
                                        new_path = new_data;
                                    }
                                }

                                if(new_path.exists()){
                                    user_select_database = util::to_string_default(&new_path);
                                    global_var::set_string("user::config::user_select_path",user_select_database.to_string());

                                    user_select_database_dir_input.set_value(wh_mod::get_show_mask_text(&user_select_database).as_str());

                                }
                            }

                            if !user_select_database.is_empty() {
                                let active_user_list = wh_mod::convert::get_active_user(user_select_database.as_str());
                                select_user_data_choice.clear();

                                if active_user_list.is_empty() {
                                    select_user_data_choice.add_choice("【状态】  未找到用户列表");
                                    select_user_data_choice.set_value(0);
                                    gui_util::message::sub_message(libWxIkunPlus::findWindow(THE_WINDOW_CLASS_NAME, ""), gui_util::message::IconType::Failure, "未找到用户列表 请注意路径有效性", 3500u64);
                                    return false;
                                }

                                // 添加到列表
                                for active_user in active_user_list {
                                    if let Some(accinfo) = active_user.accinfo.clone() {
                                        select_user_data_choice.add_choice(format!("{} <{}>", wh_mod::get_show_mask_text(&accinfo.wx_id), wh_mod::get_show_mask_text(&accinfo.name)).as_str());
                                    } else {
                                        select_user_data_choice.add_choice( wh_mod::get_show_mask_text(&active_user.user_data).as_str());
                                    }

                                    lib::push_active_user_list(active_user.clone());
                                }

                                // 开始扫描
                                if let Some(item) = lib::get_active_user_list().get(0) {
                                    select_user_data_choice.set_value(0);
                                    lib::store_wx_user_path_history(item.user_root.clone(), item.user_wxid.clone());
                                    global_var::set_string("user::config::user_select_path", item.user_root.clone());
                                    global_var::set_string("user::config::user_select_wxid", item.user_wxid.clone());
                                    lib::initialize_thumbnail_preview(item.user_root.as_str(), item.user_wxid.as_str());
                                    lib::initialize_watch_path_puppet(format!("{}\\{}", &item.user_root, &item.user_wxid));
                                }
                            }
                        } else {
                            // dialog::alert_default("没有选择WX文件默认保存位置");
                            gui_util::message::sub_message(hwnd, gui_util::message::IconType::Warning, "没有选择WX文件默认保存位置", 3500u64);
                        }
                    }

                    // 显示扫描获取面板
                    if button_show_drag.existPoint(x, y) {
                        println!("[click] existPoint {}", "显示扫描获取面板");

                        if (!libWxIkunPlus::hasWeChat() || !libWxIkunPlus::hasWeChatWin()) {
                            gui_util::message::sub_message(hwnd, gui_util::message::IconType::Failure, "WX未登录 为避免滥用 面板开启请求被拒绝", 3500u64);
                            // fltk::dialog::alert_default("尚未找到已经登录的WX进程 为避免滥用 面板开启请求被拒绝 ");
                            return false;
                        }

                        if global_var::get_string_default("user::config::user_select_wxid").is_empty() {
                            gui_util::message::sub_message(hwnd,gui_util::message::IconType::Warning,"尚未选择用户",3500u64);
                            return false;
                        }
                        if global_var::get_string_default("user::config::user_select_path").is_empty() {
                            gui_util::message::sub_message(hwnd,gui_util::message::IconType::Warning,"没有选择WX根目录",3500u64);
                            return false;
                        }

                        // gui_drag_scan::main_window();


                        scan_preview_window.main.show();
                        // libWxIkunPlus::setWindowEnabled(libWxIkunPlus::findWindow(THE_SUB_WINDOW_CLASS_NAME_SCAN,""),true);
                        // user_select_database_dir_input.deactivate();
                        // select_attach_card.input_rename.deactivate();
                        // select_attach_card.input_remark.deactivate();
                        // let mut scan_drag_window = show_scan_drag_window();
                        // scan_drag_window.hide();
                        scan_win_show = true;
                    }

                }


                // 卡片按钮 > 完成选定
                if select_attach_card.btn_select.existPoint(x, y) {
                    println!("[click] existPoint {}", "卡片按钮 > 完成选定");

                    let mut result_data = String::new();
                    let mut is_effective = true;

                    let mut rename_rule = select_attach_card.input_rename.value();

                    // 没有选定的路径
                    if user_select_database_dir_input.value().is_empty() {
                        fltk::dialog::alert_default("没有选定Wx路径");
                        is_effective = false;
                        return false;
                    }
                    // 没有选定WX用户
                    if global_var::get_string_default("user::config::user_select_wxid").is_empty() {
                        fltk::dialog::alert_default("没有选定WX用户");
                        is_effective = false;
                        return false;
                    }

                    //  判断是否有Att id
                    else if g_the_select_attach_id.is_empty() || g_the_select_attach_id.len() < 25 {
                        // fltk::dialog::alert_default("attach id 无效 （尚未选定有效聊天对象）");
                        // gui_util::message::message(x+100,y+80,gui_util::message::IconType::Warning,"attach id 无效 （尚未选定有效聊天对象）",3500u64);
                        gui_util::message::sub_message(hwnd, gui_util::message::IconType::Warning, "attach id 无效 （尚未选定有效聊天对象）", 3500u64);
                        is_effective = false;
                        return false;
                    }
                    // 有命名规则 要求规则最少有一个%N.. 自变量
                    if !rename_rule.is_empty() && (!rename_rule.contains("<N") || !rename_rule.contains("N>")) {
                        rename_rule.push_str("<NN>");
                    }
                    let mut select_dir = global_var::get_string_default("user::config::user_select_path");
                    let mut user_select_wxid = global_var::get_string_default("user::config::user_select_wxid");

                    let eq_wxid_dir = eq_wxid_dir!(select_dir);

                    // 拼合路径并判断有效性 有且为文件夹
                    let mut attach_path = PathBuf::from(select_dir).join(user_select_wxid.as_str()).join("FileStorage\\MsgAttach").join(g_the_select_attach_id.as_str());

                    println!("attach_path=> {:?}", &attach_path);

                    if let Some(attach_path_str) = attach_path.to_str() {
                        result_data.push_str(attach_path_str);
                        // 识标
                        if frame_check.thumbnail.is_checked() {
                            result_data.push_str("*wizards");
                        }
                        // 可选项
                        if frame_check.thumbnail.is_checked() {
                            result_data.push_str("*thumbnail");
                        }
                        if frame_check.source.is_checked() {
                            result_data.push_str("*source");
                        }
                        if frame_check.video.is_checked() {
                            result_data.push_str("*video");
                        }
                        if frame_check.sync.is_checked() {
                            result_data.push_str("*Sync");
                        }
                        if frame_check.the_month.is_checked() {
                            result_data.push_str("*the_month");
                        }
                        // 添加名称格式化自变量
                        if !select_attach_card.input_rename.value().is_empty() {
                            result_data.push_str(format!("*rename_rule={}*", &rename_rule).as_str());
                        }

                    }else{
                        gui_util::message::sub_message(hwnd, gui_util::message::IconType::Warning, "路径转义失败 错误代码[3061]", 3500u64);
                        return false;
                    }

                    if !attach_path.exists() && !attach_path.exists() {

                        // dialog::alert_default("attach 目录无效");
                        gui_util::message::sub_message(hwnd, gui_util::message::IconType::Warning, "attach目录无效 <聊天对象目录无法打开>", 3500u64);
                        is_effective = false;
                        return false;
                    }

                    if is_effective && eq_wxid_dir {
                        println!("result_data->{}",&result_data);
                        global_var::set_string(the_token.as_str(),result_data);
                        lib::gc_select_user_ui();
                        fltk::window::Window::delete(win.clone());
                    }
                }

                // 卡片按钮 > 备注名称 完成按钮
                if select_attach_card.btn_remark.existPoint(x, y) {
                    println!("[click] existPoint {}", "卡片按钮 > 备注名称 完成按钮");

                    let wxid = global_var::get_string_default("user::config::user_select_wxid");
                    let attach_id = global_var::get_string_default("user::config::user_select_attach");//select_attach_card.input_attach.value();
                    let remark_name = select_attach_card.input_remark.value();

                    if wxid.is_empty() {
                        // dialog::alert_default("没有选择用户 (WXID)");
                        gui_util::message::sub_message(hwnd, gui_util::message::IconType::Warning, "没有选择用户 (WXID)", 3500u64);
                        return false;
                    }

                    if attach_id.is_empty() {
                        // dialog::alert_default("没有选择聊天对象 (attach ID)");
                        gui_util::message::sub_message(hwnd, gui_util::message::IconType::Warning, "没有选择聊天对象 (attach ID)", 3500u64);

                        return false;
                    }

                    if remark_name.is_empty() {
                        // dialog::alert_default("没有备注内容 (备注将用于命名与显示对象名称)");
                        gui_util::message::sub_message(hwnd, gui_util::message::IconType::Warning, "没有备注内容 (用于命名与显示对象名称)", 3500u64);

                        return false;
                    }

                    lib::set_store_user_remark(wxid, attach_id, remark_name);
                }

                // 卡片按钮 > 编辑命名规则
                if select_attach_card.btn_rename.existPoint(x, y) {
                    println!("[click] existPoint {}", "卡片按钮 > 编辑命名规则");
                   let mut rename_token = rename_tool_main(select_attach_card.input_rename.value().as_str());
                    let mut input_rename = select_attach_card.input_rename.clone();

                    app::add_timeout3(0.3,move|handle|{
                        if global_var::has_string(rename_token.as_str()) {
                            let data = global_var::get_string_default(rename_token.as_str());
                            if data.is_empty() {
                                println!("{} 用户取消 data-> [{}]",&rename_token,&data);
                                gui_util::message::sub_message(get_the_hwnd!(),gui_util::message::IconType::Warning,"用户取消处理",3500u64);
                            }else{
                                if !input_rename.value().as_bytes().eq(data.as_bytes()) {
                                    input_rename.set_value(data.as_str());
                                    println!("{} 名称更新 data-> [{}]",&rename_token,&data);
                                    gui_util::message::sub_message(get_the_hwnd!(),gui_util::message::IconType::Success,"名称已成功更新",3500u64);
                                }else {
                                    println!("{} 没有需要更新的名称内容 data-> [{}]",&rename_token,&data);
                                    gui_util::message::sub_message(get_the_hwnd!(),gui_util::message::IconType::Info,"名称内容没有变化",3500u64);
                                }
                            }
                            global_var::set_string(rename_token.as_str(),String::new());
                            app::remove_timeout3(handle);
                        }else {
                            app::repeat_timeout3(0.3, handle);
                        }
                    });

                    /*
                    std::thread::spawn(move|| loop{
                        std::thread::sleep(std::time::Duration::from_millis(300u64));
                        if global_var::has_string(rename_token.as_str()) {
                            let data = global_var::get_string_default(rename_token.as_str());
                            if data.is_empty() {
                                println!("{} 用户取消 data-> [{}]",&rename_token,&data);
                            }else{
                                input_rename.set_value(data.as_str());
                                println!("{} 名称更新 data-> [{}]",&rename_token,&data);
                            }
                            break;
                        }
                    });
                    */
                }

                true
            }

            enums::Event::Move => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;
                let mut new_show_cursor = false;

                initialize_window_hwnd!(hwnd);

                // 启用了预览图片模式
                if preview_win_show {

                    let mut new_show_cursor = false;

                    // 可选项
                    if !new_show_cursor { new_show_cursor = frame_check.existPoint(x, y) }

                    // 卡片按钮
                    if !new_show_cursor {
                        new_show_cursor = {
                            select_attach_card.btn_select.existPoint(x, y)||
                                select_attach_card.btn_remark.existPoint(x, y)||
                                select_attach_card.btn_rename.existPoint(x, y)
                        }
                    }

                    if preview_win.btn_close.existPoint(x,y)||new_show_cursor{
                        win.set_cursor(fltk::enums::Cursor::Hand);
                    } else {
                        win.set_cursor(fltk::enums::Cursor::Default);
                    }

                }
                else if scan_win_show {

                    if scan_preview_window.btn_clip.existPoint(x,y)
                     ||scan_preview_window.btn_scan_drag.existPoint(x,y)
                     ||scan_preview_window.btn_all_obj.existPoint(x,y)
                     ||scan_preview_window.btn_close.existPoint(x,y)
                    {
                        win.set_cursor(fltk::enums::Cursor::Hand);
                    } else {
                        win.set_cursor(fltk::enums::Cursor::Default);
                    }

                }
                // 正常窗口
                else  {
                // 可选项
                if !new_show_cursor { new_show_cursor = frame_check.existPoint(x, y) }

                // 卡片按钮
                if !new_show_cursor {
                    new_show_cursor = {
                            select_attach_card.btn_select.existPoint(x, y)||
                            select_attach_card.btn_remark.existPoint(x, y)||
                            select_attach_card.btn_rename.existPoint(x, y)
                    }
                }

                // 主界面按钮 打开 / 显示拖拽 / 显示帮助 / 开始
                if !new_show_cursor {
                    new_show_cursor = {
                        button_open_dir.existPoint(x, y)||
                            button_show_drag.existPoint(x, y)||
                            button_show_help.existPoint(x, y)||
                            button_start.existPoint(x,y)
                    }
                }

                // 缩略图（5张）
                if !new_show_cursor {
                    let mut index = 0;
                    for hotspot in &frame_thumbnail_preview.hotspot_list {
                        index+=1;
                        if hotspot.existPoint(x,y) {
                            new_show_cursor=true;
                            break;
                        }
                    }
                }

                // 图片预览大图
                if select_attach_card.thumbnail_preview.existPoint(x,y){
                    new_show_cursor=true;
                }

                if new_show_cursor!=show_cursor{
                    // 判断是否显示手型鼠标
                    if new_show_cursor {
                        win.set_cursor(fltk::enums::Cursor::Hand);
                    } else {
                        win.set_cursor(fltk::enums::Cursor::Default);
                    }
                    show_cursor=new_show_cursor ;
                }

                }
                true
            }

            _ => {
                false
            },
        }

    });



    initialize_window_hwnd!(hwnd);
    the_token
}
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

pub use hotspot::{*};
pub use text::{*};
pub use img::{*};
pub use message::{*};

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



#![allow(warnings, unused)]

use std::{env, thread};
use std::ffi::{c_int, c_long, c_void, OsStr,c_uint,};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use libc::c_longlong;
use serde_json::json;
use serde::{Deserialize, Serialize};
use serde_json::{Value};

pub type PCSTR =*const c_char;

type wchar_t = u16;
type WCHAR = wchar_t;

type LPCWCHAR = *const WCHAR;

use self::util::{encode_lpcstr, ansi_codepage_cstring};
pub mod util;

// #![crate_type = "staticlib"]
//  请注意 所有传入的文本都必须是utf8
#[link(name = "libWxIkunPlus", kind = "static")]
extern "C" {
    fn _setWinIcon(_hWnd: c_long) -> c_void;
    fn _setWinIconMain(_hWnd: c_long) -> c_void;
    fn _setShowWindows(_hWnd: c_long, visible: bool) -> bool;
    fn _set_tray() -> c_void;
    fn _createMutex(mutex:PCSTR) -> bool;
    fn _removeMutex(mutex:PCSTR) -> bool;
    fn _hasMutex(mutex:PCSTR) -> bool;
    fn _setStartup() -> bool;
    fn _hasStartup() -> bool;
    fn _openSelectFolder() -> c_void;
    fn _setWindowsTop(_hWnd: c_long, visible: bool) -> bool;
    fn _setCloseWindow(_hWnd: c_long, closeRoot: bool) -> bool;
    fn _openSelectFolder2() ->PCSTR;
    fn _Error(title:PCSTR, info:PCSTR) -> c_void;
    fn _Stop(mutex:PCSTR, info:PCSTR) -> c_void;
    fn _Confirm(title:PCSTR, info:PCSTR) -> bool;
    fn _Alert(mutex:PCSTR, info:PCSTR) -> bool;
    fn _getRegistrValue(hKey: c_long, _subKey:PCSTR, _key:PCSTR)->PCSTR;
    fn _hasWeChat() -> bool;
    fn _setTaskbarWin(_hWnd: c_long) -> c_void;
    fn _setMinWindows(_hWnd: c_long) -> bool;
    fn _findWindow(className:PCSTR, title:PCSTR) -> c_long;
    // fn _findWindowW(className:LPCWCHAR, title:LPCWCHAR) -> c_long;
    // fn _findWindowU8(className:PCSTR, title:PCSTR) -> c_long;
    fn _has_auto_sync() -> bool;
    fn _set_auto_sync(value:bool);
    fn _has_sync_token()-> bool;
    fn _hasStartupGlobalVar()-> bool;
    fn _getFocusTopWindow()->c_long;
    fn _getFocusWindow()->c_long;
    fn _findAllWindow(className:PCSTR, title:PCSTR) -> PCSTR;
    fn _isWindow(_hWnd: c_long) -> bool;
    fn _setWindowShake(hWnd: c_long);
    fn _getWindowRect(hWnd: c_long) -> PCSTR;
    fn _randomNum()->c_longlong;
    fn _setWindowTransparent(hWnd:c_long,transparent:c_int);
    fn _getfilePathSingle()->PCSTR;
    fn _setWindowEnabled(_hWnd: c_long, enabled: bool) -> bool;
    fn _hasInitWindowIsDisplayed ()->bool;
    fn _setInitWindowIsDisplayed(initWindowIsDisplayed:bool)->bool;
}

// 设置主窗口图标 从当前二进制获取
pub fn setWinIconMain(hWnd: i128) {
    unsafe {
        match hWnd.try_into() {
            Ok(hwnds) => {
                _setWinIconMain(hwnds);
            }
            Err(_) => {}
        }
    };
}

// 设置窗口图标 从当前二进制获取
pub fn setWinIcon(hWnd: i128) {
    unsafe {
        match hWnd.try_into() {
            Ok(hwnds) => {
                _setWinIcon(hwnds);
            }
            Err(_) => {}
        }
    };
}


// 关闭窗口
pub fn closeWindow(hWnd: i128, destroy: bool) {
    unsafe {
        match hWnd.try_into() {
            Ok(hwnds) => {
                _setCloseWindow(hwnds, destroy);
            }
            Err(_) => {}
        }
    };
}

// 设置窗口可见 如果可见会激活窗口
pub fn setwinVisible(hWnd: i128, visible: bool) -> bool {
    let mut res = false;

    unsafe {
        match hWnd.try_into() {
            Ok(hwnds) => {
                _setShowWindows(hwnds, visible);
            }
            Err(_) => {
                res = false;
            }
        }
    };
    return res;
}

// 设置窗口顶置
pub fn setWinTop(hWnd: i128, visible: bool) -> bool {
    let mut res = false;

    unsafe {
        match hWnd.try_into() {
            Ok(hwnds) => {
                _setWindowsTop(hwnds, visible);
            }
            Err(_) => {
                res = false;
            }
        }
    };
    return res;
}

// 启用托盘
pub fn set_tray() {
    unsafe {
        _set_tray();
    };
}

// 创建互斥体
pub fn createMutex(mutex: String) -> bool {
    let mut result = false;
    unsafe {
        match CString::new(mutex) {
            Ok(mutexs) => {
                result = _createMutex(mutexs.as_ptr());
            }
            Err(_) => {}
        }
    };

    result
}

// 删除互斥体
pub fn removeMutex(mutex: String) -> bool {
    let mut result = false;
    unsafe {
        match CString::new(mutex) {
            Ok(mutexs) => {
                result = _removeMutex(mutexs.as_ptr());
            }
            Err(_) => {}
        }
    };

    result
}

// 判断是否有互斥体
pub fn hasMutex(mutex: String) -> bool {
    let mut result = false;
    unsafe {
        match CString::new(mutex) {
            Ok(mutexs) => {
                result = _hasMutex(mutexs.as_ptr());
            }
            Err(_) => {}
        }
    };

    result
}

// 设置自启
pub fn setStartup() -> bool {
    unsafe {
        return _setStartup();
    };
}

pub fn hasStartup() -> bool {
    unsafe {
        return _hasStartup();
    };
}

// 文件夹选取器
pub fn openSelectFolder() -> String {
    unsafe {
        _openSelectFolder();
        let mut open_path = env::var("IKUN@SelectedFolderPath").unwrap_or_else(|_| "".to_owned());
        return open_path;
    };
}

// 将C字符串转换为Rust字符串
fn c_string_to_rust_string(ptr:PCSTR) -> String {
    unsafe {
        let c_str = CStr::from_ptr(ptr);
        let bytes = c_str.to_bytes();
        String::from_utf8_lossy(bytes).into_owned()
    }
}

// 启用托盘
pub fn openSelectFolder2() -> String {
    let mut result = String::new();
    
    let the_win = getFocusWindow();

    setwinVisible(the_win.clone(), false);
    
    unsafe { result = c_string_to_rust_string(_openSelectFolder2()) };
    
    setwinVisible(the_win.clone(), true);
    
    return result;
}

// 将Rust字符串转换为C字符串
fn rust_string_to_c_string(s: String) -> CString {
    if let Result::Ok(mut buff) = CString::new(s.as_str()) {
        return buff;
    };
    let c_ptr = CString::new("").unwrap();
    return c_ptr;
}

fn rust_string_to_ansi_str(s: String)->Vec<i8>{
    if let Result::Ok(item) = ansi_codepage_cstring(s) {
        return item;
    }
    let c_ptr = CString::new("").unwrap();
    let as_bytes = c_ptr.as_bytes().to_vec();
    let mut result = Vec::new();
    for value in as_bytes {
        result.push(value as i8);
    }

    return result;
}

// fn option_vec_u8_to_cstring(option_vec: Option<Vec<u8>>) -> Result<CString, &'static str> {
//     match option_vec {
//         Some(vec) => {
//             match CString::new(vec) {
//                 Ok(cstring) => Ok(cstring),
//                 Err(_) => Err("Failed to create CString"),
//             }
//         }
//         None => Err("Option<Vec<u8>> is None"),
//     }
// }


// // 将Rust UTF-8字符串转换为Windows API中的A字符
// fn utf8_to_ansi(s: &str) -> Vec<c_char> {
//     let wide: Vec<u16> = OsStr::new(s).encode_wide().collect();
//     let wide_len = wide.len() + 1;

//     let mut ansi: Vec<c_char> = Vec::with_capacity(wide_len);
//     let ansi_len = wide.len();

//     unsafe {
//         WideCharToMultiByte(
//             CP_UTF8,
//             0,
//             wide.as_ptr(),
//             wide_len as i32,
//             ansi.as_mut_ptr(),
//             ansi_len as i32,
//             ptr::null(),
//             ptr::null_mut(),
//         );
//         // 确保在末尾添加一个空字符
//         ansi.push(0);
//         ansi.set_len(ansi_len);
//     }

//     ansi
// }

// MessageBox -> alert
pub fn alert(title: String, message: String) -> bool {
    unsafe {
        return _Alert(
            rust_string_to_ansi_str(title).as_ptr(),
            rust_string_to_ansi_str(message).as_ptr(),
        );
    }
    return false;
}

// MessageBox -> confirm
pub fn confirm(title: String, message: String) -> bool {
    unsafe {
        return _Confirm(
            rust_string_to_ansi_str(title).as_ptr(),
            rust_string_to_ansi_str(message).as_ptr(),
        );
    }
    return false;
}

// MessageBox -> stop
pub fn stop(title: String, message: String) {
    unsafe {
        _Stop(
            rust_string_to_ansi_str(title).as_ptr(),
            rust_string_to_ansi_str(message).as_ptr(),
        );
    }
}

// MessageBox -> error
pub fn error(title: String, message: String) {
    unsafe {
        _Error(
            rust_string_to_ansi_str(title).as_ptr(),
            rust_string_to_ansi_str(message).as_ptr(),
        );
    }
}

pub enum HKEY {
    HKEY_CLASSES_ROOT = 0x80000000,
    HKEY_CURRENT_USER = 0x80000001,
    HKEY_LOCAL_MACHINE = 0x80000002,
    HKEY_USERS = 0x80000003,
}


pub fn getRegistrValue(hKey: HKEY, subKey: String, valueKey: String) -> String {
    let mut result = String::new();
    unsafe {
        let mut c_result = _getRegistrValue(
            c_long::from(hKey as i32),
            rust_string_to_ansi_str(subKey).as_ptr(),
            rust_string_to_ansi_str(valueKey).as_ptr(),
        );
        result =c_string_to_rust_string(c_result);
    }
    result
}

// 判断wx进程是否存在
pub fn hasWeChat()->bool {
    let mut result = false;
    unsafe {
        result= _hasWeChat();
    }
    result
}

pub fn hasWeChatWin()->bool {
    let mut result = false;
    unsafe {
        let hwnd_01 = findWindow("WeChatMainWndForPC", "");
        if(hwnd_01!=0){
           return true;
        }

        let hwnd_02 = findWindow("ChatWnd", "");
        if(hwnd_02!=0){
            return true;
        }

        let hwnd_03 = findWindow("SubscriptionWnd", "");
        if(hwnd_03!=0){
            return true;
        }
    }
    result
}

// 把一个傀儡窗口变成主窗口的托盘
pub fn setTaskbarWin(hWnd: i128) {
    unsafe {
       _setTaskbarWin(hWnd as i32);
    }
}

pub fn setMinWindows(hWnd: i128) -> bool {
    unsafe {
        _setMinWindows(hWnd as i32)
    }
}

pub fn isWindow(hWnd: i128) -> bool {
    unsafe {
        _isWindow(hWnd as i32)
    }
}

pub fn setWindowShake(hWnd: i128)  {
    unsafe {
        _setWindowShake(hWnd as i32)
    }
}

// 搜索窗口
pub fn findWindow(className: &str, titleName: &str)->i128 {
    let mut hwnd:i128 = 0;
    unsafe {
        let mut className = rust_string_to_ansi_str(className.to_string());
        let mut titleName = rust_string_to_ansi_str(titleName.to_string());
      
        hwnd= _findWindow(className.as_ptr(), titleName.as_ptr()/*,className_len as i32,titleName_len as i32 */).into();
    }
    return hwnd;
}

// // 搜索窗口
// pub fn findWindowU8(className: String, titleName: String)->i128 {
//     let mut hwnd:i128 = 0;
//     unsafe {
//         // let mut className = rust_string_to_c_string(className);
//         // let mut titleName = rust_string_to_c_string(titleName);
//         hwnd= _findWindowU8(encode_lpcstr(className.as_str()).as_ptr(), encode_lpcstr(titleName.as_str()).as_ptr() /*,className_len as i32,titleName_len as i32 */).into();
//     }
//     return hwnd;
// }

// 判断是否启用了自动更新（如果wx进程不存在也会返回false）
pub fn has_auto_sync() -> bool{
    let mut result = false;
    unsafe {
        result =_has_auto_sync();
    }

    result
}

// 判断是否启用了自动更新（如果wx进程不存在也会返回false）
pub fn has_auto_sync_all() -> bool{
    let mut result = false;
    unsafe {
        result = hasWeChat()&&hasWeChatWin()&&_has_auto_sync();
    }
    println!("has_auto_sync-> {}",&result);
    result
}

// 设置自动更新
pub fn set_auto_sync(value: bool){
    unsafe {
       _set_auto_sync(value);
    }
}

// 是否立即同步
pub fn has_sync_token()->bool{
    unsafe{
        _has_sync_token()
    }
}

// 是否立即同步
pub fn hasStartupGlobalVar()->bool{
    unsafe{
        _hasStartupGlobalVar()
    }
}

pub fn getFocusWindow()->i128{
    unsafe{
        _getFocusWindow() as i128
    }
}

pub fn getFocusTopWindow()->i128{
    unsafe{
        _getFocusTopWindow() as i128
    }
}

fn get_str_to_long_vec(c_result:PCSTR)->Vec<i128>{
    let mut list:Vec<i128> = Vec::new();
    let result =c_string_to_rust_string(c_result);

    let long_str = String::from("1234567890");
    let mut the_data = String::new();

    for char in result.chars() {
        if(long_str.contains(char)){
            the_data.push(char);
        }else{
            if !the_data.is_empty() {

                let parsed_number: Result<i32, _> = the_data.parse();
                if let Ok(parsed_number) = parsed_number {
                    list.push(parsed_number as i128);
                }
                the_data.clear();
            }

        }
    }

    if !the_data.is_empty() {
        let parsed_number: Result<i32, _> = the_data.parse();
        if let Ok(parsed_number) = parsed_number {
            list.push(parsed_number as i128);
        }
        the_data.clear();
    }

    list
}


pub fn findAllWindow(className: &str, titleName: &str)->Vec<i128>{
    unsafe{
        let mut className = rust_string_to_ansi_str(className.to_string());
        let mut titleName = rust_string_to_ansi_str(titleName.to_string());
        let c_result = _findAllWindow(className.as_ptr(),titleName.as_ptr());
        get_str_to_long_vec(c_result)
    }
}

#[derive(Debug  )]
pub struct RECT {
    pub left:i32,
    pub top:i32,
    pub bottom:i32,
    pub right:i32,
    pub height:i32,
    pub width:i32
}
pub fn getWindowRect(hWnd: i128)->RECT{
    let mut rect = RECT{
        left: 0,
        top: 0,
        bottom: 0,
        right: 0,
        height: 0,
        width: 0,
    };

    unsafe {
       let c_result_json = _getWindowRect(hWnd as c_long);

        let data = c_string_to_rust_string(c_result_json);

        if let Ok(c_rect) = serde_json::from_str(data.as_str()) as serde_json::Result<Value> {
            rect.bottom = c_rect["bottom"].as_i64().unwrap_or_else(||{0}) as i32;
            rect.left = c_rect["left"].as_i64().unwrap_or_else(||{0}) as i32;
            rect.top = c_rect["top"].as_i64().unwrap_or_else(||{0}) as i32;
            rect.right = c_rect["right"].as_i64().unwrap_or_else(||{0}) as i32;
            rect.height = c_rect["height"].as_i64().unwrap_or_else(||{0}) as i32;
            rect.width = c_rect["width"].as_i64().unwrap_or_else(||{0}) as i32;
        }

    }

    rect
}

// 随机数
pub(crate) fn randomNum() -> i128{
    unsafe {
        _randomNum() as i128
    }
}

pub fn setWindowTransparent(hWnd:i128,transparent:i32){
    unsafe {
        _setWindowTransparent(hWnd as c_long,transparent as c_int);
    }
}

pub fn getClipFilePathSingle()->String{
    unsafe{
        let c_result = _getfilePathSingle();
        c_string_to_rust_string(c_result)
    }
}

pub fn setWindowEnabled(hWnd: i128, enabled: bool) -> bool{
    unsafe {
        _setWindowEnabled(hWnd as c_long,enabled)
    }
}

pub fn hasInitWindowIsDisplayed()->bool{
    unsafe {
        _hasInitWindowIsDisplayed()
    }
}
pub fn setInitWindowIsDisplayed(initWindowIsDisplayed:bool)->bool{
    unsafe {
        _setInitWindowIsDisplayed(initWindowIsDisplayed)
    }
}
#![allow(warnings, unused)]

use chrono::Local;
use glob::glob;
use hotwatch::{
    blocking::{Flow, Hotwatch},
    EventKind,
};

use libc::c_void;
use rusqlite::{params, Connection, Result};

use fltk::draw::font;
use fltk::enums::{Cursor, Event, Font, LabelType};
use fltk::frame::Frame;
use fltk::group::Group;
use fltk::input::{Input, InputType, IntInput};
use fltk::text::TextDisplay;
use fltk::{
    app::handle,
    text::{TextBuffer, TextEditor},
};
use fltk::{button::Button, enums::Align, window::DoubleWindow};
use fltk::{enums::Color, enums::FrameType};
use fltk::{prelude::*, window::Window, *};
use magic_crypt::MagicCryptTrait;
use msgbox::IconType;
use serde_json::json;
use std::collections::{hash_map::DefaultHasher, HashMap};
use std::mem::transmute;
use fltk_theme::ColorTheme;
use fltk_theme::color_themes;
use fltk_theme::WidgetTheme;
use fltk_theme::ThemeType;
use fltk_theme::WidgetScheme;
use fltk_theme::SchemeType;
use crate::{console_log, gui_manage_item};
use std::{
    env,
    ffi::{c_int, c_long, OsStr},
    fs,
    hash::{Hash, Hasher},
    io,
    path::{Path, PathBuf},
    process,
    process::{Command, Stdio},
    thread,
    time::Duration,
};

use crate::{atomic_util, global_var, handle_dat, libWxIkunPlus::{self, setTaskbarWin}, util::{self, str_eq_ostr, str_eq_str, Sleep}, wh_mod::convert::{convert_bat_images}, wh_mod, global_var_util, get_bool,  gui_util, set_bool, gui_select_user_ui};
use crate::wh_mod::parse_dat_path;

use std::sync::atomic::{AtomicBool, AtomicI32, AtomicUsize, Ordering,AtomicI64};
use std::sync::{Arc, Condvar, Mutex,RwLock};
use serde_json::Value as Json;
use toml::Value as Toml;



pub struct AppVersionInfo {

}

fn toml2json(toml: Toml) -> Json {
    match toml {
        Toml::String(s) => Json::String(s),
        Toml::Integer(i) => Json::Number(i.into()),
        Toml::Float(f) => {
            let n = serde_json::Number::from_f64(f).expect("float infinite and nan not allowed");
            Json::Number(n)
        }
        Toml::Boolean(b) => Json::Bool(b),
        Toml::Array(arr) => Json::Array(arr.into_iter().map(toml2json).collect()),
        Toml::Table(table) => {
            Json::Object(table.into_iter().map(|(k, v)| (k, toml2json(v))).collect())
        }
        Toml::Datetime(dt) => Json::String(dt.to_string()),
    }
}

pub fn get_app_version_info () -> Json {
    const APP_VERSION: &str = include_str!("../../../Cargo.toml");
    // println!("toml2json-> {:?}",toml2json(APP_VERSION));

    match APP_VERSION.parse() {
        Ok(toml) => {
            let json = toml2json(toml);
             return json
        }
        Err(error) => println!("failed to parse TOML: {}", error),
    }

    json!("")
}

struct MainTheme {
    /**主背景颜色 */
    background: Color,
    /**次背景*/
    backgroundMain: Color,
    /**顶部文字和logo */
    logo: Color,
    /**卡片文本成功 */
    cardSuccessText: Color,
    /**卡片文本失败 */
    cardFailureText: Color,
    /**卡片文本 */
    cardText: Color,
    /**卡片描边 */
    cardStroke: Color,
    /**分割线 */
    cuttingLine: Color,
    /** 底部三个按钮的颜色*/
    botBtnColor: Color,
    /** 底部三个按钮的图标颜色*/
    botBtnIconColor: Color,
    // null
    not: Color,
}

// 统一在这里定义主题颜色
fn getMainTheme() -> MainTheme {
    let mut mainTheme: MainTheme = MainTheme {
        /**主背景颜色 */
        background: Color::rgb_color(24, 24, 24),
        /**次背景*/
        backgroundMain: Color::rgb_color(17, 17, 17),
        /**顶部文字和logo */
        logo: Color::rgb_color(122, 120, 120),
        /**卡片文本成功 */
        cardSuccessText: Color::rgb_color(99, 138, 99),
        /**卡片文本失败 */
        cardFailureText: Color::rgb_color(189, 79, 79),
        /**卡片文本 */
        cardText: Color::rgb_color(255, 255, 255),
        /**卡片描边 */
        cardStroke: Color::rgb_color(46, 46, 46),
        /**分割线 */
        cuttingLine: Color::rgb_color(38, 38, 38),
        /** 底部三个按钮的颜色*/
        botBtnColor: Color::rgb_color(0, 0, 0),
        /** 底部三个按钮的图标颜色*/
        botBtnIconColor: Color::rgb_color(125, 125, 125),
        not: Color::from_u32(0),
    };
    return mainTheme;
}

// 设置界面主题
pub fn setMainTheme() {
    let mut mainTheme: MainTheme = getMainTheme();
    app::set_background_color(24, 24, 24);
    // app::set_fonts("name");
    app::set_frame_shadow_width(0);
    app::set_frame_color(mainTheme.not);
    app::set_background2_color(17, 17, 17);
    app::set_foreground_color(17, 17, 17);
    app::set_selection_color(17, 17, 17);
    // app::set_frame_type2(old_frame, new_frame);

    app::set_frame_type(FrameType::NoBox);
    app::set_inactive_color(24, 24, 24);
    app::set_frame_border_radius_max(0);
    app::set_frame_type2(FrameType::BorderBox, FrameType::NoBox);
    app::set_visible_focus(false);
    app::set_frame_shadow_width(0);
    app::swap_frame_type(FrameType::NoBox);
    app::set_menu_linespacing(0);
    app::set_scrollbar_size(0);
}

// 设置背景为图片（主视图）
fn setWinBackground_forRoot_image(appMainWin: &mut window::DoubleWindow) -> Frame {
    let background_image = image::PngImage::from_data(include_bytes!("../../assets/main_back.png"))
        .expect("set main icon error");
    // image::SvgImage::from_data(include_str!("../../assets/main_back.svg"))
    // .expect("set main icon error");
    let mut frame = Frame::default().with_size(600, 0).center_of(appMainWin);
    frame.set_frame(FrameType::EngravedBox);
    frame.set_image(Some(background_image));
    return frame;
}

// 界面会回传为这个参数 用来控制鼠标手型
struct PointExistHasmap {
    // 关闭按钮
    quit: bool,
    // 按钮:: 打开dat所在路径
    shellOpenDatDir: bool,
    // 按钮:: 导出此文件夹
    shellOpenExportDir: bool,
    // 按钮:: 管理
    manageItme: bool,
    // 按钮:: 测试
    test: bool,
    // 按钮:: 创建
    create: bool,
    // 选项::自启动
    starting: bool,

    // 鼠标在按钮原件中
    existAllBtn: bool,
}

// 判断鼠标坐标是否在此元素内
fn getFormPointSpace(x: i32, y: i32) -> PointExistHasmap {
    let mut point_exist_hasmap = PointExistHasmap {
        quit: false,
        shellOpenDatDir: false,
        shellOpenExportDir: false,
        manageItme: false,
        test: false,
        starting: false,
        create: false,
        existAllBtn: false,
    };

    point_exist_hasmap.quit = x > 545 && x < 575 && y > 13 && y < 51;
    point_exist_hasmap.manageItme = x > 342 && x < 342 + 60 && y > 273 && y < 273 + 36;
    point_exist_hasmap.shellOpenDatDir = x > 511 && x < 511 + 36 && y > 147 && y < 147 + 39;
    point_exist_hasmap.shellOpenExportDir = x > 511 && x < 511 + 36 && y > 219 && y < 219 + 39;
    point_exist_hasmap.starting = x > 85 && x < 85 + 25 && y > 490 && y < 490 + 25;
    point_exist_hasmap.test = x > 413 && x < 413 + 60 && y > 273 && y < 273 + 36;
    point_exist_hasmap.create = x > 486 && x < 486 + 60 && y > 273 && y < 273 + 36;

    let mut win_coords_cursor_list = vec![
        point_exist_hasmap.quit,
        point_exist_hasmap.manageItme,
        point_exist_hasmap.shellOpenDatDir,
        point_exist_hasmap.shellOpenExportDir,
        point_exist_hasmap.starting,
        point_exist_hasmap.test,
        point_exist_hasmap.create,
    ];

    let mut has_cursor = false;

    for value in win_coords_cursor_list.iter() {
        // 关闭按钮
        if *(value) {
            has_cursor = true;
        }
    }

    point_exist_hasmap.existAllBtn = has_cursor;

    return point_exist_hasmap;
}


// 设置自启动按钮的状态
fn addBtnEnableStarting(appMainWin: &mut window::DoubleWindow) -> gui_util::img::ImgPreview  {
    let w_h = 20;
    let mut preview = gui_util::img::ImgPreview::new(90-3, 493, w_h, w_h, "gui::preview_main::index::user_select");

    if libWxIkunPlus::hasStartup() {
        preview.from_data(include_bytes!("../../assets/enable.png").to_vec(), 0, 0, w_h, w_h);
    }else{
        preview.from_data(include_bytes!("../../assets/un_enable.png").to_vec(), 0, 0, w_h, w_h);
    }
    
   

    return preview;
}

// dat的路径的输入框
fn addInput_shellOpenDatDir(appMainWin: &mut window::DoubleWindow) -> ConsoleItme {
    let mut mainTheme: MainTheme = getMainTheme();

    let mut buf = fltk::text::TextBuffer::default();
    let mut txt = fltk::text::TextEditor::default()
        .with_size(447, 25)
        .center_of_parent();
    // txt.set
    txt.set_buffer(buf.clone());
    txt.set_frame(FrameType::NoBox);
    txt.set_text_color(Color::from_rgb(120, 120, 120));
    txt.set_color(mainTheme.backgroundMain);
    txt.set_label_type(fltk::enums::LabelType::None);
    // txt.set_text_size(15);
    txt.set_pos(46, 153 + 3);
    txt.set_text_size(11);
    // txt.set_scrollbar_size(0);
    txt.set_scrollbar_size(3);
    txt.set_callback(move |usetup| {
        println!(
            "addInput_shellOpenExportDir => {} {}",
            usetup.buffer().unwrap().text(),
            usetup.buffer().unwrap().length()
        );
        if !wh_mod::convert::is_developer(){
        let mut buff = usetup.buffer().unwrap();
        buff.remove(0, buff.length());
        console_log!("[错误] 编辑被禁止".to_string());
        }
    });
  
    // buf.set(true);

    txt.show();

    return ConsoleItme {
        edit: txt,
        buff: buf,
    };
}

// 保存到的输入框
fn addInput_shellOpenExportDir(appMainWin: &mut window::DoubleWindow) -> ConsoleItme {
    let mut mainTheme: MainTheme = getMainTheme();

    let mut buf = fltk::text::TextBuffer::default();
    let mut txt = fltk::text::TextEditor::default()
        .with_size(447, 27)
        .center_of_parent();
    txt.set_buffer(buf.clone());
    txt.set_frame(FrameType::NoBox);
    txt.set_text_color(Color::from_rgb(120, 120, 120));
    txt.set_color(mainTheme.backgroundMain);
    txt.set_label_type(fltk::enums::LabelType::None);
    txt.set_text_size(13);
    txt.set_pos(46, 223 + 5);
    txt.set_scrollbar_size(2);
    // txt.set_scrollbar_align(Align:);
    txt.set_callback(move |usetup| {
        println!(
            "addInput_shellOpenDatDir => {} {}",
            usetup.buffer().unwrap().text(),
            usetup.buffer().unwrap().length()
        );
    });
    txt.show();

    return ConsoleItme {
        edit: txt,
        buff: buf,
    };
}

// 名称:
fn addInput_shellName(appMainWin: &mut window::DoubleWindow) -> ConsoleItme {
    let mut mainTheme: MainTheme = getMainTheme();

    let mut buf = fltk::text::TextBuffer::default();
    let mut txt = fltk::text::TextEditor::default()
        .with_size(180+8, 27)
        .center_of_parent();
    txt.set_buffer(buf.clone());
    txt.set_frame(FrameType::NoBox);
    txt.set_text_color(Color::from_rgb(120, 120, 120));
    txt.set_color(mainTheme.backgroundMain);
    txt.set_label_type(fltk::enums::LabelType::None);
    txt.set_text_size(15);
    txt.set_pos(98, 279);

    // txt.set_changed();
    txt.set_callback(move |usetup| {
        let mut stext = usetup.buffer().unwrap();
        if (stext.length() > 30) {
            stext.remove(30, stext.length());
        }
        println!("addInput_shellName => {} {}", stext.text(), stext.length());
    });
    txt.show();

    return ConsoleItme {
        edit: txt,
        buff: buf,
    };
}

// 打印台的控制
struct ConsoleItme {
    edit: TextEditor,
    buff: TextBuffer,
}

// 初始化打印台元素
fn addConsole(appMainWin: &mut window::DoubleWindow) -> ConsoleItme {
    let mut mainTheme: MainTheme = getMainTheme();

    let mut buf = fltk::text::TextBuffer::default();
    let mut txt = fltk::text::TextEditor::default()
        .with_size(528, 98)
        .center_of_parent();
    txt.set_buffer(buf.clone());
    txt.set_frame(FrameType::NoBox);
    txt.set_text_color(Color::from_rgb(120, 120, 120));
    txt.set_color(mainTheme.backgroundMain);
    txt.set_label_type(fltk::enums::LabelType::None);
    txt.set_text_size(12);
    txt.set_pos(34, 365);
    txt.set_scrollbar_size(6);
    txt.show();

    return ConsoleItme {
        edit: txt,
        buff: buf,
    };
}

// 处理文本添加时候风格的宏
macro_rules! setTheStyleToInterface {
    ($b:expr) => {{
        let MainTheme: MainTheme = getMainTheme();
        $b.show_cursor(false);
        $b.set_text_color(MainTheme.botBtnIconColor);
        $b.set_text_size(11);
        $b.set_label_type(LabelType::None);
        $b.set_color(MainTheme.backgroundMain);
        $b.clear_visible_focus();
        $b.set_frame(FrameType::FlatBox);
        $b.show_cursor(false);
        $b.deactivate();
        $b.set_text_color(MainTheme.cardText);
    }};
    ($b:expr,$x:expr,$y:expr,$w:expr,$h:expr) => {{
        let MainTheme: MainTheme = getMainTheme();
        $b.show_cursor(false);
        $b.set_text_color(MainTheme.botBtnIconColor);
        $b.set_text_size(11);
        $b.resize($x, $y, $w, $h);
        $b.set_label_type(LabelType::None);
        $b.set_color(MainTheme.backgroundMain);
        $b.clear_visible_focus();
        $b.set_frame(FrameType::FlatBox);
        $b.show_cursor(false);
        $b.deactivate();
        $b.set_text_color(MainTheme.cardText);
    }};

    ($b:expr,$x:expr,$y:expr,$w:expr,$h:expr,$fsize:expr) => {{
        let MainTheme: MainTheme = getMainTheme();
        $b.show_cursor(false);
        $b.set_text_color(MainTheme.botBtnIconColor);
        $b.set_text_size($fsize);
        $b.resize($x, $y, $w, $h);
        $b.set_label_type(LabelType::None);
        $b.set_color(MainTheme.backgroundMain);
        $b.clear_visible_focus();
        $b.set_frame(FrameType::NoBox);
        $b.show_cursor(false);
        $b.deactivate();
        $b.set_text_color(MainTheme.cardText);
    }};
}

struct MsgAttachExport {
    id: i32,
    time: String,
    name: String,
    ext: String,
    input: String,
    ouput: String,
    message: String,
    user_name: String,
}

pub struct MianWindowItme {
    appMainWin: DoubleWindow,
    appRootView: DoubleWindow,
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


fn get_window_hwnd(win:&window::Window) -> i128 {
    let mut xclass = win.xclass().unwrap_or_else(||String::new());
    let mut xtitle =String::new();// win.label();

    let hwnd = libWxIkunPlus::findWindow(xclass.as_str(), xtitle.as_str()) ;
    println!("xclass<{}> xtitle<{}> hwnd<{}>", xclass,xtitle, hwnd);
    hwnd
}

// 主界面
pub fn mianWindow(show: bool) -> Result<MianWindowItme> {

    set_theme!();

    let version_info = get_app_version_info();
    let version =  (version_info["package"]["version"]).as_str().unwrap();
    println!("{}",&version);
    let mut mainTheme: MainTheme = getMainTheme();

    let mut appMainWin = Window::new(0, 0, 600, 531, "Ikun导出");
    appMainWin.set_xclass("app_main_win_wx_dat_viewer_auto_export_rust");

    app::set_scrollbar_size(3);

    app::set_selection_color(24, 24, 24);
    let mut cwd = env::current_dir().expect("get current_dir error ");
    appMainWin.set_border(false);

    // 主界面的窗口 2  悬浮在主窗口1上面
    let mut appRootView = window::Window::new(0, 0, 600, 531, "mian");
    setWinBackground_forRoot_image(&mut appRootView);
    // 界面
    let mut btnEnableStarting = addBtnEnableStarting(&mut appRootView);
    let mut input_shellOpenExportDir = addInput_shellOpenExportDir(&mut appRootView);
    let mut input_shellOpenDatDir = addInput_shellOpenDatDir(&mut appRootView);
    let mut input_Console = addConsole(&mut appRootView);
    let mut input_shellName = addInput_shellName(&mut appRootView);
    let mut sync_type = String::new();
    let mut build_name = if wh_mod::convert::is_build_52pojie() {"52破解专版"} else {"开源版"};

    if(libWxIkunPlus::has_auto_sync()){
        sync_type=(format!("[用户] 自动同步开启"));
    }
    else if (wh_mod::convert::is_developer()) {
        sync_type=(format!("[同步]{}", "自动同步已启用 因为开发者模式有效"));
        build_name = "开发者版";
    }
    else {
        sync_type=(format!("[用户] 自动同步关闭"));
    }

    if !wh_mod::convert::is_developer(){
    input_Console.buff.set_text(
        format!(
        r#"作者 @Ikun 软件开源协议 GPL 3.0 (但是并不包含解码算法) 版本：{} ({})
        本软件 是免费软件 如果付费请维权退款
        本软件只供备份自己的图片禁止用于其他用途
        {}"#
        ,version ,build_name,sync_type).replace("  ","").as_str()
    );
    }else {
        input_Console.buff.set_text(("初始化成功 [开发者模式]"));
    }

    // 界面
    appRootView.end();
    appMainWin.clone().center_screen(); // 将窗口居中

    appMainWin.hide();
    appRootView.hide();
    appMainWin.end();

    let mut input_buff_Console_move = input_Console.buff.clone();

    thread::spawn(move || loop {
        Sleep(150);
        let mut console_message = handle_dat::get_console_message().replace("\n\n", "\n");

        if console_message.starts_with('\n') {
            console_message = console_message.trim_start_matches('\n').to_string();
        }

        if (console_message.len() < 5) {
            continue;
        };

        let mut newline_count = 0;

        for line in input_buff_Console_move.text().lines() {
            newline_count += 1
        }

        if (newline_count > 5) {
            input_buff_Console_move.remove(0, input_buff_Console_move.length());
            input_buff_Console_move.set_text(&console_message);
        } else {
            input_buff_Console_move.append(&format!("\n{}", &console_message));
        }
    });

    let mut dat_buff_move= input_shellOpenDatDir.buff.clone();
    let mut copy_btnEnableStarting = btnEnableStarting.clone();
    thread::spawn(move || loop {
        let mut oid_app_start = false;

        if !libWxIkunPlus::has_auto_sync()!=oid_app_start{
            oid_app_start = true;
            copy_btnEnableStarting.from_data(include_bytes!("../../assets/enable.png").to_vec(), 0, 0, 20, 20);
        }else{
            oid_app_start = false;
            copy_btnEnableStarting.from_data(include_bytes!("../../assets/un_enable.png").to_vec(), 0, 0, 20, 20);
        }

        Sleep(550);

        let input_select_dir = global_var::get_string_default("user::config::input_select_dir");
        let user_select_path = global_var::get_string_default("user::config::user_select_path");
        let user_select_wxid = global_var::get_string_default("user::config::user_select_wxid");

        if !user_select_path.is_empty()&&!input_select_dir.is_empty()&&global_var::get_bool_default("gui::open::handle_dat") {


            let mut new_buff = format!("{}/{}/FileStorage/MsgAttach/{}",input_select_dir,user_select_wxid,user_select_path);

            // 判断路径有效性 无效则换文件夹  因为有些用户是可以多账户登录的
            if(!Path::new(new_buff.as_str()).exists()){
                let read_root_wxid_list  = wh_mod::wx_read_root_wxid(&Path::new(input_select_dir.as_str()).to_path_buf());
                for read_root_wxid in read_root_wxid_list {
                    if Path::new(read_root_wxid.attach.join(user_select_path.as_str() ) .as_os_str() ).exists(){
                        new_buff = format!("{}/{}",read_root_wxid.attach.to_str().unwrap(),user_select_path);
                        break;
                    }
                }
            }

            if(global_var::get_bool_default("user::config::check_button_the_month")){
                new_buff = new_buff+"*the_month";
            }
            if(global_var::get_bool_default("user::config::check_button_source")){
                new_buff = new_buff+"*source";
            }
            if(global_var::get_bool_default("user::config::check_button_thumbnail")){
                new_buff = new_buff+"*thumbnail";
            }

            if(!new_buff.as_bytes().eq(dat_buff_move.text().as_bytes() )){
                dat_buff_move.remove(0,dat_buff_move.length());
                dat_buff_move.append(new_buff.as_str());
            }

        }

    });

    let mut copy_AppRootView = appRootView.clone();
    let mut copy_appMainWin = appMainWin.clone();
    // let mut copy_dock_win = dock_win.clone();

    let mut g_appMainWinHwnd = 0;
    // let mut g_copy_dock_win_hwnd = 0;

    appMainWin.handle({
        let mut x = 0;
        let mut y = 0;
        let mut point_exist_hasmap = getFormPointSpace(x, y);
        let mut has_show = false;
        let mut copy_appMainWin = copy_appMainWin.clone();
        if(g_appMainWinHwnd.eq(&0)){
            g_appMainWinHwnd = get_window_hwnd(&copy_appMainWin);
            }
        move |win, ev| match ev {
            enums::Event::Focus=>{
                if(g_appMainWinHwnd.eq(&0)){
                    g_appMainWinHwnd = get_window_hwnd(&copy_appMainWin);
                    }
                true
            }
            enums::Event::Show => {
                copy_AppRootView.set_visible_focus();

                if(g_appMainWinHwnd.eq(&0)){
                    g_appMainWinHwnd = get_window_hwnd(&copy_appMainWin);
                    }

                env::set_var("ikunWinHwnd", format!("{}",g_appMainWinHwnd).to_string());
                // unsafe { setWinIcon(appMainWinHwnd.try_into().unwrap()) };
                libWxIkunPlus::setWinIconMain(g_appMainWinHwnd);

                
                // libWxIkunPlus::setwinVisible(g_copy_dock_win_hwnd , true);

                println!("Show => window hwnd:{}",g_appMainWinHwnd);
                true
            }
            enums::Event::Close=>{

                println!("Close => window as {}",0);
                true
            }
            enums::Event::Focus=>{
            
            
                true
            }
            enums::Event::Push => {
                // 关闭按钮
                if (point_exist_hasmap.quit) {
                    libWxIkunPlus::setwinVisible(g_appMainWinHwnd , false);
                    fltk::app::quit();
                    // libWxIkunPlus::setwinVisible(g_copy_dock_win_hwnd , false);
                    // unsafe { setShowWindows((copyappMainWin.raw_handle() as i128).try_into().unwrap(), false) };
                }
                let mut has_inputPath = false;
                let mut has_ouputPath = false;
                let mut has_name = false;

                if (point_exist_hasmap.create) {
                    input_Console.buff.set_text("[用户] 创建新的配置文件");
                    println!("click => create");
                } else if (point_exist_hasmap.manageItme) {
                    // input_Console
                    //     .buff
                    //     .set_text("[用户] 很抱歉 当前还不支持配置管理");
                    println!("click => manageItme");

                    gui_manage_item::ManageItmeMain();
                } else if (point_exist_hasmap.shellOpenDatDir) {
                    input_Console
                        .buff
                        .set_text("[用户] 打开选取原始文件夹(dat 原目录)");
                  
                    // 有wx进程 而且有窗口
                    if(wh_mod::convert::is_developer()||(libWxIkunPlus::hasWeChat()&&libWxIkunPlus::hasWeChatWin())){
                        gui_select_user_ui::manage_tool_main();
                    // gui_select_user_base::mian_window();

                    }else{
                        // thread::spawn(||{
                        //     // libWxIkunPlus::stop("错误".to_owned(),"当前未发现wx进程或者未登录 拒绝提供选取方案".to_owned());
                        //     dialog::alert_default(format!("\n[错误] {}","当前未发现wx进程或者未登录 拒绝提供选取方案").as_str());

                        // });
                        libWxIkunPlus::stop("错误".to_owned(),"当前未发现wx进程或者未登录 拒绝提供选取方案".to_owned());
                        // dialog::alert_default(format!("\n[错误] {}","当前未发现wx进程或者未登录 拒绝提供选取方案").as_str());
                    }

                    println!("click => shellOpenDatDir");
                } else if (point_exist_hasmap.shellOpenExportDir) {
                    input_Console.buff.set_text("[用户] 打开选取导出到的文件夹");
                    
                    let mut open_path = libWxIkunPlus::openSelectFolder2();

                    input_Console
                        .buff
                        .append(format!("\n[选取器] 用户输入的文件路径为: {}",open_path).as_str());
                   
                    if(open_path.len()>2){
                        input_shellOpenExportDir.buff.remove(0, input_shellOpenExportDir.buff.length());
                        input_shellOpenExportDir.buff.append(&open_path);
                        if(input_shellName.buff.length()<2){
                            input_shellName.buff.remove(0, input_shellName.buff.length());
                            let file_name = Path::new(&open_path).file_name().unwrap();
                            input_shellName.buff.append(&format!("{:#?}",file_name).replace("\"", ""));
                        }
                    }
                  

                    println!("click => shellOpenExportDir");
                } else if (point_exist_hasmap.starting) {
                    input_Console.buff.set_text("[用户] 配置自启动");
                    // input_Console
                    // .buff
                    // .append(format!("\n[错误] 暂时不支持此功能 使用其他软件添加").as_str());
                    
                    if(libWxIkunPlus::setStartup()){
                       input_Console
                   .buff
                 .append(format!("\n[状态] 添加自启动成功").as_str());  
                    }else{
                        input_Console
                        .buff
                      .append(format!("\n[状态] 自启动已被移除").as_str());  
                    }
                    
                    // if libWxIkunPlus::hasStartup() {
                    //     btnEnableStarting.from_data(include_bytes!("../../assets/enable.png").to_vec(), 0, 0,20, 20);
                    // }else{
                    //     btnEnableStarting.from_data(include_bytes!("../../assets/un_enable.png").to_vec(), 0, 0,20, 20);
                    // }

                    println!("click => starting");
                } else if (point_exist_hasmap.test) {
                    input_Console.buff.set_text("[用户] 测试新的配置文件");
                    println!("click => test");
                }

                if (point_exist_hasmap.test || point_exist_hasmap.create) {
                    if (input_shellOpenDatDir.buff.length() < 1) {
                        input_Console
                            .buff
                            .append(format!("\n[错误] 尚未输入dat目录文件夹").as_str());
                    } else {
                        let mut path_dir = parse_dat_path(input_shellOpenDatDir.buff.text());

                        has_inputPath = true;

                        match fs::metadata(path_dir.attach_dir.clone()) {
                            Ok(metadata) => {
                                if (!metadata.is_dir()) {
                                    input_Console.buff.append(
                                        format!("\n[错误] dat目录文件夹 不是文件夹").as_str(),
                                    );
                                } else if point_exist_hasmap.test {
                                    input_Console.buff.append(
                                        format!("\n[测试] 正在扫描当前文件夹存在的dat图片")
                                            .as_str(),
                                    );
                                    fn bool_to_str (b:bool) -> &'static str {
                                        if b {"是"} else { "否" }
                                    }

                                    input_Console.buff.append(
                                        format!("\n[测试] 处理范围: 仅本月:{}   缩略图:{}   原图:{}   全部:{}   ",bool_to_str(path_dir.is_the_month),bool_to_str(path_dir.is_thumbnail),bool_to_str(path_dir.is_source),bool_to_str(path_dir.is_all))
                                            .as_str(),
                                    );

                                    let pattern = format!(
                                        "{}",
                                        Path::new(&path_dir.attach_dir.clone())
                                            .join("**/*.dat")
                                            .display()
                                            .to_string()
                                    );
                                    let mut index = 0;

                                    input_Console.buff.append(
                                        format!("\n[测试] 开始扫描 “{}” 中的dat文件",pattern)
                                            .as_str(),
                                    );

                                    for entry in glob(&pattern).unwrap() {
                                        let path = entry.unwrap().display().to_string();
                                        let base = Path::new(&path).file_name().unwrap().to_str().unwrap();
                                        index = index + 1;
                                    }

                                    input_Console.buff.append(
                                        format!("\n[测试] 在 “{}” \n中发现了 [{}] 个dat文件",pattern,index)
                                            .as_str(),
                                    );
                                }
                            }
                            Err(err) => {
                                input_Console.buff.append(
                                    format!(
                                        "\n[错误] dat目录文件夹 无法被读取 因为-> {}",
                                        err.to_string()
                                    )
                                    .as_str(),
                                );
                            }
                        };
                    }

                    if (input_shellName.buff.length() < 1) {
                        
                        input_Console
                            .buff
                            .append(format!("\n[错误] 配置名称为空").as_str());
                    }else{
                        has_name = true;
                    }

                    if (input_shellOpenExportDir.buff.length() < 1) {
                        
                        

                        input_Console
                            .buff
                            .append(format!("\n[错误] 尚未输入存储转码文件的目录").as_str());
                    } else {
                        has_ouputPath = true;
                        match fs::metadata(input_shellOpenExportDir.buff.text()) {
                            Ok(metadata) => {
                                if (!metadata.is_dir()) {
                                    input_Console.buff.append(
                                        format!("\n[错误] 存储转码文件的目录 不是文件夹").as_str(),
                                    );
                                } }
                            Err(err) => {
                                // input_Console.buff.append(
                                //     format!(
                                //         "\n[提醒] 存储转码文件的目录 无法被读取 因为-> {}",
                                //         err.to_string()
                                //     )
                                //     .as_str(),
                                // );
                            }
                        };
                    }
                }
               
                // println!("{} , {} , {} , {}",has_name,has_inputPath,has_ouputPath,point_exist_hasmap.create);

                if (has_name&&has_inputPath&&has_ouputPath&&point_exist_hasmap.create){
                    if(wh_mod::convert::is_developer()||(libWxIkunPlus::hasWeChat()&&libWxIkunPlus::hasWeChatWin())){
                        let conn: Connection = Connection::open("../../../ikun_user_data.db").unwrap();
                    
                        handle_dat::initialize_table(&conn);
                        match  conn.execute(
                            "INSERT INTO export_dir_path (name,time,path,ouput) values (?1, ?2, ?3, ?4)",
                            [input_shellName.buff.text(),Local::now().format("%Y-%m-%d").to_string(),input_shellOpenDatDir.buff.text(),input_shellOpenExportDir.buff.text()],
                        ) {
                          Ok(_)=>{
                            input_Console.buff.append(
                                format!("\n[存储] 添加成功").as_str(),
                            );
                          } 
                          Err(err)=>{
                            if(str_eq_ostr(err.to_string(),"UNIQUE constraint failed: export_dir_path.path")){
                                input_Console.buff.append(
                                    format!("\n[错误] 添加失败 因为-> {}","当前被导出的路径已经存在").as_str(),
                                );
                            }else
    
                            {
                                input_Console.buff.append(
                                    format!("\n[错误] 添加失败 因为-> {}",err.to_string()).as_str(),
                                );
                            }
                          } 
                        }
    
                        conn.close();
                        global_var_util::update_export_dir_itme_list();

                     }else{
                        //  libWxIkunPlus::stop("错误".to_owned(),"当前未发现wx进程 拒绝提供选取方案".to_owned())
                        // dialog::alert_default(format!("\n[错误] {}","当前未发现wx进程或者未登录 拒绝提供添加").as_str());
                        libWxIkunPlus::stop("错误".to_owned(),"当前未发现wx进程或者未登录 拒绝提供添加".to_owned());

                        // input_Console.buff.append(
                        //     format!("\n[错误] 添加失败 因为-> {}","当前未发现wx进程或者未登录 拒绝提供添加").as_str(),
                        // );
                       
                     }

                }
           

               
               
                true
            }

            enums::Event::Move => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;
                point_exist_hasmap = getFormPointSpace(x, y);
                // -处理鼠标图标的逻辑

                if (point_exist_hasmap.existAllBtn) {
                    win.clone().set_cursor(Cursor::Hand);
                } else {
                    win.clone().set_cursor(Cursor::Default);
                }

                true
            }

            // enums::Event:
            enums::Event::Drag => {
                if (y < 74) {
                    win.clone()
                        .set_pos(app::event_x_root() - x, app::event_y_root() - y);
                }

                true
            }
            _ => false,
        }
    });

    
    loop {
        Sleep(200);
        if (util::getVarBooleanValue("K9V7OKIIMR1E1_theInitializationWindowIsDisplayed".to_owned()))
        {
            appMainWin.show();
            appRootView.show();
            break;
        }
    }
    appRootView.set_visible_focus();
    // appMainWin.hide();
    // let path = gui_select_user_base::mian_window();


    Ok(MianWindowItme {
        appRootView,
        appMainWin,
    })
}

#![allow(warnings, unused)]

pub(crate) mod convert;
pub(crate) mod watch_path;
pub(crate) mod config;
mod mobile_screenshot;

use crate::wh_mod::convert::detect_image_format;
use chrono::{DateTime, Local};
// use lazy_static::lazy_static;
use std::collections::HashSet;
use std::{fs, path};
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{collections::hash_map::HashMap, ptr};
use std::{
    sync::atomic::Ordering,
    sync::Arc,
    sync::MutexGuard,
    sync::{atomic::AtomicUsize, OnceLock},
};
use std::ffi::OsStr;
use crate::util;

// lazy_static! {
//     static ref WALK_ATTACH_FILE_LIST: Mutex<HashMap<String, Vec<PathBuf>>> = Mutex::new(HashMap::new());
// }

static mut WALK_ATTACH_FILE_LIST: Option<HashMap<String, Vec<PathBuf>>> = Option::None;
static WALK_ATTACH_FILE_LIST_BIND: AtomicUsize = AtomicUsize::new(0);
static VARIABLE_INITIALIZE_WALK_ATTACH_FILE_LIST: OnceLock<bool> = OnceLock::new();

fn initialize_watch_path_token() {
    if *(VARIABLE_INITIALIZE_WALK_ATTACH_FILE_LIST
        .get()
        .unwrap_or_else(|| &false))
    {
        return;
    }
    unsafe {
        if WALK_ATTACH_FILE_LIST.is_none() {
            WALK_ATTACH_FILE_LIST.replace(HashMap::new());
        }
    }
    VARIABLE_INITIALIZE_WALK_ATTACH_FILE_LIST.set(true);
}

// 清理历史枚举记录
pub fn gc_walk_attach_file_list() {
    initialize_watch_path_token();
    let mutex = Arc::new(Mutex::new(&WALK_ATTACH_FILE_LIST_BIND));
    mutex.lock();
    let the_value: usize = WALK_ATTACH_FILE_LIST_BIND.load(Ordering::SeqCst);
    unsafe {
        if let Some(lazy_value) = WALK_ATTACH_FILE_LIST.as_mut() {
            lazy_value.clear();
        }
    }

    WALK_ATTACH_FILE_LIST_BIND.store(the_value + 1, Ordering::SeqCst);
    drop(mutex);
    // let mut lazy_value = WALK_ATTACH_FILE_LIST.lock().unwrap();
    // lazy_value.clear();
    // drop(lazy_value);
}

// 获取历史枚举列表
pub fn get_walk_attach_file_history() -> HashMap<String, Vec<PathBuf>> {
    initialize_watch_path_token();
    let mut result = HashMap::new();

    let mutex = Arc::new(Mutex::new(&WALK_ATTACH_FILE_LIST_BIND));
    mutex.lock();
    let the_value: usize = WALK_ATTACH_FILE_LIST_BIND.load(Ordering::SeqCst);
    unsafe {
        println!("get_walk_attach_file -> WALK_ATTACH_FILE_LIST -> {:?}",&WALK_ATTACH_FILE_LIST.as_mut().unwrap().len());

        if let Some(lazy_value) = WALK_ATTACH_FILE_LIST.as_mut() {
            for (key, value) in lazy_value.iter() {
                result.insert(key.to_string(), value.clone());
            }
        }
        println!("get_walk_attach_file -> WALK_ATTACH_FILE_LIST -> {:?}",&WALK_ATTACH_FILE_LIST.as_mut().unwrap().len());

    }

    WALK_ATTACH_FILE_LIST_BIND.store(the_value + 1, Ordering::SeqCst);
    drop(mutex);

    result
}

#[derive(Debug)]
pub struct WxFsRetrievalStruct {
    pub path: String,
    pub path2arr: Vec<String>,
    pub root: String,
    pub user_data: String,
    pub wxid: String,
    pub attach_id: String,
    pub attach_path: String,
    pub month: String,
    pub is_thumbnail: bool,
    pub is_dat: bool,
}

pub fn split_path(input_path: String) -> Vec<String> {
    let mut path2arr: Vec<String> = Vec::new();
    let mut str: String = String::new();
    // 按照 \\ / 分割路径
    for char in input_path.chars() {
        if char.to_string().bytes().eq("\\".to_string().bytes())
            || char.to_string().bytes().eq("/".to_string().bytes())
        {
            path2arr.push(str.clone());
            str.clear();
            continue;
        }
        str.push(char);
    }

    // 最后一个文本
    if str.len() != 0 {
        path2arr.push(str.clone());
    }

    let mut path2arr_filter = Vec::new();

    for path2 in path2arr {
        if (!path2.is_empty()) {
            path2arr_filter.push(path2);
        }
    }

    path2arr_filter
}

/**
 * 格式化wx位置路径
 * path : D:\...\weixin\WeChat Files\wxid_...\FileStorage\MsgAttach\attach_id\any\2023-08\*.dat
 */
pub fn wx_parse_path(wx_path: String) -> WxFsRetrievalStruct {
    let mut wx_fs_retrieval_struct = WxFsRetrievalStruct {
        path: "".to_string(),
        path2arr: vec![],
        root: "".to_string(),
        user_data: "".to_string(),
        wxid: "".to_string(),
        attach_id: "".to_string(),
        month: "".to_string(),
        is_thumbnail: false,
        is_dat: false,
        attach_path: String::new(),
    };

    let mut path2arr: Vec<String> = Vec::new();

    path2arr = split_path(wx_path.clone());
    if let Some(last) = path2arr.last() {
        wx_fs_retrieval_struct.is_thumbnail = last.to_ascii_lowercase().contains("_t.dat");
    }
    // if wx_path
    //     .to_ascii_lowercase()
    //     .contains("filestorage\\msgattach")
    //     || wx_path
    //         .to_ascii_lowercase()
    //         .contains("filestorage/msgattach")
    // {
    //
    // }

    if path2arr.len() > 1 {
        // 基础路径
        wx_fs_retrieval_struct.path = path2arr.join("\\");
        wx_fs_retrieval_struct.path2arr = path2arr.to_vec();
        let path_parse = Path::new(wx_fs_retrieval_struct.path.as_str());

        // 判断后缀名为dat 不区分大小写
        wx_fs_retrieval_struct.is_dat = path_parse
            .extension()
            .unwrap_or_else(|| "".as_ref())
            .to_string_lossy()
            .to_ascii_lowercase()
            .bytes()
            .eq("dat".bytes());

        // 根目录
        wx_fs_retrieval_struct.root = String::from(path2arr.get(0).unwrap());

        //解析 wx_id msg (置信度为 最后出现的一次)

        let mut the_path_next_name_list: Vec<String> = Vec::new();
        let mut msg_attach: Vec<String> = Vec::new();
        let mut wx_id = String::new();
        let mut user_data: Vec<String> = Vec::new();
        let mut path2buff = "".bytes();
        let mut len: usize = 0;
        for for_path_data in path2arr.to_vec() {
            let to_ascii_lowercase = for_path_data.to_ascii_lowercase();
            path2buff = to_ascii_lowercase.bytes();
            the_path_next_name_list.push(for_path_data.clone());

            // magat
            if path2buff.clone().eq("msgattach".bytes()) {
                for the_path in the_path_next_name_list.clone() {
                    msg_attach.push(the_path);
                }
                if path2arr.len() > len + 1 {
                    wx_fs_retrieval_struct.attach_id = path2arr[len + 1].clone();
                }
            }

            // user data root
            if path2buff.clone().eq("wechat files".bytes()) {
                for the_path in the_path_next_name_list.clone() {
                    user_data.push(the_path);
                }
            }

            // wx id
            if for_path_data.to_ascii_lowercase().contains("wxid_") {
                wx_id = for_path_data.clone();
            }

            // 判断是否是日期 2023-05
            if (for_path_data.contains("202") && for_path_data.len() > 5) {
                if (for_path_data.find("-") == Some(4)) {
                    let mut is_not_month = false;
                    let eq_data = "0123456789-";

                    for char in for_path_data.chars() {
                        if is_not_month {
                            break;
                        };

                        let mut eq_for = false;
                        for eq_char in eq_data.chars() {
                            if char.to_string().bytes().eq(eq_char.to_string().bytes()) {
                                eq_for = true;
                            }
                        }

                        if !eq_for {
                            is_not_month = true;
                        }
                    }

                    if (!is_not_month) {
                        wx_fs_retrieval_struct.month = for_path_data;
                    }
                }
            }

            len = len + 1;
        }

        wx_fs_retrieval_struct.wxid = wx_id;
        wx_fs_retrieval_struct.attach_path = msg_attach.join("\\");
        wx_fs_retrieval_struct.user_data = user_data.join("\\");
    }

    // println!("{},{} , {} , path2arr-> {:?}",wx_path.clone(),wx_path.contains("wxid_") ,wx_fs_retrieval_struct.wxid.is_empty(),path2arr.clone() );

    if wx_path.contains("wxid_") && wx_fs_retrieval_struct.wxid.is_empty() {
        for path2 in path2arr {
            // println!("{} ->  {}",path2.clone(), path2.contains("wxid_"));

            if path2.contains("wxid_") {
                wx_fs_retrieval_struct.wxid = path2;
            }
        }
    }

    wx_fs_retrieval_struct
}

#[derive(Debug)]
pub struct AccountItem {
    pub id: String,
    pub update_time: SystemTime,
    pub time_str: String,
}

/**
 * 从文件名中获取 account_id （如果有）
 * path : D:\...\weixin\WeChat Files\
 */
pub fn wx_account_id(path: PathBuf) -> AccountItem {
    let modified_list = sort_modified_dir_meta(path.as_path());
    let mut id = String::new();
    let mut up_time = UNIX_EPOCH;

    for read_name in modified_list {
        let file_name = read_name
            .dir
            .file_name()
            .unwrap_or_else(|| "".as_ref())
            .to_string_lossy();

        let time = read_name.metadata.modified().unwrap();

        if let Ok(time) = read_name.metadata.modified() {
            if up_time < time {
                up_time = time;
            }
        }

        if file_name.contains("account_") {
            let account_id = file_name.replace("account_", "");
            id = account_id;
        }
    }

    let modified_datetime: DateTime<Local> = up_time.into();
    let formatted_time = modified_datetime.format("%Y-%m-%d %H:%M:%S").to_string();

    return AccountItem {
        id,
        update_time: up_time,
        time_str: formatted_time,
    };
}

#[derive(Debug)]
pub struct WxReadWxid {
    pub account_id: String,
    pub wxid: String,
    pub update_time: SystemTime,
    pub update_time_str: String,
    pub attach: PathBuf,
    pub user_root: PathBuf,
}

// 格式化路径到 String
pub fn path2string<P: AsRef<Path>, S: AsRef<OsStr>, E: AsRef<String>>(path: P) -> Option<String> {
    let s = path.as_ref().to_string_lossy();
    if s.is_empty() { None } else { Some(s.into_owned()) }
}

//
pub fn list_path<P: AsRef<Path>, S: AsRef<OsStr>, E: AsRef<String>>(_path: P) -> Vec<String> {
    let path = path2string::<P,S,E>(_path).unwrap();
    let path_str = path.replace("\\", "/");
    let mut result = Vec::new();

    if let Ok(entries) = fs::read_dir(path_str) {
        for entry in entries {
            if let Ok(entry) = entry {
                // 如果它是一个目录，就获取它的名字
                if entry.file_type().map(|s| s.is_dir()).unwrap_or(false) {
                    if let Some(file_name) = entry.file_name().to_str() {
                        let path = file_name.to_string().replace("/","\\");
                        result.push(path);
                    }
                }
            }
        }
    }

    return result;
}

pub fn wildcard_path<P: AsRef<Path>, S: AsRef<OsStr>, E: AsRef<String>>(_path: P) -> Vec<String>{
    let path = path2string::<P,S,E>(_path).unwrap();
    let mut result = Vec::new();

    result
}

// 格式化路径到 D:\usersData\...\WeChat Files
pub fn format_wx_root(wx_root: &str) -> Option<path::PathBuf> {
    let match_feature = vec![
        vec!["*","FileStorage","MsgAttach"],
        vec!["*","config","AccInfo.dat"],
        vec!["*","Msg"],
    ];
    let mut split_path = split_path(wx_root.to_string());
    let mut split_path_join_to_wfs = String::new();

    for split in split_path.iter() {
        if split.as_bytes().eq("WeChat Files".as_bytes()) {

            //  D:\usersData\...\WeChat Files\ 必须得是文件夹
            if path::Path::new(split_path_join_to_wfs.as_str()).is_dir(){
                split_path_join_to_wfs.push_str(split);

                // 所有特征
                // D:\usersData\...\WeChat Files\wxid_0x666\FileStorage\MsgAttach
                // D:\usersData\...\WeChat Files\wxid_0x666\config\AccInfo.dat
                // D:\usersData\...\WeChat Files\wxid_0x666\Msg
                let mut temp_all_feature_path:Vec<String> = Vec::new();

                // 循环并格式化出特征路径
                for match_feature in match_feature.iter() {
                    let list_path  = format!("{}\\{}",split_path_join_to_wfs,match_feature.join("\\"));

                    // temp_all_feature_path.push();
                }

            }else {
                return None;
            }
        }
        split_path_join_to_wfs.push_str(split);
    }

    return None;
}

pub fn wx_search_wxid_root (wx_root: &str) -> Vec<String> {
    let result = Vec::new();
    let match_feature = vec![
        vec!["*","FileStorage","MsgAttach"],
        vec!["*","config","AccInfo.dat"],
        vec!["*","Msg"],
    ];

    result
}

/**
 * 获取id
 * path : D:\...\weixin\WeChat Files\
 */
pub fn wx_read_root_wxid(wx_root: &Path) -> Vec<WxReadWxid> {
    let mut wx_read_item_list: Vec<WxReadWxid> = Vec::new();

    // 获取基础路径信息
    for dir in read_dir(wx_root) {
        let dir_path = dir.path();
        let base = dir.file_name().to_string_lossy().to_string();

        if base.contains("wxid_") {
            let item = WxReadWxid {
                account_id: "".to_string(),
                wxid: base,
                update_time: UNIX_EPOCH,
                update_time_str: "".to_string(),
                attach: dir_path.join("FileStorage\\MsgAttach"),
                user_root: dir_path,
            };
            wx_read_item_list.push(item);
        }
    }

    // 读取更新时间和wxid(如果有)
    for index in 0..wx_read_item_list.len() {
        let get_wx_account_id = wx_account_id(wx_read_item_list[index].user_root.clone());
        wx_read_item_list[index].account_id = get_wx_account_id.id.clone();
        wx_read_item_list[index].update_time = get_wx_account_id.update_time.clone();
        wx_read_item_list[index].update_time_str = get_wx_account_id.time_str.clone();
    }

    wx_read_item_list
}

/**
 * 排序出最近更新的文件夹
 * path : any
 */
pub fn sort_modified_dir(dir_path: &Path) -> Vec<std::path::PathBuf> {
    let mut res: Vec<std::path::PathBuf> = Vec::new();
    let data = sort_modified_dir_meta(dir_path);
    for index in 0..data.len() {
        res.push(data[index].dir.clone());
    }
    res
}

#[derive(Debug)]
pub struct TmpMetadata {
    pub metadata: fs::Metadata,
    pub dir: std::path::PathBuf,
}

/**
 * 排序出最近更新的文件夹
 * path : any
 */
pub fn sort_modified_dir_meta(dir_path: &Path) -> Vec<TmpMetadata> {
    let mut modified_dir_list = Vec::new();
    let mut temp_metadata_list: Vec<TmpMetadata> = Vec::new();

    // 获取root 下的所有文件夹
    match fs::read_dir(dir_path) {
        Ok(rade_dir) => {
            for entry in rade_dir {
                let entry_value = entry.unwrap();

                // 获取文件属性表
                match entry_value.metadata() {
                    Ok(metadata) => {
                        let path = entry_value.path();
                        if path.is_dir() {
                            let obj = TmpMetadata {
                                metadata,
                                dir: entry_value.path(),
                            };
                            temp_metadata_list.push(obj);
                        }
                    }
                    Err(_) => {}
                }
            }
        }
        Err(_) => {}
    }

    temp_metadata_list.sort_by(|a, b| {
        a.metadata
            .modified()
            .unwrap()
            .cmp(&b.metadata.modified().unwrap())
    });

    for temp_metadata in temp_metadata_list {
        modified_dir_list.push(temp_metadata);
    }

    modified_dir_list
}

#[derive(Debug)]
pub struct TmpMetaDataf {
    pub metadata: fs::Metadata,
    pub file: std::path::PathBuf,
}
pub fn sort_modified_file_meta(dir_path: &Path) -> Vec<TmpMetaDataf> {
    let mut modified_dir_list = Vec::new();
    let mut temp_metadata_list: Vec<TmpMetaDataf> = Vec::new();

    // 获取root 下的所有文件夹
    match fs::read_dir(dir_path) {
        Ok(rade_dir) => {
            for entry in rade_dir {
                let entry_value = entry.unwrap();

                // 获取文件属性表
                match entry_value.metadata() {
                    Ok(metadata) => {
                        let path = entry_value.path();
                        if path.is_file() {
                            let obj = TmpMetaDataf {
                                metadata,
                                file: entry_value.path(),
                            };
                            temp_metadata_list.push(obj);
                        }
                    }
                    Err(_) => {}
                }
            }
        }
        Err(_) => {}
    }

    temp_metadata_list.sort_by(|a, b| {
        a.metadata
            .modified()
            .unwrap()
            .cmp(&b.metadata.modified().unwrap())
    });

    for temp_metadata in temp_metadata_list {
        modified_dir_list.push(temp_metadata);
    }

    modified_dir_list
}

/**
 * 读取文件夹下的路径
 * path : any
 */
pub fn read_dir(dir_path: &Path) -> Vec<fs::DirEntry> {
    let mut read_dir_list = Vec::new();

    match fs::read_dir(dir_path) {
        Ok(rade_dir) => {
            for entry in rade_dir {
                match entry {
                    Ok(entry) => read_dir_list.push(entry),
                    Err(_) => {}
                }
            }
        }
        Err(_) => {}
    }
    read_dir_list
}

#[derive(Debug)]
pub struct AttachThumbnail {
    pub is_thumbnail: bool,
    pub is_source: bool,
    // exists_source:bool,
    // exists_thumbnail: bool,
    pub thumbnail_path: String,
    // source_path: String,
    pub thumbnail: Vec<u8>,
    pub thumbnail_type: String,
    pub imag_id: String,
    pub attach_id: String,
}

impl Clone for AttachThumbnail {
    fn clone(&self) -> Self {
        AttachThumbnail {
            is_thumbnail: self.is_thumbnail.clone(),
            is_source: self.is_source.clone(),
            thumbnail_path: self.thumbnail_path.clone(),
            thumbnail: self.thumbnail.clone(),
            thumbnail_type: self.thumbnail_type.clone(),
            imag_id: self.imag_id.clone(),
            attach_id: self.attach_id.clone(),
        }
    }
}

/**
 * 获取指定 id 下的按照修改时间排序的 缩略图
 * path : D:\...\weixin\WeChat Files\wxid_...\FileStorage\MsgAttach\AttachID\
 */
pub fn read_attach_buff_thumbnail_data(dir_path: &PathBuf, thumbnail_len: usize,) -> Vec<AttachThumbnail> {
    let mut attach_thumbnail_list: Vec<AttachThumbnail> = Vec::new();
    let mut new_path = Path::new(dir_path).join("Thumb");
    let mut new_path_images = Path::new(dir_path).join("Image");

    if new_path.is_dir() {
        let modified_datetime: DateTime<Local> = SystemTime::now().into();
        let formatted_time: String = modified_datetime.format("%Y-%m").to_string();
        let mut new_path = new_path.join(formatted_time.clone());
        let mut new_path_images = new_path_images.join(formatted_time.clone());

        if new_path.is_dir() {
            let mut file_list = sort_modified_file_meta(&new_path);
            let mut path_list = Vec::new();
            for (index, file) in file_list.iter().rev().enumerate() {
                if index > thumbnail_len {
                    break;
                }
                path_list.push(file.file.clone());
            }
            drop(file_list);

            for path in path_list {
                let path_str = path.to_string_lossy().to_string();
                let name_str = path
                    .file_name()
                    .unwrap_or_else(|| (&"").as_ref())
                    .to_string_lossy()
                    .to_string();
                let path_str_imag =
                    Path::new(new_path_images.as_path()).join(name_str.replace("_t.dat", ".dat"));

                let wx_parse = wx_parse_path(path_str.clone());

                let mut attach_thumbnail = AttachThumbnail {
                    // 1
                    thumbnail_path: path_str.clone(),
                    // 1
                    is_thumbnail: true,
                    // 1
                    is_source: false,
                    // 1
                    // exists_source: path_str_imag.is_file(),
                    //
                    // source_path:path_str_imag.to_string_lossy().to_string(),
                    // 1
                    thumbnail: convert::convert_dat_images_buff(path.clone())
                        .unwrap_or_else(|_| Vec::new()),
                    // 1
                    thumbnail_type: String::new(),
                    // 1
                    attach_id: wx_parse.attach_id.clone(),
                    // 1
                    imag_id: name_str.replace("_t.dat", "").replace(".dat", ""),
                    // 1
                    // exists_thumbnail: true,
                };

                // 获取缩略图的格式
                match detect_image_format(&attach_thumbnail.thumbnail.clone()) {
                    Some(format) => attach_thumbnail.thumbnail_type = format.to_string(),
                    None => println!("Unknown image format."),
                }

                attach_thumbnail_list.push(attach_thumbnail);
            }
        }
    }

    attach_thumbnail_list
}

/**
 * 读取attach 下的最后更新N个 thumbnail (与read_attach_buff_thumbnail_data 不一样的是此函数一次性获取的是多个人的)
 * path : D:\...\weixin\WeChat Files\wxid_...\FileStorage\MsgAttach
 */
pub fn read_attach_buff_thumbnail_list(dir_path: &Path, len: usize, extract_len: usize, ) -> Vec<AttachThumbnail> {
    let mut attach_thumbnail_list: Vec<AttachThumbnail> = Vec::new();

    let sort_modified_list = sort_modified_dir_meta(dir_path);

    let n = if len >= sort_modified_list.len() {
        sort_modified_list.len()
    } else {
        len
    };

    for (i, sort_modified) in sort_modified_list.iter().rev().enumerate() {
        if i > n {
            break;
        }
        let dir_path = sort_modified.dir.clone();
        let read_attach_list = read_attach_buff_thumbnail_data(&dir_path, extract_len);

        for read_attach in read_attach_list {
            attach_thumbnail_list.push(read_attach);
        }
    }

    attach_thumbnail_list
}

/**
 * v2 版本将排序所有文件 而不是文件夹
 * 读取attach 下的最后更新N个 thumbnail (与read_attach_buff_thumbnail_data 不一样的是此函数一次性获取的是多个人的)
 * path : D:\...\weixin\WeChat Files\wxid_...\FileStorage\MsgAttach
 */
pub fn read_attach_buff_thumbnail_list_v2(dir_path: &Path, len: usize, extract_len: usize, ) -> Vec<AttachThumbnail> {
    let mut attach_thumbnail_list: Vec<AttachThumbnail> = Vec::new();

    let modified_datetime: DateTime<Local> = SystemTime::now().into();
    let formatted_time: String = modified_datetime.format("%Y-%m").to_string();

    let mut all_tmp_meta_data_file = Vec::new();

    if let Ok(dir_dir) = fs::read_dir(dir_path) {

        // D:\...\weixin\WeChat Files\wxid_...\FileStorage\MsgAttach\{attid}

        for dir_entry in dir_dir {
            if let Ok(entry) = dir_entry {

                // D:\...\weixin\WeChat Files\wxid_...\FileStorage\MsgAttach\{attid}\Thumb\2023-10

                let for_path = entry.path().join("Thumb").join(formatted_time.as_str());
                if for_path.is_dir() {
                    let mut c = sort_modified_file_meta(for_path.as_path());
                    if let Some(c) = c.pop() {
                        all_tmp_meta_data_file.push(c);
                    }
                }

            }
        }
    }

    // 排序创建时间
    all_tmp_meta_data_file.sort_by(|a, b| {
        let mut a_created = UNIX_EPOCH;
        let mut b_created = UNIX_EPOCH;

        if let Result::Ok(create) = a.metadata.created() {
            a_created = create;
        }

        if let Result::Ok(create) = b.metadata.created() {
            b_created = create;
        }

        a_created.cmp(&b_created)
    });

    all_tmp_meta_data_file.reverse();

    println!("all_tmp_meta_data_file-->{}",all_tmp_meta_data_file.len());
    // 取出五个任务
    for index in 0..len {
        if let Some(meta) = all_tmp_meta_data_file.get(index) {
            if let Some(str) = meta.file.to_str() {
                let dir_path = wx_parse_path(str.to_string());
                // println!("dir_path-> {:?}",&dir_path);

                let read_attach_list = read_attach_buff_thumbnail_data(&PathBuf::from(dir_path.attach_path.as_str()).join(dir_path.attach_id.as_str()), extract_len);

                println!("read_attach_list ->[{}] {}",dir_path.attach_path,read_attach_list.len(),);

                for read_attach in read_attach_list {
                    attach_thumbnail_list.push(read_attach);
                }
            }
        }

    }

    attach_thumbnail_list
}


fn visit_dirs(dir: &Path) -> Vec<PathBuf> {
    let mut path_list: Vec<PathBuf> = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    for visit_dir in visit_dirs(&path) {
                        path_list.push(visit_dir);
                    }
                } else {
                    path_list.push(path);
                }
            }
        }
    }
    path_list
}

/**
 * 匹配出 imag id
 */
pub fn walk_file(dir_path: &Path,run_tx: mpsc::Sender<(String, Vec<PathBuf>)>,img_id: String,) -> HashMap<String, Vec<PathBuf>> {
    // let mut file_list: Vec<PathBuf> = Vec::new();
    let mut wk_list: HashMap<String, Vec<PathBuf>> = HashMap::new();

    let (tx, rx) = mpsc::channel();

    let dir_root = read_dir(dir_path);
    let mut max_len = dir_root.len();

    for dir in dir_root {
        let tx = tx.clone();
        let paths = dir.path();
        let key = paths.to_string_lossy().to_string();

        thread::spawn(move || {
            let mut path_list: Vec<PathBuf> = visit_dirs(paths.as_path());
            tx.send((key, path_list));
            drop(tx);
            return;
        });
    }

    drop(tx);
    let mut index = 0;
    while index < 5 {
        let (key, data_vec) = rx.recv().unwrap_or_else(|_| (String::new(), Vec::new()));
        if key.is_empty() {
            index = index + 1;
            continue;
        }

        // 如果有img id
        if !img_id.is_empty() {
            for data in data_vec.to_vec() {
                if data
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .contains(img_id.clone().as_str())
                {
                    let key = format!("img_id::{}", img_id);
                    run_tx.send((key.clone(), vec![data.clone()]));
                    wk_list.insert(key, vec![data.clone()]);
                }
            }
        }
        run_tx.send((key.clone(), data_vec.clone()));
        wk_list.insert(key, data_vec);
    }

    // // set_walk_attach_file(wk_list.clone());
    // let mut lazy_value = WALK_ATTACH_FILE_LIST.lock().unwrap();

    // for (key, value) in wk_list.iter() {
    //     match lazy_value.insert(key.to_string(), value.clone()) {
    //         std::option::Option::Some(_) => {}
    //         // std::option::Option::None()=>{

    //         // }
    //         _ => {}
    //     };
    // }

    // drop(lazy_value);
    initialize_watch_path_token();
    let mutex = Arc::new(Mutex::new(&WALK_ATTACH_FILE_LIST_BIND));
    mutex.lock();
    let the_value: usize = WALK_ATTACH_FILE_LIST_BIND.load(Ordering::SeqCst);
    unsafe {
        println!("walk_file -> WALK_ATTACH_FILE_LIST -> {:?}",&WALK_ATTACH_FILE_LIST.as_mut().unwrap().len());
        println!("walk_file -> wk_list -> {:?}",wk_list.len());

        if let Some(lazy_value) = &WALK_ATTACH_FILE_LIST {
            for (key, value) in wk_list.iter() {
             
             if let Some(item) = WALK_ATTACH_FILE_LIST.as_mut() {
                    let key = key.to_string();
                    let push_value: Vec<PathBuf> = value.clone();

                    item.insert(key, push_value);
                };
            }
        }
        println!("walk_file -> WALK_ATTACH_FILE_LIST -> {:?}",&WALK_ATTACH_FILE_LIST.as_mut().unwrap().len());

    }

    WALK_ATTACH_FILE_LIST_BIND.store(the_value + 1, Ordering::SeqCst);
    drop(mutex);

    return wk_list;
}

#[derive(Debug)]
pub struct Dat2VarParseMeta {
    pub attach_id: String,
    pub attach_dir: String,
    // pub format_dir : String,
    pub is_the_month: bool,
    pub is_thumbnail: bool,
    pub is_source: bool,
    pub is_all: bool,
    pub is_sync:bool,
    pub is_video:bool,
    pub rename_rule: String,
    format_path_list: Vec<std::path::PathBuf>,
}

impl Dat2VarParseMeta {
    // 获取此可变体格式的格式化后的路径列表
    pub fn format(&mut self) -> Vec<std::path::PathBuf> {
        if (self.format_path_list.len() != 0) {
            return self.format_path_list.clone();
        }
        let mut path_list: Vec<std::path::PathBuf> = Vec::new();
        let attach_dir = Path::new(self.attach_dir.as_str());
        let the_month = chrono::Local::now().format("%Y-%m").to_string();

        if (attach_dir.exists()) {
            // 限定本月
            if (self.is_the_month) {
                if self.is_thumbnail {
                    let mut thumbnail_path = Path::new(attach_dir.clone())
                        .join("Thumb")
                        .join(the_month.clone());
                    path_list.push(thumbnail_path.clone());
                }
                if self.is_source {
                    let mut thumbnail_path = Path::new(attach_dir.clone())
                        .join("Image")
                        .join(the_month.clone());
                    path_list.push(thumbnail_path.clone());
                }
            }
            // 有限定缩略图和原图要求
            else if self.is_thumbnail || self.is_source {
                if self.is_thumbnail {
                    let mut thumbnail_path = Path::new(attach_dir.clone()).join("Thumb");
                    path_list.push(thumbnail_path.clone());
                }
                if self.is_source {
                    let mut thumbnail_path = Path::new(attach_dir.clone()).join("Image");
                    path_list.push(thumbnail_path.clone());
                }
            }
            // 没有声明 则全部
            else {
                path_list.push(attach_dir.to_path_buf());
            }
        }

        for path in path_list.clone() {
            self.format_path_list.push(path);
        }

        return path_list;
    }

    // 判断此路径是否属于此 可变体的路径
    pub fn exists(&mut self, eq_path: String) -> bool {
        let format_list = self.format();
        let eq_path = split_path(eq_path);

        let mut is_exists = false;
        for format in format_list {
            let split_format_path = split_path(format.to_string_lossy().to_string());
            let mut index: usize = 0;
            let mut match_path: bool = true;
            for eq_path in eq_path.clone() {
                if (split_format_path.len() <= index) {
                    break;
                }
                if !split_format_path[index].as_bytes().eq(eq_path.as_bytes()) {
                    match_path = false;
                }
                index = index + 1;
            }
            if match_path {
                is_exists = true;
            }
            if is_exists {
                break;
            }
        }
        is_exists
    }

    // 获取可变命名路径的实际需要生成的路径
    pub fn get_rename_output(&mut self) -> String{
        let mut result = self.rename_rule.clone();
        let mut the_data = HashMap::new();
        let time_info =util::get_time_info();

        the_data.insert("<现在>",time_info.time);
        the_data.insert("<年>",time_info.years);
        the_data.insert("<月>",time_info.month);
        the_data.insert("<日>",time_info.day);
        the_data.insert("<时>",time_info.hour);
        the_data.insert("<分>",time_info.minutes);
        // the_data.insert("<别名>","");
        // the_data.insert("<任务名>","");
        let mut mk_time_years = time_info.time_years;
        the_data.insert("<创建月>",mk_time_years);

        // let mut _type = "图片";
        // if self.is_video {
        //     _type = "视频";
        // }
        // else if self.is_thumbnail {
        //     _type = "缩略图";
        // }
        // // else if self.is_source {
        // //     _type = "手机截图";
        // // }
        // else {
        //     _type = "图片";
        // }

        // the_data.insert("<类型>",_type.to_string());
        the_data.insert("<哈希>",self.attach_id.clone());


        for (key,data) in the_data {
            result = result.replace(key,data.as_str());
        }

        result
    }
    pub fn writeFile(ex_dir:&str,){

    }
}

impl Clone for Dat2VarParseMeta {
    fn clone(&self) -> Self {
        Dat2VarParseMeta {
            attach_id: self.attach_id.clone(),
            attach_dir: self.attach_dir.clone(),
            is_the_month: self.is_the_month.clone(),
            is_thumbnail: self.is_thumbnail.clone(),
            is_source: self.is_source.clone(),
            format_path_list: self.format_path_list.clone(),
            is_all: self.is_all.clone(),
            is_video:self.is_video.clone(),
            is_sync: self.is_sync.clone(),
            rename_rule:self.rename_rule.clone(),
        }
    }
}

/**
 * 解析可变化路径
 */
pub fn parse_dat2var_path<T: util::OverloadedAnyStr >(input: T) -> Dat2VarParseMeta {
    // D:\usersData\weixin\WeChat Files/wxid_y.....1/FileStorage/MsgAttach/99e.......d..f,the_month,source,thumbnail
    let mut dat_parse_meta = Dat2VarParseMeta {
        attach_id: "".to_string(),
        attach_dir: "".to_string(),
        rename_rule:"".to_string(),
        // format_dir: "".to_string(),
        is_the_month: false,
        is_thumbnail: false,
        is_sync:false,
        is_video:false,
        is_source: false,
        is_all: false,
        format_path_list: Vec::new(),
    };
    let mut path_dir = input.to_string_default();
    let mut path_list = Vec::new();
    let binding = split_path(path_dir).join("\\");
    let lines: Vec<&str> = binding.split('*').collect();

    if (lines.is_empty()) {
        return dat_parse_meta;
    }

    for line in lines {
        let line_f = format!("{}", line);
        if (line_f.is_empty()) {
            continue;
        }
        if line_f.as_bytes().eq("the_month".as_bytes()) {
            dat_parse_meta.is_the_month = true;
        }
        if line_f.as_bytes().eq("source".as_bytes()) {
            dat_parse_meta.is_source = true;
        }
        if line_f.as_bytes().eq("thumbnail".as_bytes()) {
            dat_parse_meta.is_thumbnail = true;
        }
        
        if line_f.as_bytes().eq("video".as_bytes()) {
            dat_parse_meta.is_video = true;
        }

        if line_f.as_bytes().eq("Sync".as_bytes()) {
            dat_parse_meta.is_sync = true;
        }
        
        if line_f.contains("rename_rule=") {
            dat_parse_meta.rename_rule = line_f.to_string().replace("rename_rule=","");
        }

        dat_parse_meta.is_all = !dat_parse_meta.is_thumbnail.clone()
            && dat_parse_meta.is_source.clone()
            && dat_parse_meta.is_the_month.clone()&& dat_parse_meta.is_video.clone();

        path_list.push(line_f);
    }

    if let Some(attach_dir) = path_list.get(0) {
        dat_parse_meta.attach_dir = format!("{}",attach_dir);
    }

    if let Some(attach_id) = split_path(dat_parse_meta.attach_dir.clone()).pop() {
        dat_parse_meta.attach_id=attach_id;
    }
    
    return dat_parse_meta;
}


pub fn resolve_path(path: String) -> String {
    return split_path(path).join("\\");
}




// 自动按照设置获取显示的文本的消敏
pub fn get_show_mask_text <T: util::OverloadedAnyStr >(input: T) -> String {
    if config::is_show_mask() {
        format!("{}",util::get_mask_text(input.to_string_default().as_str()))
    }else {
        input.to_string_default()
    }
}
#![allow(warnings, unused)]

use crate::{global_var, gui_util, handle_dat, libWxIkunPlus, util::{str_eq_str, Sleep}, wh_mod};
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
use rusqlite::{Connection};
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

use crate::gui_util::img::ImgPreview;
use crate::libWxIkunPlus::closeWindow;
use crate::watching::insert_watch_path_token;
use fltk::draw::{height, width};
use fltk::image::PngImage;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::OnceLock;
use winapi::um::winnt::LPWSTR;
use winapi::um::winuser::{CloseWindow, SetActiveWindow};

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

    if( global_var::get_bool_default("gui::open::gui_detect_config")){
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
    win.set_id("gui::DoubleWindow::gui_detect_config::main");
    global_var::set_bool("gui::open::gui_detect_config",true);

    let mut main_window_back = ImgPreview::new(0, 0, 450, 453, "gui::ImgPreview::main_window::back");
    main_window_back.from_data(include_bytes!("../../assets/select_user_base/detect/detect.png").to_vec(), 0,0,win.width(),win.height());

    let mut next_btn = gui_util::hotspot::create_hotspot(203, 570, 233, 72);
    let mut gui_text_form01_state =gui_util::text::TextControl::new(320,53,0,0,11,"已经完成",[96, 139, 153]);
    let mut gui_text_form02_state =gui_util::text::TextControl::new(320,168,0,0,11,"已经完成",[96, 139, 153]);
    let mut gui_text_form03_state =gui_util::text::TextControl::new(320, 285,0,0, 11, "已经完成",[96, 139, 153]);

    let mut gui_imag_from01_state = ImgPreview::new(43,58,50,50,"gui_imag_from01_state");
    let mut gui_imag_from02_state = ImgPreview::new(43,175,50,50,"gui_imag_from02_state");
    let mut gui_imag_from03_state = ImgPreview::new(43,296,50,50,"gui_imag_from03_state");
    gui_imag_from01_state.from_data(include_bytes!("../../assets/select_user_base/detect/success.png").to_vec(),0,0,50,50);
    gui_imag_from02_state.from_data(include_bytes!("../../assets/select_user_base/detect/success.png").to_vec(),0,0,50,50);
    gui_imag_from03_state.from_data(include_bytes!("../../assets/select_user_base/detect/success.png").to_vec(),0,0,50,50);

    let mut gui_text_form01_title =gui_util::text::TextControl::new(111-60-10,55,0,0,12,"选择WX存储位置",[207, 207, 207]);
    let mut gui_text_form02_title =gui_util::text::TextControl::new(111-78-10,175,0,0,12,"选择被保存的对象",[207, 207, 207]);
    let mut gui_text_form03_title =gui_util::text::TextControl::new(111-65-10, 288,0,0, 12, "选择存储的选项",[207, 207, 207]);

    let mut gui_text_form01_cont =gui_util::text::TextControl::new(100,80,300,0,11,"此路径在您的 WX 中的  设置  /  文件管理  / 文件管理",[78, 78, 78]);
    let mut gui_text_form01_cont_2 =gui_util::text::TextControl::new(-8,96,300,0,11,"找到此路径",[78, 78, 78]);

    let mut gui_text_form02_cont =gui_util::text::TextControl::new(100-3,80+120,300,0,11,"您需要选择需要同步的对象， 在选择最近对象      ",[78, 78, 78]);
    let mut gui_text_form02_cont_2 =gui_util::text::TextControl::new(100,96+120,300,0,11,"如果不存在的话 您可以向找的人 随意发送一张图片",[78, 78, 78]);

    let mut gui_text_form03_cont =gui_util::text::TextControl::new(110,80+120+110,300,0,11,   "1.保存缩略  就是很小的图片 显示在聊天的 所有图片都有",[78, 78, 78]);
    let mut gui_text_form03_cont_2 =gui_util::text::TextControl::new(110,96+120+110,300,0,11,   "2.保存原图  当您打开了图片 就会下载大图 此图片为原图",[78, 78, 78]);
    let mut gui_text_form03_cont_3 =gui_util::text::TextControl::new(110,96+16+120+110,300,0,11,"3.保存本月  当此项打开 只会保存本月开始 之前将被忽略",[78, 78, 78]);

    let mut gui_text_btn_name =gui_util::text::TextControl::new(173, 405+3+1, 103,22,13, "朕知道了",[121, 121, 121]);

    let mut next_btn = gui_util::hotspot::create_hotspot(140, 395, 162, 51);

    // global_var::set_bool("user::config::check_button_the_month",false);
    // global_var::set_bool("user::config::check_button_source",false);
    // global_var::set_bool("user::config::check_button_thumbnail",false);
    // global_var::set_str("user::config::input_select_dir","".to_string());
    // global_var::set_i32("user::config::select_user_thumbnail_obj",-1);
    macro_rules! update_gui_state {
                    () => {
        
        if(
            // !global_var::get_bool_default("user::config::check_button_sync")&&
            !global_var::get_bool_default("user::config::check_button_video")&&
            !global_var::get_bool_default("user::config::check_button_thumbnail")&&
            !global_var::get_bool_default("user::config::check_button_source")&&
            !global_var::get_bool_default("user::config::check_button_the_month")
        ){
        gui_text_form03_state.set_label("尚未选择".to_string());
        gui_text_form03_state.set_color(215, 97, 97);
        gui_imag_from03_state.from_data(include_bytes!("../../assets/select_user_base/detect/fail.png").to_vec(),0,0,50,50);
    }else{
        gui_text_form03_state.set_label("已经完成".to_string());
        gui_text_form03_state.set_color(96, 139, 153);
        gui_imag_from03_state.from_data(include_bytes!("../../assets/select_user_base/detect/success.png").to_vec(),0,0,50,50);
    }

    if(global_var::get_i32_or("user::config::select_user_thumbnail_obj",-1)==-1){
        gui_text_form02_state.set_label("尚未选择".to_string());
        gui_text_form02_state.set_color(215, 97, 97);
        gui_imag_from02_state.from_data(include_bytes!("../../assets/select_user_base/detect/fail.png").to_vec(),0,0,50,50);

    }else{
        gui_text_form02_state.set_label("已经完成".to_string());
        gui_text_form02_state.set_color(96, 139, 153);
        gui_imag_from02_state.from_data(include_bytes!("../../assets/select_user_base/detect/success.png").to_vec(),0,0,50,50);
    }

    if(global_var::get_string_default("user::config::user_select_path").is_empty()){
        gui_text_form01_state.set_label("尚未选择".to_string());
        gui_text_form01_state.set_color(215, 97, 97);
        gui_imag_from01_state.from_data(include_bytes!("../../assets/select_user_base/detect/fail.png").to_vec(),0,0,50,50);
    }else{
        gui_text_form01_state.set_label("已经完成".to_string());
        gui_text_form01_state.set_color(96, 139, 153);
        gui_imag_from01_state.from_data(include_bytes!("../../assets/select_user_base/detect/success.png").to_vec(),0,0,50,50);
    }
                    };
                }

    thread::spawn(move||{
        let mut is_open_win  = global_var::get_bool_default("gui::open::gui_detect_config");
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
}#![allow(unused_variables)]
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
#![allow(warnings, unused)]

use fltk::enums::{Color, Cursor, Event, FrameType, Key};
use fltk::{prelude::*, *};
use fltk::app::event_key;
use fltk_theme::{color_themes, ColorTheme, SchemeType, ThemeType, WidgetScheme, WidgetTheme};

pub struct TextPreview {
    pub buf:text::TextBuffer,
    pub preview:text::TextDisplay,
    x:i32,
    y:i32,
    height:i32,
    width:i32,
    size:i32
}
impl Clone for TextPreview {
    fn clone(&self) -> Self {
        TextPreview{
            buf: self.buf.clone(),
            preview:self.preview.clone(),
            x: self.x.clone(),
            y: self.y.clone(),
            width: self.width.clone(),
            height: self.height.clone(),
            size:self.size.clone()
        }
    }
}
impl TextPreview {
    // 创建
    pub fn new(x:i32, y:i32, width: i32, height: i32, size:i32, input:&str, rgb: [u8; 3]) -> Self {
        let mut buf = text::TextBuffer::default();
        buf.set_text(input);

        let mut txt = text::TextDisplay::default()
            .with_size(width, height)
            .center_of_parent();
        txt.set_buffer(buf.clone());
        txt.set_pos(x,y);
        txt.set_frame(fltk::enums::FrameType::NoBox);
        txt.set_scrollbar_size(-1);
        txt.set_text_size(size);
        txt.set_text_color(Color::from_rgb(rgb[0],rgb[1],rgb[2]));
        txt.scroll(0, 0);
        txt.set_label_type(fltk::enums::LabelType::None);

        // txt.handle(move |txt, event| match event {
        //     Event::Move=>{
        //
        //         true
        //     }
        //     Event::Leave=>{
        //
        //         true
        //     }
        //     _ => false,
        // });

        TextPreview{
            buf:buf,
            preview:txt,
            x,
            y,
            height,
            width,
            size
        }
    }

    // 获取文本内容
    pub fn get_label(&self)->String {
        return self.preview.label();
    }

    // 设置文本内容
    pub fn set_label(&mut self,input:String){
        self.preview.set_label(input.as_str());
        self.preview.redraw_label();
        self.preview.redraw();
    }

    // 设置颜色
    pub fn set_color(&mut self, r: u8, g: u8, b: u8) -> &mut TextPreview {
        self.preview.set_text_color(Color::from_rgb(r,g,b));
        self
    }

    // 判断鼠标是否在当前元素
    pub fn existPoint(&self, x: i32, y: i32) -> bool {
        return x > self.preview.x()
            && x < self.preview.x() + self.preview.width()
            && y > self.preview.y()
            && y < self.preview.y() + self.preview.height();
    }
    pub fn set_back_color(&mut self, r: u8, g: u8, b: u8) -> &mut TextPreview {
        self.preview.set_color(Color::from_rgb(r,g,b));
        self
    }

}




pub struct TextControl {
    pub text: frame::Frame,
}

impl TextControl {
    // 创建
    pub fn new(x:i32, y:i32, width: i32, height: i32, size:i32, input: &str, rgb: [u8; 3]) -> Self {
        let mut text = frame::Frame::new(x, y, (if width==0 {input.len()as i32 * size}else {width}), (if height==0 {size+2}else {height}), "");
        text.set_label(input);
        text.set_label_size(size);
        text.set_label_color(Color::from_rgb(rgb[0],rgb[1],rgb[2]));

        Self { text  }
    }
    // 获取文本内容
    pub fn get_label(&self)->String {
        return self.text.label();
    }
    // 设置文本内容
    pub fn set_label(&mut self,input:String){
        self.text.set_label(input.as_str());
        self.text.redraw_label();
        self.text.redraw();
    }
    // 设置颜色
    pub fn set_color(&mut self, r: u8, g: u8, b: u8)  {
        self.text.set_label_color(Color::from_rgb(r,g,b));

    }
    // 判断鼠标是否在当前元素
    pub fn existPoint(&self, x: i32, y: i32) -> bool {
        return x > self.text.x()
            && x < self.text.x() + self.text.width()
            && y > self.text.y()
            && y < self.text.y() + self.text.height();
    }

}
impl Clone for TextControl {
    fn clone(&self) -> Self {
        TextControl {
            text:self.text.clone()
        }
    }
}

use std::ffi::{c_int, c_uint, c_ulong, OsStr};
use std::iter::once;
use std::mem::transmute;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;
type DWORD = c_ulong;

pub fn encode_lpcstr(s: &str) -> Vec<i8> {
    let mut arr: Vec<i8> = s.bytes().map(|x| x as i8).collect();
    arr.push(0);
    arr
}

pub fn encode_wide_with_null(s: impl AsRef<str>) -> Vec<u16> {
    let wide: Vec<u16> = OsStr::new(s.as_ref())
        .encode_wide()
        .chain(once(0))
        .collect();
    wide
}

extern "system" {
    fn WideCharToMultiByte(
        page: c_uint,
        flags: c_ulong,
        wide_str: *const u16,
        wide_str_len: c_int,
        multi_str: *mut i8,
        multi_str_len: c_int,
        default_char: *const i8,
        used_default_char: *mut i32,
    ) -> c_int;
    fn MultiByteToWideChar(
        CodePage: c_uint,
        dwFlags: DWORD,
        lpMultiByteStr: *const u8,
        cbMultiByte: c_int,
        lpWideCharStr: *mut u16,
        cchWideChar: c_int,
    ) -> c_int;
}

/// Convert a rust string to a winapi-usable 0-terminated unicode u16 Vec
pub fn winapi_str<T: AsRef<OsStr>>(input: T) -> Vec<u16> {
    let mut buf = Vec::with_capacity(input.as_ref().len());
    buf.extend(input.as_ref().encode_wide());
    buf.push(0);
    buf
}

const CP_ACP: c_uint = 0;
const CP_OEMCP: c_uint = 1; // default to OEM  code page
const CP_MACCP: c_uint = 2; // default to MAC  code page
const CP_THREAD_ACP: c_uint = 3; // current thread's ANSI code page
const CP_SYMBOL: c_uint = 42; // SYMBOL translations

const CP_UTF7: c_uint = 65000; // UTF-7 translation
const CP_UTF8: c_uint = 65001;

// If the conversion was lossy, returns Err(lossy_result)
pub fn ansi_codepage_cstring<T: AsRef<OsStr>>(input: T) ->Result<Vec<i8>,Vec<i8>> {

    unsafe {
        let os_str = input.as_ref();
        let unicode = winapi_str(os_str);
        let length = WideCharToMultiByte(
            CP_ACP,
            0,
            unicode.as_ptr(),
            unicode.len() as i32,
            null_mut(),
            0,
            null_mut(),
            null_mut(),
        );
        let mut buffer = vec![0i8; length as usize];
        let mut used_default_char = 0;
        WideCharToMultiByte(
            CP_ACP,
            0,
            unicode.as_ptr(),
            unicode.len() as i32,
            buffer.as_mut_ptr() as *mut i8,
            length,
            null_mut(),
            &mut used_default_char,
        );
        
        if used_default_char != 0 {
            Err(buffer)
        } else {
            Ok(buffer)
        }
    }
    
}


pub fn utf16_to_utf8(utf16_string: &[u16]) -> String {
    let utf8_vec: Vec<u8> = utf16_string
        .iter()
        .flat_map(|&c| std::char::from_u32(c as u32))
        .flat_map(|c| c.to_string().as_bytes().to_vec())
        .collect();
    
    String::from_utf8(utf8_vec).unwrap_or_else(|_| String::new())
}#![allow(warnings, unused)]

use fltk::enums::{Color, Cursor, Event, FrameType, Key};
use fltk::{prelude::*, *};
use fltk::app::event_key;
use fltk_theme::{color_themes, ColorTheme, SchemeType, ThemeType, WidgetScheme, WidgetTheme};


pub struct varTagControl {
    pub title: frame::Frame,
    pub name: frame::Frame,
    x:i32, y:i32, width: i32, height: i32,
    pub id:String,
    pub data:String
}

impl varTagControl {
    // 创建
    pub fn new(x:i32, y:i32, width: i32, height: i32,title:&str , name: &str, data:&str) -> Self {
        // 按照总宽度分配可用控件
        // let all_input_len = format!("{}{}",&title,&name).len() as i32;
        // let width_per_char = width / all_input_len;
        // let title_len = title.len() as i32;


        // let name_width = width_per_char * (all_input_len - title_len)+5;
        // let title_width = width - name_width;

        // 计算字符宽度 如果包含了宽字符 前面要加几像素
        let title_len =  title.chars().count() as i32;
        let mut title_width = title_len*11 ;
        let has_title_wide = title_len!=title.len() as i32;
        let mut title_x =(if has_title_wide {5} else {0} )+ x.clone();

        // 添加文本显示控件
        let mut title_frame = frame::Frame::new(title_x, y,title_width ,height , "");
        title_frame.set_label(title);
        title_frame.set_label_size(12);
        title_frame.set_label_color(Color::from_rgb(77, 77, 77));


        let name_len =  title.chars().count() as i32;
        let mut name_width = title_len*12 ;
        let has_name_wide = title_len!=title.len() as i32;
        let mut name_x =(if has_title_wide {5} else {-20} )+ x.clone()+name_len*11;
        let name_text_size = name.len();
        if  name_text_size==1 {
            name_x += 6;
        }

        if  name.contains("月") {
            name_x -= 10;
        }

        if name_text_size>9 {
            name_x += 15;
        }

        if name_text_size>11 {
            name_x += 25;
        }

        if name_text_size>28 {
            name_x += 25;
        }

        let mut name_frame = frame::Frame::new(name_x, y, name_width ,height, "");
        name_frame.set_label(name);
        name_frame.set_label_size(12);
        name_frame.set_label_color(Color::from_rgb(40, 40, 40));

        Self { title:title_frame , name:name_frame, x, y, width, height , data:data.to_string(),id:title.to_string().replace(" ","").replace(":","")}
    }


    // 判断鼠标是否在当前元素
    pub fn existPoint(&self, x: i32, y: i32) -> bool {
        return x > self.x
            && x < self.x + self.width
            && y > self.y
            && y < self.y + self.height;
    }
    pub(crate) fn get_var(&self) -> String{
        return format!("<{}>",self.id.clone().replace("%",""));
    }
}
impl Clone for varTagControl {
    fn clone(&self) -> Self {
        varTagControl {
            title:self.title.clone(),
            name:self.name.clone(),
            x: self.x.clone(),
            y: self.y.clone(),
            width: self.width.clone(),
            height: self.height.clone(),
            id:self.id.clone(),
            data:self.data.clone()
        }
    }
}

#![allow(dropping_references)]

use crate::util::{str_eq_str, Sleep};
use crate::{global_var, util, get_bool,set_bool};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize,AtomicBool, Ordering};
use std::sync::{mpsc, OnceLock};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;
use crate::console_log;
static WATCH_PATH_ID: AtomicUsize = AtomicUsize::new(0);

// lazy_static! {
//     static ref WATCH_NEXT_EXITS: Mutex<bool> = Mutex::new(false);
// }

static WATCH_NEXT_EXITS: AtomicBool = AtomicBool::new(false);

struct TmepMetadata {
    pub metadata: fs::Metadata,
    pub dir: std::path::PathBuf,
}

pub fn has_next_exits() -> bool {
    get_bool!(WATCH_NEXT_EXITS)
}

pub fn un_next_exits() -> bool {
    set_bool!(WATCH_NEXT_EXITS,false)
}

pub fn initialize_next_exits() -> bool {
    set_bool!(WATCH_NEXT_EXITS,true)
}

fn get_next_id() -> usize {
    let mut id: usize = 0;
    let mutex = Arc::new(Mutex::new(&WATCH_PATH_ID));
    mutex.lock();
    id = WATCH_PATH_ID.fetch_add(1, Ordering::Relaxed);
    drop(mutex);
    id
}
pub fn get_the_id() -> usize {
    let mut id: usize = 0;
    let mutex = Arc::new(Mutex::new(&WATCH_PATH_ID));
    mutex.lock();
    id = WATCH_PATH_ID.load(Ordering::SeqCst);
    drop(mutex);
    id
}

/**
 *处理文件刷新
 */
pub fn watch_path_puppet(dir_path: String, send_main_tx: mpsc::Sender<PathBuf>) ->usize{
    initialize_next_exits();
    let mut watch_puppet_id = get_next_id();
    watch_puppet_id = get_the_id();

    thread::spawn(move || {
        let (tx, rx) = std::sync::mpsc::channel();
        println!(
            "watch_path_puppet-> {:?}  -> id {}",
            dir_path.clone(),
            watch_puppet_id.clone()
        );

        // let mut config = Config::default().with_poll_interval(Duration::from_millis(1200));

        let mut watcher = RecommendedWatcher::new(tx,Config::default()).unwrap();
        watcher
            .watch(dir_path.as_ref(), RecursiveMode::Recursive)
            .unwrap();

        //  需要处理的任务已经更新了 释放
        thread::spawn(move || loop {
            Sleep(300);
            if (watch_puppet_id != get_the_id() || !has_next_exits()) {                
                drop(&watcher);
                return;
            }
        });

        let mut shake_path = std::collections::HashSet::new();
        for res in rx {
            // 需要处理的任务已经更新了 释放 为什么不用un 因为会误操作其他的
            match res {
                Ok(event) => {

                    if shake_path.len()>5 {
                        shake_path.clear();
                    }

                    for value in event.clone().paths {
                        let mut paths = value.clone().display().to_string();
                        let mut ext = util::path_extension(&value);

                        // 是文件 后缀是dat 更新方式是修改
                        if (value.is_file()
                            &&paths.contains("MsgAttach")
                            && (event.clone().kind.is_modify())
                            && str_eq_str("dat".to_owned(), ext.clone()))
                        {
                            if shake_path.insert(value.clone()) {
                                let send_main_tx = send_main_tx.clone();
                                thread::spawn(move || {
                                    Sleep(1888);
                                    send_main_tx.send(value.clone());
                                    println!("is_modify [is_modify] -> {:?}  id ->  {}", value.clone(),watch_puppet_id.clone());
                                });

                            }

                        }
                    }
                }
                Err(e) => {
                    console_log!(format!(
                        "[报错] 检测选择器界面文件更新错误 因为-> {}",
                        e.to_string()
                    ));
                }
            }
        }
    });
    return watch_puppet_id.clone();
}




// 判断当前是否处于开发者模式
pub fn is_developer() -> bool {

    // std::env::var("_debug").is_ok()
    !false
}

// 编译版本是 52破解专版
pub fn is_build_52pojie() -> bool {
    false
}

// 是否对显示的数据进行消敏
pub fn is_show_mask() -> bool {
    is_show_dome()||true
}

// 是否在选择对象后自动显示最近十张照片
pub fn is_click_open_preview() -> bool {
    false
}

// 演示模式
pub fn is_show_dome() -> bool {
    true
}#![allow(warnings, unused)]

use std::collections::hash_map::DefaultHasher;
use std::io::Read;
use std::mem::transmute;
use std::{
    env,
    ffi::{c_int, c_long, OsStr},
    fs,
    hash::{Hash, Hasher},
    io,
    path::{Path, PathBuf},
    process,
    process::{Command, Stdio},
    thread,
    time::Duration,
};

use libc::getenv;
use std::string::FromUtf8Error;

use crate::{libWxIkunPlus, wh_mod};

// 判断文件头(魔术数字)
pub fn detect_image_format(data: &[u8]) -> Option<&'static str> {
    if data.len() < 4 {
        return None; // 数据不够长，无法识别
    }

    match &data[0..4] {
        [0x89, 0x50, 0x4E, 0x47] => Some("png"),
        [0xFF, 0xD8, _, _] => Some("jpg"),
        [0x47, 0x49, 0x46, 0x38] => Some("gif"),
        [0x52, 0x49, 0x46, 0x46]
            if data.len() >= 12 && &data[8..12] == [0x57, 0x45, 0x42, 0x50] =>
        {
            Some("webp")
        }
        _ => None, // 未知格式
    }
}

// !请注意  异或解密算法是不开源的部分
// 对指定的文件进行异或算法 以此获取解码的文件
// 对指定的文件进行异或算法 以此获取解码的文件
pub fn convert_bat_images(input_path: PathBuf, mut output_path: PathBuf) -> io::Result<String> {

    let buff =  convert_dat_images_buff(output_path.clone())?;
    match detect_image_format(&buff.clone()) {
        Some(format) => {
            output_path.set_extension(format);
            let mut out_dir = Path::new(&output_path).join("..");

            if !out_dir.exists() {
                fs::create_dir_all(out_dir)?;
            }

            fs::write(&output_path, &buff)?;
        }
        None => println!("Unknown image format."),
    }

    Ok(output_path.display().to_string())
}


// !请注意  异或解密算法是不开源的部分
// 对指定的文件进行异或算法 以此获取解码的文件
pub fn convert_dat_images_buff(input_path: PathBuf) -> io::Result<Vec<u8>> {
    let content = fs::read(&input_path)?;

    // let ** = content[0];
    // let ** = content[1];

    // let ** = ** ^ 0x66666;
    // let ** = ** ^ 0x66666;
    // let ** = ** ^ 0x66666;
    // let ** = ** ^ 0x66666;
    // let ** = ** ^ 0x66666;
    // let ** = ** ^ 0x66666;

    // let mut ** = ** ^ 0x66666;

    // if ** == ** {
    //     vi = **;
    // } else if ** == ** {
    //     vi = **;
    // } else if ** == ** {
    //     vi = **;
    // }

    // let buff: Vec<u8> = content.iter().map(|br| br ^ vi).collect();
   
    let buff: Vec<u8> = Vec::new();
    Ok(buff)
}

// 按照换行分割文本
pub fn split_line(input_path: String) -> Vec<String> {
    let mut path2arr: Vec<String> = Vec::new();
    let mut str: String = String::new();
    // 按照 \\ / 分割路径
    for char in input_path.chars() {
        if char.to_string().bytes().eq("\n".to_string().bytes())
            || char.to_string().bytes().eq("\r".to_string().bytes())
        {
            path2arr.push(str.clone());
            str.clear();
            continue;
        }
        str.push(char);
    }

    // 最后一个文本
    if str.len() != 0 {
        path2arr.push(str.clone());
    }

    let mut path2arr_filter = Vec::new();

    for path2 in path2arr {
        if (!path2.is_empty()) {
            path2arr_filter.push(path2);
        }
    }

    path2arr_filter
}

// 获取用户数据根文件夹
pub fn get_user_data_path() -> Option<String> {
    // ! 这部分也是不开源的部分
    Option::None
}

// 获取用户名
#[derive(Debug)]
pub struct WxidNameAccinfo {
    // 头像url
    pub avatar: String,
    // 名称
    pub name: String,
    // wxid
    pub wx_id: String,
}

// 获取指定数据的用户名
pub fn get_wxid_name(user_data_path: String, wxid: String) -> Option<WxidNameAccinfo> {
    // ! 这部分也是不开源的部分
    Option::None
}

// 开发者用户id
pub fn get_user_id1() -> String {
    // ! 这部分也是不开源的部分
    return String::new();
}

// 开发者用户id
pub fn get_user_id2() -> String {
    // ! 这部分也是不开源的部分
    return String::new();
}

// 获取活动用户信息
#[derive(Debug)]
pub struct WxActiveUser {
    // 获取用户名
    pub accinfo: Option<WxidNameAccinfo>,
    // wxid
    pub user_wxid: String,
    // 根数据目录 D:\...\WeChat Files
    pub user_root: String,
    // 数据目录 D:\...\WeChat Files\%wxid%
    pub user_data: String
}

// 获取活动用户的路径
pub fn get_active_user(user_root: &str) -> Vec<WxActiveUser> {
    let mut active_users = Vec::new();
    let mut get_size = libWxIkunPlus::findAllWindow("WeChatMainWndForPC", "").len();
    let mut read_dir_list: Vec<fs::DirEntry> = Vec::new();

    // 读取包含 wxid_ 的文件夹
    if let Ok(read_dir) = fs::read_dir(user_root) {
        for dir in read_dir {
            if let Ok(dir) = dir {
                let string_lossy = format!("{:?}", dir.file_name());

                if string_lossy.contains("wxid_") {
                    read_dir_list.push(dir);
                }
            }
        }
    }
    
    let mut vec_wxid_list:Vec<PathBuf> = Vec::new();
    
    for value in read_dir_list {
        // 通过高更新率的文件判断出最后修改时间
        let config_path = value.path().join("config");

        let mut read_file_list = Vec::new();

        if let Ok(read_dir) = fs::read_dir(config_path) {
            for dir in read_dir {
                if let Ok(read_file) = dir {
                    read_file_list.push(read_file);
                }
            }
        }

        //按照修改时间排序
        read_file_list.sort_by(|a, b| {
            let mut a_created = std::time::UNIX_EPOCH;
            let mut b_created = std::time::UNIX_EPOCH;

            if let Ok(metadata) = a.metadata() {
                if let Result::Ok(create) = metadata.modified() {
                    a_created = create;
                }
            }

            if let Ok(metadata) = b.metadata() {
                if let Result::Ok(create) = metadata.modified() {
                    b_created = create;
                }
            }

            a_created.cmp(&b_created)
        });
        read_file_list.reverse();

        if !read_file_list.is_empty() {
            vec_wxid_list.push(read_file_list[0].path());
        }   

    }

    // 排序出根目录的路径
    vec_wxid_list.sort_by(|a, b| {
        let mut a_created = std::time::UNIX_EPOCH;
        let mut b_created = std::time::UNIX_EPOCH;

        if let Ok(metadata) = a.metadata() {
            if let Result::Ok(create) = metadata.modified() {
                a_created = create;
            }
        }

        if let Ok(metadata) = b.metadata() {
            if let Result::Ok(create) = metadata.modified() {
                b_created = create;
            }
        }

        a_created.cmp(&b_created)
    });

    vec_wxid_list.reverse();

    for value in vec_wxid_list.to_vec() {
        if active_users.len() >= get_size &&!is_developer(){
            break;
        }
     
       let parse_path = wh_mod::wx_parse_path(format!("{}",value.to_str().unwrap()));
        
        if !parse_path.user_data.is_empty()&&!parse_path.wxid.is_empty() {
            active_users.push(WxActiveUser{
                accinfo: get_wxid_name(parse_path.user_data.clone(),parse_path.wxid.clone()),
                user_wxid: parse_path.wxid.clone(),
                user_root: parse_path.user_data.clone(),
                user_data: format!("{}\\{}",parse_path.user_data.clone(),parse_path.wxid.clone()),
            });  
        }
        
      

    //    println!("parse_path-> {:?}",&parse_path);
    }
   
    // println!("read_file_list ->  {:?}",&vec_wxid_list);

    active_users
}

#![allow(warnings, unused)]

use crate::{atomic_util, global_var, gui_hotspot, gui_imge, handle_dat, inject_fltk_theme, libWxIkunPlus, util::{str_eq_str, Sleep}, wh_mod};
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

fn initialize_watch_attach_puppet(img_path: String) {
    atomic_util::set_bool(&INITIALIZED_PUPPET, true);
    thread::spawn(move || {
        atomic_util::set_bool(&WATCH_PUPPET_ING, true);
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

        let history_attach_list = wh_mod::get_walk_attach_file_history();
        if (history_attach_list.len() != 0) {
            println!("扫描历史查找... 共-> {} 条",history_attach_list.len());
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
                    global_var::set_string(
                        "user::config::walk_drag_path",
                        path.to_string_lossy().to_string(),
                    );
                    atomic_util::set_bool(&WATCH_PUPPET_ING, false);
              
              
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
            let input_select_dir = global_var::get_string_default("user::config::input_select_dir");
            let user_select_wxid = global_var::get_string_default("user::config::user_select_wxid");
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
                        global_var::set_string(
                            "user::config::walk_drag_path",
                            path.to_string_lossy().to_string(),
                        );
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
        atomic_util::set_bool(&WATCH_PUPPET_ING, false);
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
    atomic_util::set_bool(&INITIALIZED_PUPPET, false);
    atomic_util::set_bool(&WINDOW_STATE_AVAILABLE, true);
    atomic_util::set_i64(&WINDOW_HWND, 0);
    global_var::set_string("user::config::walk_drag_path", String::new());

    if (global_var::get_bool_default("gui::open::gui_drag_scan")) {
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

    inject_fltk_theme!();
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

    thread::spawn(move || {
        let mut window_state_available = atomic_util::get_bool(&WINDOW_STATE_AVAILABLE);

        while window_state_available {
            Sleep(300);

            if atomic_util::get_bool(&INITIALIZED_PUPPET) {
                if atomic_util::get_bool(&WATCH_PUPPET_ING) {
                    println!("开始扫描了-> ");
                    state_info.set_label("正在扫描中...");
                    btn_text.set_label("取消任务");
                } else {
                    let walk_drag_path =
                        global_var::get_string_default("user::config::walk_drag_path");

                    state_title.set_label("扫描已结束");
                    println!("walk_drag_path-> {}", walk_drag_path.as_str());
                    if (walk_drag_path.len() < 3) {
                        state_info.set_label("未扫描到此文件");
                        btn_text.set_label("关闭窗口");
                    } else {
                        state_info.set_label("已经获取到此文件");
                        btn_text.set_label("完成选定");
                        // atomic_util::set_bool(&WINDOW_STATE_AVAILABLE,false);pr
                        // println!("user_select_path-> {}",global_var::get_str("user::config::user_select_path"));

                        global_var::set_string(
                            "user::config::user_select_path",
                            wh_mod::wx_parse_path(walk_drag_path.clone()).attach_id,
                        );
                        global_var::set_i32("user::config::select_user_thumbnail_obj", -2);
                        
                        // atomic_util::set_bool(&WINDOW_STATE_AVAILABLE, false);
                        // global_var::set_bool("gui::open::gui_drag_scan", false);
                        // atomic_util::set_i64(&WINDOW_HWND, 0);
                        // if let Some(mut win) = fltk::app::widget_from_id("gui::DoubleWindow::gui_drag_scan::main") as Option<DoubleWindow> {
                        //     win.hide();
                        //     win.clear();
                            
                        //     fltk::window::Window::delete(win.clone());
                        // }

                    }
                    return;
                }
            }
            window_state_available = atomic_util::get_bool(&WINDOW_STATE_AVAILABLE);
        }
        println!("gui_drag_scan 窗口关闭了-> ");
    });

    macro_rules! is_closeWindow {
        () => {
            if !atomic_util::get_bool(&WINDOW_STATE_AVAILABLE) {
                closeWindow(atomic_util::get_i64(&WINDOW_HWND) as i128, true);
                global_var::set_bool("gui::open::gui_drag_scan", false);
            }
        };
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
                atomic_util::set_i64(&WINDOW_HWND, hwnd as i64);
                println!("walk_drag_page hwnd -> :  {}", hwnd.clone());
                true
            }
            enums::Event::Hide => {
                is_closeWindow!();
                false
            }
            enums::Event::Push => {
                if (next_btn.existPoint(x, y)) {
                    atomic_util::set_bool(&WINDOW_STATE_AVAILABLE, false);
                    // closeWindow(win.raw_handle() as i128, true);
                    fltk::window::Window::delete(win.clone());
                    global_var::set_bool("gui::open::gui_drag_scan", false);
                    atomic_util::set_i64(&WINDOW_HWND, 0);
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
            Event::Unfocus => true,
            Event::Leave => true,
            Event::NoEvent => true,
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

                    if (!atomic_util::get_bool(&INITIALIZED_PUPPET)) {
                        if !path_list.is_empty() {
                            path_list.reverse();
                            for for_path in path_list {
                                // if wh_mod::convert::is_developer()||(for_path.contains("wxid_")&&(for_path.contains("FileStorage\\MsgAttach")||for_path.contains("FileStorage/MsgAttach"))){

                                if PathBuf::from(for_path.clone()).exists() {
                                    let path = Path::new(&for_path.clone()).to_path_buf();
                                    drag_path = path.clone();

                                    next_scan_ing_gui.show();
                                    btn_text_next.set_label("结束任务");
                                    initialize_watch_attach_puppet(
                                        drag_path.to_string_lossy().to_string(),
                                    );
                                    break;
                                }
                                
                            }
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
            enums::Event::Close=>{
                atomic_util::set_bool(&WINDOW_STATE_AVAILABLE, false);
                global_var::set_bool("gui::open::gui_drag_scan", false);
                atomic_util::set_i64(&WINDOW_HWND, 0);
                // fltk::window::Window::delete(win.clone());
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
#![allow(warnings, unused)]

pub struct HotspotItmeControl {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl HotspotItmeControl {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        let mut check_item_control = HotspotItmeControl {
            x,
            y,
            width,
            height,
        };

        check_item_control
    }

    // 判断鼠标是否在当前元素
    pub fn existPoint(&self, x: i32, y: i32) -> bool {
        return x > self.x.clone()
            && x < self.x.clone() + self.width.clone()
            && y > self.y.clone()
            && y < self.y.clone() + self.height.clone();
    }
}

pub fn create_hotspot(x: i32, y: i32, width: i32, height: i32) -> HotspotItmeControl {
    HotspotItmeControl::new(x, y, width, height)
}
#![allow(warnings, unused)]

use fltk::{
    app::{self, handle},
    button::{self, Button},
    draw::font,
    enums::{self, Color, Cursor, Event, Font, FrameType, LabelType},
    frame::{self, Frame},
    group::{self, Flex, Group},
    image::{self, Image, PngImage},
    input::{InputType, IntInput},
    prelude::*,
    text::TextDisplay,
    tree,
    window::{self, DoubleWindow, Window},
};
use fltk_table::{SmartTable, TableOpts};
use std::{
    fs,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    process,
    process::{Command, Stdio},
    thread,
};

pub struct ImagesItmeControl {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub id: String,
}

impl ImagesItmeControl {
    pub fn new(
        appMainWin: &mut window::DoubleWindow,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        images: PngImage,
        id: String,
    ) -> Self {
        let mut check_itme_control = ImagesItmeControl {
            x,
            y,
            width,
            height,
            id: id.clone(),
        };
        let mut frame = Frame::default()
            .with_size(width, height)
            .center_of(appMainWin);

        frame.set_frame(FrameType::NoBox);
        frame.set_color(Color::from_u32(0));
        frame.set_id(id.as_str());

        frame.set_image(Some(images));
        frame.set_pos(x, y);
        frame.show();

        check_itme_control
    }

    /**
     * 获取主窗口
     */
    pub fn get_main(&self) -> Frame {
        let mut frame: Frame = app::widget_from_id(self.id.as_str()).unwrap();
        frame
    }

    // 判断鼠标是否在当前元素
    pub fn existPoint(&self, x: i32, y: i32) -> bool {
        return x > self.x.clone()
            && x < self.x.clone() + self.width.clone()
            && y > self.y.clone()
            && y < self.y.clone() + self.height.clone();
    }
}

pub fn create_Images(
    appMainWin: &mut window::DoubleWindow,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    images: PngImage,
    id: String,
) -> ImagesItmeControl {
    ImagesItmeControl::new(appMainWin, x, y, width, height, images, id)
}

pub struct ImgPreview {
    pub preview: frame::Frame,
    x:i32,
    y:i32,
    width: i32, 
    height: i32
}
impl Clone for ImgPreview {
    fn clone(&self) -> Self {
        ImgPreview {
            preview:self.preview.clone(),
            x: self.x.clone(),
            y: self.y.clone(),
            width: self.width.clone(),
            height: self.height.clone(),
        }
    }
}

impl ImgPreview {
    pub fn detect_image_format(data: &[u8]) -> Option<&'static str> {
        if data.len() < 4 {
            return None; // 数据不够长，无法识别
        }

        match &data[0..4] {
            [0x89, 0x50, 0x4E, 0x47] => Some("png"),
            [0xFF, 0xD8, _, _] => Some("jpg"),
            [0x47, 0x49, 0x46, 0x38] => Some("gif"),
            [0x52, 0x49, 0x46, 0x46]
                if data.len() >= 12 && &data[8..12] == [0x57, 0x45, 0x42, 0x50] =>
            {
                Some("webp")
            }
            _ => None, // 未知格式
        }
    }

    pub fn new(x: i32, y: i32, width: i32, height: i32, id: &'static str) -> Self {
        let mut preview = frame::Frame::new(x, y, width, height, "");
        preview.set_frame(enums::FrameType::FlatBox);
        preview.set_color(enums::Color::from_rgb(80, 80, 80));
        preview.set_id(id);
        Self { preview, x, y, width, height }
    }

    // 判断鼠标是否在当前元素
    pub fn existPoint(&self, x: i32, y: i32) -> bool {
        return x > self.x.clone()
            && x < self.x.clone() + self.width.clone()
            && y > self.y.clone()
            && y < self.y.clone() + self.height.clone();
    }

    pub fn load(&mut self, path: String, x: i32, y: i32, width: i32, height: i32) -> bool {
        let mut res = false;
        // macro_rules! re_imag {
        //     ($imag:expr) => {
        //         $imag.scale(width, height, false, true);
        //         self.preview.draw(move |cb| {
        //             let cbx: i32 = cb.x();
        //             let cby: i32 = cb.y();
        //             let cbh: i32 = cb.h();
        //             let cbw: i32 = cb.w();
        //             let cx: i32 = x;
        //             let cy: i32 = y;
        //             $imag.draw_ext(cbx, cby, cbw, cbh, cx, cy);
        //         });
        //         self.preview.redraw();
        //         self.preview.redraw_label();
        //         res = true;
        //     };
        // }

        if let Result::Ok(data) = fs::read(path) {
            res = self.from_data(data, x, y, width, height);
        }
        res
    }

    pub fn from_data(&mut self, data: Vec<u8>, x: i32, y: i32, width: i32, height: i32) -> bool {
        let mut res = false;
        macro_rules! re_imag {
            ($imag:expr) => {
                $imag.scale(width, height, false, true);
                self.preview.draw(move |cb| {
                    let cbx: i32 = cb.x();
                    let cby: i32 = cb.y();
                    let cbh: i32 = cb.h();
                    let cbw: i32 = cb.w();
                    let cx: i32 = x;
                    let cy: i32 = y;
                    $imag.draw_ext(cbx, cby, cbw, cbh, cx, cy);
                });
                self.preview.redraw();
                self.preview.redraw_label();
                res = true;
            };
        }

        if let Some(ext) = ImgPreview::detect_image_format(&data) {
            if (ext.as_bytes().eq("png".as_bytes())) {
                if let Result::Ok(mut img) = image::PngImage::from_data(&*data) {
                    re_imag!(img);
                }
            } else if (ext.as_bytes().eq("jpg".as_bytes())) {
                if let Result::Ok(mut img) = image::JpegImage::from_data(&*data) {
                    re_imag!(img);
                }
            } else if (ext.as_bytes().eq("gif".as_bytes())) {
                if let Result::Ok(mut img) = image::GifImage::from_data(&*data) {
                    re_imag!(img);
                }
            }
        }

        res
    }

    // pub fn from_imag<T : fltk::prelude::ImageExt >(&mut self, mut data:  T, x: i32, y: i32, width: i32, height: i32){
    //     let mut res = false;
    //     self.preview.draw(move |cb| {
    //         let cbx: i32 = cb.x();
    //         let cby: i32 = cb.y();
    //         let cbh: i32 = cb.h();
    //         let cbw: i32 = cb.w();
    //         let cx: i32 = x;
    //         let cy: i32 = y;
    //         data.draw_ext(cbx, cby, cbw, cbh, cx, cy);
    //     });
    //     self.preview.redraw();
    //     self.preview.redraw_label();
    //     res = true;
    //
    // }
}
#![allow(warnings, unused)]


use crate::{
    global_var, handle_dat, libWxIkunPlus,
    util::{str_eq_str, Sleep}, global_var_util,
};
use fltk::{
    app::{self, handle},
    button::{self, Button},
    draw::font,
    enums::{self, Color, Cursor, Event, Font, FrameType, LabelType},
    frame::Frame,
    group::{self, Flex, Group},
    image,
    input::{InputType, IntInput},
    prelude::*,
    text::TextDisplay,
    tree,
    window::{self, DoubleWindow, Window},
};
use fltk_table::{SmartTable, TableOpts};
use rusqlite::Connection;
use std::{
    fs,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    process,
    process::{Command, Stdio},
    thread,
};
use winapi::um::winuser::{CloseWindow, SetActiveWindow};
use crate::console_log;
// 按钮基本都是以坐标判断的
struct PointExistHasmap {
    // 关闭按钮
    quit: bool,
    // 下个
    next: bool,
    // 上个
    backup: bool,
    // 卡片 1
    card_01: bool,
    // 卡片 2
    card_02: bool,
    // 卡片 3
    card_03: bool,

    // 卡片 1
    rm_card_01: bool,
    // 卡片 2
    rm_card_02: bool,
    // 卡片 3
    rm_card_03: bool,

    // 所有按钮
    existCursor: bool,
}

// 判断鼠标坐标是否在此元素内
fn getFormPointSpace(x: i32, y: i32) -> PointExistHasmap {
    // 输入: x,y,w,y
    macro_rules! check_point_in_space {
        ($xx:expr, $yy:expr, $width:expr, $height:expr) => {
            x > $xx && x < $xx + $width && y > $yy && y < $yy + $height
        };
    }

    let mut point_exist_hasmap = PointExistHasmap {
        quit: false,
        next: false,
        backup: false,
        card_01: false,
        card_02: false,
        card_03: false,
        rm_card_01: false,
        rm_card_02: false,
        rm_card_03: false,
        existCursor: false,
    };

    point_exist_hasmap.quit = check_point_in_space!(273, 17, 40, 40);
    point_exist_hasmap.backup = check_point_in_space!(41, 492, 121, 44);
    point_exist_hasmap.next = check_point_in_space!(171, 492, 121, 44);
    point_exist_hasmap.card_01 = check_point_in_space!(20, 90, 286, 110);
    point_exist_hasmap.card_02 = check_point_in_space!(20, 216, 286, 110);
    point_exist_hasmap.card_03 = check_point_in_space!(20, 345, 286, 110);

    // 移除
    point_exist_hasmap.rm_card_01 = check_point_in_space!(230, 164, 65, 22);
    point_exist_hasmap.rm_card_02 = check_point_in_space!(230, 291, 65, 22);
    point_exist_hasmap.rm_card_03 = check_point_in_space!(230, 419, 65, 22);

    let mut win_coords_cursor_list = vec![
        point_exist_hasmap.quit,
        point_exist_hasmap.backup,
        point_exist_hasmap.next,
        point_exist_hasmap.rm_card_01,
        point_exist_hasmap.rm_card_02,
        point_exist_hasmap.rm_card_03,
    ];

    let mut existCursor = false;
    for value in win_coords_cursor_list.iter() {
        // 关闭按钮
        if *(value) {
            existCursor = true;
        }
    }

    point_exist_hasmap.existCursor = existCursor;

    return point_exist_hasmap;
}

// 设置背景为图片（主视图）
fn setInterfaceBackgroundImage(appMainWin: &mut window::DoubleWindow) -> Frame {
    let background_image = image::PngImage::from_data(include_bytes!("../assets/manage_itme.png"))
        .expect("set main icon error");
    let mut frame = Frame::default().with_size(326, 0).center_of(appMainWin);
    frame.set_frame(FrameType::EngravedBox);
    frame.set_image(Some(background_image));
    return frame;
}

// 剪裁路径 字数不超过XXX
pub fn get_form_slice_path(path: String, max_size: usize) -> Result<String, String> {
    let mut new_path: String = String::new();
    let mut size = 0;

    for ch in path.chars() {
        new_path.push(ch);
        size = size + 1;
        if (size > max_size) {
            break;
        }
        // println!("{}", ch);
    }

    // let mut size = path.len();

    // if size > max_size {
    // size = max_size;
    // return Err("Input string exceeds maximum size".to_string());
    // }

    // let new_path: String = path[..size].to_string();
    Ok(new_path)
}

// 卡片会回传为这个参数 用来控制
struct CardItme {
    background: Frame,
    path: Frame,
    ouput: Frame,
    name: Frame,
    status_err: Frame,
    status_ok: Frame,
    main: DoubleWindow,
    nameStr: String,
    pathStr: String,
    ouputStr: String,
    remove: bool,
}

// 任务卡片 （不会动态创建 而是引用同一个）
fn create_card(x: i32, y: i32, name: String, path: String, ouput: String) -> CardItme {
    let mut card_win = window::Window::new(x, y, 289, 113, None);
    card_win.set_color(Color::from_rgb(24, 24, 24));

    // 背景
    let background_image = image::PngImage::from_data(include_bytes!("../assets/card.png"))
        .expect("set main icon error");
    let mut frame = Frame::default().with_size(289, 0).center_of(&card_win);
    frame.set_frame(FrameType::EngravedBox);
    frame.set_image(Some(background_image));

    // 文本-> path
    let mut frame_info_path = Frame::new(17, 18, 200, 18, None); //.with_size(200, 18).left_of(&flex,1);
                                                                 // let slice_path =path.get(0..35).unwrap();

    frame_info_path.set_label(&get_form_slice_path(path.clone(), 20).unwrap_or(path.clone()));
    frame_info_path.set_label_size(12);
    frame_info_path.set_label_color(Color::from_rgb(186, 186, 186));
    frame_info_path.resize(32, 18, 200, 18);

    // 文本-> ouput
    let mut frame_info_ouput = Frame::new(17, 18, 200, 18, None); //::default().with_size(200, 18).left_of(&flex,1);
    frame_info_ouput.set_label(&get_form_slice_path(ouput.clone(), 20).unwrap_or(ouput.clone()));
    frame_info_ouput.set_label_size(12);
    frame_info_ouput.set_label_color(Color::from_rgb(186, 186, 186));
    frame_info_ouput.resize(32, 44, 200, 18);

    // 文本-> name
    let mut frame_info_name = Frame::default().with_size(200, 22);
    frame_info_name.set_label(&get_form_slice_path(name.clone(), 7).unwrap_or(name.clone()));
    frame_info_name.set_label_size(15);
    frame_info_name.set_label_color(Color::from_rgb(255, 255, 255));
    frame_info_name.resize(32, 80, 200, 22);

    let card_ok = image::PngImage::from_data(include_bytes!("../assets/card_ok.png"))
        .expect("set main icon error");

    let card_error = image::PngImage::from_data(include_bytes!("../assets/card_error.png"))
        .expect("set main icon error");

    let mut card_status_ok = Frame::default().with_size(26, 26).center_of(&card_win);
    card_status_ok.set_frame(FrameType::EngravedBox);
    card_status_ok.set_image(Some(card_ok));
    card_status_ok.set_pos(244, 11);

    let mut card_status_error = Frame::default().with_size(26, 26).center_of(&card_win);
    card_status_error.set_frame(FrameType::EngravedBox);
    card_status_error.set_image(Some(card_error));
    card_status_error.set_pos(244, 11);
    card_status_error.hide();

    card_win.end();

    let move_frame_info_path = frame_info_path.clone();
    let mut move_card_status_error = card_status_error.clone();
    let mut move_card_status_ok = card_status_ok.clone();

    return CardItme {
        main: card_win,
        background: frame,
        path: frame_info_path,
        ouput: frame_info_ouput,
        name: frame_info_name,
        status_err: card_status_error,
        status_ok: card_status_ok,
        ouputStr: ouput,
        nameStr: name,
        pathStr: path,
        remove: false,
    };
}

// 主窗口
pub fn ManageItmeMain() {
    if (global_var::get_bool_default("gui::open::manage_item")) {
        if let Some(mut wins) =
            app::widget_from_id("gui::DoubleWindow::manage_item::main") as Option<DoubleWindow>
        {
            wins.show();
            wins.set_visible_focus();
        }

        return ;
    }

    global_var::set_bool("gui::open::manage_item", true);

    let mut win: DoubleWindow = DoubleWindow::new(0, 0, 326, 554, "管理分组");
    win.set_color(Color::from_rgb(24, 24, 24));
    win.set_border(false);
    win.set_id ("gui::DoubleWindow::manage_item::main");

    let mut export_dir_path_list: Vec<global_var_util::ExportDirItme> =
    global_var_util::get_export_dir_itme_list();

    // 偏移量
    let mut offset = 0;

    fltk::app::set_scrollbar_size(3);
    fltk::app::set_selection_color(24, 24, 24);
    setInterfaceBackgroundImage(&mut win);

    let mut default_itme_text = "没有数据...";

    macro_rules! create_card_itme {
        ($card_id:expr) => {
            create_card(
                18,
                (if ($card_id == 1) {
                    88
                } else if ($card_id == 2) {
                    215
                } else {
                    342
                }),
                default_itme_text.to_owned(),
                default_itme_text.to_owned(),
                default_itme_text.to_owned(),
            )
        };

        ($card_id:expr,$itme_id:expr) => {
            create_card(
                18,
                (if ($card_id == 1) {
                    88
                } else if ($card_id == 2) {
                    215
                } else {
                    342
                }),
                export_dir_path_list[$itme_id].name.clone().to_owned(),
                export_dir_path_list[$itme_id].path.clone().to_owned(),
                export_dir_path_list[$itme_id].ouput.clone().to_owned(),
            )
        };

        ($card_id:expr,$name:expr,$path:expr,$ouput:expr ) => {
            create_card(
                18,
                (if ($card_id == 1) {
                    88
                } else if ($card_id == 2) {
                    215
                } else {
                    342
                }),
                $name.to_owned(),
                $path.to_owned(),
                $ouput.to_owned(),
            )
        };
    }

    let mut card_01 = if export_dir_path_list.len() >= 1 {
        create_card_itme!(1, 0)
    } else {
        create_card_itme!(1)
    };
    let mut card_02 = if export_dir_path_list.len() >= 2 {
        create_card_itme!(2, 1)
    } else {
        create_card_itme!(2)
    };
    let mut card_03 = if export_dir_path_list.len() >= 3 {
        create_card_itme!(3, 2)
    } else {
        create_card_itme!(3)
    };

    if export_dir_path_list.len() < 2 {
        card_02.main.hide();
    } else {
        // offset = 1;
    }

    if export_dir_path_list.len() < 3 {
        card_03.main.hide();
    } else {
        // offset = 2;
    }

    // card_01
    macro_rules! push_card_itme {
        ($card_itme:expr,$name:expr,$path:expr,$ouput:expr) => {
            $card_itme
                .path
                .set_label(&get_form_slice_path($path.clone(), 20).unwrap_or($path.clone()));
            $card_itme.path.set_label_size(12);
            $card_itme
                .path
                .set_label_color(Color::from_rgb(186, 186, 186));
            $card_itme.path.resize(32, 18, 200, 18);
            $card_itme.pathStr = $path;

            $card_itme
                .ouput
                .set_label(&get_form_slice_path($ouput.clone(), 20).unwrap_or($ouput.clone()));
            $card_itme.ouput.set_label_size(12);
            $card_itme
                .ouput
                .set_label_color(Color::from_rgb(186, 186, 186));
            $card_itme.ouput.resize(32, 44, 200, 18);
            $card_itme.ouputStr = $ouput;

            $card_itme
                .name
                .set_label(&get_form_slice_path($name.clone(), 7).unwrap_or($name.clone()));
            $card_itme.name.set_label_size(15);
            $card_itme
                .name
                .set_label_color(Color::from_rgb(255, 255, 255));
            $card_itme.name.resize(32, 80, 200, 22);
            $card_itme.nameStr = $name;

            $card_itme.status_ok.show();
            $card_itme.status_err.hide();
            $card_itme.remove = false;
        };

        ($card_id:expr,$itme_id:expr) => {
            if $card_id == 1 {
                push_card_itme!(
                    card_01,
                    export_dir_path_list[$itme_id].name.clone().to_owned(),
                    export_dir_path_list[$itme_id].path.clone().to_owned(),
                    export_dir_path_list[$itme_id].ouput.clone().to_owned()
                );
            }
            if $card_id == 2 {
                push_card_itme!(
                    card_02,
                    export_dir_path_list[$itme_id].name.clone().to_owned(),
                    export_dir_path_list[$itme_id].path.clone().to_owned(),
                    export_dir_path_list[$itme_id].ouput.clone().to_owned()
                );
            }
            if $card_id == 3 {
                push_card_itme!(
                    card_03,
                    export_dir_path_list[$itme_id].name.clone().to_owned(),
                    export_dir_path_list[$itme_id].path.clone().to_owned(),
                    export_dir_path_list[$itme_id].ouput.clone().to_owned()
                );
            }
        };

        ($card_id:expr,$itme_list:expr,$itme_id:expr) => {
            if $card_id == 1 {
                push_card_itme!(
                    card_01,
                    $itme_list[$itme_id].name.clone().to_owned(),
                    $itme_list[$itme_id].path.clone().to_owned(),
                    $itme_list[$itme_id].ouput.clone().to_owned()
                );
            }
            if $card_id == 2 {
                push_card_itme!(
                    card_02,
                    $itme_list[$itme_id].name.clone().to_owned(),
                    $itme_list[$itme_id].path.clone().to_owned(),
                    $itme_list[$itme_id].ouput.clone().to_owned()
                );
            }
            if $card_id == 3 {
                push_card_itme!(
                    card_03,
                    $itme_list[$itme_id].name.clone().to_owned(),
                    $itme_list[$itme_id].path.clone().to_owned(),
                    $itme_list[$itme_id].ouput.clone().to_owned()
                );
            }
        };
    }

    win.clone().center_screen();
    win.end();
    win.show();
    let mut num_items_to_take = 3;

    win.handle({
        let mut x = 0;
        let mut y = 0;
        let mut point_exist_hasmap = getFormPointSpace(x, y);
        let mut has_show = false;
        // let mut move_
        move |win, ev| match ev {
            enums::Event::Show => {
                win.set_visible_focus();
                libWxIkunPlus::setwinVisible(win.raw_handle() as i128 , true);
                // libWxIkunPlus::setWinIcon(win.raw_handle() as i128);

                println!("hwnd -> :  {}",win.raw_handle() as i128);
                true
            }

            enums::Event::Push => {
                // 关闭按钮
                if (point_exist_hasmap.quit) {
                    let mut hwnd = win.raw_handle() as i128;
                    win.clear();
                    global_var::set_bool("gui::open::manage_item", false);
                    libWxIkunPlus::setwinVisible(hwnd, false);                  
                    libWxIkunPlus::closeWindow(hwnd, true);
                    fltk::window::Window::delete(win.clone());
                    // fltk::window::Window::
                    println!("closeWindow hwnd -> :  {}",hwnd);
                }

                macro_rules! handle_point_card {
                    ($rm_card_id:expr, $card_itme:expr) => {
                        if ($rm_card_id && !$card_itme.remove) {
                          
                      
                            let mut path_string = $card_itme.pathStr.clone();
                            let conn: Connection =
                                Connection::open("ikun_user_data.db").expect("无法 创建/打开 数据库");
                            handle_dat::initialize_table(&conn);
                            match conn.execute(
                                "DELETE FROM export_dir_path WHERE path = ?1",
                                [path_string.clone()],
                            ) {
                                Ok(updated) => {
                                    $card_itme.status_err.show();
                                    $card_itme.status_ok.hide();
                                    // println!(
                                    //     "[移除] 移除的文件夹是: 名称：{}  路径：{}",
                                    //     $card_itme.nameStr,
                                    //     $card_itme.path.label()
                                    // );
                                    $card_itme.remove = true;
                                    console_log!(format!(
                                        "[移除] 移除的文件夹是: 名称：{}  路径：{}",
                                        $card_itme.nameStr,
                                        $card_itme.path.label()
                                    ));

                                    export_dir_path_list.clear();
                                    for cat in global_var_util::update_export_dir_itme_list() {
                                        export_dir_path_list.push(cat);
                                    }

                                }
                                Err(err) => {
                                    // $card_itme.status_err.show();
                                    // $card_itme.status_ok.hide();
                                    // println!(
                                        // "[移除失败] 移除的文件夹是: 名称：{}  路径：{}",
                                        // $card_itme.nameStr,
                                        // $card_itme.path.label()
                                    // );
                                    $card_itme.remove = true;
                                    // handle_dat::push_console_message(format!(
                                    //     "[移除失败] 移除的文件夹是: 名称：{}  路径：{}",
                                    //     $card_itme.nameStr,
                                    //     $card_itme.path.label()
                                    // ));
                                    console_log!(format!(
                                        "[移除失败] 移除的文件夹是: 名称：{}  路径：{}",
                                        $card_itme.nameStr,
                                        $card_itme.path.label()
                                    ));

                                }
                            }
            
                            let _ = conn.close();

                            
                        }
                    };
                }
                
                handle_point_card!(point_exist_hasmap.rm_card_01, card_01);
                handle_point_card!(point_exist_hasmap.rm_card_02, card_02);
                handle_point_card!(point_exist_hasmap.rm_card_03, card_03);
            
                if (point_exist_hasmap.next){
                    let mut start_index = 0;
                  
                    let mut end_index = offset+3;

                    // 偏移量不能超出数组
                    if(end_index >= export_dir_path_list.len()){
                        end_index = export_dir_path_list.len();
                    }
                    start_index = offset;
                    offset= end_index ;

                    if(start_index==end_index){
                        start_index= end_index-3;
                        if(start_index>9999999){
                            start_index=0;
                        }
                    }
                    let mut index = 0;
                    loop{
                        let mut indexof = start_index+index;
                        if(indexof>export_dir_path_list.len()-1){
                            break;
                        }

                        push_card_itme!(index+1,indexof);

                        if(index>=3){
                            card_03.main.clone().show();
                            break;
                        }
                        
                        index = index+1;
                       
                    }

                    println!("next-> {} -> {}",start_index,end_index);

                }
                if (point_exist_hasmap.backup){
                    // let start_index = if offset >= 3 { offset - 3 } else { 0 };
                    // let end_index = if offset + 3 < get_item.len() { offset + 3 } else { get_item.len() };
                
                    let mut start_index = 0;
            
                    
                    let mut end_index = offset-3;
                    
                    // 这里有个usize溢出问题 负数会巨大
                    if(end_index<999999){
                        end_index = 3;
                    }
                    
                    // end 不低于三个 除非没有那么多
                    if(end_index<3){
                        end_index=if export_dir_path_list.len()<=3 {export_dir_path_list.len()} else {3};
                    }

                    // 偏移量不能超出数组
                    if(end_index >= export_dir_path_list.len()){
                        end_index = export_dir_path_list.len();
                    }

                    if(end_index-start_index>3){
                        start_index= start_index-3;
                    }

                    if(start_index<999999){
                        start_index=0;
                    }
                    
                    let mut index = 0;
                    loop{
                        let mut indexof = start_index+index;
                            if(indexof>export_dir_path_list.len()-1){
                            break;
                        }
                        push_card_itme!(index+1,indexof);
                        if(index>=3){
                            card_03.main.clone().show();
                            break;
                        }
                        index = index+1;
                    }
                    
                    offset= end_index ;

                    println!("backup-> {} -> {}",start_index,end_index);

                }

                true
            }

            enums::Event::Move => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;
                point_exist_hasmap = getFormPointSpace(x, y);
                // -处理鼠标图标的逻辑

                if (point_exist_hasmap.existCursor) {
                    win.clone().set_cursor(Cursor::Hand);
                } else {
                    win.clone().set_cursor(Cursor::Default);
                }

                true
            }

            // enums::Event:
            enums::Event::Drag => {
                if (y < 69) {
                    win.clone()
                        .set_pos(app::event_x_root() - x, app::event_y_root() - y);
                }

                true
            }
            _ => false,
        }
    });

    // libWxIkunPlus::setWinIcon(win.raw_handle() as i128);
    // libWxIkunPlus::setWinTop(win.raw_handle() as i128, true);
}
#![allow(warnings, unused)]

use crate::{
    get_arc_bind_variable, global_var, gui_detect_config, gui_drag_scan, gui_hotspot, gui_imge,
    gui_text_control, handle_dat, libWxIkunPlus::{self, getFocusTopWindow}, read_rw_lazy_lock, read_rw_lock,
    set_arc_bind_variable, set_arc_bind_variable_insert,
    util::{str_eq_str, Sleep},
    wh_mod::{self, AttachThumbnail},
    write_rw_lock, write_rw_lock_insert,
};
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
use once_cell::sync::Lazy;
use rusqlite::Connection;
use std::sync::{mpsc, MutexGuard};
use std::time::{SystemTime, UNIX_EPOCH};

use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::ops::Deref;
use std::ptr::null;
use std::sync::{Arc, Condvar, Mutex, RwLock};
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
use arc_swap::ArcSwap;
use fltk::draw::{height, width};
use fltk::image::PngImage;
// use lazy_static::lazy_static;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::OnceLock;
use winapi::um::winnt::LPWSTR;
use winapi::um::winuser::{CloseWindow, SetActiveWindow};

static REQUEST_RECV: AtomicUsize = AtomicUsize::new(0);
// static mut static_var:Vec<ImgPreview> = Vec::new();
// static mut static_atomic :AtomicUsize = AtomicUsize::new(0);

// 使用原子锁代替线程锁 避免死锁

// 图片预览 全局变量
static mut IMG_PREVIEW_LIST: Vec<ImgPreview> = Vec::new();
static IMG_PREVIEW_LIST_BIND: AtomicUsize = AtomicUsize::new(0);

// 缩略图 全局变量
static mut THUMBNAIL_LIST: Vec<wh_mod::AttachThumbnail> = Vec::new();
static THUMBNAIL_LIST_BIND: AtomicUsize = AtomicUsize::new(0);

// wxid 全局变量
static mut WX_ID: String = String::new();
static WX_ID_BIND: AtomicUsize = AtomicUsize::new(0);

// 用户wx存储位置 全局变量
static mut USER_PATH: String=String::new();
static USER_PATH_BIND: AtomicUsize = AtomicUsize::new(0);

// 使用原子锁代替线程锁 避免死锁
// fn set_IMG_PREVIEW_LIST(value:Vec<ImgPreview>){

//     set_arc_bind_variable!(THUMBNAIL_LIST,THUMBNAIL_LIST_BIND,value);

//     get_arc_bind_variable!(IMG_PREVIEW_LIST,IMG_PREVIEW_LIST_BIND);

// }

// thread_local! {
// // static WX_ID_ARC: RwLock<Arc<String>> = RwLock::new(Arc::new(String::new()));
// static USER_PATH_ARC: RwLock<Arc<String>> = RwLock::new(Arc::new(String::new()));
// // static THUMBNAIL_LIST_ARC: RwLock<Arc<Vec<wh_mod::AttachThumbnail>>> = RwLock::new(Arc::new(Vec::new()));
// // static IMG_PREVIEW_LIST_ARC: RwLock<Arc<Vec<ImgPreview>>> = RwLock::new(Arc::new(Vec::new()));


// }

// thread_local! {
//         static IMG_PREVIEW_LIST_ARCLAZY: ArcSwap< Vec<ImgPreview> > = ArcSwap::from_pointee(Vec::new().into());
// }

// 设置背景为图片（主视图）
fn setInterfaceBackgroundImage(appMainWin: &mut window::DoubleWindow) -> Frame {
    let background_image =
        // image::SvgImage::from_data(include_str!("./assets/select_user_base/main.svg"))
        image::PngImage::from_data(include_bytes!("./assets/select_user_base/main.png"))
            .expect("set main icon error");
    let mut frame = Frame::default().with_size(600, 0).center_of(appMainWin);
    frame.set_frame(FrameType::EngravedBox);
    frame.set_image(Some(background_image));

    return frame;
}

// 按钮基本都是以坐标判断的
struct PointExistHasmap {
    // 显示手型
    existCursor: bool,
}

// 判断鼠标坐标是否在此元素内
fn getFormPointSpace(x: i32, y: i32) -> PointExistHasmap {
    // 输入: x,y,w,y
    macro_rules! check_point_in_space {
        ($xx:expr, $yy:expr, $width:expr, $height:expr) => {
            x > $xx && x < $xx + $width && y > $yy && y < $yy + $height
        };
    }
    let existCursor = false;
    // IMG_PREVIEW_LIST_KEEP;
    let mut point_exist_hasmap = PointExistHasmap { existCursor: false };

    point_exist_hasmap.existCursor = existCursor;

    return point_exist_hasmap;
}

pub struct SelectUserBaseMain {
    // 选择的路径
    pub select_path: Vec<String>,
    // pub win: DoubleWindow,
    pub thumbnail: bool,
    // 是否导出原图
    pub source: bool,
    // 是否仅限本月
    pub the_month: bool,
    // 数据库位置
    pub database: String,
}

impl Clone for SelectUserBaseMain {
    fn clone(&self) -> Self {
        SelectUserBaseMain {
            select_path: self.select_path.clone(),
            // win:self.win.clone(),
            thumbnail: self.thumbnail.clone(),
            source: self.source.clone(),
            the_month: self.the_month.clone(),
            database: self.database.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}

struct UpdateJpgItme {
    index: usize,
    jpg: image::JpegImage,
    path: String,
}

struct PicturePreviewItem {
    // pub main_id:String,
    pub picture_id: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl PicturePreviewItem {
    pub fn get_picture(&self) -> Frame {
        let mut frame: Frame = app::widget_from_id(self.picture_id.as_str()).unwrap();
        frame
    }

    // 判断鼠标是否在当前元素
    pub fn existPoint(&self, x: i32, y: i32) -> bool {
        return x > self.x.clone()
            && x < self.x.clone() + self.width.clone()
            && y > self.y.clone()
            && y < self.y.clone() + self.height.clone();
    }
}

fn get_next_id() -> usize {
    let mut id: usize = 0;
    let mutex = Arc::new(Mutex::new(&REQUEST_RECV));
    mutex.lock();
    id = REQUEST_RECV.fetch_add(1, Ordering::Relaxed);
    drop(mutex);
    println!("id-> {}", id);
    id
}

fn push_wx_user_table(select_path: String, user_name: String) {
    // let mut lazy_value = USER_PATH.lock().unwrap();

    // if lazy_value.contains(select_path.as_str()) {
    //     return;
    // }

    // *lazy_value = select_path.clone();
    // drop(lazy_value);
    // if read_rw_lock!(USER_PATH_ARC, String::new()).contains(select_path.as_str()) {
    //     return;
    // }
    if(get_arc_bind_variable!(USER_PATH,USER_PATH_BIND).contains(&select_path)){
        return;
    }
    // write_rw_lock!(USER_PATH_ARC, select_path.clone());
    set_arc_bind_variable!(USER_PATH,USER_PATH_BIND,select_path.clone());

    
    thread::spawn(|| {
        let conn: Connection = Connection::open("ikun_user_data.db").unwrap();
        handle_dat::initialize_table(&conn);
        match conn.execute(
            "DELETE FROM user_wx_root_history WHERE path = ?1",
            [select_path.clone()],
        ) {
            Ok(updated) => {}
            Err(err) => {}
        }

        match conn.execute(
            "INSERT INTO user_wx_root_history (time,path,name) values (?1, ?2, ?3)",
            [
                Local::now().format("%Y-%m-%d").to_string(),
                select_path,
                user_name,
            ],
        ) {
            Ok(_) => {}
            Err(err) => {}
        }

        conn.close();
    });
}
struct UserWxRootHistory {
    time: String,
    path: String,
    name: String,
}
fn get_wx_user_history_path() -> Result<UserWxRootHistory, rusqlite::Error> {
    let mut user_wx_root_history = UserWxRootHistory {
        time: "".to_string(),
        path: "".to_string(),
        name: "".to_string(),
    };

    let conn: Connection = Connection::open("ikun_user_data.db").unwrap();
    handle_dat::initialize_table(&conn);
    if let Ok(mut stmt) =
        conn.prepare("SELECT time,name,path  FROM user_wx_root_history ORDER BY time DESC LIMIT 1")
    {
        let cats = stmt.query_map([], |row| {
            Ok(UserWxRootHistory {
                time: row.get(0)?,
                name: row.get(1)?,
                path: row.get(2)?,
            })
        })?;

        for cat in cats {
            let cat = cat?;
            user_wx_root_history.path = cat.path;
            user_wx_root_history.name = cat.name;
            user_wx_root_history.time = cat.time;
        }
    }

    conn.close();
    Ok(user_wx_root_history)
}

fn update_preview_main() {
    // println!(
    //     "[preview_main] IMG_PREVIEW_LIST_ARC-> {}",
    //     read_rw_lock!(IMG_PREVIEW_LIST_ARC, Vec::new()).len()
    // );
    // // println!(
    // //     "[preview_main] IMG_PREVIEW_LIST_ARCSWAP-> {}",
    // //     IMG_PREVIEW_LIST_ARCLAZY.with().len()
    // // );

    // println!(
    //     "[preview_main] IMG_PREVIEW_LIST_ARCSWAP-> {}",
    //     read_rw_lazy_lock!(IMG_PREVIEW_LIST_ARCLAZY).len()
    // );

    // println!(
    //     "[preview_main]THUMBNAIL_LIST-> {}",
    //     get_arc_bind_variable!(IMG_PREVIEW_LIST,IMG_PREVIEW_LIST_BIND).len()
    // );

    println!(
        "[preview-main]IMG_PREVIEW_LIST-> {}",
        get_arc_bind_variable!(IMG_PREVIEW_LIST, IMG_PREVIEW_LIST_BIND).len()
    );
    println!(
        "[preview-main]THUMBNAIL_LIST-> {}",
        get_arc_bind_variable!(THUMBNAIL_LIST, THUMBNAIL_LIST_BIND).len()
    );

    // 取出缩略图列表 并将其缩减到5条以内
    let mut thumbnail_list = {
        // let mut thumbnail_list = read_rw_lock!(THUMBNAIL_LIST_ARC, Vec::new()).to_vec();
        let mut thumbnail_list =
            get_arc_bind_variable!(THUMBNAIL_LIST, THUMBNAIL_LIST_BIND).to_vec();
        let mut atid_list: HashMap<String, AttachThumbnail> = HashMap::new();

        for value in thumbnail_list {
            let key = value.attach_id.clone();
            let mut oid_created = UNIX_EPOCH;
            let mut new_created = UNIX_EPOCH;

            // oid create time
            if let Some(thumbnail) = atid_list.get(&key) {
                if let Ok(metadata) = fs::metadata(thumbnail.thumbnail_path.clone()) {
                    if let Result::Ok(create) = metadata.created() {
                        oid_created = create;
                    }
                }
            }

            // new create time
            if let Ok(metadata) = fs::metadata(value.thumbnail_path.clone()) {
                if let Result::Ok(create) = metadata.created() {
                    new_created = create;
                }
            }

            // 按照创建时间判断是否更新视图
            if (new_created > oid_created) {
                atid_list.insert(value.attach_id.clone(), value);
            }
        }

        println!("atid_list size -> {}", atid_list.len());

        let mut thumbnail_list: Vec<AttachThumbnail> = Vec::new();

        for (key, value) in atid_list {
            thumbnail_list.push(value);
        }

        thumbnail_list.sort_by(|a, b| {
            let mut a_created = UNIX_EPOCH;
            let mut b_created = UNIX_EPOCH;

            if let Ok(metadata) = fs::metadata(a.thumbnail_path.clone()) {
                if let Result::Ok(create) = metadata.created() {
                    a_created = create;
                }
            }

            if let Ok(metadata) = fs::metadata(b.thumbnail_path.clone()) {
                if let Result::Ok(create) = metadata.created() {
                    b_created = create;
                }
            }

            a_created.cmp(&b_created)
        });

        let mut new_thumbnail_list = Vec::new();

        thumbnail_list.reverse();
        for value in thumbnail_list {
            if (new_thumbnail_list.len() > 5 - 1) {
                break;
            }
            new_thumbnail_list.push(value);
        }

        new_thumbnail_list
    };

    // println!("[328] thumbnail_list-> {}",thumbnail_list.len());

    // write_rw_lock!(THUMBNAIL_LIST_ARC, thumbnail_list.to_vec());
    set_arc_bind_variable!(THUMBNAIL_LIST, THUMBNAIL_LIST_BIND, thumbnail_list.to_vec());

    // println!(
    //     "[preview_main_end] IMG_PREVIEW_LIST_ARC-> {}",
    //     read_rw_lock!(IMG_PREVIEW_LIST_ARC, Vec::new()).len()
    // );
    // println!(
    //     "[preview_main_end] IMG_PREVIEW_LIST_ARCSWAP-> {}",
    //     read_rw_lazy_lock!(IMG_PREVIEW_LIST_ARCLAZY).len()
    // );
    // println!(
    //     "[preview_main_end]THUMBNAIL_LIST-> {}",
    //     get_arc_bind_variable!(IMG_PREVIEW_LIST,IMG_PREVIEW_LIST_BIND).len()
    // );

    // 更新到视图中
    let thumbnail_list = get_arc_bind_variable!(THUMBNAIL_LIST, THUMBNAIL_LIST_BIND);

    // 锁定缩略图更新
    let mutex = Arc::new(Mutex::new(&THUMBNAIL_LIST_BIND));
    mutex.lock();

    let img_preview_list = get_arc_bind_variable!(IMG_PREVIEW_LIST, IMG_PREVIEW_LIST_BIND);

    let (width, height) = (75, 75);

    for index in 0..img_preview_list.len() {
        if let Some(mut img_preview) = img_preview_list.get(index) {
            if let Some(thumbnail) = thumbnail_list.get(index) {
                img_preview.clone().from_data(
                    thumbnail.thumbnail.clone(),
                    -1,
                    -1,
                    width - 2,
                    height - 2,
                );
            } else {
            }
        }
    }
    drop(mutex);
}

// 开始获取更新
fn initialize_watch_path_puppet(path: String) {
    let mut frame: Frame = app::widget_from_id("watch_path_user_name").unwrap();
    let mut btn_next: Frame = app::widget_from_id("gui::btn_text_recent_pictures").unwrap();
    let copy_path = path.clone();
    let watch_path = path.clone();
    // println!("initialize_watch_path_puppet-> {}",path.clone());

    let path_list = wh_mod::sort_modified_dir(path.clone().as_ref());
    let mut has_wx = false;
    let mut match_wxid_len = 0;
    let mut wxid = String::new();
    for path in path_list.clone() {
        if (path.to_string_lossy().contains("wxid_")) {
            match_wxid_len = match_wxid_len + 1;
        }

        if (path.to_string_lossy().contains("wxid_")
            || path.to_string_lossy().contains("Applet")
            || path.to_string_lossy().contains("All Users"))
        {
            has_wx = true;
        }
    }

    if (!has_wx) {
        dialog::alert_default("此路径不是有效的wx的根目录");
        return;
    }

    if (match_wxid_len >= 1) {
        for for_path in path_list.clone() {
            let file_name = for_path
                .file_name()
                .unwrap_or_else(|| "".as_ref())
                .to_string_lossy()
                .to_string();

            if file_name.contains("wxid_") {
                wxid = file_name.clone();
                // let mut lazy_value = WX_ID.lock().unwrap();
                // *lazy_value = file_name.clone();
                // drop(lazy_value);

                // write_rw_lock!(WX_ID_ARC, file_name.clone());
                set_arc_bind_variable!(WX_ID,WX_ID_BIND,file_name.clone());

                // let get_wxid_acc = wh_mod::convert::get_wxid_name(format!("{}",for_path.to_str()),wxid.clone());
                if let Some(get_wxid_acc) = wh_mod::convert::get_wxid_name(
                    format!("{}", for_path.to_str().unwrap_or_else(|| &"")),
                    wxid.clone(),
                ) {
                    frame.set_label(&format!("{}  [ {} ]", file_name, get_wxid_acc.name));
                } else {
                    frame.set_label(
                        format!("{}  [ {} ]", file_name, wh_mod::wx_account_id(for_path).id)
                            .as_str(),
                    );
                };
                // 显示到ui
                // frame.set_label(
                //     format!("{}  [ {} ]", file_name,get_wxid_acc /*wh_mod::wx_account_id(for_path).id).as_str()*/,
                // );
                frame.redraw();
                btn_next.set_label("检测");
                // btn_next.redraw();
                // let attach_path = Path::new(copy_path.as_str());
                push_wx_user_table(path.clone(), file_name);
                global_var::set_string("user::config::user_select_wxid", wxid.clone());

                let copy_path = format!("{}/{}/FileStorage/MsgAttach", copy_path.as_str(), wxid);
                let copy_path_wake = format!("{}", watch_path);

                // 取得缩略图
                thread::spawn(move || {
                    // 扫描最近文件夹
                    let path = Path::new(copy_path.as_str());
                    let imag = wh_mod::read_attach_buff_thumbnail_list(path, 5, 1);

                    let mut data_list = Vec::new();

                    //
                    for imag in imag {
                        println!("{}", imag.thumbnail_path.clone());
                        data_list.push(imag);
                    }
                    // *lazy_value = data_list.clone();
                    // drop(lazy_value);

                    // write_rw_lock_insert!(THUMBNAIL_LIST_ARC, data_list.to_vec());
                    // let mut oid_thumbnail_list = get_arc_bind_variable!(THUMBNAIL_LIST,THUMBNAIL_LIST_BIND).to_vec();

                    // for value in data_list {
                    //     oid_thumbnail_list.push(value);
                    // }

                    set_arc_bind_variable_insert!(
                        THUMBNAIL_LIST,
                        THUMBNAIL_LIST_BIND,
                        data_list.to_vec()
                    );

                    // println!("read_rw_lock!(THUMBNAIL_LIST_ARC).unwrap().len() ->  {}",read_rw_lock!(THUMBNAIL_LIST_ARC).unwrap().len());

                    if (data_list.len() > 0) {
                        update_preview_main();
                    }

                    // 释放 并已更新

                    // 启动日志检测模式
                    let (tx, rx) = std::sync::mpsc::channel();

                    let wh_id = wh_mod::watch_path::watch_path_puppet(copy_path_wake.clone(), tx);
                    println!("copy_path_wake-> {}", copy_path_wake.clone());
                    while wh_id == wh_mod::watch_path::get_the_id() {
                        if let Result::Ok(data) = rx.recv() {
                            let path = data.join("..").join("..").join("..");
                            let data_list = wh_mod::read_attach_buff_thumbnail_data(&path, 1);
                            // write_rw_lock_insert!(THUMBNAIL_LIST_ARC, data_list.to_vec());
                            set_arc_bind_variable_insert!(
                                THUMBNAIL_LIST,
                                THUMBNAIL_LIST_BIND,
                                data_list.to_vec()
                            );

                            if (data_list.len() > 0) {
                                update_preview_main();
                            }
                        }
                    }
                    // wh_mod::watch_path
                });

                break;
            }
        }
    } else {
        frame.set_label("存在多个用户 请手动发送图片确认...");
        frame.redraw();
        btn_next.set_label("刷新");
        btn_next.redraw();
    }

    // frame.set_label("开始扫描中...");
}

fn wx_ready_initialize_open_preview_main_up(path: String) {
    let path_list = wh_mod::sort_modified_dir(path.clone().as_ref());

    for path in path_list {
        println!("{:?}", path);
        // let path_list = sort_modified_dir(path.as_ref());
        //
        // for path in path_list {
        //
        // }
    }
}

// 创建选择的窗口
pub fn mian_window() -> SelectUserBaseMain {
    if (global_var::get_bool_default("gui::open::handle_dat")) {
        if let Some(mut wins) =
            app::widget_from_id("gui::DoubleWindow::handle_dat::main") as Option<DoubleWindow>
        {
            wins.show();
            wins.set_visible_focus();
        }
        let mut select_user_base: SelectUserBaseMain = SelectUserBaseMain {
            // win:DoubleWindow::new(0, 0, 600, 531, "Ikun导出"),
            select_path: Vec::new(),
            thumbnail: false,
            source: false,
            the_month: false,
            database: String::new(),
        };

        return select_user_base;
    }

    global_var::set_bool("gui::open::handle_dat", true);

    // 设置主题
    let theme = ColorTheme::new(color_themes::BLACK_THEME);
    let widget_theme = WidgetTheme::new(ThemeType::HighContrast);
    widget_theme.apply();
    let widget_scheme = WidgetScheme::new(SchemeType::Aqua);
    widget_scheme.apply();
    theme.apply();

    let mut select_user_base: SelectUserBaseMain = SelectUserBaseMain {
        // win:DoubleWindow::new(0, 0, 600, 531, "Ikun导出"),
        select_path: Vec::new(),
        thumbnail: false,
        source: false,
        the_month: false,
        database: String::new(),
    };

    let mut win: DoubleWindow = DoubleWindow::new(0, 0, 600, 470, "任务创建向导").center_screen();
    win.set_color(Color::from_rgb(24, 24, 24));
    win.set_border(false);
    win.set_id("gui::DoubleWindow::handle_dat::main");

    // fltk::app::set_scrollbar_size(3);
    // fltk::app::set_selection_color(24, 24, 24);
    setInterfaceBackgroundImage(&mut win);
    // 背景已经绘制

    let mut btn_exit_imag = gui_imge::create_Images(
        &mut win,
        540,
        15,
        37,
        37,
        image::PngImage::from_data(include_bytes!("./assets/select_user_base/exit.png"))
            .expect("set exit_imag_btn icon error"),
        "btn_exit_select".to_owned(),
    );

    let mut flex = group::Flex::default()
        .with_size(395 + 60, 30)
        .row()
        .center_of_parent();

    flex.set_pos(75, 396);

    macro_rules! show_cursor {
        ($show_cursor_itme:expr) => {
            let mut new_win = win.clone();
            $show_cursor_itme.handle({
                move |win, ev| match ev {
                    enums::Event::Move => {
                        new_win.set_cursor(Cursor::Hand);
                        true
                    }
                    enums::Event::Leave => {
                        new_win.set_cursor(Cursor::Default);
                        true
                    }
                    _ => false,
                }
            });
        };
    }

    let mut check_button_thumbnail = button::CheckButton::default().with_label("保存缩略图");
    let mut check_button_source = button::CheckButton::default().with_label("保存原图(如果有)");
    let mut check_button_the_month = button::CheckButton::default().with_label("只保存本月图片");

    check_button_thumbnail.set_callback(|win| {
        global_var::set_bool("user::config::check_button_thumbnail", win.is_checked());
    });

    check_button_source.set_callback(|win| {
        global_var::set_bool("user::config::check_button_source", win.is_checked());
    });

    check_button_the_month.set_callback(|win| {
        global_var::set_bool("user::config::check_button_the_month", win.is_checked());
    });

    show_cursor!(check_button_thumbnail);
    show_cursor!(check_button_source);
    show_cursor!(check_button_the_month);

    flex.end();

    let mut user_name = Frame::new(59, 207, 269, 37, "尚未找到更新图片的用户");
    user_name.set_id("watch_path_user_name");
    user_name.set_label_size(12);

    let mut text_fall_pictures = Frame::new(364, 218, 85, 15, "通过拽入获取");
    text_fall_pictures.set_label_size(12);

    let mut text_recent_pictures = Frame::new(495, 218, 30, 15, "扫描");
    text_recent_pictures.set_label_size(12);
    text_recent_pictures.set_id("gui::btn_text_recent_pictures");

    let mut text_title01 = Frame::new(30, 96, 190, 21, "请选择 WX文件的保存位置 *");
    text_title01.set_label_size(12);
    text_title01.set_label_color(Color::from_rgb(105, 105, 105));

    let mut text_title02 = Frame::new(
        6,
        96 + 70,
        490,
        21,
        "选择用户*    [ 涉及用户数据安全   仅限已登录中的用户 并且能够收发的用户 ]",
    );
    text_title02.set_label_size(12);
    text_title02.set_label_color(Color::from_rgb(105, 105, 105));

    let mut text_title03 = Frame::new(
        6 + 50,
        96 + 70 + 90,
        490,
        21,
        "选择最近对象*  （ 如果不存在请随意发送一张的图片给对方 [不能是表情]  更新后约5秒显示 ） ",
    );
    text_title03.set_label_size(12);
    text_title03.set_label_color(Color::from_rgb(105, 105, 105));
    text_title03.set_id("gui::gui_select_user_base::text_title03");

    thread::spawn(move || {
        let mut title = text_title03.clone();
        while global_var::get_bool_default("gui::open::handle_dat") {
            // if let Some (mut title) = app::widget_from_id("gui::gui_select_user_base::text_title03") as  Option<Frame> {
            let data = global_var::get_string_default("user::config::user_select_path");
            let id = global_var::get_i32_default("user::config::select_user_thumbnail_obj");

            if (data.is_empty()) {
                title.set_label("选择最近对象*  （ 如果不存在请随意发送一张的图片给对方 [不能是表情]  更新后约5秒显示 ） ");
            } else {
                title.set_label(
                    format!(
                        "已选定 ：[ {} ] {}  [再次点击取消]",
                        (if id == -2 {
                            "拖拽".to_string()
                        } else {
                            id.to_string()
                        }),
                        data
                    )
                    .as_str(),
                );
            }

            title.resize(6 + 50, 96 + 70 + 90, 490, 21);

            // }
            Sleep(500);
        }
        println!("gui_select_user_base 线程退出");
    });
    // 刷新最近图片
    let mut btn_recent_pictures = gui_hotspot::create_hotspot(480, 204, 62, 39);
    // 通过拖拽确认用户
    let mut btn_fall_pictures_path = gui_hotspot::create_hotspot(334, 203, 125, 42);
    // 打开微信文件所在位置
    let mut btn_open_select_dir = gui_hotspot::create_hotspot(515, 123, 39, 35);
    // 输入文件夹路径
    let mut input_select_dir = input::Input::new(50, 127, 450 - 5, 27, "");
    input_select_dir.set_id("gui::input_select_dir");

    if let Ok(history) = get_wx_user_history_path() {
        let paths = history.path;
        input_select_dir.set_value(paths.as_str());
        global_var::set_string("user::config::input_select_dir", paths);

        // initialize_watch_path_puppet(paths);
    }
    if (input_select_dir.value().is_empty()) {
        if let Some(paths) = wh_mod::convert::get_user_data_path() {
            input_select_dir.set_value(paths.as_str());
            global_var::set_string("user::config::input_select_dir", paths);
        }
    }

    // 输入文件夹路径 的热区
    let mut input_select_dir_hotspot = gui_hotspot::create_hotspot(50, 127, 450 - 5, 27);

    // initialize_preview_main
    let mut preview_main_point_list = [
        [71, 296, 75, 75],
        [167, 296, 75, 75],
        [263, 296, 75, 75],
        [359, 296, 75, 75],
        [455, 296, 75, 75],
    ];
    let mut preview_main_hotspot_01 = gui_hotspot::create_hotspot(76, 303, 73, 73);
    let mut preview_main_hotspot_02 = gui_hotspot::create_hotspot(169, 303, 73, 73);
    let mut preview_main_hotspot_03 = gui_hotspot::create_hotspot(264, 303, 73, 73);
    let mut preview_main_hotspot_04 = gui_hotspot::create_hotspot(355, 303, 73, 73);
    let mut preview_main_hotspot_05 = gui_hotspot::create_hotspot(447, 303, 73, 73);

    let mut preview_main = Vec::new();
    for index in 0..preview_main_point_list.len() {
        let point = preview_main_point_list[index];
        let [x, y, width, height] = point;
        // let id = format!("gui::preview_main::index::{}",index);
        // let idc = id.as_str();
        let mut preview = ImgPreview::new(x, y, width, height, "gui::preview_main::index::");
        // -1,-1,width-2,height-2 每个边 向内缩进1像素 使其有1px描边的效果
        preview.from_data(
            include_bytes!("./assets/select_user_base/not.png").to_vec(),
            -1,
            -1,
            width - 2,
            height - 2,
        );
        preview_main.push(preview);
    }
    let mut preview = ImgPreview::new(455, 296, 75, 75, "gui::preview_main::index::user_select");
    preview.preview.hide();
    preview_main.push(preview);

    println!("preview_main size-> {}", preview_main.len());

    // // 写入到全局变量中
    // write_rw_lock!(IMG_PREVIEW_LIST_ARC, preview_main.to_vec());
    // // write_rw_lock!(IMG_PREVIEW_LIST_ARCLAZY,preview_main.to_vec().into());

    // // *IMG_PREVIEW_LIST_ARCLAZY.write().unwrap() = preview_main.to_vec().into();
    // // IMG_PREVIEW_LIST_ARCLAZY.store(preview_main.to_vec().into());

    set_arc_bind_variable!(
        IMG_PREVIEW_LIST,
        IMG_PREVIEW_LIST_BIND,
        preview_main.to_vec()
    );

    // // get_arc_bind_variable!(IMG_PREVIEW_LIST,IMG_PREVIEW_LIST_BIND);
    // println!(
    //     "[win-main]IMG_PREVIEW_LIST_ARC size-> {}",
    //     read_rw_lock!(IMG_PREVIEW_LIST_ARC, Vec::new()).len()
    // );
    // println!(
    //     "[win-main]IMG_PREVIEW_LIST_ARCSWAP-> {}",
    //     read_rw_lazy_lock!(IMG_PREVIEW_LIST_ARCLAZY).len()
    // );
    // println!(
    //     "[win-main]THUMBNAIL_LIST-> {}",
    //     get_arc_bind_variable!(IMG_PREVIEW_LIST,IMG_PREVIEW_LIST_BIND).len()
    // );

    println!(
        "[win-main]IMG_PREVIEW_LIST-> {}",
        get_arc_bind_variable!(IMG_PREVIEW_LIST, IMG_PREVIEW_LIST_BIND).len()
    );
    println!(
        "[win-main]THUMBNAIL_LIST-> {}",
        get_arc_bind_variable!(THUMBNAIL_LIST, THUMBNAIL_LIST_BIND).len()
    );

    let mut preview_tips = ImgPreview::new(165, 18, 27, 27, "gui::preview_main::next::tips");
    preview_tips.from_data(
        include_bytes!("./assets/select_user_base/tips.png").to_vec(),
        0,
        0,
        27,
        27,
    );

    let mut title_tips =
        gui_text_control::TextControl::new(46, 18, 330, 27, 12, "帮助".to_string(), [84, 84, 84]);
    // end

    win.end();
    win.show();

    win.handle({
        let mut x = 0;
        let mut y = 0;
        let mut point_exist_hasmap = getFormPointSpace(x, y);
        let mut has_show = false;
        let mut hwnd = 0;
        // let mut move_
        move |win, ev| match ev {
            enums::Event::Show => {
                win.set_visible_focus();
                hwnd = win.raw_handle() as i128;
                libWxIkunPlus::setwinVisible(hwnd.clone(), true);
                println!("hwnd -> :  {}", &hwnd);
                true
            }

            enums::Event::Push => {
                // 处理 check 组件
                macro_rules! check_select_click {
                    ($check_select:expr) => {
                        $check_select.existPoint(x, y)
                        // check_select_source.set_value(global_var);
                    };
                }

                //  打开文件夹选择器
                if check_select_click!(btn_open_select_dir) {

                    let the_win = libWxIkunPlus::getFocusWindow();
                    
                    let user_select_path = libWxIkunPlus::openSelectFolder2();


                    if user_select_path.len() > 1 {
                        initialize_watch_path_puppet(user_select_path.clone());
                        user_name.set_label("开始扫描...");
                        input_select_dir
                            .clone()
                            .set_value(user_select_path.clone().as_str());
                        global_var::set_string(
                            "user::config::input_select_dir",
                            user_select_path.clone(),
                        );
                    }
                }

                if btn_recent_pictures.existPoint(x, y) {}

                // 刷新按钮
                if btn_recent_pictures.existPoint(x, y) {
                    if text_recent_pictures.label().contains("检测") {
                        if (check_button_the_month.is_checked()
                            || check_button_source.is_checked()
                            || check_button_thumbnail.is_checked())
                            && global_var::get_i32_or("user::config::select_user_thumbnail_obj",-1) != -1
                        {
                        } else {
                            gui_detect_config::main_window();
                            // title_tips.set_label("帮助:[未选存储内容]".to_string());
                        }
                    }
                }

                // 通过拖拽获取
                if (btn_fall_pictures_path.existPoint(x, y)) {
                    let value = input_select_dir.value();

                    if (value.len() > 3) {
                        match fs::metadata(Path::new(value.as_str())) {
                            Ok(meta) => {
                                user_name.set_label("开始扫描...");
                                initialize_watch_path_puppet(value.clone());
                            }
                            Err(err) => {
                                let choice = dialog::alert_default(&*format!(
                                    "输入的路径存在错误 错误-> {:?}",
                                    err
                                ));
                            }
                        }
                        gui_drag_scan::main_window();
                    } else {
                        gui_detect_config::main_window();
                        let choice = dialog::alert_default(&*format!(
                            "输入的路径存在错误 错误-> 没有选择WX路径"
                        ));
                    }
                }

                // 选择 或者刷新
                if (btn_recent_pictures.existPoint(x, y)
                    && !text_recent_pictures.label().contains("检测"))
                {
                    let value = input_select_dir.value();
                    global_var::set_string("user::config::input_select_dir", value.clone());
                    if (value.len() > 3) {
                        match fs::metadata(Path::new(value.as_str())) {
                            Ok(meta) => {
                                user_name.set_label("开始扫描...");
                                initialize_watch_path_puppet(value.clone());
                            }
                            Err(err) => {
                                let choice = dialog::alert_default(&*format!(
                                    "输入的路径存在错误 错误-> {:?}",
                                    err
                                ));
                            }
                        }
                    } else {
                        let choice = dialog::alert_default(&*format!("输入的路径过短或者不存在"));
                    }
                }

                // 关闭
                if (btn_exit_imag.existPoint(x, y)) {
                    // libWxIkunPlus::closeWindow(win.raw_handle() as i128, true);
                    // write_rw_lock!(THUMBNAIL_LIST_ARC, Vec::new());
                    // write_rw_lock!(IMG_PREVIEW_LIST_ARC,Vec::new());
                    set_arc_bind_variable!(THUMBNAIL_LIST, THUMBNAIL_LIST_BIND, Vec::new());
                    set_arc_bind_variable!(IMG_PREVIEW_LIST, IMG_PREVIEW_LIST_BIND, Vec::new());

                    // 终止更新检测
                    wh_mod::watch_path::un_next_exits();
                    global_var::set_i32("user::config::select_user_thumbnail_obj", -1);
                    global_var::set_string("user::config::user_select_path", String::new());
                    global_var::set_string("user::config::user_select_wxid", String::new());
                    global_var::set_bool("gui::open::handle_dat", false);
                    fltk::window::Window::delete(win.clone());

                }

                if preview_tips.existPoint(x, y) {
                    gui_detect_config::main_window();
                }

                macro_rules! select_user_preview {
                    ($select_user_preview:expr,$id:expr) => {
                        if ($select_user_preview.existPoint(x, y)) {
                            let select_id =
                                global_var::get_i32_or("user::config::select_user_thumbnail_obj",-1);

                            if (select_id == $id) {
                                global_var::set_string(
                                    "user::config::user_select_path",
                                    "".to_string(),
                                );
                                global_var::set_i32("user::config::select_user_thumbnail_obj", -1);
                            }

                            if (select_id != $id) {
                                let mut str_path = String::new();
                                let thumbnail_list = get_arc_bind_variable!(THUMBNAIL_LIST,THUMBNAIL_LIST_BIND);
                                
                                if let Some(item) = thumbnail_list.get($id - 1) {
                                    str_path = item.attach_id.clone();

                                    println!("[select_user_preview] -> {} [{}] ",&str_path,&$id);

                                    global_var::set_string("user::config::user_select_path", str_path);
                                    global_var::set_i32("user::config::select_user_thumbnail_obj", $id);
                                }
                            }
                        }
                    };
                }
                // app::grab().expect("msg")
                select_user_preview!(preview_main_hotspot_01, 1);
                select_user_preview!(preview_main_hotspot_02, 2);
                select_user_preview!(preview_main_hotspot_03, 3);
                select_user_preview!(preview_main_hotspot_04, 4);
                select_user_preview!(preview_main_hotspot_05, 5);

                true
            }

            enums::Event::Move => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;

                macro_rules! move_show_cursor {
                    ($check_select:expr) => {
                        $check_select.existPoint(x, y)
                    };
                }

                if move_show_cursor!(btn_exit_imag)
                    || move_show_cursor!(preview_main_hotspot_01)
                    || move_show_cursor!(preview_main_hotspot_02)
                    || move_show_cursor!(preview_main_hotspot_03)
                    || move_show_cursor!(preview_main_hotspot_04)
                    || move_show_cursor!(preview_main_hotspot_05)
                    || move_show_cursor!(btn_recent_pictures)
                    || move_show_cursor!(btn_fall_pictures_path)
                    || move_show_cursor!(btn_open_select_dir)
                    || preview_tips.existPoint(x, y)
                {
                    win.clone().set_cursor(Cursor::Hand);
                } else {
                    win.clone().set_cursor(Cursor::Default);
                }

                true
            }

            // enums::Event:
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

    win.set_visible_focus();

    select_user_base
}
use fltk::enums::Color;
use fltk::frame;
use fltk::prelude::{WidgetBase, WidgetExt};

pub struct TextControl {
    pub text: frame::Frame,
}

impl TextControl {
    // 创建
    pub fn new(x:i32, y:i32, width: i32, height: i32, size:i32, input:String, rgb: [u8; 3]) -> Self {
        let mut text = frame::Frame::new(x, y, (if width==0 {input.len()as i32 * size}else {width}), (if height==0 {size+2}else {height}), "");
        text.set_label(input.as_str());
        text.set_label_size(size);
        text.set_label_color(Color::from_rgb(rgb[0],rgb[1],rgb[2]));

        Self { text  }
    }
    // 获取文本内容
    pub fn get_label(&self)->String {
        return self.text.label();
    }
    // 设置文本内容
    pub fn set_label(&mut self,input:String){
        self.text.set_label(input.as_str());
        self.text.redraw_label();
        self.text.redraw();
    }
    // 设置颜色
    pub fn set_color(&mut self, r: u8, g: u8, b: u8)  {
        self.text.set_label_color(Color::from_rgb(r,g,b));

    }
    // 判断鼠标是否在当前元素
    pub fn existPoint(&self, x: i32, y: i32) -> bool {
        return x > self.text.x()
            && x < self.text.x() + self.text.width()
            && y > self.text.y()
            && y < self.text.y() + self.text.height();
    }

}
impl Clone for TextControl {
    fn clone(&self) -> Self {
        TextControl {
            text:self.text.clone()
        }
    }
}
#![allow(warnings, unused)]

use fltk::enums::FrameType;
use fltk::prelude::{WidgetBase, WidgetExt};

pub struct HotspotItmeControl {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}
impl HotspotItmeControl {
    pub(crate) fn clone(&self) -> Self {
        HotspotItmeControl{
            x: self.x.clone(),
            y: self.y.clone(),
            width: self.width.clone(),
            height: self.height.clone(),
        }
    }
}
impl HotspotItmeControl {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        let mut check_item_control = HotspotItmeControl {
            x,
            y,
            width,
            height,
        };

        check_item_control
    }

    // 判断鼠标是否在当前元素
    pub fn existPoint(&self, x: i32, y: i32) -> bool {
        return x > self.x.clone()
            && x < self.x.clone() + self.width.clone()
            && y > self.y.clone()
            && y < self.y.clone() + self.height.clone();
    }

    pub fn set_callback (&mut self, cb: fn()){
        let mut main = fltk::frame::Frame::new(self.x,self.y,self.width,self.height, "");
        main.set_frame(FrameType::NoBox);
        main.handle(move |txt, event| match event {
            fltk::enums::Event::Push=>{
                cb();
                true
            }
           fltk::enums::Event::Move=>{
                // cb();
                true
            }
            fltk::enums::Event::Leave=>{
                // cb();
                true
            }
            _ => false,
        });
    }

}

pub fn create_hotspot(x: i32, y: i32, width: i32, height: i32) -> HotspotItmeControl {
    HotspotItmeControl::new(x, y, width, height)
}
use fltk::{
    app::{self, handle},
    button::{self, Button},
    draw::font,
    enums::{self, Color, Cursor, Event, Font, FrameType, LabelType},
    frame::{self, Frame},
    group::{self, Flex, Group},
    image::{self, Image, PngImage},
    input::{InputType, IntInput},
    prelude::*,
    text::TextDisplay,
    tree,
    window::{self, DoubleWindow, Window},
};
use std::{
    fs,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    process,
    process::{Command, Stdio},
    thread,
};

pub struct ImgPreview {
    pub preview: frame::Frame,
    pub x:i32,
    pub y:i32,
    pub width: i32,
    pub height: i32,
    pub(crate) data: Vec<u8>,
    pub img_type:ImgPreviewDataType,
    pub data_x:i32,
    pub data_y:i32,
    pub data_width: i32,
    pub data_height: i32,
}
pub enum ImgPreviewDataType {
    NoneS,
    Svg,
    Jpeg,
    Png,
    Gif
}

impl Clone for ImgPreview {
    fn clone(&self) -> Self {
        ImgPreview {
            preview:self.preview.clone(),
            x: self.x.clone(),
            y: self.y.clone(),
            width: self.width.clone(),
            height: self.height.clone(),
            data:self.data.clone(),
            img_type: ImgPreviewDataType::Svg,
            data_x: self.data_x.clone(),
            data_y: self.data_y.clone(),
            data_width: self.data_width.clone(),
            data_height: self.data_height.clone(),
        }
    }
}

impl ImgPreview {
    pub fn detect_image_format(data: &[u8]) -> Option<&'static str> {
        if data.len() < 4 {
            return None; // 数据不够长，无法识别
        }

        match &data[0..4] {
            [0x89, 0x50, 0x4E, 0x47] => Some("png"),
            [0xFF, 0xD8, _, _] => Some("jpg"),
            [0x47, 0x49, 0x46, 0x38] => Some("gif"),
            [0x52, 0x49, 0x46, 0x46]
            if data.len() >= 12 && &data[8..12] == [0x57, 0x45, 0x42, 0x50] =>
                {
                    Some("webp")
                }
            _ => None, // 未知格式
        }
    }

    pub fn new(x: i32, y: i32, width: i32, height: i32, id: &'static str) -> Self {
        let mut preview = frame::Frame::new(x, y, width, height, "");
        preview.set_frame(enums::FrameType::FlatBox);
        preview.set_color(enums::Color::from_rgb(80, 80, 80));
        preview.set_id(id);
        Self { preview, x, y, width, height ,data:Vec::new(), img_type: ImgPreviewDataType::NoneS, data_x: 0, data_y: 0, data_width: 0, data_height: 0 }
    }

    pub fn new2(x: i32, y: i32, width: i32, height: i32, id: &'static str, data_x: i32, data_y: i32, data_width: i32, data_height: i32) -> Self {
        let mut preview = frame::Frame::new(x, y, width, height, "");
        preview.set_frame(enums::FrameType::FlatBox);
        preview.set_color(enums::Color::from_rgb(80, 80, 80));
        preview.set_id(id);
        Self { preview, x, y, width, height ,data:Vec::new(), img_type: ImgPreviewDataType::NoneS, data_x , data_y, data_width, data_height}
    }

    pub fn new_border(x: i32, y: i32, width: i32, height: i32,svg_data:&str) -> Self {
        let mut preview = frame::Frame::new(x, y, width, height, "");
        preview.set_frame(enums::FrameType::NoBox);
        preview.set_color(enums::Color::from_rgb(80, 80, 80));
        let mut self_data =  Self { preview, x, y, width, height ,data:svg_data.as_bytes().to_vec(), img_type: ImgPreviewDataType::Svg, data_x:0 , data_y:0, data_width:width, data_height:height};
        self_data.from_svg(svg_data,0,0,width,height);
        self_data
    }

    // 判断鼠标是否在当前元素
    pub fn existPoint(&self, x: i32, y: i32) -> bool {
        return x > self.x.clone()
            && x < self.x.clone() + self.width.clone()
            && y > self.y.clone()
            && y < self.y.clone() + self.height.clone();
    }

    pub fn load(&mut self, path: String, x: i32, y: i32, width: i32, height: i32) -> bool {
        let mut res = false;
        if let Result::Ok(data) = fs::read(path) {
            res = self.from_data(data, x, y, width, height);
        }
        res
    }

    pub fn from_data(&mut self, data: Vec<u8>, x: i32, y: i32, width: i32, height: i32) -> bool {
        let mut res = false;
        macro_rules! re_imag {
            ($imag:expr) => {
                $imag.scale(width, height, false, true);
                self.preview.draw(move |cb| {
                    let cbx: i32 = cb.x();
                    let cby: i32 = cb.y();
                    let cbh: i32 = cb.h();
                    let cbw: i32 = cb.w();
                    let cx: i32 = x;
                    let cy: i32 = y;
                    $imag.draw_ext(cbx, cby, cbw, cbh, cx, cy);
                });
                self.data_height = height;
                self.data_width = width;
                self.data_x = x;
                self.data_y = y;
                self.preview.redraw();
                self.preview.redraw_label();
                res = true;
            };
        }

        if let Some(ext) = ImgPreview::detect_image_format(&data) {
            self.data = data.to_vec();

            if (ext.as_bytes().eq("png".as_bytes())) {
                if let Result::Ok(mut img) = image::PngImage::from_data(&*data) {
                    re_imag!(img);
                    self.img_type = ImgPreviewDataType::Png
                }
            } else if (ext.as_bytes().eq("jpg".as_bytes())) {
                if let Result::Ok(mut img) = image::JpegImage::from_data(&*data) {
                    self.img_type = ImgPreviewDataType::Jpeg;
                    re_imag!(img);
                }
            } else if (ext.as_bytes().eq("gif".as_bytes())) {
                if let Result::Ok(mut img) = image::GifImage::from_data(&*data) {
                    self.img_type = ImgPreviewDataType::Gif;
                    re_imag!(img);
                }
            }
        }

        res
    }

    pub fn re_data(&mut self, data: Vec<u8>){
        self.from_data(data,self.data_x,self.data_y,self.data_width,self.data_height);
    }

    pub fn from_svg(&mut self, data: &str, x: i32, y: i32, width: i32, height: i32) -> bool {
        let mut res = false;
        macro_rules! re_imag {
            ($imag:expr) => {
                $imag.scale(width, height, false, true);
                self.preview.draw(move |cb| {
                    let cbx: i32 = cb.x();
                    let cby: i32 = cb.y();
                    let cbh: i32 = cb.h();
                    let cbw: i32 = cb.w();
                    let cx: i32 = x;
                    let cy: i32 = y;
                    $imag.draw_ext(cbx, cby, cbw, cbh, cx, cy);
                });
                self.preview.redraw();
                self.preview.redraw_label();
                res = true;
            };
        }
        if let Result::Ok(mut img) = image::SvgImage::from_data(data) {
            self.img_type = ImgPreviewDataType::Svg;
            self.data = data.as_bytes().to_vec();

            re_imag!(img);
        }
        res
    }

    pub fn get_data (&self) -> Vec<u8> {
        self.data.to_vec()
    }

    pub fn as_mut (&mut self) -> &mut ImgPreview {
         self
    }

}
use chrono::Local;
use rusqlite::Connection;

use crate::{atomic_util, get_arc_bind_variable, global_var, handle_dat, libWxIkunPlus::getFocusTopWindow, read_rw_lazy_lock, read_rw_lock, set_arc_bind_variable, set_arc_bind_variable_insert, set_arc_bind_variable_vec_clear, set_arc_bind_variable_vec_replace_data, util::{str_eq_str, Sleep}, wh_mod::{self, AttachThumbnail}, write_rw_lock, write_rw_lock_insert, gui_util, libWxIkunPlus};

use std::{
    sync::atomic::{AtomicUsize, Ordering},
    sync::{Arc, Condvar, Mutex, RwLock},
    thread, time::UNIX_EPOCH, collections::HashMap, path::{Path, PathBuf},
};
use std::sync::atomic::AtomicBool;
use crate::gui_select_user_ui::{ASSETS_DEMO_DATA, ASSETS_DEMO_NOT_DATA, THE_WINDOW_CLASS_NAME};

static HAS_SELECT_USER_WINDOW_NORMAL: AtomicBool = AtomicBool::new(false);

// 图片预览 全局变量
static mut IMG_PREVIEW_LIST: Vec<gui_util::img::ImgPreview> = Vec::new();
static IMG_PREVIEW_LIST_BIND: AtomicUsize = AtomicUsize::new(0);

// 缩略图 全局变量
static mut THUMBNAIL_LIST: Vec<wh_mod::AttachThumbnail> = Vec::new();
static THUMBNAIL_LIST_BIND: AtomicUsize = AtomicUsize::new(0);


// wxid 全局变量
static mut WX_ID: String = String::new();
static WX_ID_BIND: AtomicUsize = AtomicUsize::new(0);

// 用户wx存储位置 全局变量
static mut USER_PATH: String = String::new();
static USER_PATH_BIND: AtomicUsize = AtomicUsize::new(0);

// 用户列表绑定
static mut ACTIVE_USER_LIST: Vec<wh_mod::convert::WxActiveUser> = Vec::new();
static ACTIVE_USER_LIST_BIND: AtomicUsize = AtomicUsize::new(0);

pub struct UserWxRootHistory {
    pub time: String,
    pub path: String,
    pub name: String,
}

macro_rules! get_the_hwnd {
    ($class_id:expr) => {
        {
        let mut _hwnd = 0 ;
        for _ in 0..8 {
          _hwnd = libWxIkunPlus::findWindow($class_id,"");
            if !libWxIkunPlus::isWindow(_hwnd) {
                 _hwnd=0;
            }else {
              break;
          }
            fltk::app::sleep(0.020);
        }
        _hwnd as i128}
    };
    ()=>{
        get_the_hwnd!(THE_WINDOW_CLASS_NAME)
    }
}

// 从数据库读取历史记录
pub fn get_wx_user_history_path() -> Result<UserWxRootHistory, rusqlite::Error> {
    let mut user_wx_root_history = UserWxRootHistory {
        time: "".to_string(),
        path: "".to_string(),
        name: "".to_string(),
    };

    let conn: Connection = Connection::open("ikun_user_data.db").unwrap();
    handle_dat::initialize_table(&conn);
    if let Ok(mut stmt) =
        conn.prepare("SELECT time,name,path  FROM user_wx_root_history ORDER BY time DESC LIMIT 1")
    {
        let cats = stmt.query_map([], |row| {
            Ok(UserWxRootHistory {
                time: row.get(0)?,
                name: row.get(1)?,
                path: row.get(2)?,
            })
        })?;

        for cat in cats {
            let cat = cat?;
            user_wx_root_history.path = cat.path;
            user_wx_root_history.name = cat.name;
            user_wx_root_history.time = cat.time;
        }
    }

    conn.close();
    Ok(user_wx_root_history)
}

// 保存读取历史
pub fn store_wx_user_path_history(select_path: String, user_name: String) {
    thread::spawn(move || {
        let conn: Connection = Connection::open("ikun_user_data.db").unwrap();
        handle_dat::initialize_table(&conn);

        match conn.execute(
            "DELETE FROM user_wx_root_history WHERE path = ?1",
            [select_path.clone()],
        ) {
            Ok(updated) => {}
            Err(err) => {}
        }

        match conn.execute(
            "INSERT INTO user_wx_root_history (time,path,name) values (?1, ?2, ?3)",
            [
                Local::now().format("%Y-%m-%d").to_string(),
                select_path,
                user_name,
            ],
        ) {
            Ok(_) => {}
            Err(err) => {}
        }

        conn.close();
    });
}

// 保存备注
pub fn set_store_user_remark(wxid: String, attach_id: String, remark_name: String) {
    // thread::spawn(move || {
        let conn: Connection = Connection::open("ikun_user_data.db").unwrap();
        handle_dat::initialize_table(&conn);

        match conn.execute(
            "DELETE FROM user_remark WHERE wxid = ?1 AND attach_id = ?2",
            [wxid.clone(), attach_id.clone()],
        ) {
            Ok(updated) => {
            }
            Err(err) => {}
        }

        match conn.execute(
            "INSERT INTO user_remark (time,wxid,attach_id,remark_name) values (?1, ?2, ?3, ?4)",
            [
                Local::now().format("%Y-%m-%d").to_string(),
                wxid.clone(),
                attach_id.clone(),
                remark_name.clone(),
            ],
        ) {
            Ok(_) => {
                gui_util::message::sub_message(get_the_hwnd!(),gui_util::message::IconType::Success,"当前别名备注已经更新",5000u64);

            }
            Err(err) => {
                gui_util::message::sub_message(get_the_hwnd!(),gui_util::message::IconType::Failure,format!("别名更新失败 因为-> {:?}",err).as_str(),5000u64);

            }
        }

        conn.close();
    // });
}

struct UserRemark {
    time: String,
    wxid: String,
    attach_id: String,
    remark_name: String,
}

// 获取备注
pub fn get_store_user_remark(wxid: String, attach_id: String) -> Option<String> {
    let mut res_data = Option::None;

    let conn: Connection = Connection::open("ikun_user_data.db").unwrap();
    handle_dat::initialize_table(&conn);
    if let Ok(mut stmt) =
        conn.prepare("SELECT time,wxid,attach_id,remark_name  FROM user_remark  WHERE wxid = ?1 AND attach_id = ?2")
    {
        let cats = stmt.query_map([wxid.clone(),attach_id.clone()], |row| {
            let mut row_data = UserRemark {
                time: String::new(),
                wxid: String::new(),
                attach_id: String::new(),
                remark_name: String::new()
             };

             if let Ok(item) = row.get(0) as Result<String,_> {
                row_data.time = item.clone();
             }
             
             if let Ok(item) = row.get(1) as Result<String,_> {
                row_data.wxid = item.clone();
             }
             
             if let Ok(item) = row.get(2) as Result<String,_> {
                row_data.attach_id = item.clone();
             }

             if let Ok(item) = row.get(3) as Result<String,_> {
                row_data.remark_name = item.clone();
             }

            Ok(row_data)
        });

        if let Ok(cats) = cats {
            for cat in cats {
                if let Ok(cat) = cat {
                    res_data.replace(cat.remark_name);
                }
          
        } 
        }
    }

    conn.close();

    res_data
}

// 添加active_user_list到全局变量
pub fn set_active_user_list(active_user_list: Vec<wh_mod::convert::WxActiveUser>) {
    set_arc_bind_variable!(ACTIVE_USER_LIST, ACTIVE_USER_LIST_BIND, active_user_list);
}

// 获取active_user_list到全局变量
pub fn get_active_user_list() -> Vec<wh_mod::convert::WxActiveUser> {
    let active_user_list = get_arc_bind_variable!(ACTIVE_USER_LIST, ACTIVE_USER_LIST_BIND);

    active_user_list.clone()
}

// 添加 active_user_list到全局变量
pub fn push_active_user_list(active_user: wh_mod::convert::WxActiveUser) {
    let mutex = Arc::new(Mutex::new(&ACTIVE_USER_LIST_BIND));
    mutex.lock();
    let the_value: usize = ACTIVE_USER_LIST_BIND.load(Ordering::SeqCst);

    unsafe {
        ACTIVE_USER_LIST.push(active_user);
    }

    ACTIVE_USER_LIST_BIND.store(the_value + 1, Ordering::SeqCst);
    drop(mutex);
}

// 更新进视图
pub fn update_thumbnail_preview_list()  {
        // 取出缩略图列表 并将其缩减到5条以内
        let mut thumbnail_list = {

            let mut thumbnail_list =
                get_arc_bind_variable!(THUMBNAIL_LIST, THUMBNAIL_LIST_BIND).to_vec();
            let mut atid_list: HashMap<String, AttachThumbnail> = HashMap::new();
    
            for value in thumbnail_list {
                let key = value.attach_id.clone();
                let mut oid_created = UNIX_EPOCH;
                let mut new_created = UNIX_EPOCH;
    
                // oid create time
                if let Some(thumbnail) = atid_list.get(&key) {
                    if let Ok(metadata) = std::fs::metadata(thumbnail.thumbnail_path.clone()) {
                        if let Result::Ok(create) = metadata.created() {
                            oid_created = create;
                        }
                    }
                }
    
                // new create time
                if let Ok(metadata) = std::fs::metadata(value.thumbnail_path.clone()) {
                    if let Result::Ok(create) = metadata.created() {
                        new_created = create;
                    }
                }
    
                // 按照创建时间判断是否更新视图
                if (new_created > oid_created) {
                    atid_list.insert(value.attach_id.clone(), value);
                }
            }
    
    
            let mut thumbnail_list: Vec<AttachThumbnail> = Vec::new();
    
            for (key, value) in atid_list {
                thumbnail_list.push(value);
            }
    
            // 排序
            thumbnail_list.sort_by(|a, b| {
                let mut a_created = UNIX_EPOCH;
                let mut b_created = UNIX_EPOCH;
    
                if let Ok(metadata) = std::fs::metadata(a.thumbnail_path.clone()) {
                    if let Result::Ok(create) = metadata.created() {
                        a_created = create;
                    }
                }
    
                if let Ok(metadata) = std::fs::metadata(b.thumbnail_path.clone()) {
                    if let Result::Ok(create) = metadata.created() {
                        b_created = create;
                    }
                }
    
                a_created.cmp(&b_created)
            });
    
            let mut new_thumbnail_list = Vec::new();
    
            thumbnail_list.reverse();
            for value in thumbnail_list {
                if (new_thumbnail_list.len() > 5 - 1) {
                    break;
                }
                new_thumbnail_list.push(value);
            }
    
            new_thumbnail_list
        };

        set_arc_bind_variable!(THUMBNAIL_LIST, THUMBNAIL_LIST_BIND, thumbnail_list.to_vec());
    
    
        // 更新到视图中
        let thumbnail_list = get_arc_bind_variable!(THUMBNAIL_LIST, THUMBNAIL_LIST_BIND);
    
        // 锁定缩略图更新
        let mutex = Arc::new(Mutex::new(&THUMBNAIL_LIST_BIND));
        mutex.lock();
    
        let img_preview_list = get_arc_bind_variable!(IMG_PREVIEW_LIST, IMG_PREVIEW_LIST_BIND);
    
        let (width, height) = (75, 75);

        if thumbnail_list.is_empty(){
            gui_util::message::sub_message(get_the_hwnd!(),gui_util::message::IconType::Warning,"没有发现图片列表 可以找开发者反馈",5000u64);
        }

        // 更新到视图中  
        for index in 0..img_preview_list.len() {
            if let Some(mut img_preview) = img_preview_list.get(index) {

                if let Some(thumbnail) = thumbnail_list.get(index) {
                    let pre: Vec<u8> = if wh_mod::config::is_show_dome() { ASSETS_DEMO_DATA() } else{ thumbnail.thumbnail.clone() };

                    img_preview.clone().from_data(
                        pre,
                        -1,
                        -1,
                        width - 2,
                        height - 2,
                    );
                } else {
                    img_preview.clone().from_data(
                        ASSETS_DEMO_NOT_DATA(),
                        -1,
                        -1,
                        width - 2,
                        height - 2,
                    );
                }
            }
        }

        drop(mutex);

}

// 初始化五张图片到视图
pub fn initialize_thumbnail_preview(user_root: &str,wxid: &str){
    
    let msg_attach_dir = PathBuf::from(format!("{}\\{}\\FileStorage\\MsgAttach",user_root,wxid).as_str());

    println!("msg_attach_dir-> {:?}",msg_attach_dir);

    let mut read_imag_list = wh_mod::read_attach_buff_thumbnail_list_v2(&msg_attach_dir.as_path(), 5, 1);

    // v2 没有内容就是说明这个库可能修改时间有误 尝试用v1 获取
    if read_imag_list.is_empty() {
        read_imag_list = wh_mod::read_attach_buff_thumbnail_list(&msg_attach_dir.as_path(), 5, 1);
    }

    set_arc_bind_variable_vec_replace_data!(THUMBNAIL_LIST,THUMBNAIL_LIST_BIND,read_imag_list);
    update_thumbnail_preview_list();

}

// 开始获取更新
pub fn initialize_watch_path_puppet(path: String) {
    std::thread::spawn(move ||{
        // 启动日志检测模式
        let (tx, rx) = std::sync::mpsc::channel();

        let wh_id = wh_mod::watch_path::watch_path_puppet(path.clone(), tx);
        println!("copy_path_wake-> {}", path.clone());
        while wh_id == wh_mod::watch_path::get_the_id() {
            if let Result::Ok(data) = rx.recv() {
                let path = data.join("..").join("..").join("..");
                let data_list = wh_mod::read_attach_buff_thumbnail_data(&path, 1);
                // write_rw_lock_insert!(THUMBNAIL_LIST_ARC, data_list.to_vec());
                set_arc_bind_variable_insert!(
                                THUMBNAIL_LIST,
                                THUMBNAIL_LIST_BIND,
                                data_list.to_vec()
                            );

                if (data_list.len() > 0) {
                   update_thumbnail_preview_list();
                }
            }
        }

    });
}

macro_rules! gc_select_user_ui {
   ()=>{
        if !atomic_util::get_bool(&HAS_SELECT_USER_WINDOW_NORMAL){
                println!("[gc] initialize_gc_select_user_ui");
                wh_mod::watch_path::un_next_exits();
                global_var::set_string("user::config::user_select_path", String::new());
                global_var::set_string("user::config::user_select_wxid", String::new());
                set_arc_bind_variable_vec_clear!(THUMBNAIL_LIST,THUMBNAIL_LIST_BIND);
                set_arc_bind_variable_vec_clear!(IMG_PREVIEW_LIST,IMG_PREVIEW_LIST_BIND);
                println!("[gc] initialize_gc_select_user_ui ok! ");
                return;
            }
   }
}

// 自动在窗口销毁时候自动清理
pub fn initialize_gc_select_user_ui(hwnd:i128){

    if atomic_util::get_bool(&HAS_SELECT_USER_WINDOW_NORMAL){
        return;
    }

    atomic_util::set_bool(&HAS_SELECT_USER_WINDOW_NORMAL, true);
    
    thread::spawn(move ||{
        loop {
            if !atomic_util::get_bool(&HAS_SELECT_USER_WINDOW_NORMAL)||!libWxIkunPlus::isWindow(hwnd){
                println!("[gc] initialize_gc_select_user_ui");
                atomic_util::set_bool(&HAS_SELECT_USER_WINDOW_NORMAL,false);
                wh_mod::watch_path::un_next_exits();
                global_var::set_string("user::config::user_select_path", String::new());
                global_var::set_string("user::config::user_select_wxid", String::new());
                set_arc_bind_variable_vec_clear!(THUMBNAIL_LIST,THUMBNAIL_LIST_BIND);
                set_arc_bind_variable_vec_clear!(IMG_PREVIEW_LIST,IMG_PREVIEW_LIST_BIND);
                println!("[gc] initialize_gc_select_user_ui ok! ");
                return;
            }
            Sleep(150);
        }
    });
}

// 绑定视图5张的显示控件
pub fn initialize_img_preview_list (img_preview_list:&Vec<gui_util::img::ImgPreview>){
    use std::sync::{Arc, Condvar, Mutex, RwLock};
    let mutex = Arc::new(Mutex::new(&IMG_PREVIEW_LIST_BIND));
    mutex.lock();
    let the_value: usize = IMG_PREVIEW_LIST_BIND.load(Ordering::SeqCst);
    unsafe {
        IMG_PREVIEW_LIST.clear();
        for value in img_preview_list {
            IMG_PREVIEW_LIST.push(value.clone())

        }
    }
    IMG_PREVIEW_LIST_BIND.store(the_value + 1, Ordering::SeqCst);

    drop(mutex);

    // set_arc_bind_variable_vec_replace_data!(IMG_PREVIEW_LIST,IMG_PREVIEW_LIST_BIND,img_preview_list);
}

// GC掉大部分高内存的存储
pub fn gc_select_user_ui(){
    atomic_util::set_bool(&HAS_SELECT_USER_WINDOW_NORMAL,false);
    wh_mod::gc_walk_attach_file_list();
    gc_select_user_ui!();
}

// 获取缩略图绑定列表
pub fn get_thumbnail_list() -> Vec<AttachThumbnail> {
    get_arc_bind_variable!(THUMBNAIL_LIST,THUMBNAIL_LIST_BIND).clone()
}

pub fn bool_to_str (b:bool) -> &'static str {
    if b {"是"} else { "否" }
}
use crate::util::str_eq_ostr;
use crate::{console_log, global_var_util, gui_util, handle_dat, libWxIkunPlus, wh_mod};
use chrono::Local;
use glob::glob;
use rusqlite::{params, Connection, Result};
use serde_json::json;
use serde_json::Value as Json;
use toml::Value as Toml;
use crate::global_var;
use crate::gui_main_ui::THE_WIN_CLASS_NAME;
pub struct AppVersionInfo {}

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

fn toml2json(toml: Toml) -> Json {
    match toml {
        Toml::String(s) => Json::String(s),
        Toml::Integer(i) => Json::Number(i.into()),
        Toml::Float(f) => {
            let n = serde_json::Number::from_f64(f).expect("float infinite and nan not allowed");
            Json::Number(n)
        }
        Toml::Boolean(b) => Json::Bool(b),
        Toml::Array(arr) => Json::Array(arr.into_iter().map(toml2json).collect()),
        Toml::Table(table) => {
            Json::Object(table.into_iter().map(|(k, v)| (k, toml2json(v))).collect())
        }
        Toml::Datetime(dt) => Json::String(dt.to_string()),
    }
}

pub fn get_app_version_info() -> Json {
    const APP_VERSION: &str = include_str!("../../../Cargo.toml");
    // println!("toml2json-> {:?}",toml2json(APP_VERSION));

    match APP_VERSION.parse() {
        Ok(toml) => {
            let json = toml2json(toml);
            return json;
        }
        Err(error) => println!("failed to parse TOML: {}", error),
    }

    json!("")
}

// 获取初始化文本
pub fn get_init_text() -> String {
    let mut result = String::new();
    let mut sync_type = String::new();
    let mut build_name = if wh_mod::config::is_build_52pojie() {
        "52破解专版"
    } else {
        "开源版"
    };
    let version_info = get_app_version_info();
    let version = (version_info["package"]["version"]).as_str().unwrap();

    if !wh_mod::config::is_developer() {
        result.push_str(
            format!(
                r#"作者 @Ikun 软件开源协议 GPL 3.0 (但是并不包含解码算法)
        当前版本：{} ({})
        本软件 是免费软件 如果付费请维权退款
        本软件只供节约自己另存为图片时间，禁止用于其他用途
        "#,
                version, build_name
            )
            .replace("  ", "")
            .as_str(),
        );
    } else {
        result.push_str(("初始化成功 [开发者模式]"));
    }

    if libWxIkunPlus::has_auto_sync() {
        result.push_str(format!("\n[用户] 自动同步开启").as_str());
    } else if wh_mod::config::is_developer() {
        result.push_str("\n[同步] 自动同步已启用 因为开发者模式有效");
    } else {
        result.push_str("\n[同步] 自动同步关闭");
    }

    result
}

// 添加进数据库
pub fn push_sql_export_dir_path(name: &str, export_dir: &str, task_command: &str) {
    if !eq_next() {
        libWxIkunPlus::stop(
            "错误".to_owned(),
            "当前未发现wx进程或者未登录 拒绝提供添加".to_owned(),
        );
        return;
    }
    if name.is_empty() {
        console_log!(format!("\n[错误] 没有名称"));
        gui_util::sub_message(get_the_hwnd!(), gui_util::IconType::Failure, "[错误] 没有名称", 3500u64);
        return;
    }

    if export_dir.is_empty() {
        console_log!(format!("\n[错误] 没有设置导出到的路径"));
        gui_util::sub_message(get_the_hwnd!(), gui_util::IconType::Failure, "[错误] 没有设置导出到的路径", 3500u64);
        return;
    }

    if task_command.is_empty() {
        console_log!(format!("\n[错误] 没有任务命令"));
        gui_util::sub_message(get_the_hwnd!(), gui_util::IconType::Failure, "[错误] 没有任务命令", 3500u64);
        return;
    }

    let conn: Connection = Connection::open("ikun_user_data.db").unwrap();

    handle_dat::initialize_table(&conn);
    match conn.execute(
        "INSERT INTO export_dir_path (name,time,path,ouput) values (?1, ?2, ?3, ?4)",
        [
            name,
            Local::now().format("%Y-%m-%d").to_string().as_str(),
            export_dir,
            task_command,
        ],
    ) {
        Ok(_) => {
            console_log!(format!("\n[存储] 添加成功"));
            gui_util::sub_message(
                get_the_hwnd!(),
                gui_util::IconType::Success,
                "添加成功",
                3500u64,
            );
        }
        Err(err) => {
            if (str_eq_ostr(
                err.to_string(),
                "UNIQUE constraint failed: export_dir_path.path",
            )) {
                console_log!(format!("\n[错误] 添加失败 因为-> {}", "当前任务已经存在"));
                gui_util::sub_message(
                    get_the_hwnd!(),
                    gui_util::IconType::Warning,
                    "当前任务已经存在",
                    3500u64,
                );
            } else {
                gui_util::sub_message(
                    get_the_hwnd!(),
                    gui_util::IconType::Failure,
                    "任务添加失败",
                    3500u64,
                );
            }
        }
    }

    conn.close();
    global_var_util::update_export_dir_itme_list();
}

pub fn eq_next() -> bool {
    (wh_mod::config::is_developer()
        || (libWxIkunPlus::hasWeChat() && libWxIkunPlus::hasWeChatWin()))
}

// 测试
pub fn test_task(name: &str, export_dir: &str, task_command: &str) {
    let mut path_dir = wh_mod::parse_dat2var_path(format!("{}", task_command));

    if name.is_empty() {
        console_log!(format!("\n[警告] 没有名称"));
    }

    if export_dir.is_empty() {
        console_log!(format!("\n[警告] 没有设置导出到的路径"));
    }

    if task_command.is_empty() {
        console_log!(format!("\n[警告] 没有任务命令"));
    }

    if let Ok(metadata) = std::fs::metadata(path_dir.attach_dir.clone()) {
        if (!metadata.is_dir()) {
            console_log!(format!("\n[错误] dat目录文件夹 不是文件夹"));
            gui_util::sub_message(
                get_the_hwnd!(),
                gui_util::IconType::Failure,
                "dat目录文件夹 不是文件夹",
                3500u64,
            );
            return;
        }

        console_log!(format!("\n[测试] 正在扫描当前文件夹存在的dat图片"));
        console_log!(format!(
            "\n[测试] 处理范围: 仅本月:{}   缩略图:{}   原图:{}   视频:{}   同步:{}   全部:{}   ",
            bool_to_str(path_dir.is_the_month),
            bool_to_str(path_dir.is_thumbnail),
            bool_to_str(path_dir.is_source),
            bool_to_str(path_dir.is_video),
            bool_to_str(path_dir.is_sync),
            bool_to_str(path_dir.is_all)
        ));

        let pattern = format!(
            "{}",
            std::path::Path::new(&path_dir.attach_dir.clone())
                .join("**/*.dat")
                .display()
                .to_string()
        );

        let mut index = 0;

        console_log!(format!("\n[测试] 开始扫描 “{}” 中的dat文件", pattern));

        for entry in glob(&pattern).unwrap() {
            index = index + 1;
        }

        console_log!(format!(
            "\n[测试] 在 “{}” \n中发现了 [{}] 个dat文件",
            pattern, index
        ));
        gui_util::sub_message(
            get_the_hwnd!(),
            gui_util::IconType::Success,
            "测试成功",
            3500u64,
        );

        return;
    }
    console_log!(format!(
        "\n[错误] dat目录文件夹 无法被读取",
    ));
    gui_util::sub_message(
        get_the_hwnd!(),
        gui_util::IconType::Failure,
        "dat目录文件夹 打开失败",
        3500u64,
    );
}

fn bool_to_str (b:bool) -> &'static str {
    if b {"是"} else { "否" }
}
#![allow(warnings, unused)]

use fltk::app::{event_key, sleep};
use fltk::enums::{Color, Cursor, Event, FrameType, Key};
use fltk::{prelude::*, *};
use fltk_theme::{color_themes, ColorTheme, SchemeType, ThemeType, WidgetScheme, WidgetTheme};
use crate::gui_util::hotspot::create_hotspot;
use crate::gui_util::img;
use crate::{gui_util, libWxIkunPlus};

use std::collections::{HashMap, HashSet};

use std::sync::atomic::{AtomicBool, AtomicI32, AtomicI64, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, MutexGuard, OnceLock, RwLock};
use crate::util::OverloadedAnyStr;

static mut MESS_HASH_MAP: Option< HashMap<String,i128> > = Option::None;
static MESS_HASH_MAP_BIND: AtomicUsize = AtomicUsize::new(0);

static mut WINDOW_CLASS_HASH_SET: Option< HashSet<String> > = Option::None;
static WINDOW_CLASS_HASH_SET_BIND: AtomicUsize = AtomicUsize::new(0);


// 已经初始化哈希表了
static VARIABLE_INITIALIZE: OnceLock<bool> = OnceLock::new();

// 初始化全部类型哈希表
fn initialize() {
    if *(VARIABLE_INITIALIZE.get().unwrap_or_else(|| &false)) {
        return;
    }

    unsafe {
        if MESS_HASH_MAP.is_none() {
            MESS_HASH_MAP.replace(HashMap::new());
        }
    }

    unsafe {
        if WINDOW_CLASS_HASH_SET.is_none() {
            WINDOW_CLASS_HASH_SET.replace(HashSet::new());
        }
    }

    VARIABLE_INITIALIZE.set(true);
}

fn has_hash_message(hwnd:i128, mess:&str) -> bool {
    let mutex = Arc::new(Mutex::new(&MESS_HASH_MAP_BIND));
    mutex.lock();
    let the_value: usize = MESS_HASH_MAP_BIND.load(Ordering::SeqCst);
    let mut result = false;

    unsafe {
        let mut mut_hash = MESS_HASH_MAP.as_mut().unwrap();
        if let Some(mut_hash_) = mut_hash.get_key_value(&*mess.to_string()) {
            result = mut_hash_.1.clone()==hwnd;
        }
    };

    MESS_HASH_MAP_BIND.store(the_value + 1, Ordering::SeqCst);
    drop(mutex);
    result
}

fn del_hash_message(mess:&str){
    let mutex = Arc::new(Mutex::new(&MESS_HASH_MAP_BIND));
    mutex.lock();
    let the_value: usize = MESS_HASH_MAP_BIND.load(Ordering::SeqCst);
    unsafe {
        let mut mut_hash = MESS_HASH_MAP.as_mut().unwrap();
        if let Some(mut_hash_) = mut_hash.get_key_value(&*mess.to_string()) {
            mut_hash.remove(&*mess.to_string());
        }
    };

    MESS_HASH_MAP_BIND.store(the_value + 1, Ordering::SeqCst);
    drop(mutex);
}

fn set_hash_message(hwnd:i128,mess:&str){
    let mutex = Arc::new(Mutex::new(&MESS_HASH_MAP_BIND));
    mutex.lock();
    let the_value: usize = MESS_HASH_MAP_BIND.load(Ordering::SeqCst);
    unsafe {
        let mut mut_hash = MESS_HASH_MAP.as_mut().unwrap();
        mut_hash.insert(mess.to_string(), hwnd.clone());
    };

    MESS_HASH_MAP_BIND.store(the_value + 1, Ordering::SeqCst);
    drop(mutex);
}


fn get_window_class_list() -> Vec<String> {
    let mutex = Arc::new(Mutex::new(&WINDOW_CLASS_HASH_SET_BIND));
    mutex.lock();
    let the_value: usize = WINDOW_CLASS_HASH_SET_BIND.load(Ordering::SeqCst);
    let mut result: Vec<String> = Vec::new();

    unsafe {
        let mut mut_hash = WINDOW_CLASS_HASH_SET.as_mut().unwrap();
        for mut_hash in mut_hash.iter() {
            result.push(mut_hash.clone());
        }
    };

    WINDOW_CLASS_HASH_SET_BIND.store(the_value + 1, Ordering::SeqCst);
    drop(mutex);
    result
}

fn del_window_class(class:&str){
    let mutex = Arc::new(Mutex::new(&WINDOW_CLASS_HASH_SET_BIND));
    mutex.lock();
    let the_value: usize = WINDOW_CLASS_HASH_SET_BIND.load(Ordering::SeqCst);
    unsafe {
        let mut mut_hash = WINDOW_CLASS_HASH_SET.as_mut().unwrap();
        mut_hash.remove(class);
    };

    WINDOW_CLASS_HASH_SET_BIND.store(the_value + 1, Ordering::SeqCst);
    drop(mutex);
}

fn set_window_class(class:&str){
    let mutex = Arc::new(Mutex::new(&WINDOW_CLASS_HASH_SET_BIND));
    mutex.lock();
    let the_value: usize = WINDOW_CLASS_HASH_SET_BIND.load(Ordering::SeqCst);
    unsafe {
        let mut mut_hash = WINDOW_CLASS_HASH_SET.as_mut().unwrap();
        mut_hash.insert(class.to_string());
    };

    WINDOW_CLASS_HASH_SET_BIND.store(the_value + 1, Ordering::SeqCst);
    drop(mutex);
}


pub enum IconType {
    Success,
    Failure,
    Warning,
    Info
}

macro_rules! set_item_id {
    ($win:expr,$id:expr) => {
        $win.set_xclass($id);
        $win.set_id($id);
    };
}

#[derive(Debug)]
pub struct TextSize{
    pub size:usize,
    pub utf8_len:usize,
    pub ansi_len:usize,
    pub all_len:usize,
    pub prediction_len:usize
}

pub fn text_size (data:&str) -> TextSize {
    let mut result = TextSize{
        size: 0,
        utf8_len: 0,
        ansi_len: 0,
        all_len: 0,
        prediction_len:0
    };
    
    let mut name_len = 0;

    for value in data.chars() {
        if value.len_utf8()>2 {
            result.utf8_len+=1;
            result.prediction_len+=2;

        }else {
            result.ansi_len+=1;
            result.prediction_len+=1;
        }

        result.all_len+=1;
    }


    result.size = data.len();

    result
}

pub fn message(x:i32, y:i32,icon: IconType, message: &str,close_sleep:u64) {
    let mut hwnd = 0;
    let mut max_top = 0;

    // 消息不叠加到同个位置
    for get_window_class in get_window_class_list() {
        let hwnd = libWxIkunPlus::findWindow(get_window_class.as_str(),"");
        let rect = libWxIkunPlus::getWindowRect(hwnd);
        // println!("hwnd->{}  rect-> {:?}",&hwnd,&rect);
        if rect.top >= max_top {
            max_top = rect.top+55;
        }
    }
    let mut new_y = if max_top!=0 {max_top} else {y};

    // println!("get_window_class_list()->{:?}  new_y => {} ",get_window_class_list(),new_y);

    let mut win = window::DoubleWindow::new(x,new_y, 350, 45, None);
    win.set_color(Color::from_rgb(25, 25, 25));
    win.set_border(false);

    let win_id = format!("hmc_message_id<{}>",libWxIkunPlus::randomNum());
    set_item_id!(win,win_id.as_str());
    let text_size_data = text_size(message);

    set_window_class(win_id.as_str());

    // println!("win_id->{}",&win_id);
    // println!("{:?}",text_size(message));

    win.set_pos(x, new_y);

    let mut back_border = img::ImgPreview::new(0,0,win.w(),win.h(),"");
    let mut text_size = 13;
    // 计算文本大小
    if text_size_data.prediction_len>37 {
        text_size-=1;
    }

    if text_size_data.prediction_len>42 {
        text_size-=1;
    }

    if text_size_data.prediction_len>45 {
        text_size-=1;
    }

    let mut text = gui_util::text::TextPreview::new(40,15,275,15,text_size,message,[255,255,255]);
    // TextSize { size: 48, utf8_len: 16, ansi_len: 0, all_len: 16 }
    // TextSize { size: 38, utf8_len: 0, ansi_len: 38, all_len: 38 }

    // 设置消息内容背景
    match icon {
        IconType::Success => {
            back_border.from_svg(include_str!("./src/success.svg"), 0, 0, win.w(),win.h());
            text.set_back_color(27, 175, 93);
        }
        IconType::Failure => {
            back_border.from_svg(include_str!("./src/error.svg"), 0, 0, win.w(),win.h());
            text.set_back_color(228, 88, 62);
        }
        IconType::Warning => {
            back_border.from_svg(include_str!("./src/warning.svg"), 0, 0, win.w(),win.h());
            text.set_back_color(198, 134, 88);
        }
        IconType::Info => {
            back_border.from_svg(include_str!("./src/meassge.svg"), 0, 0, win.w(),win.h());
            text.set_back_color(60, 60, 60);
        }
    }

    back_border.preview.set_frame(FrameType::NoBox);

    let close = create_hotspot(320,12,21,21);

    win.handle({
        let mut x = 0;
        let mut y = 0;
        // 是否显示手型
        let mut show_cursor = false;
        let win_id = win_id.clone();
        move |win, ev| match ev {
            enums::Event::Show=>{
                win.set_visible_focus();
                libWxIkunPlus::setWinTop(libWxIkunPlus::findWindow(win_id.as_str(),""),true);
                true
            }
            enums::Event::Drag => {
                win.clone()
                    .set_pos(app::event_x_root() - x, app::event_y_root() - y);
                true
            }
            enums::Event::Push =>{
                if close.existPoint(x,y){
                    fltk::window::Window::delete(win.clone());
                }

                true
            }
            enums::Event::Move => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;

                if close.existPoint(x,y){
                    win.set_cursor(Cursor::Hand);
                } else {
                    win.set_cursor(Cursor::Default);
                }

                true
            }
            enums::Event::Hide=>{
                fltk::window::Window::delete(win.clone());
                // println!(" fltk::window::Window::delete(win);");
                true
            }
            _ => {
                false },
        }
    });

    win.end();
    win.show();



    libWxIkunPlus::setWinTop(libWxIkunPlus::findWindow(win_id.as_str(),""),true);

    let win_id2 = win_id.clone();

    // 置顶
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(100u64));
        libWxIkunPlus::setWinTop(libWxIkunPlus::findWindow(win_id2.as_str(),""),true);

        std::thread::sleep(std::time::Duration::from_millis(500u64));
        libWxIkunPlus::setWinTop(libWxIkunPlus::findWindow(win_id2.as_str(),""),true);

        std::thread::sleep(std::time::Duration::from_millis(1500u64));
        libWxIkunPlus::setWinTop(libWxIkunPlus::findWindow(win_id2.as_str(),""),true);
    });


    let message_copy = format!("{}",message);
    let win_id3 = win_id.clone();

    // 释放
    std::thread::spawn(move || {
        let hwnd = libWxIkunPlus::findWindow(win_id3.as_str(),"");
        // Sleep(close_sleep);
        std::thread::sleep(std::time::Duration::from_millis(close_sleep));
        libWxIkunPlus::setwinVisible(hwnd,false);
        del_hash_message(message_copy.as_str());
        del_window_class(win_id3.as_str());
    });

}

pub fn sub_message(hwnd:i128,icon: IconType, _message: &str,close_sleep:u64){
    initialize();
    let mut rect =libWxIkunPlus::getWindowRect(hwnd);
    if !has_hash_message(hwnd,_message) {
        set_hash_message(hwnd,_message);
    }


    let [x,y] = [rect.left + (rect.width/2)-(350/2),rect.top+50];
    message(x,y,icon,_message,close_sleep);
}

pub fn message_the_win(icon: IconType, _message: &str,close_sleep:u64){
    initialize();
    let hwnd = libWxIkunPlus::getFocusWindow();
    let mut rect =libWxIkunPlus::getWindowRect(hwnd);
    let [x,y] = [rect.left + (rect.width/2)-(350/2),rect.top+50];
    message(x,y,icon,_message,close_sleep);
}


use image::{GenericImageView, ImageBuffer};
use imagesize;
const MOBILE_SCREENSHOT_SIZE: [[usize; 2]; 130] = [[1284,2778],[1170,2532],[1170,2532],[1080,2340],[1284,2778],[1170,2532],[1080,2340],[1125,2436],[1242,2688],[828,1792],[1242,2688],[1125,2436],[1242,2208],[750,1334],[640,1136],[480,854],[1080,2160],[1080,1920],[1440,2560],[1080,2160],[1080,2270],[1080,2160],[1080,2246],[720,1280],[1080,1920],[1080,1920],[1080,1920],[1440,2560],[1440,2560],[1080,2400],[1080,2340],[720,1560],[1080,2340],[1080,2280],[1440,3040],[1440,3040],[1440,2960],[1440,2960],[1440,2960],[1440,2960],[1440,3040],[1080,2280],[1440,2960],[1440,2560],[1440,2560],[1080,1920],[1440,2560],[1440,2560],[1080,1920],[1080,1920],[1440,2560],[1080,2242],[1080,2160],[1080,2160],[1080,2160],[1080,1920],[1440,3120],[1080,2340],[1080,1920],[1080,2340],[1080,1920],[1080,1920],[1080,1920],[1080,2340],[1080,2400],[1080,2340],[1080,2340],[1080,2340],[1080,2340],[1080,2340],[1080,2340],[720,1520],[1080,2340],[720,1520],[720,1280],[720,1280],[720,1280],[1080,2340],[1080,2400],[1080,2340],[720,1560],[1080,2340],[1080,2340],[1080,2160],[1080,2040],[1080,1920],[1080,1920],[1080,1920],[1080,1920],[1080,1920],[1080,1920],[1080,2340],[1080,2340],[1080,1920],[720,1280],[1080,2340],[1080,2340],[720,1544],[720,1520],[720,1520],[1080,2340],[1080,2340],[1080,2460],[1080,2340],[1080,2280],[1080,2280],[1080,2280],[1080,2340],[1080,2340],[1080,1920],[1080,1920],[1200,2640],[1200,2640],[1080,2310],[1080,2340],[1080,2312],[1080,2310],[1080,2340],[1176,2400],[1080,2340],[1440,3120],[1440,3120],[1080,2244],[1080,2244],[1080,2340],[1080,2340],[1080,2240],[1080,2244],[1080,1920],[1440,2560]];

pub fn has_mobile_screenshot(data:Vec<u8>)-> bool{
    match imagesize::blob_size(&data) {
        Ok(size) =>{
            for [width,height] in MOBILE_SCREENSHOT_SIZE {
                if size.width == width  && size.height == height {
                    return true;
                }
            }
            return true
        },
        Err(why) => {
            return  false
        },
    }
}#![allow(warnings, unused)]

use crate::{get_arc_bind_variable, get_bool, get_option_arc_bind_variable, get_option_arc_bind_variable_or, global_var, gui_util, inject_fltk_theme, libWxIkunPlus, set_arc_bind_variable, set_arc_bind_variable_string_replace_data, set_bool, set_item_id, set_option_arc_bind_variable, wh_mod};
use fltk::enums::{Color, FrameType};
use fltk::window::DoubleWindow;
use fltk::{prelude::*, *};
use fltk_theme::{color_themes, ColorTheme, SchemeType, ThemeType, WidgetScheme, WidgetTheme};
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Local};
use crate::gui_select_user_ui;
use std::collections::{HashMap, HashSet};
use std::path;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicI64, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, MutexGuard, OnceLock, RwLock};

const THE_WIN_UI_BORDER: &str =  include_str!("./src/ui.svg");
const THE_WIN_CLASS_NAME: &str = "wx_auto_ex_im::gui_util::drag_scan2_ui::main<56136>";

static mut STATUS_TIME:Option< std::time::Instant > = Option::None;
static STATUS_TIME_BIND: AtomicUsize = AtomicUsize::new(0);

static mut STATUS_PREVIEW_BUF:Option< fltk::text::TextBuffer > = Option::None;
static STATUS_PREVIEW_BUF_BIND: AtomicUsize = AtomicUsize::new(0);

static SCAN_SCAN_ING: AtomicBool = AtomicBool::new(false);
static PUSH_MESSAGE_ING: AtomicBool = AtomicBool::new(false);

macro_rules! gc_the_window {
    ($win:expr) => {
      fltk::window::Window::delete($win.clone());
      // wh_mod::gc_walk_attach_file_list();
      // set_arc_bind_variable_string_replace_data!(STATUS_PREVIEW_TEXT,STATUS_PREVIEW_TEXT_BIND,"");
      println!("[gc_window] [{}] [{}]",THE_WIN_CLASS_NAME,!has_window());
    };
}

macro_rules! get_the_hwnd {
    ($class_id:expr) => {
        {
        let mut _hwnd = 0 ;
        for _ in 0..8 {
          _hwnd = libWxIkunPlus::findWindow($class_id,"");
            if !libWxIkunPlus::isWindow(_hwnd) {
                 _hwnd=0;
            }else {
              break;
          }
            fltk::app::sleep(0.020);
        }
        _hwnd as i128}
    };
    ()=>{
        get_the_hwnd!(THE_WIN_CLASS_NAME)
    }
}

// 获取图片id
pub fn get_wx_temp_imag_id (img_path:&str) -> String {
    let file_name = std::path::Path::new(img_path)
        .file_name()
        .unwrap_or_else(|| "".as_ref());

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

    id
}

// 从历史记录中查找图片id
pub fn get_history_attach_name (temp_imag_id:&str) -> Option<String>{
    let mut walk_attach_file_history = wh_mod::get_walk_attach_file_history();

    for (key, path_list) in walk_attach_file_history {
        for path in path_list {
            let resolve_path = path.to_string_lossy();
            if resolve_path.contains(temp_imag_id) {
                return Some(resolve_path.to_string());
            }
        }
    }

    Option::None
}

fn push_message (message:&str,must_reach:bool) -> bool {

    if !must_reach && get_bool!(PUSH_MESSAGE_ING) {
        return false;
    }

    set_bool!(PUSH_MESSAGE_ING,true);

    let mutex = Arc::new(Mutex::new(&STATUS_PREVIEW_BUF_BIND));
    mutex.lock();
    let the_value:usize = STATUS_PREVIEW_BUF_BIND.load(Ordering::SeqCst);

    unsafe{

        let mut buf = STATUS_PREVIEW_BUF.as_mut().unwrap();
        buf.remove(0,buf.length());
        buf.append(message);

    };
    STATUS_PREVIEW_BUF_BIND.store(the_value+1, Ordering::SeqCst);

    drop(mutex);

    std::thread::spawn(||{
        std::thread::sleep(std::time::Duration::from_millis(500u64));
        set_bool!(PUSH_MESSAGE_ING,false);
    });

    true
}

fn initialize_watch_attach_puppet(imag_id: &str){
    set_bool!(SCAN_SCAN_ING,true);

    let mut user_select_wxid = global_var::get_string_default("user::config::user_select_wxid");
    let mut input_select_dir = global_var::get_string_default("user::config::user_select_path");
    let imag_id_copy2 = format!("{}",&imag_id);

    if input_select_dir.is_empty() {
        set_bool!(SCAN_SCAN_ING,false);
        let start = get_option_arc_bind_variable_or!(STATUS_TIME,STATUS_TIME_BIND,std::time::Instant::now());
        let mut input_data = format!("没有 WX根目录 已经结束扫描 用时: {:?}",start.elapsed());
        gui_util::message::sub_message(get_the_hwnd!(),gui_util::message::IconType::Failure,input_data.as_str(),3500u64);

        unsafe {
            let mut buf = STATUS_PREVIEW_BUF.as_mut().unwrap();
            buf.remove(0,buf.length());
            buf.append(input_data.as_str());
        }

        return;
    }

    let (tx, rx) = std::sync::mpsc::channel();

    // 开始启动扫描的线程
    std::thread::spawn(move || {
        // 启动扫描线程
        // 用户数据根目录
        let mut msg_attach_root = HashSet::new();


        // 优先考虑当前已选用户
        let msg_attach_dir =  format!("{}\\{}\\FileStorage\\MsgAttach", &input_select_dir,&user_select_wxid);
        msg_attach_root.insert(wh_mod::resolve_path(msg_attach_dir));

        // 扩展到全局
        let wx_read_root_wxid = wh_mod::wx_read_root_wxid(std::path::Path::new(input_select_dir.as_str()));
        for wx_read_wxid in wx_read_root_wxid {
            let path2str = wx_read_wxid.attach.to_string_lossy().to_string();
            msg_attach_root.insert(wh_mod::resolve_path(path2str));
        }

        // 启动扫描线程
        for path_for in msg_attach_root {
            if (has_window()) {
                wh_mod::walk_file(std::path::Path::new(path_for.as_str()), tx.clone(), "".to_string());
            }
        }
    });

    // 获取数据的线程
    std::thread::spawn(move || {
        let mut walk_next_not_message: usize = 0;

        loop {
            if let Result::Ok((attach_key, paths)) = rx.recv() {

                if let Some(file_name) = path::Path::new(attach_key.as_str()).file_name() {
                    push_message(format!("[扫描]当前：{:?}",wh_mod::get_show_mask_text(file_name)).as_str(),false);
                }

                for path in paths {
                    let resolve_path = path.to_string_lossy();
                    if resolve_path.contains(imag_id_copy2.as_str()) {
                        let start = get_option_arc_bind_variable_or!(STATUS_TIME,STATUS_TIME_BIND,std::time::Instant::now());
                        let att_info = wh_mod::wx_parse_path(resolve_path.to_string());

                        let mut input_data = format!("用户<{}> [已选定] 用时: {:?}", wh_mod::get_show_mask_text(&att_info.attach_id), start.elapsed());
                        global_var::set_string("user::config::walk_drag_path",resolve_path.clone().to_string());

                        set_bool!(SCAN_SCAN_ING,false);
                        println!("{}", &input_data);
                        push_message(input_data.as_str(),true);
                        break;
                    }
                }
            } else {
                walk_next_not_message += 1;

                if (walk_next_not_message > 50) {
                    set_bool!(SCAN_SCAN_ING,false);
                    // let start = get_option_arc_bind_variable_or!(STATUS_TIME,STATUS_TIME_BIND,std::time::Instant::now());
                    // let mut input_data = format!("扫描结束 用时: {:?}", start.elapsed());

                    // push_message(input_data.as_str(),true);

                    // println!("{}", &input_data);
                    break;
                }
            }
        }
    });


}

// 判断窗口是否有效
pub fn has_window()->bool{
    let hwnd = get_the_hwnd!(THE_WIN_CLASS_NAME);
    libWxIkunPlus::isWindow(hwnd)
}

struct  UiControl{
    btn_close: gui_util::hotspot::HotspotItmeControl,
    id_preview: gui_util::text::TextPreview,
    status_preview: gui_util::text::TextPreview,
    progress_bar:gui_util::img::ImgPreview
}

// 進度條
fn show_progress_bar_border(x: i32, y: i32) -> gui_util::img::ImgPreview {
    let mut progress_bar_border = gui_util::img::ImgPreview::new_border(x,y,521,15,"<svg width=\"520\" height=\"15\" viewBox=\"0 0 520 15\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\"> <rect x=\"0.5\" y=\"0.5\" width=\"520\" height=\"14\" rx=\"7\" fill=\"#181818\" stroke=\"#2C2C2C\"/> </svg> ");
    let width = progress_bar_border.width;
    let mut progress_bar_border_slider = gui_util::img::ImgPreview::new_border(x, y, 41, 15, "<svg width=\"40\" height=\"15\" viewBox=\"0 0 40 15\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\">\n<rect x=\"0.5\" y=\"0.5\" width=\"39\" height=\"14\" rx=\"7\" fill=\"#333333\" stroke=\"#2C2C2C\"/>\n</svg>");
    progress_bar_border_slider.preview.hide();
    progress_bar_border.preview.hide();

    progress_bar_border.preview.handle({
        let mut progress_bar_border_slider = progress_bar_border_slider.clone();
        let mut preview_main = progress_bar_border.preview.clone();

        move |win, ev| match ev {
            enums::Event::Show => {
                progress_bar_border_slider.preview.show();

                let mut preview = progress_bar_border_slider.preview.clone();
                let mut preview_main = preview_main.clone();

                app::add_timeout3(0.0086, move |handle| {
                    if !preview.visible()||!preview_main.visible() {
                        preview_main.hide();
                        preview.hide();
                        app::remove_timeout3(handle);
                        return;
                    }

                    let mut the_x = preview.x() + 2;
                    if the_x > width {
                        the_x = x + preview.width();
                        the_x -= preview.width();
                    }
                    preview.set_pos(the_x, preview.y());
                    preview.parent().unwrap().redraw();
                    app::repeat_timeout3(0.0086, handle);
                });
                true
            }
            enums::Event::Hide => {
                progress_bar_border_slider.preview.hide();
                true
            }
            _ => false,
        }
    });

    // progress_bar_border.preview.show();
    progress_bar_border
}

fn add_ui_control() -> UiControl{
    gui_util::text::TextControl::new(150,39,225,15, 12, "请从聊天窗口拖拽一张图片到本窗口  (部分PC需要拖拽两次)", [149, 149, 149]);
    gui_util::text::TextControl::new(35,258,75,15, 12, "当前图片ID:", [149, 149, 149]);
    gui_util::text::TextControl::new(35,281,75,15, 12, "当 前 状 态 :", [149, 149, 149]);
    gui_util::text::TextControl::new(265,180,70,15, 12, "拖拽到此处", [57, 57, 57]);
    let btn_close = gui_util::hotspot::create_hotspot(537,33,25,25);
    let mut id_preview = gui_util::text::TextPreview::new(35+75,258,450,15, 12, "null", [57, 57, 57]);
    let mut status_preview = gui_util::text::TextPreview::new(35+75,281,450,15, 12, "尚未拖拽入文件", [57, 57, 57]);
    let mut progress_bar = show_progress_bar_border(40,313);

    let mut  buf = status_preview.buf.clone();
    let mut progress_bar_preview =progress_bar.preview.clone();

    set_option_arc_bind_variable!(STATUS_PREVIEW_BUF,STATUS_PREVIEW_BUF_BIND,status_preview.buf.clone());

    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_millis(500u64));
        if !has_window() {
            return;
        }

        if !get_bool!(SCAN_SCAN_ING) {
            if has_window() { progress_bar_preview.hide();}

            let start = get_option_arc_bind_variable!(STATUS_TIME,STATUS_TIME_BIND);

            if let Some(start) = start {
                std::thread::sleep(std::time::Duration::from_millis(300u64));

                if !has_window() {
                    buf.remove(0,buf.length());
                    let walk_drag_path = global_var::get_string_default("user::config::walk_drag_path");
                    if walk_drag_path.is_empty() {
                        let mut input_data = format!("扫描结束 用时约为: {:?} ", start.elapsed());
                        buf.append(wh_mod::get_show_mask_text(&input_data).as_str());
                    }
                    else{
                        let att_info = wh_mod::wx_parse_path(walk_drag_path.to_string());
                        let mut input_data = format!("ID<{}> [已选定] 用时约为: {:?}",wh_mod::get_show_mask_text(att_info.attach_id) , start.elapsed());
                        buf.append(wh_mod::get_show_mask_text(&input_data).as_str());
                    }
                };


            }

            unsafe {
                let mutex = Arc::new(Mutex::new(&STATUS_TIME_BIND));
                mutex.lock();
                let the_value:usize = STATUS_TIME_BIND.load(Ordering::SeqCst);

                unsafe{ STATUS_TIME = Option::None;};

                STATUS_TIME_BIND.store(the_value+1, Ordering::SeqCst);

                drop(mutex);
            }

        }


    });

     UiControl{
        btn_close,
        id_preview,
        status_preview,
        progress_bar,
    }
}

// 初始化窗口
pub fn main_window(match_input:&str)->Option<DoubleWindow> {

    // 禁止创建多个窗口
    if let hwnd = get_the_hwnd!() {
        if hwnd!=0 && libWxIkunPlus::isWindow(hwnd) {
            if let Some(mut win) =app::widget_from_id(THE_WIN_CLASS_NAME) as Option<DoubleWindow>
            {
                win.show();
                win.set_visible_focus();
            }
            libWxIkunPlus::setWindowShake(hwnd);
            return Option::None;
        }
    }


    let mut win: DoubleWindow = fltk::window::DoubleWindow::new(0, 0, 600,360, "扫描图源用户").center_screen();
    let mut rect = libWxIkunPlus::getWindowRect(libWxIkunPlus::findWindow(gui_select_user_ui::THE_WINDOW_CLASS_NAME,""));
    win.set_pos(rect.left+8,rect.top+31);

    inject_fltk_theme!();
    win.set_color(Color::from_rgb(24, 24, 24));
    // win.set_border(false);
    set_item_id!(win,THE_WIN_CLASS_NAME);
    gui_util::img::ImgPreview::new_border(0,0,600,360,THE_WIN_UI_BORDER);
    let mut win_control = add_ui_control();
    let mut copy_progress_bar =win_control.progress_bar.preview.clone();


    win.handle({
        let mut x = 0;
        let mut y = 0;
        let mut released = true;
        let mut dnd = true;
        let mut drag_path = std::path::PathBuf::new();
        let mut id_preview = win_control.id_preview.clone();
        let mut status_preview = win_control.status_preview.clone();

        move |win, ev| match ev {
            enums::Event::Show => {
                win.set_visible_focus();
                true
            }
            enums::Event::Close => {
                gc_the_window!(win);
                false
            }
            enums::Event::Hide => {
                gc_the_window!(win);
                false
            }
            enums::Event::Push => {
                if win_control.btn_close.existPoint(x,y) {
                    gc_the_window!(win);
                }
                true
            }

            enums::Event::Move => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;
                if win_control.btn_close.existPoint(x,y) {
                    win.set_cursor(fltk::enums::Cursor::Hand);
                } else {
                    win.set_cursor(fltk::enums::Cursor::Default);
                }

                true
            }

            enums::Event::DndEnter => {
                dnd = true;
                true
            }
            enums::Event::DndDrag => true,
            enums::Event::DndRelease => {
                released = true;
                true
            }
            enums::Event::Paste => {
                if dnd && released {
                    if get_bool!(SCAN_SCAN_ING) {
                        gui_util::message::sub_message(get_the_hwnd!(THE_WIN_CLASS_NAME), gui_util::message::IconType::Failure, "请等待本次扫描完成！", 3500u64);
                        return false;
                    }
                    set_option_arc_bind_variable!(STATUS_TIME,STATUS_TIME_BIND,std::time::Instant::now());
                    // set_arc_bind_variable_string_replace_data!(SCAN_SCAN_PATH,SCAN_SCAN_PATH_BIND,"");
                    global_var::set_string("user::config::walk_drag_path",String::new());

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

                        if path_list.is_empty() {
                            gui_util::message::sub_message(get_the_hwnd!(THE_WIN_CLASS_NAME), gui_util::message::IconType::Warning, "拖拽的内容不存在文件路径", 3500u64)
                        }else{
                            drag_path.clear();
                            if let Some(pop) = path_list.pop() {

                                drag_path.push(pop.clone());

                                // 获取id 并开始处理
                                let temp_imag_id = get_wx_temp_imag_id(pop.as_str());

                                if !temp_imag_id.is_empty() {
                                    let mut  buf = id_preview.buf.clone();
                                    buf.remove(0,buf.length());
                                    buf.append(wh_mod::get_show_mask_text(&temp_imag_id).as_str());
                                    win_control.progress_bar.preview.show();

                                    let mut new_buf_str = String::new();
                                    if let Some(get_history_attach) = get_history_attach_name(temp_imag_id.as_str()) {
                                        let start = get_option_arc_bind_variable_or!(STATUS_TIME,STATUS_TIME_BIND,std::time::Instant::now());
                                        let att_info = wh_mod::wx_parse_path(get_history_attach.to_string());
                                        global_var::set_string("user::config::walk_drag_path",get_history_attach.clone().to_string());
                                        new_buf_str.push_str( format!("ID<{}> [已选定] 用时: {:?}",att_info.attach_id,start.elapsed()).as_str());
                                        gui_util::message::sub_message(get_the_hwnd!(THE_WIN_CLASS_NAME), gui_util::message::IconType::Success, new_buf_str.as_str(), 3500u64);
                                        set_bool!(SCAN_SCAN_ING,false);

                                    }else{
                                        new_buf_str.push_str("开始扫描拽入的文件");
                                        set_bool!(SCAN_SCAN_ING,true);
                                        initialize_watch_attach_puppet(temp_imag_id.as_str());
                                    }

                                    push_message(new_buf_str.as_str(),true);
                                }

                                else{
                                    gui_util::message::sub_message(get_the_hwnd!(THE_WIN_CLASS_NAME), gui_util::message::IconType::Failure, "未找到此文件中的有效图片id", 3500u64);
                                }

                                println!("path_list-> {:?}",&drag_path);
                            }

                        }

                        dnd = false;
                        released = false;
                    true
                } else {
                    false
                }
            }
            enums::Event::DndLeave => {
                dnd = false;
                released = false;
                true
            }

            _ => false,
        }
    });

    win.end();
    win.show();

    // 支持传值
    if !match_input.is_empty(){

        set_option_arc_bind_variable!(STATUS_TIME,STATUS_TIME_BIND,std::time::Instant::now());
        global_var::set_string("user::config::walk_drag_path",String::new());

        // 获取id 并开始处理
        let temp_imag_id = get_wx_temp_imag_id(match_input);

        if !temp_imag_id.is_empty() {
            let mut  buf = win_control.id_preview.buf.clone();
            buf.remove(0,buf.length());
            buf.append(wh_mod::get_show_mask_text(&temp_imag_id).as_str());
            copy_progress_bar.show();

            let mut new_buf_str = String::new();
            if let Some(get_history_attach) = get_history_attach_name(temp_imag_id.as_str()) {
                let start = get_option_arc_bind_variable_or!(STATUS_TIME,STATUS_TIME_BIND,std::time::Instant::now());
                let att_info = wh_mod::wx_parse_path(get_history_attach.to_string());
                global_var::set_string("user::config::walk_drag_path",get_history_attach.clone().to_string());
                new_buf_str.push_str( format!("ID<{}> [已选定] 用时: {:?}",wh_mod::get_show_mask_text(att_info.attach_id),start.elapsed()).as_str());
                gui_util::message::sub_message(get_the_hwnd!(THE_WIN_CLASS_NAME), gui_util::message::IconType::Success, new_buf_str.as_str(), 3500u64);
                set_bool!(SCAN_SCAN_ING,false);

            }else{
                new_buf_str.push_str("开始扫描拽入的文件");
                set_bool!(SCAN_SCAN_ING,true);
                initialize_watch_attach_puppet(temp_imag_id.as_str());
            }

            push_message(new_buf_str.as_str(),true);
        }

        else{
            gui_util::message::sub_message(get_the_hwnd!(THE_WIN_CLASS_NAME), gui_util::message::IconType::Failure, "未找到此文件中的有效图片id", 3500u64);
        }

    }

    libWxIkunPlus::setWinIcon(get_the_hwnd!());
    libWxIkunPlus::setWinTop(get_the_hwnd!(),true);

    Some(win)
}
#![allow(warnings, unused)]

use crate::{get_bool, global_var, gui_util, inject_fltk_theme, set_bool, set_item_id};
use fltk::enums::{Color, FrameType};
use fltk::window::DoubleWindow;
use fltk::{prelude::*, *};
use fltk_theme::{color_themes, ColorTheme, SchemeType, ThemeType, WidgetScheme, WidgetTheme};
use std::collections::{HashMap, HashSet};
use std::path;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicI64, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, MutexGuard, OnceLock, RwLock};
use std::thread::sleep;
use fltk::examples::tile;
use crate::libWxIkunPlus;
use crate::gui_util::{*};
use crate::{*};
mod lib;


pub(crate) const THE_WIN_CLASS_NAME: &str = "wx_auto_ex_im::gui_util::main::main<55216>";
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
    title: TextControl,
    text_control_list: Vec<TextControl>,
    check:FrameCheckButton,
    task_command: fltk::input::Input,
    name: fltk::input::Input,
    export: fltk::input::Input,
    console: FrameConsole,
}

struct FrameCheckButton{
    flex:  fltk::group::Flex,
    sync:  button::CheckButton,
    start_up:  button::CheckButton,
}

struct FrameConsole{
    buf: fltk::text::TextBuffer,
    txt: fltk::text::TextEditor,
}

fn add_ui_control() -> UiControl {
    let btn_close = gui_util::hotspot::create_hotspot(556,26,25,25);
    let title = gui_util::TextControl::new(60-30 , 24, 150, 20, 12, " WX 图片自动备份", [122, 120, 120]);
    let text_control_list =vec![
        TextControl::new(30-2, 80, 70, 15, 12, "创建新任务", [85,85,85]),
        TextControl::new(15, 125, 385, 15, 12, "dat 图片所在文件夹（点击右侧按钮在任务创建向导中选定）：", [85,85,85]),
        TextControl::new(-8, 200, 200, 15, 12, "导出到此文件夹：", [85,85,85]),
        TextControl::new(506, 160, 35, 15, 12, "向导", [85,85,85]),
        TextControl::new(10, 280+5, 115, 15, 13, "名称：", [85,85,85]),
        TextControl::new(356-1, 280+4, 35, 15, 13, "管理", [85,85,85]),
        TextControl::new(428-1, 280+4, 35, 15, 13, "测试", [85,85,85]),
        TextControl::new(500-1, 280+4, 35, 15, 13, "创建", [85,85,85]),
        TextControl::new(30-1, 333, 56, 15, 12, "执行日志", [85,85,85]),
    ];
    let mut flex = group::Flex::default()
        .with_size(200, 30)
        .row()
        .center_of_parent();

    flex.set_pos(30, 490-5);
    let mut check_button_sync = button::CheckButton::default().with_label("同步开关");
    let mut check_button_start_up = button::CheckButton::default().with_label("开机启动");
    flex.end();

    check_button_sync.set_checked(libWxIkunPlus::has_auto_sync());
    check_button_start_up.set_checked(libWxIkunPlus::hasStartup());

    // 同步
    check_button_sync.set_callback(|win|{
        libWxIkunPlus::set_auto_sync(win.is_checked());

        if libWxIkunPlus::has_auto_sync() {
            message::sub_message(get_the_hwnd!(),message::IconType::Success,"同步状态已被启用",3500u64);
        }else {
            message::sub_message(get_the_hwnd!(),message::IconType::Info,"同步状态已被禁用",3500u64);
        }
        // 用锁定线程来实现防抖
        app::sleep(0.300);
    });

    // 自启
    check_button_start_up.set_callback(|win|{
        if win.is_checked()!=libWxIkunPlus::hasStartup() {
            libWxIkunPlus::setStartup();
        }

        if libWxIkunPlus::hasStartup() {
            message::sub_message(get_the_hwnd!(),message::IconType::Success,"开机自启动已被启用",3500u64);
        }else {
            message::sub_message(get_the_hwnd!(),message::IconType::Info,"开机自启动已被禁用",3500u64);
        }
        // 用锁定线程来实现防抖
        app::sleep(0.300);
    });

    let mut task_command_input = input::Input::new(45, 152, 423, 30, "");
    let mut export_input = input::Input::new(45, 225, 450, 30, "");
    let mut name_input = input::Input::new(96, 276, 230, 30, "");
    task_command_input.set_readonly(!wh_mod::config::is_developer());

    let mut buf = fltk::text::TextBuffer::default();
    buf.append(lib::get_init_text().as_str());
    let mut txt = fltk::text::TextEditor::default()
        .with_size(530, 105)
        .center_of_parent();
    txt.set_buffer(buf.clone());
    txt.set_frame(FrameType::NoBox);
    txt.set_text_color(Color::from_rgb(120, 120, 120));
    txt.set_label_type(fltk::enums::LabelType::None);
    txt.set_text_size(12);
    txt.set_pos(33, 362);
    txt.set_scrollbar_size(6);
    txt.show();

    let mut move_buf = buf.clone();

    thread::spawn(move || loop {
        Sleep(150);
        let mut console_message = handle_dat::get_console_message().replace("\n\n", "\n");

        if console_message.starts_with('\n') {
            console_message = console_message.trim_start_matches('\n').to_string();
        }

        if (console_message.len() < 5) {
            continue;
        };

        let mut newline_count = 0;

        for line in move_buf.text().lines() {
            newline_count += 1
        }

        if (newline_count > 5) {
            move_buf.remove(0, move_buf.length());
            move_buf.set_text(&console_message);
        } else {
            move_buf.append(&format!("\n{}", &console_message));
        }
    });


    UiControl {
        btn_close,
        title,
        text_control_list,
        task_command:task_command_input,
        name:name_input,
        export:export_input,
        check:FrameCheckButton{
            flex,
            sync:check_button_sync,
            start_up:check_button_start_up
        },
        console:FrameConsole{
            buf,
            txt
        }
}
}


pub fn main_init() ->Option<fltk::window::DoubleWindow> {
    main_init_check!();
    let hwnd:i128 = 0 ;
    let mut win: DoubleWindow = fltk::window::DoubleWindow::new(0, 0, 600,530, "WxAutoExIm").center_screen();
    inject_fltk_theme!();
    win.set_color(Color::from_rgb(24, 24, 24));
    win.set_border(false);
    set_item_id!(win,THE_WIN_CLASS_NAME);
    gui_util::img::ImgPreview::new_border(0,0,win.w(),win.h(),THE_WIN_UI_BORDER);
    let mut win_control = add_ui_control();
    let mut button_wizard = gui_util::hotspot::create_hotspot(475i32, 150i32 , 72i32, 32i32);
    let mut button_open_export_dir = gui_util::hotspot::create_hotspot(513i32, 225i32 , 33i32, 32i32);
    let mut button_show_manage = gui_util::hotspot::create_hotspot(342i32, 275i32 , 60i32, 32i32);
    let mut button_test = gui_util::hotspot::create_hotspot(415i32, 275i32 , 60i32, 32i32);
    let mut button_create = gui_util::hotspot::create_hotspot(486i32, 275i32 , 60i32, 32i32);
    let mut bottom_check_hotspot = gui_util::hotspot::create_hotspot(30, 490-5,200, 30);


    win.handle({
        let mut x = 0;
        let mut y = 0;
        move |win, ev| match ev {
            enums::Event::Show => {
                env::set_var("ikunWinHwnd", format!("{}",get_the_hwnd!()).to_string());
                libWxIkunPlus::setWinIconMain(get_the_hwnd!());

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
                // 关闭
                if win_control.btn_close.existPoint(x,y){
                    // gc_the_window!(win);
                    libWxIkunPlus::setwinVisible(get_the_hwnd!(),false);
                }
                // 向导
                if button_wizard.existPoint(x,y) {
                    if(lib::eq_next()){
                        let mut token_id =  gui_select_user_ui::manage_tool_main();
                        println!("token_id-> {}",&token_id);
                        if(!token_id.is_empty()){

                            let mut task_command =  win_control.task_command.clone();
                            app::add_timeout3(0.3,move|handle|{
                                if !gui_select_user_ui::has_window(){

                                    let data = global_var::get_string_default(token_id.as_str());
                                    if !data.is_empty() {
                                        println!("{}",data);
                                        gui_util::sub_message(get_the_hwnd!(), gui_util::IconType::Success, "向导任务命令已赋值", 3500u64);
                                        task_command.set_value(wh_mod::get_show_mask_text(data.as_str()).as_str());
                                        global_var::set_string("user::config::task_command",data.clone());
                                    }else {
                                        gui_util::sub_message(get_the_hwnd!(), gui_util::IconType::Warning, "用户取消任务创建", 3500u64);
                                    }

                                    app::remove_timeout3(handle);
                                    return ;
                                }
                                app::repeat_timeout3(0.3, handle);
                            });

                        }else {
                            if gui_select_user_ui::has_window(){
                                gui_util::sub_message(get_the_hwnd!(), gui_util::IconType::Warning, "窗口重复创建", 3500u64);
                            }else {
                                gui_util::sub_message(get_the_hwnd!(), gui_util::IconType::Warning, "创建任务id失败", 3500u64);
                            }
                        }

                    }else{
                        libWxIkunPlus::stop("错误".to_owned(),"当前未发现wx进程或者未登录 拒绝提供选取方案".to_owned());
                    }
                }

                // 选择导出到
                if button_open_export_dir.existPoint(x,y) {
                    let mut open_path = libWxIkunPlus::openSelectFolder2();
                    // 非空
                    if !open_path.is_empty() {
                        win_control.export.set_value(open_path.as_str());
                    }
                    // 没有名称 则使用目录名
                    if win_control.name.value().is_empty() {
                        let path = path::Path::new(open_path.as_str());
                        if let Some(file_name) = path.file_name() {
                            if let Some(file_name) = file_name.to_str() {
                                win_control.name.set_value(file_name);
                            }
                        }
                    }
                }

                // 测试
                if button_test.existPoint(x,y) {
                    let mut task_command = global_var::get_string_default("user::config::task_command");
                    let [name,export]= [win_control.name.value(),win_control.export.value()];
                    lib::test_task(name.as_str(),export.as_str(),task_command.as_str());
                }
                // 管理
                if button_show_manage.existPoint(x,y) {
                    gui_manage_item::ManageItmeMain();
                }
                // 创建
                if button_create.existPoint(x,y) {
                    let mut task_command = global_var::get_string_default("user::config::task_command");
                    let [name,export]= [win_control.name.value(),win_control.export.value()];
                    lib::push_sql_export_dir_path(name.as_str(),export.as_str(),task_command.as_str());
                }
                true
            }
            enums::Event::Move => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;

                if win_control.btn_close.existPoint(x,y)
                    ||button_wizard.existPoint(x,y)
                    ||button_open_export_dir.existPoint(x,y)
                    ||button_show_manage.existPoint(x,y)
                    ||button_test.existPoint(x,y)
                    ||button_create.existPoint(x,y)
                    ||bottom_check_hotspot.existPoint(x,y)
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

    loop {
        Sleep(200);
        if (libWxIkunPlus::hasInitWindowIsDisplayed())
        {
            win.show();
            win.set_visible_focus();
            return  Some(win);
        }
    }

    Option::None
}
#![allow(warnings, unused)]

use fltk::enums::{Color, FrameType};
use fltk::window::DoubleWindow;
use fltk::{prelude::*, *};
use fltk_theme::{color_themes, ColorTheme, SchemeType, ThemeType, WidgetScheme, WidgetTheme};
use crate::{gui_util, set_item_id};


pub fn manage_tool_main() {
    let mut win = window::Window::default().with_size(600, 450).center_screen();
    win.set_label("用户任务管理");
    set_item_id!(win, "gui::manage_tool::main<win>");
    // win.set_border(false);
    // 退出窗口
    let exit_btn = gui::hotspot::create_hotspot(540, 15, 37, 37);

    let mut preview =
        gui::img::ImgPreview::new(0, 0, win.w(), win.h(), "gui::rename_tool::main<win>");
    preview.from_svg(
        include_str!("src/contour.svg"),
        0,
        0,
        preview.preview.w(),
        preview.preview.h(),
    );
    preview.preview.set_id("gui::rename_tool::main<contour>");

    gui::text::TextControl::new(60 - 25, 24, 150, 20, 15, "用户任务管理", [122, 120, 120]);





    win.handle({
        let mut x = 0;
        let mut y = 0;

        move |win, ev| match ev {
            enums::Event::Show => {
                win.set_visible_focus();

                true
            }
            enums::Event::KeyUp => true,

            enums::Event::Push => {
                if exit_btn.existPoint(x, y) {
                    fltk::window::Window::delete(win.clone());
                }

                true
            }

            enums::Event::Move => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;

                // 判断是否显示手型鼠标
                if exit_btn.existPoint(x, y) {
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

    win.show();
}
#![allow(warnings, unused)]

use crate::{global_var, gui_util, libWxIkunPlus, set_item_id, util};
use fltk::enums::{Color, FrameType};
use fltk::window::DoubleWindow;
use fltk::{prelude::*, *};
use fltk_theme::{color_themes, ColorTheme, SchemeType, ThemeType, WidgetScheme, WidgetTheme};
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Local};
pub(crate) const THE_WINDOW_CLASS_NAME: &'static str = "wx_auto_ex_im::gui_util::rename_tool::main<32626>";




macro_rules! the_token {
    ()=>{
       {
        let mut _the_token =format!("token<{}>@query",libWxIkunPlus::randomNum());
        loop{
            if global_var::has_string(_the_token.as_str()) {
                _the_token = format!("token<{}>@query",libWxIkunPlus::randomNum());
            }else{
                break;
            }
        }
            _the_token
        }
    }
}

pub fn rename_tool_main(input:&str) -> String {

    let mut the_token = the_token!();

    let mut win = window::Window::default().with_size(600, 550).center_screen();
    win.set_label("命名规则工具");
    set_item_id!(win, THE_WINDOW_CLASS_NAME);
    win.set_border(false);
    let mut rename_variable_input_oid_str = String::new();
    let time_info =util::get_time_info();
    
    let mut preview =
    gui_util::img::ImgPreview::new(0, 0, win.w(), win.h(), THE_WINDOW_CLASS_NAME);
    preview.from_svg(
        include_str!("./src/contour.svg"),
        0,
        0,
        preview.preview.w(),
        preview.preview.h(),
    );
    preview.preview.set_id("gui_util::rename_tool::main<contour>");

    let mut title =
    gui_util::text::TextControl::new(60 - 25, 24, 150, 20, 15, "命名规则工具", [122, 120, 120]);
    let mut title_box_01 = gui_util::text::TextControl::new(
        210 - 15, /* -15 Frame偏差*/
        76,
        206,
        18,
        12,
        "规则自变量 (自动)   单击可以输入",
        [122, 120, 120],
    );

    // 时间变量：（第一行）
    gui_util::text::TextControl::new(36, 113, 61, 18, 12, "时间变量:", [122, 120, 120]);
    //其他变量: （第二行）
    gui_util::text::TextControl::new(36, 156, 61, 18, 12, "其他变量:", [122, 120, 120]);
    //自增序列(必选): （第三行）
    gui_util::text::TextControl::new(36, 250, 95, 18, 12, "自增序列(必选):", [122, 120, 120]);

    //预览变量 (存储文件名)：
    gui_util::text::TextControl::new(
        36 - 6 + 32,
        318,
        150,
        18,
        12,
        "命名规则  变量公式 [ 在此可编辑 ]  ：",
        [122, 120, 120],
    );
    //预览变量 (存储文件名)：
    gui_util::text::TextControl::new(
        36,
        386 - 3, /* -3 Frame偏差*/
        150,
        18,
        12,
        "预览变量 (存储文件名)：",
        [122, 120, 120],
    );

    gui_util::text::TextControl::new(192, 505 - 7, 35, 18, 13, "取消", [121, 121, 121]);
    gui_util::text::TextControl::new(371 - 3, 505 - 7, 35, 18, 13, "确认", [122, 120, 120]);

    // 命名规则
    let mut rename_variable_input = input::Input::new(35 - 1, 346, 533, 30, "");
    rename_variable_input.set_id("gui_util::rename_variable_input");
    let rename_variable_input_hotspot = gui_util::hotspot::create_hotspot(35 - 1, 346, 533, 30);
    rename_variable_input.append(if input.is_empty() {"<创建月>/<任务名>/<类型>_<NN>"}else { input });

    // 预览
    let mut rename_preview_input = input::Input::new(35 - 1, 399 + 10, 533 /*490 - 12*/, 30, "");
    rename_preview_input.set_readonly(true);
    rename_preview_input.set_id("gui_util::rename_preview_input");

    // 后缀名
    // let rename_preview_ext =
    //     gui_util::text::TextControl::new(528 - 15, 399 + 16, 35, 20, 13, ".jpg", [122, 120, 120]);
    // rename_variable_input.set_id("gui_util::rename_preview_ext");

    // 退出窗口
    let exit_btn = gui_util::hotspot::create_hotspot(540, 15, 37, 37);

    // 确认/取消
    let mut cancel_btn = gui_util::hotspot::create_hotspot(147, 488 - 2, 125, 38);
    let mut confirm_btn = gui_util::hotspot::create_hotspot(139 + 8 + 170 + 5, 488 - 2, 125, 38);
    let mut variable_list = Vec::from([
        gui_util::variable_tag_control::varTagControl::new(
            106,
            108,
            128,
            31,
            "现在:",
            &time_info.time,
            &time_info.time,
        ),
        gui_util::variable_tag_control::varTagControl::new(245, 108, 72, 31, "年:", &time_info.years, &time_info.years),
        gui_util::variable_tag_control::varTagControl::new(328, 108, 72, 31, "月:", &time_info.month, &time_info.month),
        gui_util::variable_tag_control::varTagControl::new(388, 108, 72, 31, "日:", &time_info.day, &time_info.day),
        gui_util::variable_tag_control::varTagControl::new(450 - 2, 108, 72, 31, "时:", &time_info.hour, &time_info.hour),
        gui_util::variable_tag_control::varTagControl::new(508, 108, 72, 31, "分:", &time_info.minutes, &time_info.minutes),
        gui_util::variable_tag_control::varTagControl::new(
            106,
            150,
            178,
            31,
            "别名:",
            "软件内的用户备注名",
            "事妈老板",
        ),
        gui_util::variable_tag_control::varTagControl::new(
            293,
            150,
            182,
            31,
            "任务名:",
            "当前任务的任务名",
            "工作内容备份",
        ),
        gui_util::variable_tag_control::varTagControl::new(483, 150, 83, 31, "创建月:", "月", &time_info.time_years),
        gui_util::variable_tag_control::varTagControl::new(
            33,
            196,
            221,
            31,
            "类型:",
            "缩略图,视频,图片,手机截图",
            "图片",
        ),
        gui_util::variable_tag_control::varTagControl::new(
            265,
            196,
            100,
            31,
            "哈希:",
            "dat名称",
            "666a6b6666666abc999aa9b6bc99999a",
        ),
        gui_util::variable_tag_control::varTagControl::new(146, 231 + 13, 53, 31, "%N:", "1", "1"),
        gui_util::variable_tag_control::varTagControl::new(205, 231 + 13, 73, 31, "%NN:", "01", "01"),
        gui_util::variable_tag_control::varTagControl::new(289, 231 + 13, 92, 31, "%NNN:", "001", "001"),
        gui_util::variable_tag_control::varTagControl::new(
            392,
            231 + 13,
            115,
            31,
            "%NNNN:",
            "0001",
            "0001",
        ),
    ]);


    let mut rename_variable_input_copy = rename_variable_input.clone();
    let mut rename_preview_input_copy = rename_preview_input.clone();

    macro_rules! update_variable_input {
        () => {

            if !rename_variable_input_copy.value().as_bytes().eq(rename_variable_input_oid_str.as_bytes()){

            // 更新预览
            let mut label = rename_variable_input_copy.value();

            // 替换变量值
            for index in 1..variable_list.len() + 1 {
                if let Some(variable) = variable_list.get(variable_list.len() - index) {
                    label = label.replace(variable.get_var().as_str(), variable.data.as_str());
                }
            }
            rename_variable_input_oid_str = format!("{}{}",label.as_str(),".jpg");
            // 设置预览文本
            rename_preview_input_copy.set_value(rename_variable_input_oid_str.as_str());

            }
        };
    }

    update_variable_input!();

    win.handle({
        let mut the_token =the_token.clone();
        let mut x = 0;
        let mut y = 0;
        move |win, ev| match ev {
            enums::Event::Show => {
                win.set_visible_focus();

                true
            }
            enums::Event::KeyUp=>{
                update_variable_input!();
                true
            }

            enums::Event::Push => {
                if exit_btn.existPoint(x, y) {
                    fltk::window::Window::delete(win.clone());
                }

                for variable in variable_list.iter() {
                    if variable.existPoint(x, y) {
                        println!(
                            "click[variable]-> {}  var<  {}  >",
                            variable.id.clone(),
                            variable.get_var()
                        );
                        rename_variable_input_copy.append(variable.get_var().as_str());

                        break;
                    }
                }

                if cancel_btn.existPoint(x,y) {
                    global_var::set_string(the_token.as_str(),String::new());
                    fltk::window::Window::delete(win.clone());
                }

                if confirm_btn.existPoint(x,y){
                    global_var::set_string(the_token.as_str(),rename_variable_input_copy.value());
                    fltk::window::Window::delete(win.clone());
                }

                update_variable_input!();
                true
            }

            enums::Event::Move => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;

                if(rename_variable_input_hotspot.existPoint(x,y)){
                    update_variable_input!();
                }

                //  判断鼠标是否在变量标签上
                let mut is_variable_tag_control = false;
                for variable in variable_list.iter() {
                    is_variable_tag_control = variable.existPoint(x, y);
                    if (is_variable_tag_control) {
                        break;
                    }
                }

                // 判断是否显示手型鼠标
                if exit_btn.existPoint(x, y)
                    || confirm_btn.existPoint(x, y)
                    || cancel_btn.existPoint(x, y)
                    || is_variable_tag_control
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

    win.show();

    the_token.clone()
}
#![allow(warnings, unused)]

use crate::gui_rename_ui::rename_tool_main;
use crate::{gui_util, libWxIkunPlus, global_var, wh_mod, get_arc_bind_variable, atomic_util, inject_fltk_theme, gui_drag_scan2_ui, gui_detect_config_ui, util};
use crate::gui_util::img::ImgPreview;
use crate::gui_util::text::TextControl;
use crate::gui_util::variable_tag_control::varTagControl;
use fltk::enums::{Color, FrameType};
use fltk::window::DoubleWindow;
use fltk::{prelude::*, *};
use fltk_theme::{color_themes, ColorTheme, SchemeType, ThemeType, WidgetScheme, WidgetTheme};
use crate::libWxIkunPlus::findWindow;
use std::ptr::null_mut;
use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::{
    sync::atomic::Ordering,
    sync::Arc,
    sync::MutexGuard,
    sync::{atomic::AtomicUsize, OnceLock},
};
use std::sync::atomic::AtomicBool;
use crate::gui_drag_scan2_ui::{get_history_attach_name, get_wx_temp_imag_id};
use crate::util::Sleep;

mod lib;

pub(crate) const THE_WINDOW_CLASS_NAME : &'static str = "wx_auto_ex_im::gui_util::select_user_ui::main<win:56315:>";
pub(crate) const THE_SUB_WINDOW_CLASS_NAME_FRAME_THUMBNAIL_PREVIEW: &'static str = "wx_auto_ex_im::gui_util::select_user_ui::sub_main<6103>";
pub(crate) const THE_SUB_WINDOW_CLASS_NAME_SCAN: &'static str = "wx_auto_ex_im::gui_util::select_user_ui::sub_main<126126>";
const MAIN_CONTOUR: &str = include_str!("./src/contour.svg");

pub fn ASSETS_NOT_DATA() -> Vec<u8> {
    include_bytes!("./src/not_data.png").to_vec()
}

pub fn ASSETS_DEMO_DATA() -> Vec<u8> {
    include_bytes!("../../assets/icon/demo.png").to_vec()
}

pub fn ASSETS_DEMO_NOT_DATA() -> Vec<u8> {
    include_bytes!("../../assets/icon/demo_not.png").to_vec()
}
pub fn ASSETS_DEMO_NOT_SELECT() -> Vec<u8> {
    include_bytes!("../../assets/icon/demo_not_select.png").to_vec()
}

pub fn ASSETS_NOT_SELECT () -> Vec<u8> {
    include_bytes!("./src/not_select.png").to_vec()
}

macro_rules! set_item_id {
    ($win:expr,$id:expr) => {
        $win.set_xclass($id);
        $win.set_id($id);
    };
}

macro_rules! eq_wxid_dir{
    ($select_dir:expr)=>{
        {
            let mut is_wxid_dir = false;
            if !$select_dir.is_empty(){

                        if !$select_dir.contains("WeChat Files"){
                            // 没有 WeChat Files 则尝试为路径添加 WeChat Files
                            let mut to_path = std::path::Path::new($select_dir.as_str());
                            let mut join_path = to_path.join("WeChat Files");

                            if join_path.exists() && join_path.is_dir(){
                               $select_dir.push_str("\\WeChat Files");
                            }

                        }

                        // 判断路径下是否有 wxid_ 开头的文件夹
                        if let Ok(rd_dir) = std::fs::read_dir($select_dir.as_str()) {

                            for rd_dir in rd_dir {
                                if let Ok(dir) = rd_dir {
                                    is_wxid_dir= dir.file_name().to_string_lossy().contains("wxid_");
                                    if is_wxid_dir{
                                        break;
                                    }
                                }
                            }

                            if !is_wxid_dir{
                                // dialog::alert_default("此路径可能不是有效的WX目录 因为未发现有效的用户数据");
                              gui_util::message::sub_message(libWxIkunPlus::findWindow(THE_WINDOW_CLASS_NAME,""),gui_util::message::IconType::Warning,"此WX目录 未发现有效的用户数据目录",3500u64);
                            }

                        }else{
                            gui_util::message::sub_message(libWxIkunPlus::findWindow(THE_WINDOW_CLASS_NAME,""),gui_util::message::IconType::Failure,"目录无法被打开 请注意路径有效性",3500u64);
                            // dialog::alert_default("目录无法被打开 请注意路径有效性");
                        }

                    }
                   
            is_wxid_dir
        }
    }
}

macro_rules! get_the_hwnd {
    ($class_id:expr) => {
        {
        let mut _hwnd = 0 ;
        for _ in 0..8 {
          _hwnd = libWxIkunPlus::findWindow($class_id,"");
            if !libWxIkunPlus::isWindow(_hwnd) {
                 _hwnd=0;
            }else {
              break;
          }
            fltk::app::sleep(0.020);
        }
        _hwnd as i128}
    };
    ()=>{
        get_the_hwnd!(THE_WINDOW_CLASS_NAME)
    }
}

pub fn has_window() -> bool{
    let hwnd = get_the_hwnd!(THE_WINDOW_CLASS_NAME);
    libWxIkunPlus::isWindow(hwnd)
}

struct FrameText {
    选择: TextControl,
    文件管理: TextControl,
    选择用户: TextControl,
    通过拽入获取: TextControl,
    选择最近对象: TextControl,
    帮助: TextControl,
    别名备注: TextControl,
    用户目录: TextControl,
    命名规则: TextControl,
    编辑规则: TextControl,
    完成选定: TextControl,
    备注: TextControl,
    开始:TextControl
}

// 添加无热点的文本
fn set_frame_text() -> FrameText {

    let mut preview = gui_util::img::ImgPreview::new(
        490-2, 167,
        18, 18,
        "gui_util::select_user_ui::imag<help>",
    );

    preview.from_svg(
        include_str!("./src/help.svg"),
        0,
        0,
        preview.preview.w(),
        preview.preview.h(),
    );

    FrameText {
        选择: TextControl::new(
            50 - 15,
            46,
            180,
            18,
            12,
            "请选择WX文件的默认保存位置*",
            [85, 85, 85],
        ),
        文件管理: TextControl::new(
            273,
            46,
            239,
            18,
            12,
            "此路径在：  设置  /  文件管理   /  文件管理(输入框)",
            [49,49,49],
        ),
        选择用户: TextControl::new(
            50 - 3 - 37,
            118,
            475,
            18,
            12,
            "选择用户*    [ 涉及用户数据安全   仅限已登录中的用户 并且能够收发的用户 ]",
            [85, 85, 85],
        ),
        选择最近对象: TextControl::new(
            59 ,
            207+5,
            465,
            18,
            12,
            "选择最近对象*  （如果不存在请发送一张随意的图片给对方 [不能是表情] ） ",
            [85, 85, 85],
        ),
        通过拽入获取: TextControl::new(366, 166, 85, 18, 12, "通过扫描获取", [85, 85, 85]),
        帮助: TextControl::new(510-2, 167, 30, 18, 12, "帮助", [85, 85, 85]),
        别名备注: TextControl::new(139, 398, 65, 18, 13, "别名备注:", [85, 85, 85]),
        用户目录: TextControl::new(139, 439 + 2, 65, 18, 13, "用户目录:", [85, 85, 85]),
        命名规则: TextControl::new(42, 525, 65, 18, 13, "命名规则:", [85, 85, 85]),
        编辑规则: TextControl::new(495, 523, 56, 18, 12, "编辑规则", [85, 85, 85]),
        完成选定: TextControl::new(495, 437, 58, 18, 12, "完成选定", [255, 255, 255]),
        备注: TextControl::new(513, 395, 30, 18, 13, "备注", [85, 85, 85]),
        开始: TextControl::new(451+15, 73+7, 30, 18, 12, "开始", [85, 85, 85]),
    }

}

struct FrameCheck {
    sync: button::CheckButton,
    video: button::CheckButton,
    thumbnail: button::CheckButton,
    source: button::CheckButton,
    the_month: button::CheckButton,
}

impl FrameCheck {
    pub fn existPoint(&self, x: i32, y: i32) -> bool {
        let self_x = 38;
        let self_y = 475;
        let self_w = 520;
        let self_h = 33;

        return x > self_x
            && x < self_x + self_w
            && y > self_y
            && y < self_y + self_h;
    }
}

// 条件选定
fn add_check_button() -> FrameCheck {
    let mut flex = group::Flex::default()
        .with_size(530, 30)
        .row()
        .center_of_parent();

    flex.set_pos(43, 479);

    let mut check_button_sync = button::CheckButton::default().with_label("启用同步");
    let mut check_button_video = button::CheckButton::default().with_label("转存视频");
    let mut check_button_thumbnail = button::CheckButton::default().with_label("存缩略图");
    let mut check_button_source = button::CheckButton::default().with_label("保存原图");
    let mut check_button_the_month = button::CheckButton::default().with_label("只保存本月");

    check_button_video.deactivate();

    check_button_sync.set_callback(|win|{
        global_var::set_bool("user::config::check_button_sync",win.is_checked());
    });

    check_button_video.set_callback(|win|{
        global_var::set_bool("user::config::check_button_video",win.is_checked());
    });
    
    check_button_thumbnail.set_callback(|win|{
        global_var::set_bool("user::config::check_button_thumbnail",win.is_checked());
    });
    
    check_button_source.set_callback(|win|{
        global_var::set_bool("user::config::check_button_source",win.is_checked());
    });
    
    check_button_the_month.set_callback(|win|{
        global_var::set_bool("user::config::check_button_the_month",win.is_checked());
    });

    check_button_source.set_checked(true);
    check_button_sync.set_checked(true);
    
    global_var::set_bool("user::config::check_button_source",true);
    global_var::set_bool("user::config::check_button_sync",true);

    flex.end();

    FrameCheck {
        sync: check_button_sync,
        video: check_button_video,
        thumbnail: check_button_thumbnail,
        source: check_button_source,
        the_month: check_button_the_month,
    }
}

struct FrameThumbnailPreview{
    hotspot_list:Vec<gui_util::hotspot::HotspotItmeControl>,
    thumbnail_list:Vec<ImgPreview>
}

// 缩略图列表
fn add_frame_thumbnail_preview() ->FrameThumbnailPreview {
    let mut preview_main_point_list = [
        [71, 296, 75, 75],
        [167, 296, 75, 75],
        [263, 296, 75, 75],
        [359, 296, 75, 75],
        [455, 296, 75, 75],
    ];

    let mut preview_main = Vec::new();
    let mut hotspot_list =Vec::new();
    // let mut preview_main2 = Vec::new();

    for index in 0..preview_main_point_list.len() {
        let point = preview_main_point_list[index];
        let [x, y, width, height] = point;

        let mut preview = ImgPreview::new(x, y - 52, width, height, "gui::preview_main::index::");
        let pre: Vec<u8> = if wh_mod::config::is_show_dome() { ASSETS_DEMO_NOT_DATA() } else{ ASSETS_NOT_DATA() };

        preview.from_data(
            pre,
            -1,
            -1,
            width - 2,
            height - 2,
        );

        preview_main.push(preview);
        // preview_main2.push(preview.clone());
        hotspot_list.push(gui_util::hotspot::create_hotspot(x, y - 52, width, height));
    }

    lib::initialize_img_preview_list(&preview_main);

    FrameThumbnailPreview{
        hotspot_list:hotspot_list,
        thumbnail_list:preview_main,
    }
}

// 添加背景页
macro_rules! add_preview_contour {
    ($win:expr) => {{
        let mut preview = gui_util::img::ImgPreview::new(
            0,
            0,
            $win.w(),
            $win.h(),
            "gui_util::select_user_ui::main<win>",
        );
        preview.from_svg(
            MAIN_CONTOUR,
            0,
            0,
            preview.preview.w(),
            preview.preview.h(),
        );
        preview
            .preview
            .set_id("gui_util::select_user_ui::main<contour>");
        preview
    }};
}

// 用户选择
fn select_user_data_choice() -> menu::Choice {
    let mut choice = menu::Choice::default().with_size(277, 35).center_of_parent().with_label("");
    choice.set_pos(60,158);
    choice.add_choice("请点击 [开始] 获取在线用户列表");
    choice.set_value(0);
    choice.set_color(Color::from_rgb(23, 23, 23));

    choice
}

struct AttachThumbnailPreview{
    thumbnail_preview: ImgPreview,
    btn_remark: gui_util::hotspot::HotspotItmeControl,
    btn_rename: gui_util::hotspot::HotspotItmeControl,
    btn_select: gui_util::hotspot::HotspotItmeControl,
    input_rename: fltk::input::Input,
    input_remark: fltk::input::Input,
    input_attach: fltk::input::Input,
}

impl AttachThumbnailPreview {
    pub fn gc(&mut self) {
        let pre: Vec<u8> = if wh_mod::config::is_show_dome() { ASSETS_DEMO_NOT_SELECT() } else{ ASSETS_NOT_SELECT() };

        self.thumbnail_preview.from_data(
            pre,
            -1,
            -1,
            self.thumbnail_preview.preview.w()-2,
            self.thumbnail_preview.preview.h()-2,
        );

        self.input_remark.set_value("");
        self.input_attach.set_value("");
        global_var::set_string("user::config::user_select_attach",String::new());
        // self.input_rename.set_value("");

    }
    fn clone(&self) -> Self {
        AttachThumbnailPreview{
            thumbnail_preview: self.thumbnail_preview.clone(),
            btn_remark: self.btn_remark.clone(),
            btn_rename: self.btn_rename.clone(),
            btn_select: self.btn_select.clone(),
            input_rename: self.input_rename.clone(),
            input_remark: self.input_remark.clone(),
            input_attach: self.input_attach.clone(),
        }
    }
    fn redata(&mut self,thumbnail:wh_mod::AttachThumbnail){
        self.input_remark.set_value("");
        self.input_attach.set_value("");
        global_var::set_string("user::config::user_select_attach",String::new());

        let pre: Vec<u8> = if wh_mod::config::is_show_dome() { ASSETS_DEMO_DATA() } else{ thumbnail.thumbnail.to_vec() };

        // self.input_rename.set_value("");
        // 设置预览图
        self.thumbnail_preview.from_data(pre,-1,
                                                       -1,
                                         self.thumbnail_preview.preview.width()-2,
                                         self.thumbnail_preview.preview.height()-2,);
        // 绑定内容
        self.input_attach.set_value(wh_mod::get_show_mask_text( &thumbnail.attach_id).as_str());
        global_var::set_string("user::config::user_select_attach",thumbnail.attach_id.clone());

        let retrieval_struct = wh_mod::wx_parse_path(thumbnail.thumbnail_path.to_string());

        // 获取备注
        if let Some(user_remark) = lib::get_store_user_remark(retrieval_struct.wxid,thumbnail.attach_id.clone()) {
            self.input_remark.set_value(user_remark.as_str())
        }

    }
}

// 用户选定预览的卡片(底部)
fn add_select_attach_card() -> AttachThumbnailPreview {
    let mut preview = gui_util::img::ImgPreview::new(
        41+3, 385,
        82, 82,
        "gui_util::select_user_ui::imag<add_select_attach_card>",
    );
    let pre: Vec<u8> = if wh_mod::config::is_show_dome() { ASSETS_DEMO_NOT_SELECT() } else{ ASSETS_NOT_SELECT() };

    preview.from_data(pre,
        -1,
        -1,
        preview.preview.w()-2,
        preview.preview.h()-2,
    );

    let mut button_remark = gui_util::hotspot::create_hotspot(495, 387 , 66, 32);
    let mut button_select = gui_util::hotspot::create_hotspot(484, 430 , 77, 32);
    let mut button_rename = gui_util::hotspot::create_hotspot(484, 516 , 77, 32);

    // 命名规则
    let mut rename_input = input::Input::new(115, 518, 357, 30, "");

    // 备注输入框
    let mut remark_input = input::Input::new(211, 389, 275, 30, "");

    // 用户目录
    let mut user_data_preview = input::Input::new(213, 432, 263, 30, "");
    user_data_preview.set_readonly(true);

    AttachThumbnailPreview{
        thumbnail_preview:preview,
        btn_remark:button_remark,
        btn_rename:button_rename,
        btn_select:button_select,
        input_rename:rename_input,
        input_remark:remark_input,
        input_attach:user_data_preview
    }

}

macro_rules! set_select_user_base_input_default{
    ($input_select_dir:expr)=>{{

        let mut _path = String::new();
        if let Ok(history) = lib::get_wx_user_history_path() {

            let paths = history.path;
             _path = format!("{}",paths.as_str());
            $input_select_dir.set_value(wh_mod::get_show_mask_text(paths.as_str()).as_str());

        }
        if ($input_select_dir.value().is_empty()) {
            if let Some(paths) = wh_mod::convert::get_user_data_path() {
                _path = format!("{}",paths.as_str());
                $input_select_dir.set_value(wh_mod::get_show_mask_text(paths.as_str()).as_str());
            }
        }

        global_var::set_string("user::config::user_select_path",_path);

    }}
}

macro_rules! initialize_window_hwnd{
    ($hwnd:expr)=>{
        if $hwnd==0{
            $hwnd =  libWxIkunPlus::findWindow(THE_WINDOW_CLASS_NAME, "");
            libWxIkunPlus::setWinIcon($hwnd);
            lib::initialize_gc_select_user_ui($hwnd);
            println!("[initialize-window] {} -> {}",THE_WINDOW_CLASS_NAME, $hwnd);
        }
    }
}

struct PreviewData{
    preview_list:Vec<ImgPreview>,
    preview_main: ImgPreview,
}

impl PreviewData {

    pub fn gc_data(&mut self) {
        let pre: Vec<u8> = if wh_mod::config::is_show_dome() { ASSETS_DEMO_NOT_DATA() } else{ ASSETS_NOT_DATA() };

        let mut index = 0;
        for mut preview in self.preview_list.clone() {
            index+=1;
            let mut w = 90;
            if index==1 {
                w+=1;
            }
            if index==4 {
                w+=1;
            }
            if index==6 {
                w+=1;
            }
            preview.from_data(pre.clone(),-1,-1,w ,90 - 2,);
        }
        self.preview_main.from_data(pre , -1, -1, 230-2 , 230 - 2, );
    }

    fn clone(&self) -> Self {
        PreviewData{
            preview_list: self.preview_list.clone(),
            preview_main: self.preview_main.clone(),
        }
    }

    fn redata(&mut self,thumbnail_list:Vec<wh_mod::AttachThumbnail>){
        // self.gc_data();

        if let Some(main_thumbnail) = thumbnail_list.get(0) {
            let pre: Vec<u8> = if wh_mod::config::is_show_dome() { ASSETS_DEMO_DATA() } else{ main_thumbnail.thumbnail.to_vec() };
            self.preview_main.re_data(pre/*, -1, -1, 230-2 , 230 - 2, */);

        }

        let mut index = 0;
        for mut preview in self.preview_list.clone() {
            index+=1;
            // let mut w = 90;
            // // 第一列有1px的误差
            // if index==1 ||index==4||index==7{
            //     w+=1;
            // }

            if let Some(thumbnail) = thumbnail_list.get(index) {
                let pre: Vec<u8> = if wh_mod::config::is_show_dome() { ASSETS_DEMO_DATA() } else{ thumbnail.thumbnail.to_vec() };

                preview.re_data(pre/*,-1,-1,w ,90 - 2,*/);

            }else {
                let pre: Vec<u8> = if wh_mod::config::is_show_dome() { ASSETS_DEMO_NOT_DATA() } else{ ASSETS_NOT_DATA() };
                preview.re_data(pre/*,-1,-1,w ,90 - 2,*/);
            }

        }

    }

    pub fn existPoint(&self, x: i32, y: i32) -> bool {
        let preview_pint = [
            [35,280,85,85],[35,381,85,85],[35,482,85,85],
            [137,280,85,85],[137,381,85,85],[137,482,85,85],
            [239,280,85,85],[239,381,85,85],[239,482,85,85]
        ];

        for preview in preview_pint {
            if
            x > preview[1]
                && x < preview[1] + 85
                && y > preview[0]
                && y < preview[0] + 85
            {
                return  true
            }
        }
        false
    }

    pub fn existPointIndex(&self, x: i32, y: i32) -> usize {
        let mut index = 0 ;
        let preview_pint = [
            [35,280,85,85],[35,381,85,85],[35,482,85,85],
            [137,280,85,85],[137,381,85,85],[137,482,85,85],
            [239,280,85,85],[239,381,85,85],[239,482,85,85]
        ];

        for preview in preview_pint {
            index+=1;

            if
            x > preview[1]
                && x < preview[1] + 85
                && y > preview[0]
                && y < preview[0] + 85
            {
                return  index
            }

        }
        0
    }
}

// 九宫格
fn preview_main_list() ->PreviewData{

    let mut preview_main = gui_util::img::ImgPreview::new2(35,35,230,230,"",-1, -1, 230-2 , 230 - 2);

    // 九宫格位置预设
    let preview_pint = [
        [[35,280,85,85],[35,381,85,85],[35,482,85,85]],
        [[137,280,85,85],[137,381,85,85],[137,482,85,85]],
        [[239,280,85,85],[239,381,85,85],[239,482,85,85]]
    ];

    let mut flex = group::Flex::default()
        .with_size(287, 90)
        .row()
        .center_of_parent();

    flex.set_pos(280, 35);

    let mut preview_main_1 = gui_util::img::ImgPreview::new2(0,0,85,85,"",-1,-1,91 ,90 - 2);
    let mut preview_main_2 = gui_util::img::ImgPreview::new2(0,0,85,85,"",-1,-1,90 ,90 - 2);
    let mut preview_main_3 = gui_util::img::ImgPreview::new2(0,0,85,85,"",-1,-1,90 ,90 - 2);
    flex.end();

    let mut flex = group::Flex::default()
        .with_size(287, 90)
        .row()
        .center_of_parent();

    flex.set_pos(280, 137);

    let mut preview_main_4 = gui_util::img::ImgPreview::new2(0,0,85,85,"",-1,-1,91 ,90 - 2);
    let mut preview_main_5 = gui_util::img::ImgPreview::new2(0,0,85,85,"",-1,-1,90 ,90 - 2);
    let mut preview_main_6 = gui_util::img::ImgPreview::new2(0,0,85,85,"",-1,-1,90 ,90 - 2);
    flex.end();

    let mut flex = group::Flex::default()
        .with_size(287, 90)
        .row()
        .center_of_parent();

    flex.set_pos(280, 239);

    let mut preview_main_7 = gui_util::img::ImgPreview::new2(0,0,85,85,"",-1,-1,91 ,90 - 2);
    let mut preview_main_8 = gui_util::img::ImgPreview::new2(0,0,85,85,"",-1,-1,90 ,90 - 2);
    let mut preview_main_9 = gui_util::img::ImgPreview::new2(0,0,85,85,"",-1,-1,90 ,90 - 2);

    flex.end();

   let result = vec![preview_main_1,preview_main_2,preview_main_3,preview_main_4,preview_main_5,preview_main_6,preview_main_7,preview_main_8,preview_main_9];

   let mut  preview_data =  PreviewData{
        preview_list: result,
        preview_main
    };

    preview_data.gc_data();

    preview_data
}

struct ThumbnailPreviewMain{
    main:DoubleWindow,
    btn_close: gui_util::hotspot::HotspotItmeControl,
    preview_list: PreviewData,
}

// 图片预览窗口 九宫格
fn add_frame_thumbnail_preview_main (mut win: &DoubleWindow) -> ThumbnailPreviewMain {
    // 图片预览窗口
    let mut preview_win = fltk::window::Window::new(0,0,win.w(),359,"");
    preview_win.set_color(Color::from_rgb(23, 23, 23));
    set_item_id!(preview_win,THE_SUB_WINDOW_CLASS_NAME_FRAME_THUMBNAIL_PREVIEW);

    let mut preview_win_border = gui_util::img::ImgPreview::new(0,0,preview_win.w(),preview_win.h(),"");
    preview_win_border.from_svg(include_str!("./src/preview_win.svg"),0,0,preview_win.w(),preview_win.h());

    let mut preview_main_close_btn = gui_util::hotspot::create_hotspot(82,282,130,32);
    let mut preview_main_list = preview_main_list();
    gui_util::text::TextControl::new(82,283,130,32,13,"关闭预览",[121, 121, 121]);

    preview_win.end();
    preview_win.hide();

    ThumbnailPreviewMain{
        main:preview_win,
        btn_close:preview_main_close_btn,
        preview_list:preview_main_list,

    }
}

// 通过扫描获取的界面
struct ScanPreviewMain{
    main:fltk::window::DoubleWindow,
    btn_all_obj: gui_util::hotspot::HotspotItmeControl,
    btn_clip:  gui_util::hotspot::HotspotItmeControl,
    btn_close: gui_util::hotspot::HotspotItmeControl,
    text_bottom:Vec<gui_util::text::TextControl>,
    progress_bar: ImgPreview,
    all_text_list: Vec<TextControl>,
    btn_scan_drag: gui_util::hotspot::HotspotItmeControl,
}

impl ScanPreviewMain {
    pub fn existPoint(&self, x: i32, y: i32) -> bool {
        let self_x = self.main.x();
        let self_y = self.main.y();
        let self_w = self.main.w();
        let self_h = self.main.h();

        return x > self_x
            && x < self_x + self_w
            && y > self_y
            && y < self_y + self_h;
    }

    pub fn show_progress(& mut self){
        for (mut index) in 0..3 {
            if let Some(text_bottom) = self.text_bottom.get_mut(index) {
                text_bottom.text.hide();
            }
        }
        self.progress_bar.preview.show();
    }

    pub fn show_bottom_text(& mut self){
        for (mut index) in 0..3 {
            if let Some(text_bottom) = self.text_bottom.get_mut(index) {
                text_bottom.text.show();
            }
        }
        self.progress_bar.preview.hide();
    }
}

// 進度條
fn show_progress_bar_border(x: i32, y: i32) -> gui_util::img::ImgPreview {
    let mut progress_bar_border = gui_util::img::ImgPreview::new_border(x,y,520,15,"<svg width=\"520\" height=\"15\" viewBox=\"0 0 520 15\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\"> <rect x=\"0.5\" y=\"0.5\" width=\"520\" height=\"14\" rx=\"7\" fill=\"#181818\" stroke=\"#2C2C2C\"/> </svg> ");
    let width = progress_bar_border.width;
    let mut progress_bar_border_slider = gui_util::img::ImgPreview::new_border(x, y, 41, 15, "<svg width=\"40\" height=\"15\" viewBox=\"0 0 40 15\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\">\n<rect x=\"0.5\" y=\"0.5\" width=\"39\" height=\"14\" rx=\"7\" fill=\"#333333\" stroke=\"#2C2C2C\"/>\n</svg>");
    progress_bar_border_slider.preview.hide();
    progress_bar_border.preview.hide();

    progress_bar_border.preview.handle({
        let mut progress_bar_border_slider = progress_bar_border_slider.clone();
        let mut preview_main = progress_bar_border.preview.clone();

        move |win, ev| match ev {
            enums::Event::Show => {
                progress_bar_border_slider.preview.show();

                let mut preview = progress_bar_border_slider.preview.clone();
                let mut preview_main = preview_main.clone();

                app::add_timeout3(0.0086, move |handle| {
                    if !preview.visible()||!preview_main.visible() {
                        preview_main.hide();
                        preview.hide();
                        app::remove_timeout3(handle);
                        return;
                    }

                    let mut the_x = preview.x() + 2;
                    if the_x > width {
                        the_x = x + preview.width();
                        the_x -= preview.width();
                    }
                    preview.set_pos(the_x, preview.y());
                    preview.parent().unwrap().redraw();
                    app::repeat_timeout3(0.0086, handle);
                });
                true
            }
            enums::Event::Hide => {
                progress_bar_border_slider.preview.hide();
                true
            }
            _ => false,
        }
    });

    // progress_bar_border.preview.show();
    progress_bar_border
}

// 通过扫描获取的界面
fn add_scan_preview_window() -> ScanPreviewMain {
    // 图片预览窗口
    let mut preview_win = fltk::window::Window::new(0,0,600,359,"");
    preview_win.set_color(Color::from_rgb(23, 23, 23));
    set_item_id!(preview_win,THE_SUB_WINDOW_CLASS_NAME_SCAN);

    let mut preview_win_border = gui_util::img::ImgPreview::new(0,0,preview_win.w(),preview_win.h(),"");
    preview_win_border.from_svg(include_str!("./src/scan_cursor.svg"),0,0,preview_win.w(),preview_win.h());

    let mut show_all_user_obj = gui_util::hotspot::create_hotspot(83,231,87,40);
    let mut get_clip_data = gui_util::hotspot::create_hotspot(432,231,87,40);
    let mut scan_drag_data = gui_util::hotspot::create_hotspot(256,231,87,40);
    let mut btn_close = gui_util::hotspot::create_hotspot(537,33,25,25);

    let mut text_list = Vec::new();

    // 標題
    text_list.push(gui_util::text::TextControl::new(150-25,33,345,15,12,"请选择一种您喜欢的方式扫描聊天对象",[149, 149, 149]));
    text_list.push(gui_util::text::TextControl::new(130,58,345,15,12,"如果您有很多好友需要动态管理可以设置别名 在所有好友中可显示别名备注",[68, 68, 68]));

    //  卡片中上
    text_list.push(gui_util::text::TextControl::new(57,170+3 ,135,15,12,"所有存在图片的聊天对象",[149, 149, 149]));
    text_list.push(gui_util::text::TextControl::new(257,170+3 ,85,15,12,"拖拽聊天的图片",[149, 149, 149]));
    text_list.push(gui_util::text::TextControl::new(442,170+3 ,64,15,12,"剪贴板获取",[149, 149, 149]));

    // 卡片中下
    text_list.push(gui_util::text::TextControl::new(87,196+5,78,15,13,"查看所有人",[121, 121, 121]));
    text_list.push(gui_util::text::TextControl::new(255,196+5,91,15,13,"打开扫描窗口",[121, 121, 121]));
    text_list.push(gui_util::text::TextControl::new(410,198+5,135,15,12,"复制一张图片后点击开始",[121, 121, 121]));

    // 卡片下面
    text_list.push(gui_util::text::TextControl::new(111,239+5,31,15,13,"开始",[255, 255, 255]));
    text_list.push(gui_util::text::TextControl::new(256+(460-432),239+5,31,15,13,"开始",[255, 255, 255]));
    text_list.push(gui_util::text::TextControl::new(460,239+5,31,15,13,"开始",[255, 255, 255]));

    // 底部三言 / 進度條
    let mut text_01 = gui_util::text::TextControl::new(70,308,115,15,13,"聊天对象选择面板",[195, 195, 195]);
    let mut text_02 = gui_util::text::TextControl::new(260,308,85,15,13,"通过拖拽查找",[195, 195, 195]);
    let mut text_03 = gui_util::text::TextControl::new(435,308,85,15,13,"粘贴文件查找",[195, 195, 195]);

    let mut progress_bar_border = show_progress_bar_border(40,312);

    preview_win.end();
    preview_win.hide();

    preview_win.handle({
        let mut preview = progress_bar_border.preview.clone();
        move |win, ev| match ev {
            enums::Event::Show=>{
                true
            }
            enums::Event::Hide=>{
                preview.hide();
                true
            }
            _ => false
        }

    });

    ScanPreviewMain{
        main:preview_win,
        btn_clip:get_clip_data,
        btn_all_obj:show_all_user_obj,
        btn_scan_drag:scan_drag_data,
        btn_close,
        all_text_list:text_list,
        text_bottom: vec![text_01,text_02,text_03],
        progress_bar:progress_bar_border
    }
}

fn initialize_watch_walk_drag_path (mut preview1: AttachThumbnailPreview) {
    let mut oid_walk_drag_path = String::new();
    std::thread::spawn( move || loop{
        if !has_window(){
            lib::gc_select_user_ui();
            return;
        }

        let walk_drag_path = global_var::get_string_default("user::config::walk_drag_path");
        if(!oid_walk_drag_path.as_bytes().eq(walk_drag_path.as_bytes())){
            oid_walk_drag_path.clear();
            oid_walk_drag_path.push_str(walk_drag_path.as_str());
            let wx_parse = wh_mod::wx_parse_path(walk_drag_path.clone());
            // println!("wx_parse-> {:?}",&wx_parse);
            preview1.input_attach.set_value(wh_mod::get_show_mask_text(&wx_parse.attach_id).as_str());
            global_var::set_string("user::config::user_select_attach",format!("{}",wx_parse.attach_id.as_str()));

            if let Some(remark) = lib::get_store_user_remark(wx_parse.wxid,wx_parse.attach_id.clone()) {
                preview1.input_remark.set_value(remark.as_str());
            }else {
                preview1.input_remark.set_value("");
            }

            if let Ok(buff_thumbnail_data) = wh_mod::convert::convert_dat_images_buff(std::path::PathBuf::from(walk_drag_path.as_str())) {
                let pre: Vec<u8> = if wh_mod::config::is_show_dome() { ASSETS_DEMO_DATA() } else{ buff_thumbnail_data };

                preview1.thumbnail_preview.from_data(pre,-1, -1, 80, 80);
            }

            // println!("walk_drag_path->{}",&walk_drag_path);
        }

        if !gui_drag_scan2_ui::has_window() {
            return;
        }

        Sleep(500);
    });
}

macro_rules! the_token {
    ()=>{
       {
        let mut _the_token =format!("[select_user_ui]token<{}>@query",libWxIkunPlus::randomNum());
        loop{
            if global_var::has_string(_the_token.as_str()) {
                _the_token = format!("[select_user_ui]token<{}>@query",libWxIkunPlus::randomNum());
            }else{
                break;
            }
        }
            _the_token
        }
    }
}

pub fn manage_tool_main() -> String{
    let the_token = the_token!();
    // 禁止创建多个窗口
    if let hwnd = libWxIkunPlus::findWindow(THE_WINDOW_CLASS_NAME, "") {
        if hwnd!=0 && libWxIkunPlus::isWindow(hwnd) {
            if let Some(mut win) =app::widget_from_id(THE_WINDOW_CLASS_NAME) as Option<DoubleWindow>
             {
                 win.show();
                 win.set_visible_focus();
             }
             libWxIkunPlus::setWindowShake(hwnd);
            global_var::set_string(the_token.as_str(),String::new());
            return the_token;
        }
    }

    let mut hwnd :i128 = 0;
    let mut win = window::DoubleWindow::new(0, 0,600, 595,None).center_screen();
    win.set_color(Color::from_rgb(23, 23, 23));

    inject_fltk_theme!();
    win.set_label("任务创建向导");
    set_item_id!(win, THE_WINDOW_CLASS_NAME);
    
    let mut g_the_select_wxid = String::new();
    let mut g_the_select_attach_id = String::new();
    let mut preview_win =add_frame_thumbnail_preview_main(&win);
    let mut scan_preview_window = add_scan_preview_window();

    let mut preview = add_preview_contour!(win);

    // 固定文本
    let mut frame_text = set_frame_text();
    // 条件选定
    let mut frame_check = add_check_button();
    // 五张缩略图
    let mut frame_thumbnail_preview = add_frame_thumbnail_preview();
    // 用户选择数据库
    let mut select_user_data_choice = select_user_data_choice();
    // 预览卡片
    let mut select_attach_card  = add_select_attach_card();

    // 文件的默认保存位置(D:\...\...\WeChat Files)
    let mut user_select_database_dir_input = input::Input::new(45+3, 74, 395, 30, "");
    set_select_user_base_input_default!(user_select_database_dir_input);

    // 按钮 > 打开文件选择
    let mut button_open_dir = gui_util::hotspot::create_hotspot(516, 73 , 33, 32);
    // 按钮 > 扫描获取
    let mut button_show_drag = gui_util::hotspot::create_hotspot(346, 156 , 123, 38);
   
    // 按钮 > 帮助
    let mut button_show_help = gui_util::hotspot::create_hotspot(479, 156 , 66, 38);
    // 按钮 > 开始
    let mut button_start = gui_util::hotspot::create_hotspot(451, 73 , 57, 32);

    select_attach_card.input_rename.set_value("<创建月>/<任务名>/<类型>_<NN>");

    let mut move_select_attach_card = select_attach_card.clone();

    select_user_data_choice.set_callback(move |c| {
        if let Some(choice_value) = c.choice() {
            
            if let Some(item) = lib::get_active_user_list().get(c.value() as usize).clone() {
            move_select_attach_card.gc();
            c.deactivate();

            lib::store_wx_user_path_history(item.user_root.clone(),item.user_wxid.clone());
            
            println!("[{}] {}  , {}  ,  {} ",c.value(), choice_value.as_str() ,&item.user_root,&item.user_wxid);
            
            global_var::set_string("user::config::user_select_path", item.user_root.clone());
            global_var::set_string("user::config::user_select_wxid", item.user_wxid.clone());

            lib::initialize_thumbnail_preview(item.user_root.as_str(),item.user_wxid.as_str());
            lib::initialize_watch_path_puppet(format!("{}\\{}", &item.user_root, &item.user_wxid));

            c.activate();
            }
            
            
        }
    });

    let mut drag_path = std::path::PathBuf::new();



    win.end();
    win.show();
    // preview_win.show();

    win.handle({
        let mut x = 0;
        let mut y = 0;
        // 是否显示手型
        let mut show_cursor = false;
        let mut preview_win_show = false;
        let mut scan_win_show = false;

        let mut released = true;
        let mut dnd = true;
        let the_token =the_token.clone();
        let move_select_attach_card2 = select_attach_card.clone();

        move |win, ev| match ev {

            enums::Event::Focus=>{
                initialize_window_hwnd!(hwnd);
                true
            }

            enums::Event::Show => {
                initialize_window_hwnd!(hwnd);
                true
            }

            enums::Event::KeyUp => true,

            enums::Event::Close=>{
                lib::gc_select_user_ui();
                true
            }

            enums::Event::Push => {

                macro_rules! add_preview_win_show {
                    ()=>{
                        preview_win_show = true;
                        user_select_database_dir_input.hide();
                        select_user_data_choice.hide();
                        let path = PathBuf::from(global_var::get_string_default("user::config::user_select_path"))
                            .join(global_var::get_string_default("user::config::user_select_wxid"))
                            .join("FileStorage\\MsgAttach")
                            .join(g_the_select_attach_id.as_str())
                            ;
                        let dat  = wh_mod::read_attach_buff_thumbnail_data(&path,10);
                        println!("dat<{}> path:<{:?}>",dat.len(),&path);
                        preview_win.preview_list.redata(dat);
                        preview_win.main.show();
                    }
                }
                if preview_win_show {
                    if preview_win.btn_close.existPoint(x,y) {
                        preview_win_show =false;
                        preview_win.main.hide();
                        user_select_database_dir_input.show();
                        select_user_data_choice.show();
                        preview_win.preview_list.gc_data();
                    }

                }
                else if scan_win_show {

                    // 关闭扫描窗口
                    if scan_preview_window.btn_close.existPoint(x,y){
                        scan_preview_window.main.hide();
                        // user_select_database_dir_input.activate();
                        // select_attach_card.input_rename.activate();
                        // select_attach_card.input_remark.activate();
                        scan_preview_window.show_bottom_text();
                        scan_win_show = false;
                    }

                    if !scan_preview_window.progress_bar.preview.visible(){
                        if scan_preview_window.btn_all_obj.existPoint(x,y) {
                            gui_util::message::sub_message(hwnd,gui_util::message::IconType::Warning,"作者正在玩命的开发中。。。",3500u64);
                        }
                        if scan_preview_window.btn_scan_drag.existPoint(x,y) {
                            if global_var::get_string_default("user::config::user_select_wxid").is_empty() {
                                gui_util::message::sub_message(hwnd,gui_util::message::IconType::Warning,"尚未选择用户",3500u64);
                                return false;
                            }
                            if global_var::get_string_default("user::config::user_select_path").is_empty() {
                                gui_util::message::sub_message(hwnd,gui_util::message::IconType::Warning,"没有选择WX根目录",3500u64);
                                return false;
                            }
                            // gui_drag_scan::main_window();
                            gui_drag_scan2_ui::main_window("");
                            initialize_watch_walk_drag_path (move_select_attach_card2.clone());
                        }

                        if  scan_preview_window.btn_clip.existPoint(x,y) {
                            let clip_file_path_single = libWxIkunPlus::getClipFilePathSingle();
                            if clip_file_path_single.is_empty() {
                                gui_util::message::sub_message(hwnd,gui_util::message::IconType::Warning,"当前剪贴板无内容或者不是可识别格式",3500u64);
                            }else{

                                if global_var::get_string_default("user::config::user_select_wxid").is_empty() {
                                    gui_util::message::sub_message(hwnd,gui_util::message::IconType::Warning,"尚未选择用户",3500u64);
                                    return false;
                                }
                                if global_var::get_string_default("user::config::user_select_path").is_empty() {
                                    gui_util::message::sub_message(hwnd,gui_util::message::IconType::Warning,"没有选择WX根目录",3500u64);
                                    return false;
                                }

                                // scan_preview_window.show_progress();
                                drag_path.clear();
                                drag_path.push(clip_file_path_single.clone());
                                println!("getClipFilePathSingle-> {:?}",&drag_path);

                                // 获取id 并开始处理
                                let temp_imag_id = get_wx_temp_imag_id(clip_file_path_single.as_str());

                                if !temp_imag_id.is_empty() {
                                    gui_drag_scan2_ui::main_window(clip_file_path_single.as_str());
                                    initialize_watch_walk_drag_path (move_select_attach_card2.clone());
                                }

                            }

                        }

                    }else{
                        gui_util::message::sub_message(hwnd,gui_util::message::IconType::Failure,"当前正在扫描中 不能使用此功能",3500u64);
                    }
                }
                // 正常窗口
                else {
                    // 选择最近5个对象
                    {
                        let mut index = 0;
                        for hotspot in &frame_thumbnail_preview.hotspot_list {
                            index += 1;
                            let thumbnail_list = lib::get_thumbnail_list();
                            if hotspot.existPoint(x, y) {
                                select_attach_card.gc();

                                if let Some(thumbnail) = thumbnail_list.get(index - 1usize) {
                                    println!("[click] frame_thumbnail_preview -> {}", index);
                                    global_var::set_i32("user::config::select_user_thumbnail_obj", index as i32);
                                    select_attach_card.redata(thumbnail.clone());
                                    g_the_select_attach_id.clear();
                                    g_the_select_attach_id.push_str(thumbnail.attach_id.as_str());

                                    if wh_mod::config::is_click_open_preview() {
                                        add_preview_win_show!();
                                    }
                                    break;
                                } else {
                                    gui_util::message::sub_message(hwnd, gui_util::message::IconType::Warning, "没有选中聊天对象", 3500u64);
                                }
                            }
                        }
                    }

                    // 打开文件夹选择器
                    if button_open_dir.existPoint(x, y) {
                        let mut select_dir = libWxIkunPlus::openSelectFolder2();
                        let eq_wxid_dir = eq_wxid_dir!(select_dir);
                        if !select_dir.is_empty() {
                            user_select_database_dir_input.set_value(wh_mod::get_show_mask_text(select_dir.as_str()).as_str());

                            select_user_data_choice.clear();
                            select_user_data_choice.add_choice("请点击 [开始] 获取在线用户列表");
                            select_user_data_choice.set_value(0);
                            lib::set_active_user_list(Vec::new());
                            global_var::set_string("user::config::user_select_path", select_dir.clone());

                            println!("[click] existPoint {}  select_dir-> {} eq-> {}", "打开文件夹选择器", select_dir, eq_wxid_dir);
                        } else {
                            gui_util::message::sub_message(libWxIkunPlus::findWindow(THE_WINDOW_CLASS_NAME, ""), gui_util::message::IconType::Info, "用户取消", 3500u64);
                        }
                    }

                    // 显示帮助面板
                    if button_show_help.existPoint(x, y) {
                        println!("[click] existPoint {}", "");
                        gui_detect_config_ui::main_window();
                    }

                    // 图片预览大图
                    if select_attach_card.thumbnail_preview.existPoint(x, y) {

                        let attach_id = global_var::get_string_default("user::config::user_select_attach");//select_attach_card.input_attach.value();
                        if attach_id.is_empty() {
                            // dialog::alert_default("没有选择聊天对象 (attach ID)");
                            gui_util::message::sub_message(hwnd, gui_util::message::IconType::Warning, "没有选择聊天对象 (attach ID)", 3500u64);

                            return false;
                        }

                        add_preview_win_show!();

                    }

                    // 开始
                    if button_start.existPoint(x, y)||(button_show_drag.existPoint(x, y)&&
                        (
                            global_var::get_string_default("user::config::user_select_wxid").is_empty()||
                            user_select_database_dir_input.value().is_empty()
                        )
                    ) {
                        select_attach_card.gc();

                        if !user_select_database_dir_input.value().is_empty() {
                            if (!libWxIkunPlus::hasWeChat() || !libWxIkunPlus::hasWeChatWin()) {
                                // fltk::dialog::alert_default("尚未找到已经登录的WX进程 为避免滥用 扫描被拒绝 ");
                                gui_util::message::sub_message(hwnd, gui_util::message::IconType::Failure, "WX未登录 为避免滥用 面板开启请求被拒绝", 3500u64);
                                return false;
                            }

                            // 释放资源
                            select_user_data_choice.activate();
                            lib::set_active_user_list(Vec::new());

                            select_user_data_choice.clear();
                            select_user_data_choice.add_choice("【状态】  当前正在扫描用户列表... ");
                            select_user_data_choice.set_value(0);

                            let mut user_select_database = global_var::get_string_default("user::config::user_select_path");
                            let mut user_select_database_input = user_select_database_dir_input.value();

                            // 没有* 则引用input内容 并重新设置input消敏
                            if !user_select_database_input.contains("*") {
                                let mut new_path = PathBuf::from(user_select_database_input.as_str());

                                if !user_select_database_input.contains("WeChat Files") {
                                    let new_data = new_path.clone().join("WeChat Files");
                                    if new_data.exists() {
                                        new_path = new_data;
                                    }
                                }

                                if(new_path.exists()){
                                    user_select_database = util::to_string_default(&new_path);
                                    global_var::set_string("user::config::user_select_path",user_select_database.to_string());

                                    user_select_database_dir_input.set_value(wh_mod::get_show_mask_text(&user_select_database).as_str());

                                }
                            }

                            if !user_select_database.is_empty() {
                                let active_user_list = wh_mod::convert::get_active_user(user_select_database.as_str());
                                select_user_data_choice.clear();

                                if active_user_list.is_empty() {
                                    select_user_data_choice.add_choice("【状态】  未找到用户列表");
                                    select_user_data_choice.set_value(0);
                                    gui_util::message::sub_message(libWxIkunPlus::findWindow(THE_WINDOW_CLASS_NAME, ""), gui_util::message::IconType::Failure, "未找到用户列表 请注意路径有效性", 3500u64);
                                    return false;
                                }

                                // 添加到列表
                                for active_user in active_user_list {
                                    if let Some(accinfo) = active_user.accinfo.clone() {
                                        select_user_data_choice.add_choice(format!("{} <{}>", wh_mod::get_show_mask_text(&accinfo.wx_id), wh_mod::get_show_mask_text(&accinfo.name)).as_str());
                                    } else {
                                        select_user_data_choice.add_choice( wh_mod::get_show_mask_text(&active_user.user_data).as_str());
                                    }

                                    lib::push_active_user_list(active_user.clone());
                                }

                                // 开始扫描
                                if let Some(item) = lib::get_active_user_list().get(0) {
                                    select_user_data_choice.set_value(0);
                                    lib::store_wx_user_path_history(item.user_root.clone(), item.user_wxid.clone());
                                    global_var::set_string("user::config::user_select_path", item.user_root.clone());
                                    global_var::set_string("user::config::user_select_wxid", item.user_wxid.clone());
                                    lib::initialize_thumbnail_preview(item.user_root.as_str(), item.user_wxid.as_str());
                                    lib::initialize_watch_path_puppet(format!("{}\\{}", &item.user_root, &item.user_wxid));
                                }
                            }
                        } else {
                            // dialog::alert_default("没有选择WX文件默认保存位置");
                            gui_util::message::sub_message(hwnd, gui_util::message::IconType::Warning, "没有选择WX文件默认保存位置", 3500u64);
                        }
                    }

                    // 显示扫描获取面板
                    if button_show_drag.existPoint(x, y) {
                        println!("[click] existPoint {}", "显示扫描获取面板");

                        if (!libWxIkunPlus::hasWeChat() || !libWxIkunPlus::hasWeChatWin()) {
                            gui_util::message::sub_message(hwnd, gui_util::message::IconType::Failure, "WX未登录 为避免滥用 面板开启请求被拒绝", 3500u64);
                            // fltk::dialog::alert_default("尚未找到已经登录的WX进程 为避免滥用 面板开启请求被拒绝 ");
                            return false;
                        }

                        if global_var::get_string_default("user::config::user_select_wxid").is_empty() {
                            gui_util::message::sub_message(hwnd,gui_util::message::IconType::Warning,"尚未选择用户",3500u64);
                            return false;
                        }
                        if global_var::get_string_default("user::config::user_select_path").is_empty() {
                            gui_util::message::sub_message(hwnd,gui_util::message::IconType::Warning,"没有选择WX根目录",3500u64);
                            return false;
                        }

                        // gui_drag_scan::main_window();


                        scan_preview_window.main.show();
                        // libWxIkunPlus::setWindowEnabled(libWxIkunPlus::findWindow(THE_SUB_WINDOW_CLASS_NAME_SCAN,""),true);
                        // user_select_database_dir_input.deactivate();
                        // select_attach_card.input_rename.deactivate();
                        // select_attach_card.input_remark.deactivate();
                        // let mut scan_drag_window = show_scan_drag_window();
                        // scan_drag_window.hide();
                        scan_win_show = true;
                    }

                }


                // 卡片按钮 > 完成选定
                if select_attach_card.btn_select.existPoint(x, y) {
                    println!("[click] existPoint {}", "卡片按钮 > 完成选定");

                    let mut result_data = String::new();
                    let mut is_effective = true;

                    let mut rename_rule = select_attach_card.input_rename.value();

                    // 没有选定的路径
                    if user_select_database_dir_input.value().is_empty() {
                        fltk::dialog::alert_default("没有选定Wx路径");
                        is_effective = false;
                        return false;
                    }
                    // 没有选定WX用户
                    if global_var::get_string_default("user::config::user_select_wxid").is_empty() {
                        fltk::dialog::alert_default("没有选定WX用户");
                        is_effective = false;
                        return false;
                    }

                    //  判断是否有Att id
                    else if g_the_select_attach_id.is_empty() || g_the_select_attach_id.len() < 25 {
                        // fltk::dialog::alert_default("attach id 无效 （尚未选定有效聊天对象）");
                        // gui_util::message::message(x+100,y+80,gui_util::message::IconType::Warning,"attach id 无效 （尚未选定有效聊天对象）",3500u64);
                        gui_util::message::sub_message(hwnd, gui_util::message::IconType::Warning, "attach id 无效 （尚未选定有效聊天对象）", 3500u64);
                        is_effective = false;
                        return false;
                    }
                    // 有命名规则 要求规则最少有一个%N.. 自变量
                    if !rename_rule.is_empty() && (!rename_rule.contains("<N") || !rename_rule.contains("N>")) {
                        rename_rule.push_str("<NN>");
                    }
                    let mut select_dir = global_var::get_string_default("user::config::user_select_path");
                    let mut user_select_wxid = global_var::get_string_default("user::config::user_select_wxid");

                    let eq_wxid_dir = eq_wxid_dir!(select_dir);

                    // 拼合路径并判断有效性 有且为文件夹
                    let mut attach_path = PathBuf::from(select_dir).join(user_select_wxid.as_str()).join("FileStorage\\MsgAttach").join(g_the_select_attach_id.as_str());

                    println!("attach_path=> {:?}", &attach_path);

                    if let Some(attach_path_str) = attach_path.to_str() {
                        result_data.push_str(attach_path_str);
                        // 识标
                        if frame_check.thumbnail.is_checked() {
                            result_data.push_str("*wizards");
                        }
                        // 可选项
                        if frame_check.thumbnail.is_checked() {
                            result_data.push_str("*thumbnail");
                        }
                        if frame_check.source.is_checked() {
                            result_data.push_str("*source");
                        }
                        if frame_check.video.is_checked() {
                            result_data.push_str("*video");
                        }
                        if frame_check.sync.is_checked() {
                            result_data.push_str("*Sync");
                        }
                        if frame_check.the_month.is_checked() {
                            result_data.push_str("*the_month");
                        }
                        // 添加名称格式化自变量
                        if !select_attach_card.input_rename.value().is_empty() {
                            result_data.push_str(format!("*rename_rule={}*", &rename_rule).as_str());
                        }

                    }else{
                        gui_util::message::sub_message(hwnd, gui_util::message::IconType::Warning, "路径转义失败 错误代码[3061]", 3500u64);
                        return false;
                    }

                    if !attach_path.exists() && !attach_path.exists() {

                        // dialog::alert_default("attach 目录无效");
                        gui_util::message::sub_message(hwnd, gui_util::message::IconType::Warning, "attach目录无效 <聊天对象目录无法打开>", 3500u64);
                        is_effective = false;
                        return false;
                    }

                    if is_effective && eq_wxid_dir {
                        println!("result_data->{}",&result_data);
                        global_var::set_string(the_token.as_str(),result_data);
                        lib::gc_select_user_ui();
                        fltk::window::Window::delete(win.clone());
                    }
                }

                // 卡片按钮 > 备注名称 完成按钮
                if select_attach_card.btn_remark.existPoint(x, y) {
                    println!("[click] existPoint {}", "卡片按钮 > 备注名称 完成按钮");

                    let wxid = global_var::get_string_default("user::config::user_select_wxid");
                    let attach_id = global_var::get_string_default("user::config::user_select_attach");//select_attach_card.input_attach.value();
                    let remark_name = select_attach_card.input_remark.value();

                    if wxid.is_empty() {
                        // dialog::alert_default("没有选择用户 (WXID)");
                        gui_util::message::sub_message(hwnd, gui_util::message::IconType::Warning, "没有选择用户 (WXID)", 3500u64);
                        return false;
                    }

                    if attach_id.is_empty() {
                        // dialog::alert_default("没有选择聊天对象 (attach ID)");
                        gui_util::message::sub_message(hwnd, gui_util::message::IconType::Warning, "没有选择聊天对象 (attach ID)", 3500u64);

                        return false;
                    }

                    if remark_name.is_empty() {
                        // dialog::alert_default("没有备注内容 (备注将用于命名与显示对象名称)");
                        gui_util::message::sub_message(hwnd, gui_util::message::IconType::Warning, "没有备注内容 (用于命名与显示对象名称)", 3500u64);

                        return false;
                    }

                    lib::set_store_user_remark(wxid, attach_id, remark_name);
                }

                // 卡片按钮 > 编辑命名规则
                if select_attach_card.btn_rename.existPoint(x, y) {
                    println!("[click] existPoint {}", "卡片按钮 > 编辑命名规则");
                   let mut rename_token = rename_tool_main(select_attach_card.input_rename.value().as_str());
                    let mut input_rename = select_attach_card.input_rename.clone();

                    app::add_timeout3(0.3,move|handle|{
                        if global_var::has_string(rename_token.as_str()) {
                            let data = global_var::get_string_default(rename_token.as_str());
                            if data.is_empty() {
                                println!("{} 用户取消 data-> [{}]",&rename_token,&data);
                                gui_util::message::sub_message(get_the_hwnd!(),gui_util::message::IconType::Warning,"用户取消处理",3500u64);
                            }else{
                                if !input_rename.value().as_bytes().eq(data.as_bytes()) {
                                    input_rename.set_value(data.as_str());
                                    println!("{} 名称更新 data-> [{}]",&rename_token,&data);
                                    gui_util::message::sub_message(get_the_hwnd!(),gui_util::message::IconType::Success,"名称已成功更新",3500u64);
                                }else {
                                    println!("{} 没有需要更新的名称内容 data-> [{}]",&rename_token,&data);
                                    gui_util::message::sub_message(get_the_hwnd!(),gui_util::message::IconType::Info,"名称内容没有变化",3500u64);
                                }
                            }
                            global_var::set_string(rename_token.as_str(),String::new());
                            app::remove_timeout3(handle);
                        }else {
                            app::repeat_timeout3(0.3, handle);
                        }
                    });

                    /*
                    std::thread::spawn(move|| loop{
                        std::thread::sleep(std::time::Duration::from_millis(300u64));
                        if global_var::has_string(rename_token.as_str()) {
                            let data = global_var::get_string_default(rename_token.as_str());
                            if data.is_empty() {
                                println!("{} 用户取消 data-> [{}]",&rename_token,&data);
                            }else{
                                input_rename.set_value(data.as_str());
                                println!("{} 名称更新 data-> [{}]",&rename_token,&data);
                            }
                            break;
                        }
                    });
                    */
                }

                true
            }

            enums::Event::Move => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;
                let mut new_show_cursor = false;

                initialize_window_hwnd!(hwnd);

                // 启用了预览图片模式
                if preview_win_show {

                    let mut new_show_cursor = false;

                    // 可选项
                    if !new_show_cursor { new_show_cursor = frame_check.existPoint(x, y) }

                    // 卡片按钮
                    if !new_show_cursor {
                        new_show_cursor = {
                            select_attach_card.btn_select.existPoint(x, y)||
                                select_attach_card.btn_remark.existPoint(x, y)||
                                select_attach_card.btn_rename.existPoint(x, y)
                        }
                    }

                    if preview_win.btn_close.existPoint(x,y)||new_show_cursor{
                        win.set_cursor(fltk::enums::Cursor::Hand);
                    } else {
                        win.set_cursor(fltk::enums::Cursor::Default);
                    }

                }
                else if scan_win_show {

                    if scan_preview_window.btn_clip.existPoint(x,y)
                     ||scan_preview_window.btn_scan_drag.existPoint(x,y)
                     ||scan_preview_window.btn_all_obj.existPoint(x,y)
                     ||scan_preview_window.btn_close.existPoint(x,y)
                    {
                        win.set_cursor(fltk::enums::Cursor::Hand);
                    } else {
                        win.set_cursor(fltk::enums::Cursor::Default);
                    }

                }
                // 正常窗口
                else  {
                // 可选项
                if !new_show_cursor { new_show_cursor = frame_check.existPoint(x, y) }

                // 卡片按钮
                if !new_show_cursor {
                    new_show_cursor = {
                            select_attach_card.btn_select.existPoint(x, y)||
                            select_attach_card.btn_remark.existPoint(x, y)||
                            select_attach_card.btn_rename.existPoint(x, y)
                    }
                }

                // 主界面按钮 打开 / 显示拖拽 / 显示帮助 / 开始
                if !new_show_cursor {
                    new_show_cursor = {
                        button_open_dir.existPoint(x, y)||
                            button_show_drag.existPoint(x, y)||
                            button_show_help.existPoint(x, y)||
                            button_start.existPoint(x,y)
                    }
                }

                // 缩略图（5张）
                if !new_show_cursor {
                    let mut index = 0;
                    for hotspot in &frame_thumbnail_preview.hotspot_list {
                        index+=1;
                        if hotspot.existPoint(x,y) {
                            new_show_cursor=true;
                            break;
                        }
                    }
                }

                // 图片预览大图
                if select_attach_card.thumbnail_preview.existPoint(x,y){
                    new_show_cursor=true;
                }

                if new_show_cursor!=show_cursor{
                    // 判断是否显示手型鼠标
                    if new_show_cursor {
                        win.set_cursor(fltk::enums::Cursor::Hand);
                    } else {
                        win.set_cursor(fltk::enums::Cursor::Default);
                    }
                    show_cursor=new_show_cursor ;
                }

                }
                true
            }

            _ => {
                false
            },
        }

    });



    initialize_window_hwnd!(hwnd);
    the_token
}
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

pub use hotspot::{*};
pub use text::{*};
pub use img::{*};
pub use message::{*};

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



#![allow(warnings, unused)]

use std::{env, thread};
use std::ffi::{c_int, c_long, c_void, OsStr,c_uint,};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use libc::c_longlong;
use serde_json::json;
use serde::{Deserialize, Serialize};
use serde_json::{Value};

pub type PCSTR =*const c_char;

type wchar_t = u16;
type WCHAR = wchar_t;

type LPCWCHAR = *const WCHAR;

use self::util::{encode_lpcstr, ansi_codepage_cstring};
pub mod util;

// #![crate_type = "staticlib"]
//  请注意 所有传入的文本都必须是utf8
#[link(name = "libWxIkunPlus", kind = "static")]
extern "C" {
    fn _setWinIcon(_hWnd: c_long) -> c_void;
    fn _setWinIconMain(_hWnd: c_long) -> c_void;
    fn _setShowWindows(_hWnd: c_long, visible: bool) -> bool;
    fn _set_tray() -> c_void;
    fn _createMutex(mutex:PCSTR) -> bool;
    fn _removeMutex(mutex:PCSTR) -> bool;
    fn _hasMutex(mutex:PCSTR) -> bool;
    fn _setStartup() -> bool;
    fn _hasStartup() -> bool;
    fn _openSelectFolder() -> c_void;
    fn _setWindowsTop(_hWnd: c_long, visible: bool) -> bool;
    fn _setCloseWindow(_hWnd: c_long, closeRoot: bool) -> bool;
    fn _openSelectFolder2() ->PCSTR;
    fn _Error(title:PCSTR, info:PCSTR) -> c_void;
    fn _Stop(mutex:PCSTR, info:PCSTR) -> c_void;
    fn _Confirm(title:PCSTR, info:PCSTR) -> bool;
    fn _Alert(mutex:PCSTR, info:PCSTR) -> bool;
    fn _getRegistrValue(hKey: c_long, _subKey:PCSTR, _key:PCSTR)->PCSTR;
    fn _hasWeChat() -> bool;
    fn _setTaskbarWin(_hWnd: c_long) -> c_void;
    fn _setMinWindows(_hWnd: c_long) -> bool;
    fn _findWindow(className:PCSTR, title:PCSTR) -> c_long;
    // fn _findWindowW(className:LPCWCHAR, title:LPCWCHAR) -> c_long;
    // fn _findWindowU8(className:PCSTR, title:PCSTR) -> c_long;
    fn _has_auto_sync() -> bool;
    fn _set_auto_sync(value:bool);
    fn _has_sync_token()-> bool;
    fn _hasStartupGlobalVar()-> bool;
    fn _getFocusTopWindow()->c_long;
    fn _getFocusWindow()->c_long;
    fn _findAllWindow(className:PCSTR, title:PCSTR) -> PCSTR;
    fn _isWindow(_hWnd: c_long) -> bool;
    fn _setWindowShake(hWnd: c_long);
    fn _getWindowRect(hWnd: c_long) -> PCSTR;
    fn _randomNum()->c_longlong;
    fn _setWindowTransparent(hWnd:c_long,transparent:c_int);
    fn _getfilePathSingle()->PCSTR;
    fn _setWindowEnabled(_hWnd: c_long, enabled: bool) -> bool;
    fn _hasInitWindowIsDisplayed ()->bool;
    fn _setInitWindowIsDisplayed(initWindowIsDisplayed:bool)->bool;
}

// 设置主窗口图标 从当前二进制获取
pub fn setWinIconMain(hWnd: i128) {
    unsafe {
        match hWnd.try_into() {
            Ok(hwnds) => {
                _setWinIconMain(hwnds);
            }
            Err(_) => {}
        }
    };
}

// 设置窗口图标 从当前二进制获取
pub fn setWinIcon(hWnd: i128) {
    unsafe {
        match hWnd.try_into() {
            Ok(hwnds) => {
                _setWinIcon(hwnds);
            }
            Err(_) => {}
        }
    };
}


// 关闭窗口
pub fn closeWindow(hWnd: i128, destroy: bool) {
    unsafe {
        match hWnd.try_into() {
            Ok(hwnds) => {
                _setCloseWindow(hwnds, destroy);
            }
            Err(_) => {}
        }
    };
}

// 设置窗口可见 如果可见会激活窗口
pub fn setwinVisible(hWnd: i128, visible: bool) -> bool {
    let mut res = false;

    unsafe {
        match hWnd.try_into() {
            Ok(hwnds) => {
                _setShowWindows(hwnds, visible);
            }
            Err(_) => {
                res = false;
            }
        }
    };
    return res;
}

// 设置窗口顶置
pub fn setWinTop(hWnd: i128, visible: bool) -> bool {
    let mut res = false;

    unsafe {
        match hWnd.try_into() {
            Ok(hwnds) => {
                _setWindowsTop(hwnds, visible);
            }
            Err(_) => {
                res = false;
            }
        }
    };
    return res;
}

// 启用托盘
pub fn set_tray() {
    unsafe {
        _set_tray();
    };
}

// 创建互斥体
pub fn createMutex(mutex: String) -> bool {
    let mut result = false;
    unsafe {
        match CString::new(mutex) {
            Ok(mutexs) => {
                result = _createMutex(mutexs.as_ptr());
            }
            Err(_) => {}
        }
    };

    result
}

// 删除互斥体
pub fn removeMutex(mutex: String) -> bool {
    let mut result = false;
    unsafe {
        match CString::new(mutex) {
            Ok(mutexs) => {
                result = _removeMutex(mutexs.as_ptr());
            }
            Err(_) => {}
        }
    };

    result
}

// 判断是否有互斥体
pub fn hasMutex(mutex: String) -> bool {
    let mut result = false;
    unsafe {
        match CString::new(mutex) {
            Ok(mutexs) => {
                result = _hasMutex(mutexs.as_ptr());
            }
            Err(_) => {}
        }
    };

    result
}

// 设置自启
pub fn setStartup() -> bool {
    unsafe {
        return _setStartup();
    };
}

pub fn hasStartup() -> bool {
    unsafe {
        return _hasStartup();
    };
}

// 文件夹选取器
pub fn openSelectFolder() -> String {
    unsafe {
        _openSelectFolder();
        let mut open_path = env::var("IKUN@SelectedFolderPath").unwrap_or_else(|_| "".to_owned());
        return open_path;
    };
}

// 将C字符串转换为Rust字符串
fn c_string_to_rust_string(ptr:PCSTR) -> String {
    unsafe {
        let c_str = CStr::from_ptr(ptr);
        let bytes = c_str.to_bytes();
        String::from_utf8_lossy(bytes).into_owned()
    }
}

// 启用托盘
pub fn openSelectFolder2() -> String {
    let mut result = String::new();
    
    let the_win = getFocusWindow();

    setwinVisible(the_win.clone(), false);
    
    unsafe { result = c_string_to_rust_string(_openSelectFolder2()) };
    
    setwinVisible(the_win.clone(), true);
    
    return result;
}

// 将Rust字符串转换为C字符串
fn rust_string_to_c_string(s: String) -> CString {
    if let Result::Ok(mut buff) = CString::new(s.as_str()) {
        return buff;
    };
    let c_ptr = CString::new("").unwrap();
    return c_ptr;
}

fn rust_string_to_ansi_str(s: String)->Vec<i8>{
    if let Result::Ok(item) = ansi_codepage_cstring(s) {
        return item;
    }
    let c_ptr = CString::new("").unwrap();
    let as_bytes = c_ptr.as_bytes().to_vec();
    let mut result = Vec::new();
    for value in as_bytes {
        result.push(value as i8);
    }

    return result;
}

// fn option_vec_u8_to_cstring(option_vec: Option<Vec<u8>>) -> Result<CString, &'static str> {
//     match option_vec {
//         Some(vec) => {
//             match CString::new(vec) {
//                 Ok(cstring) => Ok(cstring),
//                 Err(_) => Err("Failed to create CString"),
//             }
//         }
//         None => Err("Option<Vec<u8>> is None"),
//     }
// }


// // 将Rust UTF-8字符串转换为Windows API中的A字符
// fn utf8_to_ansi(s: &str) -> Vec<c_char> {
//     let wide: Vec<u16> = OsStr::new(s).encode_wide().collect();
//     let wide_len = wide.len() + 1;

//     let mut ansi: Vec<c_char> = Vec::with_capacity(wide_len);
//     let ansi_len = wide.len();

//     unsafe {
//         WideCharToMultiByte(
//             CP_UTF8,
//             0,
//             wide.as_ptr(),
//             wide_len as i32,
//             ansi.as_mut_ptr(),
//             ansi_len as i32,
//             ptr::null(),
//             ptr::null_mut(),
//         );
//         // 确保在末尾添加一个空字符
//         ansi.push(0);
//         ansi.set_len(ansi_len);
//     }

//     ansi
// }

// MessageBox -> alert
pub fn alert(title: String, message: String) -> bool {
    unsafe {
        return _Alert(
            rust_string_to_ansi_str(title).as_ptr(),
            rust_string_to_ansi_str(message).as_ptr(),
        );
    }
    return false;
}

// MessageBox -> confirm
pub fn confirm(title: String, message: String) -> bool {
    unsafe {
        return _Confirm(
            rust_string_to_ansi_str(title).as_ptr(),
            rust_string_to_ansi_str(message).as_ptr(),
        );
    }
    return false;
}

// MessageBox -> stop
pub fn stop(title: String, message: String) {
    unsafe {
        _Stop(
            rust_string_to_ansi_str(title).as_ptr(),
            rust_string_to_ansi_str(message).as_ptr(),
        );
    }
}

// MessageBox -> error
pub fn error(title: String, message: String) {
    unsafe {
        _Error(
            rust_string_to_ansi_str(title).as_ptr(),
            rust_string_to_ansi_str(message).as_ptr(),
        );
    }
}

pub enum HKEY {
    HKEY_CLASSES_ROOT = 0x80000000,
    HKEY_CURRENT_USER = 0x80000001,
    HKEY_LOCAL_MACHINE = 0x80000002,
    HKEY_USERS = 0x80000003,
}


pub fn getRegistrValue(hKey: HKEY, subKey: String, valueKey: String) -> String {
    let mut result = String::new();
    unsafe {
        let mut c_result = _getRegistrValue(
            c_long::from(hKey as i32),
            rust_string_to_ansi_str(subKey).as_ptr(),
            rust_string_to_ansi_str(valueKey).as_ptr(),
        );
        result =c_string_to_rust_string(c_result);
    }
    result
}

// 判断wx进程是否存在
pub fn hasWeChat()->bool {
    let mut result = false;
    unsafe {
        result= _hasWeChat();
    }
    result
}

pub fn hasWeChatWin()->bool {
    let mut result = false;
    unsafe {
        let hwnd_01 = findWindow("WeChatMainWndForPC", "");
        if(hwnd_01!=0){
           return true;
        }

        let hwnd_02 = findWindow("ChatWnd", "");
        if(hwnd_02!=0){
            return true;
        }

        let hwnd_03 = findWindow("SubscriptionWnd", "");
        if(hwnd_03!=0){
            return true;
        }
    }
    result
}

// 把一个傀儡窗口变成主窗口的托盘
pub fn setTaskbarWin(hWnd: i128) {
    unsafe {
       _setTaskbarWin(hWnd as i32);
    }
}

pub fn setMinWindows(hWnd: i128) -> bool {
    unsafe {
        _setMinWindows(hWnd as i32)
    }
}

pub fn isWindow(hWnd: i128) -> bool {
    unsafe {
        _isWindow(hWnd as i32)
    }
}

pub fn setWindowShake(hWnd: i128)  {
    unsafe {
        _setWindowShake(hWnd as i32)
    }
}

// 搜索窗口
pub fn findWindow(className: &str, titleName: &str)->i128 {
    let mut hwnd:i128 = 0;
    unsafe {
        let mut className = rust_string_to_ansi_str(className.to_string());
        let mut titleName = rust_string_to_ansi_str(titleName.to_string());
      
        hwnd= _findWindow(className.as_ptr(), titleName.as_ptr()/*,className_len as i32,titleName_len as i32 */).into();
    }
    return hwnd;
}

// // 搜索窗口
// pub fn findWindowU8(className: String, titleName: String)->i128 {
//     let mut hwnd:i128 = 0;
//     unsafe {
//         // let mut className = rust_string_to_c_string(className);
//         // let mut titleName = rust_string_to_c_string(titleName);
//         hwnd= _findWindowU8(encode_lpcstr(className.as_str()).as_ptr(), encode_lpcstr(titleName.as_str()).as_ptr() /*,className_len as i32,titleName_len as i32 */).into();
//     }
//     return hwnd;
// }

// 判断是否启用了自动更新（如果wx进程不存在也会返回false）
pub fn has_auto_sync() -> bool{
    let mut result = false;
    unsafe {
        result =_has_auto_sync();
    }

    result
}

// 判断是否启用了自动更新（如果wx进程不存在也会返回false）
pub fn has_auto_sync_all() -> bool{
    let mut result = false;
    unsafe {
        result = hasWeChat()&&hasWeChatWin()&&_has_auto_sync();
    }
    println!("has_auto_sync-> {}",&result);
    result
}

// 设置自动更新
pub fn set_auto_sync(value: bool){
    unsafe {
       _set_auto_sync(value);
    }
}

// 是否立即同步
pub fn has_sync_token()->bool{
    unsafe{
        _has_sync_token()
    }
}

// 是否立即同步
pub fn hasStartupGlobalVar()->bool{
    unsafe{
        _hasStartupGlobalVar()
    }
}

pub fn getFocusWindow()->i128{
    unsafe{
        _getFocusWindow() as i128
    }
}

pub fn getFocusTopWindow()->i128{
    unsafe{
        _getFocusTopWindow() as i128
    }
}

fn get_str_to_long_vec(c_result:PCSTR)->Vec<i128>{
    let mut list:Vec<i128> = Vec::new();
    let result =c_string_to_rust_string(c_result);

    let long_str = String::from("1234567890");
    let mut the_data = String::new();

    for char in result.chars() {
        if(long_str.contains(char)){
            the_data.push(char);
        }else{
            if !the_data.is_empty() {

                let parsed_number: Result<i32, _> = the_data.parse();
                if let Ok(parsed_number) = parsed_number {
                    list.push(parsed_number as i128);
                }
                the_data.clear();
            }

        }
    }

    if !the_data.is_empty() {
        let parsed_number: Result<i32, _> = the_data.parse();
        if let Ok(parsed_number) = parsed_number {
            list.push(parsed_number as i128);
        }
        the_data.clear();
    }

    list
}


pub fn findAllWindow(className: &str, titleName: &str)->Vec<i128>{
    unsafe{
        let mut className = rust_string_to_ansi_str(className.to_string());
        let mut titleName = rust_string_to_ansi_str(titleName.to_string());
        let c_result = _findAllWindow(className.as_ptr(),titleName.as_ptr());
        get_str_to_long_vec(c_result)
    }
}

#[derive(Debug  )]
pub struct RECT {
    pub left:i32,
    pub top:i32,
    pub bottom:i32,
    pub right:i32,
    pub height:i32,
    pub width:i32
}
pub fn getWindowRect(hWnd: i128)->RECT{
    let mut rect = RECT{
        left: 0,
        top: 0,
        bottom: 0,
        right: 0,
        height: 0,
        width: 0,
    };

    unsafe {
       let c_result_json = _getWindowRect(hWnd as c_long);

        let data = c_string_to_rust_string(c_result_json);

        if let Ok(c_rect) = serde_json::from_str(data.as_str()) as serde_json::Result<Value> {
            rect.bottom = c_rect["bottom"].as_i64().unwrap_or_else(||{0}) as i32;
            rect.left = c_rect["left"].as_i64().unwrap_or_else(||{0}) as i32;
            rect.top = c_rect["top"].as_i64().unwrap_or_else(||{0}) as i32;
            rect.right = c_rect["right"].as_i64().unwrap_or_else(||{0}) as i32;
            rect.height = c_rect["height"].as_i64().unwrap_or_else(||{0}) as i32;
            rect.width = c_rect["width"].as_i64().unwrap_or_else(||{0}) as i32;
        }

    }

    rect
}

// 随机数
pub(crate) fn randomNum() -> i128{
    unsafe {
        _randomNum() as i128
    }
}

pub fn setWindowTransparent(hWnd:i128,transparent:i32){
    unsafe {
        _setWindowTransparent(hWnd as c_long,transparent as c_int);
    }
}

pub fn getClipFilePathSingle()->String{
    unsafe{
        let c_result = _getfilePathSingle();
        c_string_to_rust_string(c_result)
    }
}

pub fn setWindowEnabled(hWnd: i128, enabled: bool) -> bool{
    unsafe {
        _setWindowEnabled(hWnd as c_long,enabled)
    }
}

pub fn hasInitWindowIsDisplayed()->bool{
    unsafe {
        _hasInitWindowIsDisplayed()
    }
}
pub fn setInitWindowIsDisplayed(initWindowIsDisplayed:bool)->bool{
    unsafe {
        _setInitWindowIsDisplayed(initWindowIsDisplayed)
    }
}
#![allow(warnings, unused)]

use chrono::Local;
use glob::glob;
use hotwatch::{
    blocking::{Flow, Hotwatch},
    EventKind,
};

use libc::c_void;
use rusqlite::{params, Connection, Result};

use fltk::draw::font;
use fltk::enums::{Cursor, Event, Font, LabelType};
use fltk::frame::Frame;
use fltk::group::Group;
use fltk::input::{Input, InputType, IntInput};
use fltk::text::TextDisplay;
use fltk::{
    app::handle,
    text::{TextBuffer, TextEditor},
};
use fltk::{button::Button, enums::Align, window::DoubleWindow};
use fltk::{enums::Color, enums::FrameType};
use fltk::{prelude::*, window::Window, *};
use magic_crypt::MagicCryptTrait;
use msgbox::IconType;
use serde_json::json;
use std::collections::{hash_map::DefaultHasher, HashMap};
use std::mem::transmute;
use fltk_theme::ColorTheme;
use fltk_theme::color_themes;
use fltk_theme::WidgetTheme;
use fltk_theme::ThemeType;
use fltk_theme::WidgetScheme;
use fltk_theme::SchemeType;
use crate::{console_log, gui_manage_item};
use std::{
    env,
    ffi::{c_int, c_long, OsStr},
    fs,
    hash::{Hash, Hasher},
    io,
    path::{Path, PathBuf},
    process,
    process::{Command, Stdio},
    thread,
    time::Duration,
};

use crate::{atomic_util, global_var, handle_dat, libWxIkunPlus::{self, setTaskbarWin}, util::{self, str_eq_ostr, str_eq_str, Sleep}, wh_mod::convert::{convert_bat_images}, wh_mod, global_var_util, get_bool,  gui_util, set_bool, gui_select_user_ui};
use crate::wh_mod::parse_dat_path;

use std::sync::atomic::{AtomicBool, AtomicI32, AtomicUsize, Ordering,AtomicI64};
use std::sync::{Arc, Condvar, Mutex,RwLock};
use serde_json::Value as Json;
use toml::Value as Toml;



pub struct AppVersionInfo {

}

fn toml2json(toml: Toml) -> Json {
    match toml {
        Toml::String(s) => Json::String(s),
        Toml::Integer(i) => Json::Number(i.into()),
        Toml::Float(f) => {
            let n = serde_json::Number::from_f64(f).expect("float infinite and nan not allowed");
            Json::Number(n)
        }
        Toml::Boolean(b) => Json::Bool(b),
        Toml::Array(arr) => Json::Array(arr.into_iter().map(toml2json).collect()),
        Toml::Table(table) => {
            Json::Object(table.into_iter().map(|(k, v)| (k, toml2json(v))).collect())
        }
        Toml::Datetime(dt) => Json::String(dt.to_string()),
    }
}

pub fn get_app_version_info () -> Json {
    const APP_VERSION: &str = include_str!("../../../Cargo.toml");
    // println!("toml2json-> {:?}",toml2json(APP_VERSION));

    match APP_VERSION.parse() {
        Ok(toml) => {
            let json = toml2json(toml);
             return json
        }
        Err(error) => println!("failed to parse TOML: {}", error),
    }

    json!("")
}

struct MainTheme {
    /**主背景颜色 */
    background: Color,
    /**次背景*/
    backgroundMain: Color,
    /**顶部文字和logo */
    logo: Color,
    /**卡片文本成功 */
    cardSuccessText: Color,
    /**卡片文本失败 */
    cardFailureText: Color,
    /**卡片文本 */
    cardText: Color,
    /**卡片描边 */
    cardStroke: Color,
    /**分割线 */
    cuttingLine: Color,
    /** 底部三个按钮的颜色*/
    botBtnColor: Color,
    /** 底部三个按钮的图标颜色*/
    botBtnIconColor: Color,
    // null
    not: Color,
}

// 统一在这里定义主题颜色
fn getMainTheme() -> MainTheme {
    let mut mainTheme: MainTheme = MainTheme {
        /**主背景颜色 */
        background: Color::rgb_color(24, 24, 24),
        /**次背景*/
        backgroundMain: Color::rgb_color(17, 17, 17),
        /**顶部文字和logo */
        logo: Color::rgb_color(122, 120, 120),
        /**卡片文本成功 */
        cardSuccessText: Color::rgb_color(99, 138, 99),
        /**卡片文本失败 */
        cardFailureText: Color::rgb_color(189, 79, 79),
        /**卡片文本 */
        cardText: Color::rgb_color(255, 255, 255),
        /**卡片描边 */
        cardStroke: Color::rgb_color(46, 46, 46),
        /**分割线 */
        cuttingLine: Color::rgb_color(38, 38, 38),
        /** 底部三个按钮的颜色*/
        botBtnColor: Color::rgb_color(0, 0, 0),
        /** 底部三个按钮的图标颜色*/
        botBtnIconColor: Color::rgb_color(125, 125, 125),
        not: Color::from_u32(0),
    };
    return mainTheme;
}

// 设置界面主题
pub fn setMainTheme() {
    let mut mainTheme: MainTheme = getMainTheme();
    app::set_background_color(24, 24, 24);
    // app::set_fonts("name");
    app::set_frame_shadow_width(0);
    app::set_frame_color(mainTheme.not);
    app::set_background2_color(17, 17, 17);
    app::set_foreground_color(17, 17, 17);
    app::set_selection_color(17, 17, 17);
    // app::set_frame_type2(old_frame, new_frame);

    app::set_frame_type(FrameType::NoBox);
    app::set_inactive_color(24, 24, 24);
    app::set_frame_border_radius_max(0);
    app::set_frame_type2(FrameType::BorderBox, FrameType::NoBox);
    app::set_visible_focus(false);
    app::set_frame_shadow_width(0);
    app::swap_frame_type(FrameType::NoBox);
    app::set_menu_linespacing(0);
    app::set_scrollbar_size(0);
}

// 设置背景为图片（主视图）
fn setWinBackground_forRoot_image(appMainWin: &mut window::DoubleWindow) -> Frame {
    let background_image = image::PngImage::from_data(include_bytes!("../../assets/main_back.png"))
        .expect("set main icon error");
    // image::SvgImage::from_data(include_str!("../../assets/main_back.svg"))
    // .expect("set main icon error");
    let mut frame = Frame::default().with_size(600, 0).center_of(appMainWin);
    frame.set_frame(FrameType::EngravedBox);
    frame.set_image(Some(background_image));
    return frame;
}

// 界面会回传为这个参数 用来控制鼠标手型
struct PointExistHasmap {
    // 关闭按钮
    quit: bool,
    // 按钮:: 打开dat所在路径
    shellOpenDatDir: bool,
    // 按钮:: 导出此文件夹
    shellOpenExportDir: bool,
    // 按钮:: 管理
    manageItme: bool,
    // 按钮:: 测试
    test: bool,
    // 按钮:: 创建
    create: bool,
    // 选项::自启动
    starting: bool,

    // 鼠标在按钮原件中
    existAllBtn: bool,
}

// 判断鼠标坐标是否在此元素内
fn getFormPointSpace(x: i32, y: i32) -> PointExistHasmap {
    let mut point_exist_hasmap = PointExistHasmap {
        quit: false,
        shellOpenDatDir: false,
        shellOpenExportDir: false,
        manageItme: false,
        test: false,
        starting: false,
        create: false,
        existAllBtn: false,
    };

    point_exist_hasmap.quit = x > 545 && x < 575 && y > 13 && y < 51;
    point_exist_hasmap.manageItme = x > 342 && x < 342 + 60 && y > 273 && y < 273 + 36;
    point_exist_hasmap.shellOpenDatDir = x > 511 && x < 511 + 36 && y > 147 && y < 147 + 39;
    point_exist_hasmap.shellOpenExportDir = x > 511 && x < 511 + 36 && y > 219 && y < 219 + 39;
    point_exist_hasmap.starting = x > 85 && x < 85 + 25 && y > 490 && y < 490 + 25;
    point_exist_hasmap.test = x > 413 && x < 413 + 60 && y > 273 && y < 273 + 36;
    point_exist_hasmap.create = x > 486 && x < 486 + 60 && y > 273 && y < 273 + 36;

    let mut win_coords_cursor_list = vec![
        point_exist_hasmap.quit,
        point_exist_hasmap.manageItme,
        point_exist_hasmap.shellOpenDatDir,
        point_exist_hasmap.shellOpenExportDir,
        point_exist_hasmap.starting,
        point_exist_hasmap.test,
        point_exist_hasmap.create,
    ];

    let mut has_cursor = false;

    for value in win_coords_cursor_list.iter() {
        // 关闭按钮
        if *(value) {
            has_cursor = true;
        }
    }

    point_exist_hasmap.existAllBtn = has_cursor;

    return point_exist_hasmap;
}


// 设置自启动按钮的状态
fn addBtnEnableStarting(appMainWin: &mut window::DoubleWindow) -> gui_util::img::ImgPreview  {
    let w_h = 20;
    let mut preview = gui_util::img::ImgPreview::new(90-3, 493, w_h, w_h, "gui::preview_main::index::user_select");

    if libWxIkunPlus::hasStartup() {
        preview.from_data(include_bytes!("../../assets/enable.png").to_vec(), 0, 0, w_h, w_h);
    }else{
        preview.from_data(include_bytes!("../../assets/un_enable.png").to_vec(), 0, 0, w_h, w_h);
    }
    
   

    return preview;
}

// dat的路径的输入框
fn addInput_shellOpenDatDir(appMainWin: &mut window::DoubleWindow) -> ConsoleItme {
    let mut mainTheme: MainTheme = getMainTheme();

    let mut buf = fltk::text::TextBuffer::default();
    let mut txt = fltk::text::TextEditor::default()
        .with_size(447, 25)
        .center_of_parent();
    // txt.set
    txt.set_buffer(buf.clone());
    txt.set_frame(FrameType::NoBox);
    txt.set_text_color(Color::from_rgb(120, 120, 120));
    txt.set_color(mainTheme.backgroundMain);
    txt.set_label_type(fltk::enums::LabelType::None);
    // txt.set_text_size(15);
    txt.set_pos(46, 153 + 3);
    txt.set_text_size(11);
    // txt.set_scrollbar_size(0);
    txt.set_scrollbar_size(3);
    txt.set_callback(move |usetup| {
        println!(
            "addInput_shellOpenExportDir => {} {}",
            usetup.buffer().unwrap().text(),
            usetup.buffer().unwrap().length()
        );
        if !wh_mod::convert::is_developer(){
        let mut buff = usetup.buffer().unwrap();
        buff.remove(0, buff.length());
        console_log!("[错误] 编辑被禁止".to_string());
        }
    });
  
    // buf.set(true);

    txt.show();

    return ConsoleItme {
        edit: txt,
        buff: buf,
    };
}

// 保存到的输入框
fn addInput_shellOpenExportDir(appMainWin: &mut window::DoubleWindow) -> ConsoleItme {
    let mut mainTheme: MainTheme = getMainTheme();

    let mut buf = fltk::text::TextBuffer::default();
    let mut txt = fltk::text::TextEditor::default()
        .with_size(447, 27)
        .center_of_parent();
    txt.set_buffer(buf.clone());
    txt.set_frame(FrameType::NoBox);
    txt.set_text_color(Color::from_rgb(120, 120, 120));
    txt.set_color(mainTheme.backgroundMain);
    txt.set_label_type(fltk::enums::LabelType::None);
    txt.set_text_size(13);
    txt.set_pos(46, 223 + 5);
    txt.set_scrollbar_size(2);
    // txt.set_scrollbar_align(Align:);
    txt.set_callback(move |usetup| {
        println!(
            "addInput_shellOpenDatDir => {} {}",
            usetup.buffer().unwrap().text(),
            usetup.buffer().unwrap().length()
        );
    });
    txt.show();

    return ConsoleItme {
        edit: txt,
        buff: buf,
    };
}

// 名称:
fn addInput_shellName(appMainWin: &mut window::DoubleWindow) -> ConsoleItme {
    let mut mainTheme: MainTheme = getMainTheme();

    let mut buf = fltk::text::TextBuffer::default();
    let mut txt = fltk::text::TextEditor::default()
        .with_size(180+8, 27)
        .center_of_parent();
    txt.set_buffer(buf.clone());
    txt.set_frame(FrameType::NoBox);
    txt.set_text_color(Color::from_rgb(120, 120, 120));
    txt.set_color(mainTheme.backgroundMain);
    txt.set_label_type(fltk::enums::LabelType::None);
    txt.set_text_size(15);
    txt.set_pos(98, 279);

    // txt.set_changed();
    txt.set_callback(move |usetup| {
        let mut stext = usetup.buffer().unwrap();
        if (stext.length() > 30) {
            stext.remove(30, stext.length());
        }
        println!("addInput_shellName => {} {}", stext.text(), stext.length());
    });
    txt.show();

    return ConsoleItme {
        edit: txt,
        buff: buf,
    };
}

// 打印台的控制
struct ConsoleItme {
    edit: TextEditor,
    buff: TextBuffer,
}

// 初始化打印台元素
fn addConsole(appMainWin: &mut window::DoubleWindow) -> ConsoleItme {
    let mut mainTheme: MainTheme = getMainTheme();

    let mut buf = fltk::text::TextBuffer::default();
    let mut txt = fltk::text::TextEditor::default()
        .with_size(528, 98)
        .center_of_parent();
    txt.set_buffer(buf.clone());
    txt.set_frame(FrameType::NoBox);
    txt.set_text_color(Color::from_rgb(120, 120, 120));
    txt.set_color(mainTheme.backgroundMain);
    txt.set_label_type(fltk::enums::LabelType::None);
    txt.set_text_size(12);
    txt.set_pos(34, 365);
    txt.set_scrollbar_size(6);
    txt.show();

    return ConsoleItme {
        edit: txt,
        buff: buf,
    };
}

// 处理文本添加时候风格的宏
macro_rules! setTheStyleToInterface {
    ($b:expr) => {{
        let MainTheme: MainTheme = getMainTheme();
        $b.show_cursor(false);
        $b.set_text_color(MainTheme.botBtnIconColor);
        $b.set_text_size(11);
        $b.set_label_type(LabelType::None);
        $b.set_color(MainTheme.backgroundMain);
        $b.clear_visible_focus();
        $b.set_frame(FrameType::FlatBox);
        $b.show_cursor(false);
        $b.deactivate();
        $b.set_text_color(MainTheme.cardText);
    }};
    ($b:expr,$x:expr,$y:expr,$w:expr,$h:expr) => {{
        let MainTheme: MainTheme = getMainTheme();
        $b.show_cursor(false);
        $b.set_text_color(MainTheme.botBtnIconColor);
        $b.set_text_size(11);
        $b.resize($x, $y, $w, $h);
        $b.set_label_type(LabelType::None);
        $b.set_color(MainTheme.backgroundMain);
        $b.clear_visible_focus();
        $b.set_frame(FrameType::FlatBox);
        $b.show_cursor(false);
        $b.deactivate();
        $b.set_text_color(MainTheme.cardText);
    }};

    ($b:expr,$x:expr,$y:expr,$w:expr,$h:expr,$fsize:expr) => {{
        let MainTheme: MainTheme = getMainTheme();
        $b.show_cursor(false);
        $b.set_text_color(MainTheme.botBtnIconColor);
        $b.set_text_size($fsize);
        $b.resize($x, $y, $w, $h);
        $b.set_label_type(LabelType::None);
        $b.set_color(MainTheme.backgroundMain);
        $b.clear_visible_focus();
        $b.set_frame(FrameType::NoBox);
        $b.show_cursor(false);
        $b.deactivate();
        $b.set_text_color(MainTheme.cardText);
    }};
}

struct MsgAttachExport {
    id: i32,
    time: String,
    name: String,
    ext: String,
    input: String,
    ouput: String,
    message: String,
    user_name: String,
}

pub struct MianWindowItme {
    appMainWin: DoubleWindow,
    appRootView: DoubleWindow,
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


fn get_window_hwnd(win:&window::Window) -> i128 {
    let mut xclass = win.xclass().unwrap_or_else(||String::new());
    let mut xtitle =String::new();// win.label();

    let hwnd = libWxIkunPlus::findWindow(xclass.as_str(), xtitle.as_str()) ;
    println!("xclass<{}> xtitle<{}> hwnd<{}>", xclass,xtitle, hwnd);
    hwnd
}

// 主界面
pub fn mianWindow(show: bool) -> Result<MianWindowItme> {

    set_theme!();

    let version_info = get_app_version_info();
    let version =  (version_info["package"]["version"]).as_str().unwrap();
    println!("{}",&version);
    let mut mainTheme: MainTheme = getMainTheme();

    let mut appMainWin = Window::new(0, 0, 600, 531, "Ikun导出");
    appMainWin.set_xclass("app_main_win_wx_dat_viewer_auto_export_rust");

    app::set_scrollbar_size(3);

    app::set_selection_color(24, 24, 24);
    let mut cwd = env::current_dir().expect("get current_dir error ");
    appMainWin.set_border(false);

    // 主界面的窗口 2  悬浮在主窗口1上面
    let mut appRootView = window::Window::new(0, 0, 600, 531, "mian");
    setWinBackground_forRoot_image(&mut appRootView);
    // 界面
    let mut btnEnableStarting = addBtnEnableStarting(&mut appRootView);
    let mut input_shellOpenExportDir = addInput_shellOpenExportDir(&mut appRootView);
    let mut input_shellOpenDatDir = addInput_shellOpenDatDir(&mut appRootView);
    let mut input_Console = addConsole(&mut appRootView);
    let mut input_shellName = addInput_shellName(&mut appRootView);
    let mut sync_type = String::new();
    let mut build_name = if wh_mod::convert::is_build_52pojie() {"52破解专版"} else {"开源版"};

    if(libWxIkunPlus::has_auto_sync()){
        sync_type=(format!("[用户] 自动同步开启"));
    }
    else if (wh_mod::convert::is_developer()) {
        sync_type=(format!("[同步]{}", "自动同步已启用 因为开发者模式有效"));
        build_name = "开发者版";
    }
    else {
        sync_type=(format!("[用户] 自动同步关闭"));
    }

    if !wh_mod::convert::is_developer(){
    input_Console.buff.set_text(
        format!(
        r#"作者 @Ikun 软件开源协议 GPL 3.0 (但是并不包含解码算法) 版本：{} ({})
        本软件 是免费软件 如果付费请维权退款
        本软件只供备份自己的图片禁止用于其他用途
        {}"#
        ,version ,build_name,sync_type).replace("  ","").as_str()
    );
    }else {
        input_Console.buff.set_text(("初始化成功 [开发者模式]"));
    }

    // 界面
    appRootView.end();
    appMainWin.clone().center_screen(); // 将窗口居中

    appMainWin.hide();
    appRootView.hide();
    appMainWin.end();

    let mut input_buff_Console_move = input_Console.buff.clone();

    thread::spawn(move || loop {
        Sleep(150);
        let mut console_message = handle_dat::get_console_message().replace("\n\n", "\n");

        if console_message.starts_with('\n') {
            console_message = console_message.trim_start_matches('\n').to_string();
        }

        if (console_message.len() < 5) {
            continue;
        };

        let mut newline_count = 0;

        for line in input_buff_Console_move.text().lines() {
            newline_count += 1
        }

        if (newline_count > 5) {
            input_buff_Console_move.remove(0, input_buff_Console_move.length());
            input_buff_Console_move.set_text(&console_message);
        } else {
            input_buff_Console_move.append(&format!("\n{}", &console_message));
        }
    });

    let mut dat_buff_move= input_shellOpenDatDir.buff.clone();
    let mut copy_btnEnableStarting = btnEnableStarting.clone();
    thread::spawn(move || loop {
        let mut oid_app_start = false;

        if !libWxIkunPlus::has_auto_sync()!=oid_app_start{
            oid_app_start = true;
            copy_btnEnableStarting.from_data(include_bytes!("../../assets/enable.png").to_vec(), 0, 0, 20, 20);
        }else{
            oid_app_start = false;
            copy_btnEnableStarting.from_data(include_bytes!("../../assets/un_enable.png").to_vec(), 0, 0, 20, 20);
        }

        Sleep(550);

        let input_select_dir = global_var::get_string_default("user::config::input_select_dir");
        let user_select_path = global_var::get_string_default("user::config::user_select_path");
        let user_select_wxid = global_var::get_string_default("user::config::user_select_wxid");

        if !user_select_path.is_empty()&&!input_select_dir.is_empty()&&global_var::get_bool_default("gui::open::handle_dat") {


            let mut new_buff = format!("{}/{}/FileStorage/MsgAttach/{}",input_select_dir,user_select_wxid,user_select_path);

            // 判断路径有效性 无效则换文件夹  因为有些用户是可以多账户登录的
            if(!Path::new(new_buff.as_str()).exists()){
                let read_root_wxid_list  = wh_mod::wx_read_root_wxid(&Path::new(input_select_dir.as_str()).to_path_buf());
                for read_root_wxid in read_root_wxid_list {
                    if Path::new(read_root_wxid.attach.join(user_select_path.as_str() ) .as_os_str() ).exists(){
                        new_buff = format!("{}/{}",read_root_wxid.attach.to_str().unwrap(),user_select_path);
                        break;
                    }
                }
            }

            if(global_var::get_bool_default("user::config::check_button_the_month")){
                new_buff = new_buff+"*the_month";
            }
            if(global_var::get_bool_default("user::config::check_button_source")){
                new_buff = new_buff+"*source";
            }
            if(global_var::get_bool_default("user::config::check_button_thumbnail")){
                new_buff = new_buff+"*thumbnail";
            }

            if(!new_buff.as_bytes().eq(dat_buff_move.text().as_bytes() )){
                dat_buff_move.remove(0,dat_buff_move.length());
                dat_buff_move.append(new_buff.as_str());
            }

        }

    });

    let mut copy_AppRootView = appRootView.clone();
    let mut copy_appMainWin = appMainWin.clone();
    // let mut copy_dock_win = dock_win.clone();

    let mut g_appMainWinHwnd = 0;
    // let mut g_copy_dock_win_hwnd = 0;

    appMainWin.handle({
        let mut x = 0;
        let mut y = 0;
        let mut point_exist_hasmap = getFormPointSpace(x, y);
        let mut has_show = false;
        let mut copy_appMainWin = copy_appMainWin.clone();
        if(g_appMainWinHwnd.eq(&0)){
            g_appMainWinHwnd = get_window_hwnd(&copy_appMainWin);
            }
        move |win, ev| match ev {
            enums::Event::Focus=>{
                if(g_appMainWinHwnd.eq(&0)){
                    g_appMainWinHwnd = get_window_hwnd(&copy_appMainWin);
                    }
                true
            }
            enums::Event::Show => {
                copy_AppRootView.set_visible_focus();

                if(g_appMainWinHwnd.eq(&0)){
                    g_appMainWinHwnd = get_window_hwnd(&copy_appMainWin);
                    }

                env::set_var("ikunWinHwnd", format!("{}",g_appMainWinHwnd).to_string());
                // unsafe { setWinIcon(appMainWinHwnd.try_into().unwrap()) };
                libWxIkunPlus::setWinIconMain(g_appMainWinHwnd);

                
                // libWxIkunPlus::setwinVisible(g_copy_dock_win_hwnd , true);

                println!("Show => window hwnd:{}",g_appMainWinHwnd);
                true
            }
            enums::Event::Close=>{

                println!("Close => window as {}",0);
                true
            }
            enums::Event::Focus=>{
            
            
                true
            }
            enums::Event::Push => {
                // 关闭按钮
                if (point_exist_hasmap.quit) {
                    libWxIkunPlus::setwinVisible(g_appMainWinHwnd , false);
                    fltk::app::quit();
                    // libWxIkunPlus::setwinVisible(g_copy_dock_win_hwnd , false);
                    // unsafe { setShowWindows((copyappMainWin.raw_handle() as i128).try_into().unwrap(), false) };
                }
                let mut has_inputPath = false;
                let mut has_ouputPath = false;
                let mut has_name = false;

                if (point_exist_hasmap.create) {
                    input_Console.buff.set_text("[用户] 创建新的配置文件");
                    println!("click => create");
                } else if (point_exist_hasmap.manageItme) {
                    // input_Console
                    //     .buff
                    //     .set_text("[用户] 很抱歉 当前还不支持配置管理");
                    println!("click => manageItme");

                    gui_manage_item::ManageItmeMain();
                } else if (point_exist_hasmap.shellOpenDatDir) {
                    input_Console
                        .buff
                        .set_text("[用户] 打开选取原始文件夹(dat 原目录)");
                  
                    // 有wx进程 而且有窗口
                    if(wh_mod::convert::is_developer()||(libWxIkunPlus::hasWeChat()&&libWxIkunPlus::hasWeChatWin())){
                        gui_select_user_ui::manage_tool_main();
                    // gui_select_user_base::mian_window();

                    }else{
                        // thread::spawn(||{
                        //     // libWxIkunPlus::stop("错误".to_owned(),"当前未发现wx进程或者未登录 拒绝提供选取方案".to_owned());
                        //     dialog::alert_default(format!("\n[错误] {}","当前未发现wx进程或者未登录 拒绝提供选取方案").as_str());

                        // });
                        libWxIkunPlus::stop("错误".to_owned(),"当前未发现wx进程或者未登录 拒绝提供选取方案".to_owned());
                        // dialog::alert_default(format!("\n[错误] {}","当前未发现wx进程或者未登录 拒绝提供选取方案").as_str());
                    }

                    println!("click => shellOpenDatDir");
                } else if (point_exist_hasmap.shellOpenExportDir) {
                    input_Console.buff.set_text("[用户] 打开选取导出到的文件夹");
                    
                    let mut open_path = libWxIkunPlus::openSelectFolder2();

                    input_Console
                        .buff
                        .append(format!("\n[选取器] 用户输入的文件路径为: {}",open_path).as_str());
                   
                    if(open_path.len()>2){
                        input_shellOpenExportDir.buff.remove(0, input_shellOpenExportDir.buff.length());
                        input_shellOpenExportDir.buff.append(&open_path);
                        if(input_shellName.buff.length()<2){
                            input_shellName.buff.remove(0, input_shellName.buff.length());
                            let file_name = Path::new(&open_path).file_name().unwrap();
                            input_shellName.buff.append(&format!("{:#?}",file_name).replace("\"", ""));
                        }
                    }
                  

                    println!("click => shellOpenExportDir");
                } else if (point_exist_hasmap.starting) {
                    input_Console.buff.set_text("[用户] 配置自启动");
                    // input_Console
                    // .buff
                    // .append(format!("\n[错误] 暂时不支持此功能 使用其他软件添加").as_str());
                    
                    if(libWxIkunPlus::setStartup()){
                       input_Console
                   .buff
                 .append(format!("\n[状态] 添加自启动成功").as_str());  
                    }else{
                        input_Console
                        .buff
                      .append(format!("\n[状态] 自启动已被移除").as_str());  
                    }
                    
                    // if libWxIkunPlus::hasStartup() {
                    //     btnEnableStarting.from_data(include_bytes!("../../assets/enable.png").to_vec(), 0, 0,20, 20);
                    // }else{
                    //     btnEnableStarting.from_data(include_bytes!("../../assets/un_enable.png").to_vec(), 0, 0,20, 20);
                    // }

                    println!("click => starting");
                } else if (point_exist_hasmap.test) {
                    input_Console.buff.set_text("[用户] 测试新的配置文件");
                    println!("click => test");
                }

                if (point_exist_hasmap.test || point_exist_hasmap.create) {
                    if (input_shellOpenDatDir.buff.length() < 1) {
                        input_Console
                            .buff
                            .append(format!("\n[错误] 尚未输入dat目录文件夹").as_str());
                    } else {
                        let mut path_dir = parse_dat_path(input_shellOpenDatDir.buff.text());

                        has_inputPath = true;

                        match fs::metadata(path_dir.attach_dir.clone()) {
                            Ok(metadata) => {
                                if (!metadata.is_dir()) {
                                    input_Console.buff.append(
                                        format!("\n[错误] dat目录文件夹 不是文件夹").as_str(),
                                    );
                                } else if point_exist_hasmap.test {
                                    input_Console.buff.append(
                                        format!("\n[测试] 正在扫描当前文件夹存在的dat图片")
                                            .as_str(),
                                    );
                                    fn bool_to_str (b:bool) -> &'static str {
                                        if b {"是"} else { "否" }
                                    }

                                    input_Console.buff.append(
                                        format!("\n[测试] 处理范围: 仅本月:{}   缩略图:{}   原图:{}   全部:{}   ",bool_to_str(path_dir.is_the_month),bool_to_str(path_dir.is_thumbnail),bool_to_str(path_dir.is_source),bool_to_str(path_dir.is_all))
                                            .as_str(),
                                    );

                                    let pattern = format!(
                                        "{}",
                                        Path::new(&path_dir.attach_dir.clone())
                                            .join("**/*.dat")
                                            .display()
                                            .to_string()
                                    );
                                    let mut index = 0;

                                    input_Console.buff.append(
                                        format!("\n[测试] 开始扫描 “{}” 中的dat文件",pattern)
                                            .as_str(),
                                    );

                                    for entry in glob(&pattern).unwrap() {
                                        let path = entry.unwrap().display().to_string();
                                        let base = Path::new(&path).file_name().unwrap().to_str().unwrap();
                                        index = index + 1;
                                    }

                                    input_Console.buff.append(
                                        format!("\n[测试] 在 “{}” \n中发现了 [{}] 个dat文件",pattern,index)
                                            .as_str(),
                                    );
                                }
                            }
                            Err(err) => {
                                input_Console.buff.append(
                                    format!(
                                        "\n[错误] dat目录文件夹 无法被读取 因为-> {}",
                                        err.to_string()
                                    )
                                    .as_str(),
                                );
                            }
                        };
                    }

                    if (input_shellName.buff.length() < 1) {
                        
                        input_Console
                            .buff
                            .append(format!("\n[错误] 配置名称为空").as_str());
                    }else{
                        has_name = true;
                    }

                    if (input_shellOpenExportDir.buff.length() < 1) {
                        
                        

                        input_Console
                            .buff
                            .append(format!("\n[错误] 尚未输入存储转码文件的目录").as_str());
                    } else {
                        has_ouputPath = true;
                        match fs::metadata(input_shellOpenExportDir.buff.text()) {
                            Ok(metadata) => {
                                if (!metadata.is_dir()) {
                                    input_Console.buff.append(
                                        format!("\n[错误] 存储转码文件的目录 不是文件夹").as_str(),
                                    );
                                } }
                            Err(err) => {
                                // input_Console.buff.append(
                                //     format!(
                                //         "\n[提醒] 存储转码文件的目录 无法被读取 因为-> {}",
                                //         err.to_string()
                                //     )
                                //     .as_str(),
                                // );
                            }
                        };
                    }
                }
               
                // println!("{} , {} , {} , {}",has_name,has_inputPath,has_ouputPath,point_exist_hasmap.create);

                if (has_name&&has_inputPath&&has_ouputPath&&point_exist_hasmap.create){
                    if(wh_mod::convert::is_developer()||(libWxIkunPlus::hasWeChat()&&libWxIkunPlus::hasWeChatWin())){
                        let conn: Connection = Connection::open("../../../ikun_user_data.db").unwrap();
                    
                        handle_dat::initialize_table(&conn);
                        match  conn.execute(
                            "INSERT INTO export_dir_path (name,time,path,ouput) values (?1, ?2, ?3, ?4)",
                            [input_shellName.buff.text(),Local::now().format("%Y-%m-%d").to_string(),input_shellOpenDatDir.buff.text(),input_shellOpenExportDir.buff.text()],
                        ) {
                          Ok(_)=>{
                            input_Console.buff.append(
                                format!("\n[存储] 添加成功").as_str(),
                            );
                          } 
                          Err(err)=>{
                            if(str_eq_ostr(err.to_string(),"UNIQUE constraint failed: export_dir_path.path")){
                                input_Console.buff.append(
                                    format!("\n[错误] 添加失败 因为-> {}","当前被导出的路径已经存在").as_str(),
                                );
                            }else
    
                            {
                                input_Console.buff.append(
                                    format!("\n[错误] 添加失败 因为-> {}",err.to_string()).as_str(),
                                );
                            }
                          } 
                        }
    
                        conn.close();
                        global_var_util::update_export_dir_itme_list();

                     }else{
                        //  libWxIkunPlus::stop("错误".to_owned(),"当前未发现wx进程 拒绝提供选取方案".to_owned())
                        // dialog::alert_default(format!("\n[错误] {}","当前未发现wx进程或者未登录 拒绝提供添加").as_str());
                        libWxIkunPlus::stop("错误".to_owned(),"当前未发现wx进程或者未登录 拒绝提供添加".to_owned());

                        // input_Console.buff.append(
                        //     format!("\n[错误] 添加失败 因为-> {}","当前未发现wx进程或者未登录 拒绝提供添加").as_str(),
                        // );
                       
                     }

                }
           

               
               
                true
            }

            enums::Event::Move => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;
                point_exist_hasmap = getFormPointSpace(x, y);
                // -处理鼠标图标的逻辑

                if (point_exist_hasmap.existAllBtn) {
                    win.clone().set_cursor(Cursor::Hand);
                } else {
                    win.clone().set_cursor(Cursor::Default);
                }

                true
            }

            // enums::Event:
            enums::Event::Drag => {
                if (y < 74) {
                    win.clone()
                        .set_pos(app::event_x_root() - x, app::event_y_root() - y);
                }

                true
            }
            _ => false,
        }
    });

    
    loop {
        Sleep(200);
        if (util::getVarBooleanValue("K9V7OKIIMR1E1_theInitializationWindowIsDisplayed".to_owned()))
        {
            appMainWin.show();
            appRootView.show();
            break;
        }
    }
    appRootView.set_visible_focus();
    // appMainWin.hide();
    // let path = gui_select_user_base::mian_window();


    Ok(MianWindowItme {
        appRootView,
        appMainWin,
    })
}

#![allow(warnings, unused)]

pub(crate) mod convert;
pub(crate) mod watch_path;
pub(crate) mod config;
mod mobile_screenshot;

use crate::wh_mod::convert::detect_image_format;
use chrono::{DateTime, Local};
// use lazy_static::lazy_static;
use std::collections::HashSet;
use std::{fs, path};
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{collections::hash_map::HashMap, ptr};
use std::{
    sync::atomic::Ordering,
    sync::Arc,
    sync::MutexGuard,
    sync::{atomic::AtomicUsize, OnceLock},
};
use std::ffi::OsStr;
use crate::util;

// lazy_static! {
//     static ref WALK_ATTACH_FILE_LIST: Mutex<HashMap<String, Vec<PathBuf>>> = Mutex::new(HashMap::new());
// }

static mut WALK_ATTACH_FILE_LIST: Option<HashMap<String, Vec<PathBuf>>> = Option::None;
static WALK_ATTACH_FILE_LIST_BIND: AtomicUsize = AtomicUsize::new(0);
static VARIABLE_INITIALIZE_WALK_ATTACH_FILE_LIST: OnceLock<bool> = OnceLock::new();

fn initialize_watch_path_token() {
    if *(VARIABLE_INITIALIZE_WALK_ATTACH_FILE_LIST
        .get()
        .unwrap_or_else(|| &false))
    {
        return;
    }
    unsafe {
        if WALK_ATTACH_FILE_LIST.is_none() {
            WALK_ATTACH_FILE_LIST.replace(HashMap::new());
        }
    }
    VARIABLE_INITIALIZE_WALK_ATTACH_FILE_LIST.set(true);
}

// 清理历史枚举记录
pub fn gc_walk_attach_file_list() {
    initialize_watch_path_token();
    let mutex = Arc::new(Mutex::new(&WALK_ATTACH_FILE_LIST_BIND));
    mutex.lock();
    let the_value: usize = WALK_ATTACH_FILE_LIST_BIND.load(Ordering::SeqCst);
    unsafe {
        if let Some(lazy_value) = WALK_ATTACH_FILE_LIST.as_mut() {
            lazy_value.clear();
        }
    }

    WALK_ATTACH_FILE_LIST_BIND.store(the_value + 1, Ordering::SeqCst);
    drop(mutex);
    // let mut lazy_value = WALK_ATTACH_FILE_LIST.lock().unwrap();
    // lazy_value.clear();
    // drop(lazy_value);
}

// 获取历史枚举列表
pub fn get_walk_attach_file_history() -> HashMap<String, Vec<PathBuf>> {
    initialize_watch_path_token();
    let mut result = HashMap::new();

    let mutex = Arc::new(Mutex::new(&WALK_ATTACH_FILE_LIST_BIND));
    mutex.lock();
    let the_value: usize = WALK_ATTACH_FILE_LIST_BIND.load(Ordering::SeqCst);
    unsafe {
        println!("get_walk_attach_file -> WALK_ATTACH_FILE_LIST -> {:?}",&WALK_ATTACH_FILE_LIST.as_mut().unwrap().len());

        if let Some(lazy_value) = WALK_ATTACH_FILE_LIST.as_mut() {
            for (key, value) in lazy_value.iter() {
                result.insert(key.to_string(), value.clone());
            }
        }
        println!("get_walk_attach_file -> WALK_ATTACH_FILE_LIST -> {:?}",&WALK_ATTACH_FILE_LIST.as_mut().unwrap().len());

    }

    WALK_ATTACH_FILE_LIST_BIND.store(the_value + 1, Ordering::SeqCst);
    drop(mutex);

    result
}

#[derive(Debug)]
pub struct WxFsRetrievalStruct {
    pub path: String,
    pub path2arr: Vec<String>,
    pub root: String,
    pub user_data: String,
    pub wxid: String,
    pub attach_id: String,
    pub attach_path: String,
    pub month: String,
    pub is_thumbnail: bool,
    pub is_dat: bool,
}

pub fn split_path(input_path: String) -> Vec<String> {
    let mut path2arr: Vec<String> = Vec::new();
    let mut str: String = String::new();
    // 按照 \\ / 分割路径
    for char in input_path.chars() {
        if char.to_string().bytes().eq("\\".to_string().bytes())
            || char.to_string().bytes().eq("/".to_string().bytes())
        {
            path2arr.push(str.clone());
            str.clear();
            continue;
        }
        str.push(char);
    }

    // 最后一个文本
    if str.len() != 0 {
        path2arr.push(str.clone());
    }

    let mut path2arr_filter = Vec::new();

    for path2 in path2arr {
        if (!path2.is_empty()) {
            path2arr_filter.push(path2);
        }
    }

    path2arr_filter
}

/**
 * 格式化wx位置路径
 * path : D:\...\weixin\WeChat Files\wxid_...\FileStorage\MsgAttach\attach_id\any\2023-08\*.dat
 */
pub fn wx_parse_path(wx_path: String) -> WxFsRetrievalStruct {
    let mut wx_fs_retrieval_struct = WxFsRetrievalStruct {
        path: "".to_string(),
        path2arr: vec![],
        root: "".to_string(),
        user_data: "".to_string(),
        wxid: "".to_string(),
        attach_id: "".to_string(),
        month: "".to_string(),
        is_thumbnail: false,
        is_dat: false,
        attach_path: String::new(),
    };

    let mut path2arr: Vec<String> = Vec::new();

    path2arr = split_path(wx_path.clone());
    if let Some(last) = path2arr.last() {
        wx_fs_retrieval_struct.is_thumbnail = last.to_ascii_lowercase().contains("_t.dat");
    }
    // if wx_path
    //     .to_ascii_lowercase()
    //     .contains("filestorage\\msgattach")
    //     || wx_path
    //         .to_ascii_lowercase()
    //         .contains("filestorage/msgattach")
    // {
    //
    // }

    if path2arr.len() > 1 {
        // 基础路径
        wx_fs_retrieval_struct.path = path2arr.join("\\");
        wx_fs_retrieval_struct.path2arr = path2arr.to_vec();
        let path_parse = Path::new(wx_fs_retrieval_struct.path.as_str());

        // 判断后缀名为dat 不区分大小写
        wx_fs_retrieval_struct.is_dat = path_parse
            .extension()
            .unwrap_or_else(|| "".as_ref())
            .to_string_lossy()
            .to_ascii_lowercase()
            .bytes()
            .eq("dat".bytes());

        // 根目录
        wx_fs_retrieval_struct.root = String::from(path2arr.get(0).unwrap());

        //解析 wx_id msg (置信度为 最后出现的一次)

        let mut the_path_next_name_list: Vec<String> = Vec::new();
        let mut msg_attach: Vec<String> = Vec::new();
        let mut wx_id = String::new();
        let mut user_data: Vec<String> = Vec::new();
        let mut path2buff = "".bytes();
        let mut len: usize = 0;
        for for_path_data in path2arr.to_vec() {
            let to_ascii_lowercase = for_path_data.to_ascii_lowercase();
            path2buff = to_ascii_lowercase.bytes();
            the_path_next_name_list.push(for_path_data.clone());

            // magat
            if path2buff.clone().eq("msgattach".bytes()) {
                for the_path in the_path_next_name_list.clone() {
                    msg_attach.push(the_path);
                }
                if path2arr.len() > len + 1 {
                    wx_fs_retrieval_struct.attach_id = path2arr[len + 1].clone();
                }
            }

            // user data root
            if path2buff.clone().eq("wechat files".bytes()) {
                for the_path in the_path_next_name_list.clone() {
                    user_data.push(the_path);
                }
            }

            // wx id
            if for_path_data.to_ascii_lowercase().contains("wxid_") {
                wx_id = for_path_data.clone();
            }

            // 判断是否是日期 2023-05
            if (for_path_data.contains("202") && for_path_data.len() > 5) {
                if (for_path_data.find("-") == Some(4)) {
                    let mut is_not_month = false;
                    let eq_data = "0123456789-";

                    for char in for_path_data.chars() {
                        if is_not_month {
                            break;
                        };

                        let mut eq_for = false;
                        for eq_char in eq_data.chars() {
                            if char.to_string().bytes().eq(eq_char.to_string().bytes()) {
                                eq_for = true;
                            }
                        }

                        if !eq_for {
                            is_not_month = true;
                        }
                    }

                    if (!is_not_month) {
                        wx_fs_retrieval_struct.month = for_path_data;
                    }
                }
            }

            len = len + 1;
        }

        wx_fs_retrieval_struct.wxid = wx_id;
        wx_fs_retrieval_struct.attach_path = msg_attach.join("\\");
        wx_fs_retrieval_struct.user_data = user_data.join("\\");
    }

    // println!("{},{} , {} , path2arr-> {:?}",wx_path.clone(),wx_path.contains("wxid_") ,wx_fs_retrieval_struct.wxid.is_empty(),path2arr.clone() );

    if wx_path.contains("wxid_") && wx_fs_retrieval_struct.wxid.is_empty() {
        for path2 in path2arr {
            // println!("{} ->  {}",path2.clone(), path2.contains("wxid_"));

            if path2.contains("wxid_") {
                wx_fs_retrieval_struct.wxid = path2;
            }
        }
    }

    wx_fs_retrieval_struct
}

#[derive(Debug)]
pub struct AccountItem {
    pub id: String,
    pub update_time: SystemTime,
    pub time_str: String,
}

/**
 * 从文件名中获取 account_id （如果有）
 * path : D:\...\weixin\WeChat Files\
 */
pub fn wx_account_id(path: PathBuf) -> AccountItem {
    let modified_list = sort_modified_dir_meta(path.as_path());
    let mut id = String::new();
    let mut up_time = UNIX_EPOCH;

    for read_name in modified_list {
        let file_name = read_name
            .dir
            .file_name()
            .unwrap_or_else(|| "".as_ref())
            .to_string_lossy();

        let time = read_name.metadata.modified().unwrap();

        if let Ok(time) = read_name.metadata.modified() {
            if up_time < time {
                up_time = time;
            }
        }

        if file_name.contains("account_") {
            let account_id = file_name.replace("account_", "");
            id = account_id;
        }
    }

    let modified_datetime: DateTime<Local> = up_time.into();
    let formatted_time = modified_datetime.format("%Y-%m-%d %H:%M:%S").to_string();

    return AccountItem {
        id,
        update_time: up_time,
        time_str: formatted_time,
    };
}

#[derive(Debug)]
pub struct WxReadWxid {
    pub account_id: String,
    pub wxid: String,
    pub update_time: SystemTime,
    pub update_time_str: String,
    pub attach: PathBuf,
    pub user_root: PathBuf,
}

// 格式化路径到 String
pub fn path2string<P: AsRef<Path>, S: AsRef<OsStr>, E: AsRef<String>>(path: P) -> Option<String> {
    let s = path.as_ref().to_string_lossy();
    if s.is_empty() { None } else { Some(s.into_owned()) }
}

//
pub fn list_path<P: AsRef<Path>, S: AsRef<OsStr>, E: AsRef<String>>(_path: P) -> Vec<String> {
    let path = path2string::<P,S,E>(_path).unwrap();
    let path_str = path.replace("\\", "/");
    let mut result = Vec::new();

    if let Ok(entries) = fs::read_dir(path_str) {
        for entry in entries {
            if let Ok(entry) = entry {
                // 如果它是一个目录，就获取它的名字
                if entry.file_type().map(|s| s.is_dir()).unwrap_or(false) {
                    if let Some(file_name) = entry.file_name().to_str() {
                        let path = file_name.to_string().replace("/","\\");
                        result.push(path);
                    }
                }
            }
        }
    }

    return result;
}

pub fn wildcard_path<P: AsRef<Path>, S: AsRef<OsStr>, E: AsRef<String>>(_path: P) -> Vec<String>{
    let path = path2string::<P,S,E>(_path).unwrap();
    let mut result = Vec::new();

    result
}

// 格式化路径到 D:\usersData\...\WeChat Files
pub fn format_wx_root(wx_root: &str) -> Option<path::PathBuf> {
    let match_feature = vec![
        vec!["*","FileStorage","MsgAttach"],
        vec!["*","config","AccInfo.dat"],
        vec!["*","Msg"],
    ];
    let mut split_path = split_path(wx_root.to_string());
    let mut split_path_join_to_wfs = String::new();

    for split in split_path.iter() {
        if split.as_bytes().eq("WeChat Files".as_bytes()) {

            //  D:\usersData\...\WeChat Files\ 必须得是文件夹
            if path::Path::new(split_path_join_to_wfs.as_str()).is_dir(){
                split_path_join_to_wfs.push_str(split);

                // 所有特征
                // D:\usersData\...\WeChat Files\wxid_0x666\FileStorage\MsgAttach
                // D:\usersData\...\WeChat Files\wxid_0x666\config\AccInfo.dat
                // D:\usersData\...\WeChat Files\wxid_0x666\Msg
                let mut temp_all_feature_path:Vec<String> = Vec::new();

                // 循环并格式化出特征路径
                for match_feature in match_feature.iter() {
                    let list_path  = format!("{}\\{}",split_path_join_to_wfs,match_feature.join("\\"));

                    // temp_all_feature_path.push();
                }

            }else {
                return None;
            }
        }
        split_path_join_to_wfs.push_str(split);
    }

    return None;
}

pub fn wx_search_wxid_root (wx_root: &str) -> Vec<String> {
    let result = Vec::new();
    let match_feature = vec![
        vec!["*","FileStorage","MsgAttach"],
        vec!["*","config","AccInfo.dat"],
        vec!["*","Msg"],
    ];

    result
}

/**
 * 获取id
 * path : D:\...\weixin\WeChat Files\
 */
pub fn wx_read_root_wxid(wx_root: &Path) -> Vec<WxReadWxid> {
    let mut wx_read_item_list: Vec<WxReadWxid> = Vec::new();

    // 获取基础路径信息
    for dir in read_dir(wx_root) {
        let dir_path = dir.path();
        let base = dir.file_name().to_string_lossy().to_string();

        if base.contains("wxid_") {
            let item = WxReadWxid {
                account_id: "".to_string(),
                wxid: base,
                update_time: UNIX_EPOCH,
                update_time_str: "".to_string(),
                attach: dir_path.join("FileStorage\\MsgAttach"),
                user_root: dir_path,
            };
            wx_read_item_list.push(item);
        }
    }

    // 读取更新时间和wxid(如果有)
    for index in 0..wx_read_item_list.len() {
        let get_wx_account_id = wx_account_id(wx_read_item_list[index].user_root.clone());
        wx_read_item_list[index].account_id = get_wx_account_id.id.clone();
        wx_read_item_list[index].update_time = get_wx_account_id.update_time.clone();
        wx_read_item_list[index].update_time_str = get_wx_account_id.time_str.clone();
    }

    wx_read_item_list
}

/**
 * 排序出最近更新的文件夹
 * path : any
 */
pub fn sort_modified_dir(dir_path: &Path) -> Vec<std::path::PathBuf> {
    let mut res: Vec<std::path::PathBuf> = Vec::new();
    let data = sort_modified_dir_meta(dir_path);
    for index in 0..data.len() {
        res.push(data[index].dir.clone());
    }
    res
}

#[derive(Debug)]
pub struct TmpMetadata {
    pub metadata: fs::Metadata,
    pub dir: std::path::PathBuf,
}

/**
 * 排序出最近更新的文件夹
 * path : any
 */
pub fn sort_modified_dir_meta(dir_path: &Path) -> Vec<TmpMetadata> {
    let mut modified_dir_list = Vec::new();
    let mut temp_metadata_list: Vec<TmpMetadata> = Vec::new();

    // 获取root 下的所有文件夹
    match fs::read_dir(dir_path) {
        Ok(rade_dir) => {
            for entry in rade_dir {
                let entry_value = entry.unwrap();

                // 获取文件属性表
                match entry_value.metadata() {
                    Ok(metadata) => {
                        let path = entry_value.path();
                        if path.is_dir() {
                            let obj = TmpMetadata {
                                metadata,
                                dir: entry_value.path(),
                            };
                            temp_metadata_list.push(obj);
                        }
                    }
                    Err(_) => {}
                }
            }
        }
        Err(_) => {}
    }

    temp_metadata_list.sort_by(|a, b| {
        a.metadata
            .modified()
            .unwrap()
            .cmp(&b.metadata.modified().unwrap())
    });

    for temp_metadata in temp_metadata_list {
        modified_dir_list.push(temp_metadata);
    }

    modified_dir_list
}

#[derive(Debug)]
pub struct TmpMetaDataf {
    pub metadata: fs::Metadata,
    pub file: std::path::PathBuf,
}
pub fn sort_modified_file_meta(dir_path: &Path) -> Vec<TmpMetaDataf> {
    let mut modified_dir_list = Vec::new();
    let mut temp_metadata_list: Vec<TmpMetaDataf> = Vec::new();

    // 获取root 下的所有文件夹
    match fs::read_dir(dir_path) {
        Ok(rade_dir) => {
            for entry in rade_dir {
                let entry_value = entry.unwrap();

                // 获取文件属性表
                match entry_value.metadata() {
                    Ok(metadata) => {
                        let path = entry_value.path();
                        if path.is_file() {
                            let obj = TmpMetaDataf {
                                metadata,
                                file: entry_value.path(),
                            };
                            temp_metadata_list.push(obj);
                        }
                    }
                    Err(_) => {}
                }
            }
        }
        Err(_) => {}
    }

    temp_metadata_list.sort_by(|a, b| {
        a.metadata
            .modified()
            .unwrap()
            .cmp(&b.metadata.modified().unwrap())
    });

    for temp_metadata in temp_metadata_list {
        modified_dir_list.push(temp_metadata);
    }

    modified_dir_list
}

/**
 * 读取文件夹下的路径
 * path : any
 */
pub fn read_dir(dir_path: &Path) -> Vec<fs::DirEntry> {
    let mut read_dir_list = Vec::new();

    match fs::read_dir(dir_path) {
        Ok(rade_dir) => {
            for entry in rade_dir {
                match entry {
                    Ok(entry) => read_dir_list.push(entry),
                    Err(_) => {}
                }
            }
        }
        Err(_) => {}
    }
    read_dir_list
}

#[derive(Debug)]
pub struct AttachThumbnail {
    pub is_thumbnail: bool,
    pub is_source: bool,
    // exists_source:bool,
    // exists_thumbnail: bool,
    pub thumbnail_path: String,
    // source_path: String,
    pub thumbnail: Vec<u8>,
    pub thumbnail_type: String,
    pub imag_id: String,
    pub attach_id: String,
}

impl Clone for AttachThumbnail {
    fn clone(&self) -> Self {
        AttachThumbnail {
            is_thumbnail: self.is_thumbnail.clone(),
            is_source: self.is_source.clone(),
            thumbnail_path: self.thumbnail_path.clone(),
            thumbnail: self.thumbnail.clone(),
            thumbnail_type: self.thumbnail_type.clone(),
            imag_id: self.imag_id.clone(),
            attach_id: self.attach_id.clone(),
        }
    }
}

/**
 * 获取指定 id 下的按照修改时间排序的 缩略图
 * path : D:\...\weixin\WeChat Files\wxid_...\FileStorage\MsgAttach\AttachID\
 */
pub fn read_attach_buff_thumbnail_data(dir_path: &PathBuf, thumbnail_len: usize,) -> Vec<AttachThumbnail> {
    let mut attach_thumbnail_list: Vec<AttachThumbnail> = Vec::new();
    let mut new_path = Path::new(dir_path).join("Thumb");
    let mut new_path_images = Path::new(dir_path).join("Image");

    if new_path.is_dir() {
        let modified_datetime: DateTime<Local> = SystemTime::now().into();
        let formatted_time: String = modified_datetime.format("%Y-%m").to_string();
        let mut new_path = new_path.join(formatted_time.clone());
        let mut new_path_images = new_path_images.join(formatted_time.clone());

        if new_path.is_dir() {
            let mut file_list = sort_modified_file_meta(&new_path);
            let mut path_list = Vec::new();
            for (index, file) in file_list.iter().rev().enumerate() {
                if index > thumbnail_len {
                    break;
                }
                path_list.push(file.file.clone());
            }
            drop(file_list);

            for path in path_list {
                let path_str = path.to_string_lossy().to_string();
                let name_str = path
                    .file_name()
                    .unwrap_or_else(|| (&"").as_ref())
                    .to_string_lossy()
                    .to_string();
                let path_str_imag =
                    Path::new(new_path_images.as_path()).join(name_str.replace("_t.dat", ".dat"));

                let wx_parse = wx_parse_path(path_str.clone());

                let mut attach_thumbnail = AttachThumbnail {
                    // 1
                    thumbnail_path: path_str.clone(),
                    // 1
                    is_thumbnail: true,
                    // 1
                    is_source: false,
                    // 1
                    // exists_source: path_str_imag.is_file(),
                    //
                    // source_path:path_str_imag.to_string_lossy().to_string(),
                    // 1
                    thumbnail: convert::convert_dat_images_buff(path.clone())
                        .unwrap_or_else(|_| Vec::new()),
                    // 1
                    thumbnail_type: String::new(),
                    // 1
                    attach_id: wx_parse.attach_id.clone(),
                    // 1
                    imag_id: name_str.replace("_t.dat", "").replace(".dat", ""),
                    // 1
                    // exists_thumbnail: true,
                };

                // 获取缩略图的格式
                match detect_image_format(&attach_thumbnail.thumbnail.clone()) {
                    Some(format) => attach_thumbnail.thumbnail_type = format.to_string(),
                    None => println!("Unknown image format."),
                }

                attach_thumbnail_list.push(attach_thumbnail);
            }
        }
    }

    attach_thumbnail_list
}

/**
 * 读取attach 下的最后更新N个 thumbnail (与read_attach_buff_thumbnail_data 不一样的是此函数一次性获取的是多个人的)
 * path : D:\...\weixin\WeChat Files\wxid_...\FileStorage\MsgAttach
 */
pub fn read_attach_buff_thumbnail_list(dir_path: &Path, len: usize, extract_len: usize, ) -> Vec<AttachThumbnail> {
    let mut attach_thumbnail_list: Vec<AttachThumbnail> = Vec::new();

    let sort_modified_list = sort_modified_dir_meta(dir_path);

    let n = if len >= sort_modified_list.len() {
        sort_modified_list.len()
    } else {
        len
    };

    for (i, sort_modified) in sort_modified_list.iter().rev().enumerate() {
        if i > n {
            break;
        }
        let dir_path = sort_modified.dir.clone();
        let read_attach_list = read_attach_buff_thumbnail_data(&dir_path, extract_len);

        for read_attach in read_attach_list {
            attach_thumbnail_list.push(read_attach);
        }
    }

    attach_thumbnail_list
}

/**
 * v2 版本将排序所有文件 而不是文件夹
 * 读取attach 下的最后更新N个 thumbnail (与read_attach_buff_thumbnail_data 不一样的是此函数一次性获取的是多个人的)
 * path : D:\...\weixin\WeChat Files\wxid_...\FileStorage\MsgAttach
 */
pub fn read_attach_buff_thumbnail_list_v2(dir_path: &Path, len: usize, extract_len: usize, ) -> Vec<AttachThumbnail> {
    let mut attach_thumbnail_list: Vec<AttachThumbnail> = Vec::new();

    let modified_datetime: DateTime<Local> = SystemTime::now().into();
    let formatted_time: String = modified_datetime.format("%Y-%m").to_string();

    let mut all_tmp_meta_data_file = Vec::new();

    if let Ok(dir_dir) = fs::read_dir(dir_path) {

        // D:\...\weixin\WeChat Files\wxid_...\FileStorage\MsgAttach\{attid}

        for dir_entry in dir_dir {
            if let Ok(entry) = dir_entry {

                // D:\...\weixin\WeChat Files\wxid_...\FileStorage\MsgAttach\{attid}\Thumb\2023-10

                let for_path = entry.path().join("Thumb").join(formatted_time.as_str());
                if for_path.is_dir() {
                    let mut c = sort_modified_file_meta(for_path.as_path());
                    if let Some(c) = c.pop() {
                        all_tmp_meta_data_file.push(c);
                    }
                }

            }
        }
    }

    // 排序创建时间
    all_tmp_meta_data_file.sort_by(|a, b| {
        let mut a_created = UNIX_EPOCH;
        let mut b_created = UNIX_EPOCH;

        if let Result::Ok(create) = a.metadata.created() {
            a_created = create;
        }

        if let Result::Ok(create) = b.metadata.created() {
            b_created = create;
        }

        a_created.cmp(&b_created)
    });

    all_tmp_meta_data_file.reverse();

    println!("all_tmp_meta_data_file-->{}",all_tmp_meta_data_file.len());
    // 取出五个任务
    for index in 0..len {
        if let Some(meta) = all_tmp_meta_data_file.get(index) {
            if let Some(str) = meta.file.to_str() {
                let dir_path = wx_parse_path(str.to_string());
                // println!("dir_path-> {:?}",&dir_path);

                let read_attach_list = read_attach_buff_thumbnail_data(&PathBuf::from(dir_path.attach_path.as_str()).join(dir_path.attach_id.as_str()), extract_len);

                println!("read_attach_list ->[{}] {}",dir_path.attach_path,read_attach_list.len(),);

                for read_attach in read_attach_list {
                    attach_thumbnail_list.push(read_attach);
                }
            }
        }

    }

    attach_thumbnail_list
}


fn visit_dirs(dir: &Path) -> Vec<PathBuf> {
    let mut path_list: Vec<PathBuf> = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    for visit_dir in visit_dirs(&path) {
                        path_list.push(visit_dir);
                    }
                } else {
                    path_list.push(path);
                }
            }
        }
    }
    path_list
}

/**
 * 匹配出 imag id
 */
pub fn walk_file(dir_path: &Path,run_tx: mpsc::Sender<(String, Vec<PathBuf>)>,img_id: String,) -> HashMap<String, Vec<PathBuf>> {
    // let mut file_list: Vec<PathBuf> = Vec::new();
    let mut wk_list: HashMap<String, Vec<PathBuf>> = HashMap::new();

    let (tx, rx) = mpsc::channel();

    let dir_root = read_dir(dir_path);
    let mut max_len = dir_root.len();

    for dir in dir_root {
        let tx = tx.clone();
        let paths = dir.path();
        let key = paths.to_string_lossy().to_string();

        thread::spawn(move || {
            let mut path_list: Vec<PathBuf> = visit_dirs(paths.as_path());
            tx.send((key, path_list));
            drop(tx);
            return;
        });
    }

    drop(tx);
    let mut index = 0;
    while index < 5 {
        let (key, data_vec) = rx.recv().unwrap_or_else(|_| (String::new(), Vec::new()));
        if key.is_empty() {
            index = index + 1;
            continue;
        }

        // 如果有img id
        if !img_id.is_empty() {
            for data in data_vec.to_vec() {
                if data
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .contains(img_id.clone().as_str())
                {
                    let key = format!("img_id::{}", img_id);
                    run_tx.send((key.clone(), vec![data.clone()]));
                    wk_list.insert(key, vec![data.clone()]);
                }
            }
        }
        run_tx.send((key.clone(), data_vec.clone()));
        wk_list.insert(key, data_vec);
    }

    // // set_walk_attach_file(wk_list.clone());
    // let mut lazy_value = WALK_ATTACH_FILE_LIST.lock().unwrap();

    // for (key, value) in wk_list.iter() {
    //     match lazy_value.insert(key.to_string(), value.clone()) {
    //         std::option::Option::Some(_) => {}
    //         // std::option::Option::None()=>{

    //         // }
    //         _ => {}
    //     };
    // }

    // drop(lazy_value);
    initialize_watch_path_token();
    let mutex = Arc::new(Mutex::new(&WALK_ATTACH_FILE_LIST_BIND));
    mutex.lock();
    let the_value: usize = WALK_ATTACH_FILE_LIST_BIND.load(Ordering::SeqCst);
    unsafe {
        println!("walk_file -> WALK_ATTACH_FILE_LIST -> {:?}",&WALK_ATTACH_FILE_LIST.as_mut().unwrap().len());
        println!("walk_file -> wk_list -> {:?}",wk_list.len());

        if let Some(lazy_value) = &WALK_ATTACH_FILE_LIST {
            for (key, value) in wk_list.iter() {
             
             if let Some(item) = WALK_ATTACH_FILE_LIST.as_mut() {
                    let key = key.to_string();
                    let push_value: Vec<PathBuf> = value.clone();

                    item.insert(key, push_value);
                };
            }
        }
        println!("walk_file -> WALK_ATTACH_FILE_LIST -> {:?}",&WALK_ATTACH_FILE_LIST.as_mut().unwrap().len());

    }

    WALK_ATTACH_FILE_LIST_BIND.store(the_value + 1, Ordering::SeqCst);
    drop(mutex);

    return wk_list;
}

#[derive(Debug)]
pub struct Dat2VarParseMeta {
    pub attach_id: String,
    pub attach_dir: String,
    // pub format_dir : String,
    pub is_the_month: bool,
    pub is_thumbnail: bool,
    pub is_source: bool,
    pub is_all: bool,
    pub is_sync:bool,
    pub is_video:bool,
    pub rename_rule: String,
    format_path_list: Vec<std::path::PathBuf>,
}

impl Dat2VarParseMeta {
    // 获取此可变体格式的格式化后的路径列表
    pub fn format(&mut self) -> Vec<std::path::PathBuf> {
        if (self.format_path_list.len() != 0) {
            return self.format_path_list.clone();
        }
        let mut path_list: Vec<std::path::PathBuf> = Vec::new();
        let attach_dir = Path::new(self.attach_dir.as_str());
        let the_month = chrono::Local::now().format("%Y-%m").to_string();

        if (attach_dir.exists()) {
            // 限定本月
            if (self.is_the_month) {
                if self.is_thumbnail {
                    let mut thumbnail_path = Path::new(attach_dir.clone())
                        .join("Thumb")
                        .join(the_month.clone());
                    path_list.push(thumbnail_path.clone());
                }
                if self.is_source {
                    let mut thumbnail_path = Path::new(attach_dir.clone())
                        .join("Image")
                        .join(the_month.clone());
                    path_list.push(thumbnail_path.clone());
                }
            }
            // 有限定缩略图和原图要求
            else if self.is_thumbnail || self.is_source {
                if self.is_thumbnail {
                    let mut thumbnail_path = Path::new(attach_dir.clone()).join("Thumb");
                    path_list.push(thumbnail_path.clone());
                }
                if self.is_source {
                    let mut thumbnail_path = Path::new(attach_dir.clone()).join("Image");
                    path_list.push(thumbnail_path.clone());
                }
            }
            // 没有声明 则全部
            else {
                path_list.push(attach_dir.to_path_buf());
            }
        }

        for path in path_list.clone() {
            self.format_path_list.push(path);
        }

        return path_list;
    }

    // 判断此路径是否属于此 可变体的路径
    pub fn exists(&mut self, eq_path: String) -> bool {
        let format_list = self.format();
        let eq_path = split_path(eq_path);

        let mut is_exists = false;
        for format in format_list {
            let split_format_path = split_path(format.to_string_lossy().to_string());
            let mut index: usize = 0;
            let mut match_path: bool = true;
            for eq_path in eq_path.clone() {
                if (split_format_path.len() <= index) {
                    break;
                }
                if !split_format_path[index].as_bytes().eq(eq_path.as_bytes()) {
                    match_path = false;
                }
                index = index + 1;
            }
            if match_path {
                is_exists = true;
            }
            if is_exists {
                break;
            }
        }
        is_exists
    }

    // 获取可变命名路径的实际需要生成的路径
    pub fn get_rename_output(&mut self) -> String{
        let mut result = self.rename_rule.clone();
        let mut the_data = HashMap::new();
        let time_info =util::get_time_info();

        the_data.insert("<现在>",time_info.time);
        the_data.insert("<年>",time_info.years);
        the_data.insert("<月>",time_info.month);
        the_data.insert("<日>",time_info.day);
        the_data.insert("<时>",time_info.hour);
        the_data.insert("<分>",time_info.minutes);
        // the_data.insert("<别名>","");
        // the_data.insert("<任务名>","");
        let mut mk_time_years = time_info.time_years;
        the_data.insert("<创建月>",mk_time_years);

        // let mut _type = "图片";
        // if self.is_video {
        //     _type = "视频";
        // }
        // else if self.is_thumbnail {
        //     _type = "缩略图";
        // }
        // // else if self.is_source {
        // //     _type = "手机截图";
        // // }
        // else {
        //     _type = "图片";
        // }

        // the_data.insert("<类型>",_type.to_string());
        the_data.insert("<哈希>",self.attach_id.clone());


        for (key,data) in the_data {
            result = result.replace(key,data.as_str());
        }

        result
    }
    pub fn writeFile(ex_dir:&str,){

    }
}

impl Clone for Dat2VarParseMeta {
    fn clone(&self) -> Self {
        Dat2VarParseMeta {
            attach_id: self.attach_id.clone(),
            attach_dir: self.attach_dir.clone(),
            is_the_month: self.is_the_month.clone(),
            is_thumbnail: self.is_thumbnail.clone(),
            is_source: self.is_source.clone(),
            format_path_list: self.format_path_list.clone(),
            is_all: self.is_all.clone(),
            is_video:self.is_video.clone(),
            is_sync: self.is_sync.clone(),
            rename_rule:self.rename_rule.clone(),
        }
    }
}

/**
 * 解析可变化路径
 */
pub fn parse_dat2var_path<T: util::OverloadedAnyStr >(input: T) -> Dat2VarParseMeta {
    // D:\usersData\weixin\WeChat Files/wxid_y.....1/FileStorage/MsgAttach/99e.......d..f,the_month,source,thumbnail
    let mut dat_parse_meta = Dat2VarParseMeta {
        attach_id: "".to_string(),
        attach_dir: "".to_string(),
        rename_rule:"".to_string(),
        // format_dir: "".to_string(),
        is_the_month: false,
        is_thumbnail: false,
        is_sync:false,
        is_video:false,
        is_source: false,
        is_all: false,
        format_path_list: Vec::new(),
    };
    let mut path_dir = input.to_string_default();
    let mut path_list = Vec::new();
    let binding = split_path(path_dir).join("\\");
    let lines: Vec<&str> = binding.split('*').collect();

    if (lines.is_empty()) {
        return dat_parse_meta;
    }

    for line in lines {
        let line_f = format!("{}", line);
        if (line_f.is_empty()) {
            continue;
        }
        if line_f.as_bytes().eq("the_month".as_bytes()) {
            dat_parse_meta.is_the_month = true;
        }
        if line_f.as_bytes().eq("source".as_bytes()) {
            dat_parse_meta.is_source = true;
        }
        if line_f.as_bytes().eq("thumbnail".as_bytes()) {
            dat_parse_meta.is_thumbnail = true;
        }
        
        if line_f.as_bytes().eq("video".as_bytes()) {
            dat_parse_meta.is_video = true;
        }

        if line_f.as_bytes().eq("Sync".as_bytes()) {
            dat_parse_meta.is_sync = true;
        }
        
        if line_f.contains("rename_rule=") {
            dat_parse_meta.rename_rule = line_f.to_string().replace("rename_rule=","");
        }

        dat_parse_meta.is_all = !dat_parse_meta.is_thumbnail.clone()
            && dat_parse_meta.is_source.clone()
            && dat_parse_meta.is_the_month.clone()&& dat_parse_meta.is_video.clone();

        path_list.push(line_f);
    }

    if let Some(attach_dir) = path_list.get(0) {
        dat_parse_meta.attach_dir = format!("{}",attach_dir);
    }

    if let Some(attach_id) = split_path(dat_parse_meta.attach_dir.clone()).pop() {
        dat_parse_meta.attach_id=attach_id;
    }
    
    return dat_parse_meta;
}


pub fn resolve_path(path: String) -> String {
    return split_path(path).join("\\");
}




// 自动按照设置获取显示的文本的消敏
pub fn get_show_mask_text <T: util::OverloadedAnyStr >(input: T) -> String {
    if config::is_show_mask() {
        format!("{}",util::get_mask_text(input.to_string_default().as_str()))
    }else {
        input.to_string_default()
    }
}
#![allow(warnings, unused)]

use crate::{global_var, gui_util, handle_dat, libWxIkunPlus, util::{str_eq_str, Sleep}, wh_mod};
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
use rusqlite::{Connection};
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

use crate::gui_util::img::ImgPreview;
use crate::libWxIkunPlus::closeWindow;
use crate::watching::insert_watch_path_token;
use fltk::draw::{height, width};
use fltk::image::PngImage;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::OnceLock;
use winapi::um::winnt::LPWSTR;
use winapi::um::winuser::{CloseWindow, SetActiveWindow};

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

    if( global_var::get_bool_default("gui::open::gui_detect_config")){
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
    win.set_id("gui::DoubleWindow::gui_detect_config::main");
    global_var::set_bool("gui::open::gui_detect_config",true);

    let mut main_window_back = ImgPreview::new(0, 0, 450, 453, "gui::ImgPreview::main_window::back");
    main_window_back.from_data(include_bytes!("../../assets/select_user_base/detect/detect.png").to_vec(), 0,0,win.width(),win.height());

    let mut next_btn = gui_util::hotspot::create_hotspot(203, 570, 233, 72);
    let mut gui_text_form01_state =gui_util::text::TextControl::new(320,53,0,0,11,"已经完成",[96, 139, 153]);
    let mut gui_text_form02_state =gui_util::text::TextControl::new(320,168,0,0,11,"已经完成",[96, 139, 153]);
    let mut gui_text_form03_state =gui_util::text::TextControl::new(320, 285,0,0, 11, "已经完成",[96, 139, 153]);

    let mut gui_imag_from01_state = ImgPreview::new(43,58,50,50,"gui_imag_from01_state");
    let mut gui_imag_from02_state = ImgPreview::new(43,175,50,50,"gui_imag_from02_state");
    let mut gui_imag_from03_state = ImgPreview::new(43,296,50,50,"gui_imag_from03_state");
    gui_imag_from01_state.from_data(include_bytes!("../../assets/select_user_base/detect/success.png").to_vec(),0,0,50,50);
    gui_imag_from02_state.from_data(include_bytes!("../../assets/select_user_base/detect/success.png").to_vec(),0,0,50,50);
    gui_imag_from03_state.from_data(include_bytes!("../../assets/select_user_base/detect/success.png").to_vec(),0,0,50,50);

    let mut gui_text_form01_title =gui_util::text::TextControl::new(111-60-10,55,0,0,12,"选择WX存储位置",[207, 207, 207]);
    let mut gui_text_form02_title =gui_util::text::TextControl::new(111-78-10,175,0,0,12,"选择被保存的对象",[207, 207, 207]);
    let mut gui_text_form03_title =gui_util::text::TextControl::new(111-65-10, 288,0,0, 12, "选择存储的选项",[207, 207, 207]);

    let mut gui_text_form01_cont =gui_util::text::TextControl::new(100,80,300,0,11,"此路径在您的 WX 中的  设置  /  文件管理  / 文件管理",[78, 78, 78]);
    let mut gui_text_form01_cont_2 =gui_util::text::TextControl::new(-8,96,300,0,11,"找到此路径",[78, 78, 78]);

    let mut gui_text_form02_cont =gui_util::text::TextControl::new(100-3,80+120,300,0,11,"您需要选择需要同步的对象， 在选择最近对象      ",[78, 78, 78]);
    let mut gui_text_form02_cont_2 =gui_util::text::TextControl::new(100,96+120,300,0,11,"如果不存在的话 您可以向找的人 随意发送一张图片",[78, 78, 78]);

    let mut gui_text_form03_cont =gui_util::text::TextControl::new(110,80+120+110,300,0,11,   "1.保存缩略  就是很小的图片 显示在聊天的 所有图片都有",[78, 78, 78]);
    let mut gui_text_form03_cont_2 =gui_util::text::TextControl::new(110,96+120+110,300,0,11,   "2.保存原图  当您打开了图片 就会下载大图 此图片为原图",[78, 78, 78]);
    let mut gui_text_form03_cont_3 =gui_util::text::TextControl::new(110,96+16+120+110,300,0,11,"3.保存本月  当此项打开 只会保存本月开始 之前将被忽略",[78, 78, 78]);

    let mut gui_text_btn_name =gui_util::text::TextControl::new(173, 405+3+1, 103,22,13, "朕知道了",[121, 121, 121]);

    let mut next_btn = gui_util::hotspot::create_hotspot(140, 395, 162, 51);

    // global_var::set_bool("user::config::check_button_the_month",false);
    // global_var::set_bool("user::config::check_button_source",false);
    // global_var::set_bool("user::config::check_button_thumbnail",false);
    // global_var::set_str("user::config::input_select_dir","".to_string());
    // global_var::set_i32("user::config::select_user_thumbnail_obj",-1);
    macro_rules! update_gui_state {
                    () => {
        
        if(
            // !global_var::get_bool_default("user::config::check_button_sync")&&
            !global_var::get_bool_default("user::config::check_button_video")&&
            !global_var::get_bool_default("user::config::check_button_thumbnail")&&
            !global_var::get_bool_default("user::config::check_button_source")&&
            !global_var::get_bool_default("user::config::check_button_the_month")
        ){
        gui_text_form03_state.set_label("尚未选择".to_string());
        gui_text_form03_state.set_color(215, 97, 97);
        gui_imag_from03_state.from_data(include_bytes!("../../assets/select_user_base/detect/fail.png").to_vec(),0,0,50,50);
    }else{
        gui_text_form03_state.set_label("已经完成".to_string());
        gui_text_form03_state.set_color(96, 139, 153);
        gui_imag_from03_state.from_data(include_bytes!("../../assets/select_user_base/detect/success.png").to_vec(),0,0,50,50);
    }

    if(global_var::get_i32_or("user::config::select_user_thumbnail_obj",-1)==-1){
        gui_text_form02_state.set_label("尚未选择".to_string());
        gui_text_form02_state.set_color(215, 97, 97);
        gui_imag_from02_state.from_data(include_bytes!("../../assets/select_user_base/detect/fail.png").to_vec(),0,0,50,50);

    }else{
        gui_text_form02_state.set_label("已经完成".to_string());
        gui_text_form02_state.set_color(96, 139, 153);
        gui_imag_from02_state.from_data(include_bytes!("../../assets/select_user_base/detect/success.png").to_vec(),0,0,50,50);
    }

    if(global_var::get_string_default("user::config::user_select_path").is_empty()){
        gui_text_form01_state.set_label("尚未选择".to_string());
        gui_text_form01_state.set_color(215, 97, 97);
        gui_imag_from01_state.from_data(include_bytes!("../../assets/select_user_base/detect/fail.png").to_vec(),0,0,50,50);
    }else{
        gui_text_form01_state.set_label("已经完成".to_string());
        gui_text_form01_state.set_color(96, 139, 153);
        gui_imag_from01_state.from_data(include_bytes!("../../assets/select_user_base/detect/success.png").to_vec(),0,0,50,50);
    }
                    };
                }

    thread::spawn(move||{
        let mut is_open_win  = global_var::get_bool_default("gui::open::gui_detect_config");
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
}#![allow(unused_variables)]
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
#![allow(warnings, unused)]

use fltk::enums::{Color, Cursor, Event, FrameType, Key};
use fltk::{prelude::*, *};
use fltk::app::event_key;
use fltk_theme::{color_themes, ColorTheme, SchemeType, ThemeType, WidgetScheme, WidgetTheme};

pub struct TextPreview {
    pub buf:text::TextBuffer,
    pub preview:text::TextDisplay,
    x:i32,
    y:i32,
    height:i32,
    width:i32,
    size:i32
}
impl Clone for TextPreview {
    fn clone(&self) -> Self {
        TextPreview{
            buf: self.buf.clone(),
            preview:self.preview.clone(),
            x: self.x.clone(),
            y: self.y.clone(),
            width: self.width.clone(),
            height: self.height.clone(),
            size:self.size.clone()
        }
    }
}
impl TextPreview {
    // 创建
    pub fn new(x:i32, y:i32, width: i32, height: i32, size:i32, input:&str, rgb: [u8; 3]) -> Self {
        let mut buf = text::TextBuffer::default();
        buf.set_text(input);

        let mut txt = text::TextDisplay::default()
            .with_size(width, height)
            .center_of_parent();
        txt.set_buffer(buf.clone());
        txt.set_pos(x,y);
        txt.set_frame(fltk::enums::FrameType::NoBox);
        txt.set_scrollbar_size(-1);
        txt.set_text_size(size);
        txt.set_text_color(Color::from_rgb(rgb[0],rgb[1],rgb[2]));
        txt.scroll(0, 0);
        txt.set_label_type(fltk::enums::LabelType::None);

        // txt.handle(move |txt, event| match event {
        //     Event::Move=>{
        //
        //         true
        //     }
        //     Event::Leave=>{
        //
        //         true
        //     }
        //     _ => false,
        // });

        TextPreview{
            buf:buf,
            preview:txt,
            x,
            y,
            height,
            width,
            size
        }
    }

    // 获取文本内容
    pub fn get_label(&self)->String {
        return self.preview.label();
    }

    // 设置文本内容
    pub fn set_label(&mut self,input:String){
        self.preview.set_label(input.as_str());
        self.preview.redraw_label();
        self.preview.redraw();
    }

    // 设置颜色
    pub fn set_color(&mut self, r: u8, g: u8, b: u8) -> &mut TextPreview {
        self.preview.set_text_color(Color::from_rgb(r,g,b));
        self
    }

    // 判断鼠标是否在当前元素
    pub fn existPoint(&self, x: i32, y: i32) -> bool {
        return x > self.preview.x()
            && x < self.preview.x() + self.preview.width()
            && y > self.preview.y()
            && y < self.preview.y() + self.preview.height();
    }
    pub fn set_back_color(&mut self, r: u8, g: u8, b: u8) -> &mut TextPreview {
        self.preview.set_color(Color::from_rgb(r,g,b));
        self
    }

}




pub struct TextControl {
    pub text: frame::Frame,
}

impl TextControl {
    // 创建
    pub fn new(x:i32, y:i32, width: i32, height: i32, size:i32, input: &str, rgb: [u8; 3]) -> Self {
        let mut text = frame::Frame::new(x, y, (if width==0 {input.len()as i32 * size}else {width}), (if height==0 {size+2}else {height}), "");
        text.set_label(input);
        text.set_label_size(size);
        text.set_label_color(Color::from_rgb(rgb[0],rgb[1],rgb[2]));

        Self { text  }
    }
    // 获取文本内容
    pub fn get_label(&self)->String {
        return self.text.label();
    }
    // 设置文本内容
    pub fn set_label(&mut self,input:String){
        self.text.set_label(input.as_str());
        self.text.redraw_label();
        self.text.redraw();
    }
    // 设置颜色
    pub fn set_color(&mut self, r: u8, g: u8, b: u8)  {
        self.text.set_label_color(Color::from_rgb(r,g,b));

    }
    // 判断鼠标是否在当前元素
    pub fn existPoint(&self, x: i32, y: i32) -> bool {
        return x > self.text.x()
            && x < self.text.x() + self.text.width()
            && y > self.text.y()
            && y < self.text.y() + self.text.height();
    }

}
impl Clone for TextControl {
    fn clone(&self) -> Self {
        TextControl {
            text:self.text.clone()
        }
    }
}

use std::ffi::{c_int, c_uint, c_ulong, OsStr};
use std::iter::once;
use std::mem::transmute;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;
type DWORD = c_ulong;

pub fn encode_lpcstr(s: &str) -> Vec<i8> {
    let mut arr: Vec<i8> = s.bytes().map(|x| x as i8).collect();
    arr.push(0);
    arr
}

pub fn encode_wide_with_null(s: impl AsRef<str>) -> Vec<u16> {
    let wide: Vec<u16> = OsStr::new(s.as_ref())
        .encode_wide()
        .chain(once(0))
        .collect();
    wide
}

extern "system" {
    fn WideCharToMultiByte(
        page: c_uint,
        flags: c_ulong,
        wide_str: *const u16,
        wide_str_len: c_int,
        multi_str: *mut i8,
        multi_str_len: c_int,
        default_char: *const i8,
        used_default_char: *mut i32,
    ) -> c_int;
    fn MultiByteToWideChar(
        CodePage: c_uint,
        dwFlags: DWORD,
        lpMultiByteStr: *const u8,
        cbMultiByte: c_int,
        lpWideCharStr: *mut u16,
        cchWideChar: c_int,
    ) -> c_int;
}

/// Convert a rust string to a winapi-usable 0-terminated unicode u16 Vec
pub fn winapi_str<T: AsRef<OsStr>>(input: T) -> Vec<u16> {
    let mut buf = Vec::with_capacity(input.as_ref().len());
    buf.extend(input.as_ref().encode_wide());
    buf.push(0);
    buf
}

const CP_ACP: c_uint = 0;
const CP_OEMCP: c_uint = 1; // default to OEM  code page
const CP_MACCP: c_uint = 2; // default to MAC  code page
const CP_THREAD_ACP: c_uint = 3; // current thread's ANSI code page
const CP_SYMBOL: c_uint = 42; // SYMBOL translations

const CP_UTF7: c_uint = 65000; // UTF-7 translation
const CP_UTF8: c_uint = 65001;

// If the conversion was lossy, returns Err(lossy_result)
pub fn ansi_codepage_cstring<T: AsRef<OsStr>>(input: T) ->Result<Vec<i8>,Vec<i8>> {

    unsafe {
        let os_str = input.as_ref();
        let unicode = winapi_str(os_str);
        let length = WideCharToMultiByte(
            CP_ACP,
            0,
            unicode.as_ptr(),
            unicode.len() as i32,
            null_mut(),
            0,
            null_mut(),
            null_mut(),
        );
        let mut buffer = vec![0i8; length as usize];
        let mut used_default_char = 0;
        WideCharToMultiByte(
            CP_ACP,
            0,
            unicode.as_ptr(),
            unicode.len() as i32,
            buffer.as_mut_ptr() as *mut i8,
            length,
            null_mut(),
            &mut used_default_char,
        );
        
        if used_default_char != 0 {
            Err(buffer)
        } else {
            Ok(buffer)
        }
    }
    
}


pub fn utf16_to_utf8(utf16_string: &[u16]) -> String {
    let utf8_vec: Vec<u8> = utf16_string
        .iter()
        .flat_map(|&c| std::char::from_u32(c as u32))
        .flat_map(|c| c.to_string().as_bytes().to_vec())
        .collect();
    
    String::from_utf8(utf8_vec).unwrap_or_else(|_| String::new())
}#![allow(warnings, unused)]

use fltk::enums::{Color, Cursor, Event, FrameType, Key};
use fltk::{prelude::*, *};
use fltk::app::event_key;
use fltk_theme::{color_themes, ColorTheme, SchemeType, ThemeType, WidgetScheme, WidgetTheme};


pub struct varTagControl {
    pub title: frame::Frame,
    pub name: frame::Frame,
    x:i32, y:i32, width: i32, height: i32,
    pub id:String,
    pub data:String
}

impl varTagControl {
    // 创建
    pub fn new(x:i32, y:i32, width: i32, height: i32,title:&str , name: &str, data:&str) -> Self {
        // 按照总宽度分配可用控件
        // let all_input_len = format!("{}{}",&title,&name).len() as i32;
        // let width_per_char = width / all_input_len;
        // let title_len = title.len() as i32;


        // let name_width = width_per_char * (all_input_len - title_len)+5;
        // let title_width = width - name_width;

        // 计算字符宽度 如果包含了宽字符 前面要加几像素
        let title_len =  title.chars().count() as i32;
        let mut title_width = title_len*11 ;
        let has_title_wide = title_len!=title.len() as i32;
        let mut title_x =(if has_title_wide {5} else {0} )+ x.clone();

        // 添加文本显示控件
        let mut title_frame = frame::Frame::new(title_x, y,title_width ,height , "");
        title_frame.set_label(title);
        title_frame.set_label_size(12);
        title_frame.set_label_color(Color::from_rgb(77, 77, 77));


        let name_len =  title.chars().count() as i32;
        let mut name_width = title_len*12 ;
        let has_name_wide = title_len!=title.len() as i32;
        let mut name_x =(if has_title_wide {5} else {-20} )+ x.clone()+name_len*11;
        let name_text_size = name.len();
        if  name_text_size==1 {
            name_x += 6;
        }

        if  name.contains("月") {
            name_x -= 10;
        }

        if name_text_size>9 {
            name_x += 15;
        }

        if name_text_size>11 {
            name_x += 25;
        }

        if name_text_size>28 {
            name_x += 25;
        }

        let mut name_frame = frame::Frame::new(name_x, y, name_width ,height, "");
        name_frame.set_label(name);
        name_frame.set_label_size(12);
        name_frame.set_label_color(Color::from_rgb(40, 40, 40));

        Self { title:title_frame , name:name_frame, x, y, width, height , data:data.to_string(),id:title.to_string().replace(" ","").replace(":","")}
    }


    // 判断鼠标是否在当前元素
    pub fn existPoint(&self, x: i32, y: i32) -> bool {
        return x > self.x
            && x < self.x + self.width
            && y > self.y
            && y < self.y + self.height;
    }
    pub(crate) fn get_var(&self) -> String{
        return format!("<{}>",self.id.clone().replace("%",""));
    }
}
impl Clone for varTagControl {
    fn clone(&self) -> Self {
        varTagControl {
            title:self.title.clone(),
            name:self.name.clone(),
            x: self.x.clone(),
            y: self.y.clone(),
            width: self.width.clone(),
            height: self.height.clone(),
            id:self.id.clone(),
            data:self.data.clone()
        }
    }
}

#![allow(dropping_references)]

use crate::util::{str_eq_str, Sleep};
use crate::{global_var, util, get_bool,set_bool};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize,AtomicBool, Ordering};
use std::sync::{mpsc, OnceLock};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;
use crate::console_log;
static WATCH_PATH_ID: AtomicUsize = AtomicUsize::new(0);

// lazy_static! {
//     static ref WATCH_NEXT_EXITS: Mutex<bool> = Mutex::new(false);
// }

static WATCH_NEXT_EXITS: AtomicBool = AtomicBool::new(false);

struct TmepMetadata {
    pub metadata: fs::Metadata,
    pub dir: std::path::PathBuf,
}

pub fn has_next_exits() -> bool {
    get_bool!(WATCH_NEXT_EXITS)
}

pub fn un_next_exits() -> bool {
    set_bool!(WATCH_NEXT_EXITS,false)
}

pub fn initialize_next_exits() -> bool {
    set_bool!(WATCH_NEXT_EXITS,true)
}

fn get_next_id() -> usize {
    let mut id: usize = 0;
    let mutex = Arc::new(Mutex::new(&WATCH_PATH_ID));
    mutex.lock();
    id = WATCH_PATH_ID.fetch_add(1, Ordering::Relaxed);
    drop(mutex);
    id
}
pub fn get_the_id() -> usize {
    let mut id: usize = 0;
    let mutex = Arc::new(Mutex::new(&WATCH_PATH_ID));
    mutex.lock();
    id = WATCH_PATH_ID.load(Ordering::SeqCst);
    drop(mutex);
    id
}

/**
 *处理文件刷新
 */
pub fn watch_path_puppet(dir_path: String, send_main_tx: mpsc::Sender<PathBuf>) ->usize{
    initialize_next_exits();
    let mut watch_puppet_id = get_next_id();
    watch_puppet_id = get_the_id();

    thread::spawn(move || {
        let (tx, rx) = std::sync::mpsc::channel();
        println!(
            "watch_path_puppet-> {:?}  -> id {}",
            dir_path.clone(),
            watch_puppet_id.clone()
        );

        // let mut config = Config::default().with_poll_interval(Duration::from_millis(1200));

        let mut watcher = RecommendedWatcher::new(tx,Config::default()).unwrap();
        watcher
            .watch(dir_path.as_ref(), RecursiveMode::Recursive)
            .unwrap();

        //  需要处理的任务已经更新了 释放
        thread::spawn(move || loop {
            Sleep(300);
            if (watch_puppet_id != get_the_id() || !has_next_exits()) {                
                drop(&watcher);
                return;
            }
        });

        let mut shake_path = std::collections::HashSet::new();
        for res in rx {
            // 需要处理的任务已经更新了 释放 为什么不用un 因为会误操作其他的
            match res {
                Ok(event) => {

                    if shake_path.len()>5 {
                        shake_path.clear();
                    }

                    for value in event.clone().paths {
                        let mut paths = value.clone().display().to_string();
                        let mut ext = util::path_extension(&value);

                        // 是文件 后缀是dat 更新方式是修改
                        if (value.is_file()
                            &&paths.contains("MsgAttach")
                            && (event.clone().kind.is_modify())
                            && str_eq_str("dat".to_owned(), ext.clone()))
                        {
                            if shake_path.insert(value.clone()) {
                                let send_main_tx = send_main_tx.clone();
                                thread::spawn(move || {
                                    Sleep(1888);
                                    send_main_tx.send(value.clone());
                                    println!("is_modify [is_modify] -> {:?}  id ->  {}", value.clone(),watch_puppet_id.clone());
                                });

                            }

                        }
                    }
                }
                Err(e) => {
                    console_log!(format!(
                        "[报错] 检测选择器界面文件更新错误 因为-> {}",
                        e.to_string()
                    ));
                }
            }
        }
    });
    return watch_puppet_id.clone();
}


#include "pch.h"
#include "./main.hpp"
#include <Psapi.h>
#include "./tray.hpp"
#include "./registr.hpp"
#include "./text.hpp"
#include "./usb.hpp"
#include "./window.hpp"
#include "./process_lib.hpp"
#include <thread>

#define HMC_CHECK_CATCH catch (char *err){};
HWND winmian = NULL;

namespace Mutex
{
    map<string, HANDLE> AllMutexHandles;
    /**
     * @brief 创建互斥体
     *
     * @param MutexName
     * @return true
     * @return false
     */
    bool create(string MutexName)
    {
        bool has_mut_exist = false;

        HANDLE hMutex = CreateMutexA(NULL, FALSE, MutexName.c_str());

        AllMutexHandles.insert(pair<string, HANDLE>(MutexName, hMutex));

        if (hMutex == NULL)
        {
            has_mut_exist = true;
        }
        // 检查是否已经存在同名的互斥体
        if (GetLastError() == ERROR_ALREADY_EXISTS)
        {
            has_mut_exist = true;
            CloseHandle(hMutex);
        }

        return !has_mut_exist;
    }

    /**
     * @brief 判断是否有这个互斥体
     *
     * @param MutexName
     * @return true
     * @return false
     */
    bool has(string MutexName)
    {
        bool has_mut_exist = true;

        HANDLE hMutex;

        hMutex = OpenMutexA(MUTEX_ALL_ACCESS, FALSE, MutexName.c_str());
        if (NULL == hMutex)
        {
            has_mut_exist = false;
        }

        CloseHandle(hMutex);
        return has_mut_exist;
    }

    /**
     * @brief 删除通过此方法创建的互斥体
     *
     * @param MutexName
     * @return true
     * @return false
     */
    bool remove(string MutexName)
    {

        auto it = AllMutexHandles.find(MutexName);
        if (it == AllMutexHandles.end())
        {
            return false;
        }
        while (it != AllMutexHandles.end())
        {
            CloseHandle(it->second);
            it++;
        }

        if (!has(MutexName))
        {
            AllMutexHandles.erase(MutexName);
            return true;
        }
        return false;
    }

    /**
     * @brief 获取当前已经创建的互斥体内容
     *
     * @return vector<string>
     */
    vector<string> list()
    {
        vector<string> list;
        map<string, HANDLE>::iterator it = AllMutexHandles.begin();

        while (it != AllMutexHandles.end())
        {
            list.push_back(it->first);
            it++;
        }
        return list;
    }
}

// 获取进程可执行文件路径
string getProcessidFilePath(int ProcessID)
{
    string Run_lpFilename = "";
    HANDLE hProcess = OpenProcess(PROCESS_ALL_ACCESS, FALSE, ProcessID);
    char lpFilename[1024];
    if (hProcess == nullptr)
    {
        CloseHandle(hProcess);
        return Run_lpFilename;
    }
    GetModuleFileNameExA(hProcess, NULL, (LPSTR)lpFilename, 1024);
    CloseHandle(hProcess);
    return string(lpFilename);
}



void _setWinIconMain(long hwnds)
{
    winmian = (HWND)hwnds;

    if (!IsWindow(winmian))
    {
        return;
    }
    // setWindowTop(winmian,true);
    string execPath = getProcessidFilePath(_getpid());
    hmc_window::setWindowIcon(winmian, execPath, 0);
}

void _setWinIcon(long hwnds)
{
    HWND hwnd = (HWND)hwnds;
    if (!IsWindow(hwnd))
    {
        return;
    }
    // setWindowTop(winmian,true);
    string execPath = getProcessidFilePath(_getpid());
    hmc_window::setWindowIcon(hwnd, execPath, 0);
}

bool _isWindow(long hwnds)
{
    HWND hwnd = (HWND)hwnds;
    return ::IsWindow(hwnd);
}

void _setWindowShake(long hwnds) {
    HWND hwnd = (HWND)hwnds;
    hmc_window::setWindowShake(hwnd);
}

void _setTaskbarWin(long hwnds) {
    HWND main = HWND(hwnds);
    hmc_window::removeWindowFrame(main);
    hmc_window::setMoveWindow(main, -66666, -666666, 1, 1);
    UpdateWindow(main);
    hmc_window::setWindowTransparent(main, 0);
    string execPath = getProcessidFilePath(_getpid());
    hmc_window::setWindowIcon(main, execPath, 0);
}

bool ikun_user_auto_disable_sync = false;
bool sync_token = false;
bool ikun_app_startup = false;
bool initializationWindowIsDisplayed = false;

bool _hasInitWindowIsDisplayed() {
    return initializationWindowIsDisplayed;
}

bool _setInitWindowIsDisplayed(bool initWindowIsDisplayed) {
    initializationWindowIsDisplayed = initWindowIsDisplayed;
    return initializationWindowIsDisplayed;
}

void _set_tray()
{
    string execPath = getProcessidFilePath(_getpid());
    hmc_window::setWindowIcon(winmian, execPath, 0);
    hmc_tray::start();
    hmc_tray::setTrayIcon(execPath, 0);
    ikun_user_auto_disable_sync = hmc_registr::hasRegistrKey(HKEY_CURRENT_USER, "SOFTWARE\\WxAutoExIm", "auto_sync");
     _hasStartup();
   
    hmc_tray::addMenuItem(hmc_tray::Menu::check("自动同步", "btn::auto_sync", ikun_user_auto_disable_sync));
     hmc_tray::addMenuItem(hmc_tray::Menu::check("开机启动", "btn::app_startup", ikun_app_startup));
     hmc_tray::addMenuItem(hmc_tray::Menu::menu("立即同步", "btn::auto_sync_token"));
     hmc_tray::addMenuItem(hmc_tray::Menu::separator("btn::separator::01"));
     hmc_tray::setMenuItmeEnable("btn::app_startup",true);
     hmc_tray::setMenuItmeEnable("btn::auto_sync", true);

    hmc_tray::addMenuItem(hmc_tray::Menu::menu("退出程序", "btn::quit_app"));

    hmc_tray::on("click", []()
                 {
            //_putenv("K9V7OKIIMR1E1_theInitializationWindowIsDisplayed=true");
            //_putenv_s("K9V7OKIIMR1E1_theInitializationWindowIsDisplayed", "true");
            initializationWindowIsDisplayed = true;

                     if (IsWindowVisible(winmian))
                     {
                         ShowWindow(winmian, 0);
                     }
                     else
                     {
                         ShowWindow(winmian, SW_RESTORE);
                         SetFocus(winmian);
                         SetActiveWindow(winmian);
                         SetForegroundWindow(winmian);
                     } });

    hmc_tray::on("btn::auto_sync", []()
                 {
            bool select = hmc_tray::getMenuItme("btn::auto_sync").select;
            ikun_user_auto_disable_sync = select;

            if (select) {
                hmc_registr::setRegistrValue(HKEY_CURRENT_USER, "SOFTWARE\\WxAutoExIm", "auto_sync", string("true"));
            }
            else {
                hmc_registr::removeRegistrValue(HKEY_CURRENT_USER, "SOFTWARE\\WxAutoExIm", "auto_sync");
            }
                 });
    hmc_tray::on("btn::auto_sync_token", []()
        {
            sync_token = true;
        });
    
    hmc_tray::on("btn::app_startup", []()
        {
            bool select = hmc_tray::getMenuItme("btn::app_startup").select;
            ikun_app_startup = select;
        });

    hmc_tray::once("btn::quit_app", []()
                   {
                       hmc_tray::close();

                       exit(0);
                   });
}

bool _setCloseWindow(long hwnds, bool force)
{
    if (force)
    {
        CloseHandle((HWND)hwnds);
        DestroyWindow((HWND)hwnds);
    }
    return CloseWindow((HWND)hwnds);
}

bool _setShowWindows(long hwnds, bool visible)
{
    HWND hwnd = (HWND)hwnds;
    ShowWindow(hwnd, visible ? SW_RESTORE : 0);
    if (visible)
    {
        SetActiveWindow(hwnd);
        SetForegroundWindow(hwnd);
    }

    return true;
}

bool _setMinWindows(long hwnds)
{
    HWND hwnd = (HWND)hwnds;
    return ShowWindow(hwnd, SW_MINIMIZE);
}


bool _setWindowsTop(long hwnds, bool visible)
{
    return hmc_window::setWindowTop((HWND)hwnds, visible);
}

bool _createMutex(const char* MutexName)
{
    string copy_MutexName = (MutexName);
    return Mutex::create(copy_MutexName);
}

bool _hasMutex(const char* MutexName)
{
    string copy_MutexName = (MutexName);
    return Mutex::has(copy_MutexName);
}

bool _removeMutex(const char* MutexName)
{
    string copy_MutexName = (MutexName);

    return Mutex::remove(copy_MutexName);
}

bool _Alert(const char* title ,const char* info) {
    string copy_title = (title);
    string copy_info = (info);

    int To_MessageBoxA = MessageBoxA(NULL, copy_info.c_str() , copy_title.c_str(),MB_OK);
    if (To_MessageBoxA == 1 || To_MessageBoxA == 6)
    {
        return true;
    }
    else
    {
        return false;
    }
}

bool _Confirm(const char* title, const char* info) {
    string copy_title = (title);
    string copy_info = (info);

    int To_MessageBoxA = MessageBoxA(NULL, copy_info.c_str(), copy_title.c_str(), MB_OKCANCEL);
    if (To_MessageBoxA == 1 || To_MessageBoxA == 6)
    {
        return true;
    }
    else
    {
        return false;
    }
}

void _Stop(const char* title, const char* info) {
    string copy_title = (title);
    string copy_info = (info);

    int To_MessageBoxA = MessageBoxA(NULL, copy_info.c_str(), copy_title.c_str(), MB_ICONERROR);
}

void _Error(const char* title, const char* info) {
    string copy_title = (title);
    string copy_info = (info);

    int To_MessageBoxA = MessageBoxA(NULL, copy_info.c_str(), copy_title.c_str(), MB_ICONEXCLAMATION);
}

bool _setStartup()
{
    string path = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run";
    string key = "IkunWxExportDat";
    string execPath = "\"";
    execPath += getProcessidFilePath(_getpid());
    execPath.append("\" -startup");

    if (hmc_registr::hasRegistrKey(HKEY_LOCAL_MACHINE, path, key))
    {
        ikun_app_startup = hmc_registr::removeRegistrValue(HKEY_LOCAL_MACHINE, path, key) ? false : true;
        hmc_tray::setMenuItmeSelect("btn::app_startup",ikun_app_startup);
        return ikun_app_startup;
    }
    else
    {
        ikun_app_startup = hmc_registr::setRegistrValue(HKEY_LOCAL_MACHINE, path, key, execPath) ? true : false;
        hmc_tray::setMenuItmeSelect("btn::app_startup", ikun_app_startup);

        return ikun_app_startup;
    }
    ikun_app_startup = false;
    hmc_tray::setMenuItmeSelect("btn::app_startup", ikun_app_startup);

    return ikun_app_startup;
}

bool _hasStartup()
{
    string path = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run";
    string key = "IkunWxExportDat";
    ikun_app_startup = hmc_registr::hasRegistrKey(HKEY_LOCAL_MACHINE, path, key);
    hmc_tray::setMenuItmeSelect("btn::app_startup", ikun_app_startup);

    return ikun_app_startup;
}

bool _hasStartupGlobalVar()
{
    return ikun_app_startup;
}

void _openSelectFolder()
{
    // setWindowTop(winmian, false);

    CoInitialize(NULL);

    BROWSEINFOA browseInfo = {0};
    char folderPath[MAX_PATH];

    browseInfo.hwndOwner = NULL;
    browseInfo.pidlRoot = NULL;
    browseInfo.pszDisplayName = folderPath;
    browseInfo.lpszTitle = "选择文件夹";
    browseInfo.ulFlags = BIF_RETURNONLYFSDIRS | BIF_NEWDIALOGSTYLE;

    LPITEMIDLIST pidl = SHBrowseForFolderA(&browseInfo);
    _putenv_s("IKUN@SelectedFolderPath", "\0");

    if (pidl != NULL)
    {
        SHGetPathFromIDListA(pidl, folderPath);
        std::cout << "Selected folder path: " << folderPath << std::endl;
        _putenv_s("IKUN@SelectedFolderPath", folderPath);
        CoTaskMemFree(pidl);
    }
    else
    {
        std::cout << "Folder selection canceled." << std::endl;
    }

    // 释放 COM 库
    CoUninitialize();
    // setWindowTop(winmian, true);

    // return string(folderPath);
}

/**
 * @brief 选择文件夹（单选）
 *
 * @param folderPath
 * @return true
 * @return false
 */
bool SelectFolder(wstring &folderPath)
{
    bool result = false;
    try
    {
        HRESULT hr;
        IFileOpenDialog *pOpenFolderDialog;
        HWND owner = NULL;

        hr = ::CoCreateInstance(CLSID_FileOpenDialog,
                                NULL,
                                CLSCTX_INPROC_SERVER,
                                IID_PPV_ARGS(&pOpenFolderDialog));

        if (SUCCEEDED(hr))
        {
            // 获取用户与对话框交互的结果
            pOpenFolderDialog->SetOptions(FOS_PICKFOLDERS);

            // 显示选择文件夹窗口
            hr = pOpenFolderDialog->Show(owner);

            if (SUCCEEDED(hr))
            {

                IShellItem *psiResult;
                hr = pOpenFolderDialog->GetResult(&psiResult);

                LPWSTR folderW = NULL;
                psiResult->GetDisplayName(SIGDN_FILESYSPATH, &folderW);
                if (folderW != NULL) {
                    folderPath.clear();
                    folderPath.append(folderW);
                    wcout << L"folderPath -> " << folderPath << endl;
                }
                
                result = true;
                ::CoTaskMemFree(folderW);
                psiResult->Release();
            }
        }
        pOpenFolderDialog->Release();
    }
    HMC_CHECK_CATCH;
    return result;
}

const char* _openSelectFolder2()
{
    string result = string();
    try
    {
        wstring temp_buf = wstring();
        if (SelectFolder(temp_buf)) {
            result.append(hmc_text_util::W2U8(temp_buf));

        }
    }
    HMC_CHECK_CATCH;
    if (result.empty()) {
        return "\0";
    }
    else {
        char* pUTF8 = new char[result.size() + 1];

        for (size_t i = 0; i < result.size(); i++)
        {
            char data = result[i];
            pUTF8[i] = data;
        }
        const int end = result.size() ;

        pUTF8[end] = *"\0";

        return pUTF8;
    }
    
}

const char* _getRegistrValue(long hKey, const char* _subKey, const char* _key)
{
    string subKey = (_subKey);
    string key = (_key);

    string result = hmc_registr::getRegistrValue<string>((HKEY)hKey, subKey, key);
  
    char* pUTF8 = new char[result.size() + 1];

    for (size_t i = 0; i < result.size(); i++)
    {
        char data = result[i];
        pUTF8[i] = data;
    }
    const int end = result.size();

    pUTF8[end] = *"\0";

    return pUTF8;

}

struct ProcessEnumDetailsCont
{
    DWORD pid;
    string baseName;
    string path;
};


void getProcessList(vector<ProcessEnumDetailsCont>& resultsData)
{
    DWORD processList[1024], cbNeeded;
    if (!EnumProcesses(processList, sizeof(processList), &cbNeeded))
    {
    }
    int numProcesses = cbNeeded / sizeof(DWORD);
    for (int i = 0; i < numProcesses; ++i)
    {
        DWORD processID = processList[i];
        HANDLE hProcess = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, processID);
        if (hProcess)
        {
            char processName[MAX_PATH];
            char Filename[1024];
            GetModuleBaseNameA(hProcess, NULL, processName, MAX_PATH);
            GetModuleFileNameExA(hProcess, NULL, Filename, 1024);
            ProcessEnumDetailsCont processEnumCont;
            processEnumCont.pid = processID;
            processEnumCont.baseName = processName;
            processEnumCont.path = Filename;
            resultsData.push_back(processEnumCont);
            CloseHandle(hProcess);
        }
    }
}


bool _hasWeChat() {
    return hmc_process::hasBaseNameProcess(string("WeChat.exe"));
}

vector<DWORD> getWeChatPidList() {
    auto app_base_name = string("WeChat.exe");
    return hmc_process:: getBaseNameProcessIDList(app_base_name);
}

const char* _enum_file_open_path () {
    string result = "";
    vector<DWORD> pid_list = getWeChatPidList();
    
    for (size_t i = 0; i < pid_list.size(); i++)
    {
        auto pid = pid_list[i];
        HANDLE hProcess = OpenProcess(PROCESS_DUP_HANDLE | PROCESS_QUERY_INFORMATION, FALSE, pid);
        if (hProcess == NULL)
        {
           
        }

    }
    return result.c_str();
}

string get_utf8_str(const char* input, int inputLen = 0) {
    string ouput = string();
    if (inputLen > 0) {
        ouput.resize(inputLen);
        for (size_t i = 0; i < inputLen; i++)
        {
            char data = input[i];
            if (data == (char)"\0") {
                break;
            }
            ouput[i] = input[i];
        }
        //ouput.append("\0");
    }
    else {
        ouput.append(input);
    }

    ouput = hmc_text_util::U82A(ouput);

    return ouput;
}

long _findWindowU8(const char* className, const char* title) {

    string copy_className = hmc_text_util::U82A(className);
    string copy_title = hmc_text_util::U82A(title);
    return (long)hmc_window::findWindow(copy_className, copy_title);

}

long _findWindowW(const wchar_t* className, const wchar_t* title) {

    return (long)hmc_window::findWindowW(className, title);

}

long _findWindow(const char* className, const char* title) {
 
    return (long)hmc_window::findWindow(className, title);

}

bool _has_auto_sync() {
    return ikun_user_auto_disable_sync;
}

void _set_auto_sync(bool value) {
   ikun_user_auto_disable_sync = value;
   try
   {
       if (value) {
           hmc_registr::setRegistrValue(HKEY_CURRENT_USER, "SOFTWARE\\WxAutoExIm", "auto_sync", string("true"));
       }
       else {
           hmc_registr::removeRegistrValue(HKEY_CURRENT_USER, "SOFTWARE\\WxAutoExIm", "auto_sync");
       }

       hmc_tray::setMenuItmeSelect("btn::auto_sync", ikun_user_auto_disable_sync);

   }HMC_CHECK_CATCH;
}

bool _has_sync_token() {
    if (sync_token) {
        sync_token = false;
        //cout << "_has_sync_token" << endl;
        return true;
    }
    return false;
}

long _getFocusWindow() {
   return (long) hmc_window::getFocusWindow();
}

long _getFocusTopWindow() {
    return (long)hmc_window::getParentWindow(hmc_window::getFocusWindow())|| hmc_window::getFocusWindow();
}

template <typename T>
const char* hwnd_list2_long_list(vector<T> &hwnd_list) {

    string _hwnd_list = string();
    for (size_t i = 0; i < hwnd_list.size(); i++)
    {
        T hwnd = hwnd_list[i];
        _hwnd_list.append(to_string((long long)hwnd));
        _hwnd_list.append(",");

    }

    if (!_hwnd_list.empty()) {
        _hwnd_list.pop_back();
    }

    //cout << "_hwnd_list->" << _hwnd_list << endl;

    char* pUTF8 = new char[_hwnd_list.size() + 1];

    for (size_t i = 0; i < _hwnd_list.size(); i++)
    {
        char data = _hwnd_list[i];
        pUTF8[i] = data;
    }
    const int end = _hwnd_list.size();

    pUTF8[end] = *"\0";

    //cout << "pUTF8->" << pUTF8 << endl;

    return pUTF8;
}


std::string removeNullCharacters(std::string str) {

    string data = string();
    data.append(str);

    // 移除开头的空字符
    while (!data.empty() && data.front() == '\0') {
        data.erase(0, 1);
    }

    // 移除末尾的空字符
    while (!data.empty() && data.back() == '\0') {
        data.pop_back();
    }

    return data;
}

const char* _findAllWindow(const char* className, const char* title) {
    vector<HWND> hwnd_list ;

    string _hwnd_list = string();
    string _className = hmc_window::removeNullCharacters(string(className));
    string _titleName = hmc_window::removeNullCharacters(string(title));

    HWND winEnumerable = GetTopWindow(0);
   
    while (winEnumerable)
    {
        if (::IsWindow(winEnumerable)) {

        string the_class = string();
        string the_titleName = string();

        if (!_className.empty()) {
            the_class = hmc_window::getClassName(winEnumerable);
            
            if (the_class == _className) {

                if (_titleName.empty()) {

                    hwnd_list.push_back(winEnumerable);

                }
            }
        }


        if (!_titleName.empty()) {
            
            the_titleName = hmc_window::getWindowText(winEnumerable);
            if (the_titleName == _titleName) {

                if (_className.empty()) {
                    hwnd_list.push_back(winEnumerable);
                }

            }
        }

        if (!_className.empty()&& !the_titleName.empty()) {
            if (the_titleName == _titleName&& the_class == _className) {
                hwnd_list.push_back(winEnumerable);
            }
        }

        }

        winEnumerable = GetNextWindow(winEnumerable, GW_HWNDNEXT);
    }

    for (size_t i = 0; i < hwnd_list.size(); i++)
    {
        HWND hwnd = hwnd_list[i];
        _hwnd_list.append(to_string((int)hwnd));
        _hwnd_list.append(",");

    }

    if (!_hwnd_list.empty()) {
        _hwnd_list.pop_back();
    }

    //cout << "_hwnd_list->" << _hwnd_list << endl;

    char* pUTF8 = new char[_hwnd_list.size() + 1];

    for (size_t i = 0; i < _hwnd_list.size(); i++)
    {
        char data = _hwnd_list[i];
        pUTF8[i] = data;
    }
    const int end = _hwnd_list.size();

    pUTF8[end] = *"\0";

    //cout << "pUTF8->" << pUTF8 << endl;

    return pUTF8;

}

const char* _getWindowRect(long hwnds){
    RECT rect;
    ::GetWindowRect(HWND(hwnds), &rect);

    int width = rect.right - rect.left; // 计算窗口宽度
    int height = rect.bottom - rect.top; // 计算窗口高度


    string res_json = string();
    res_json.append("{");
    res_json.append("\"left\":");
    res_json.append(to_string(rect.left));
    res_json.append(",\"top\":");
    res_json.append(to_string(rect.top));
    res_json.append(",\"bottom\":");
    res_json.append(to_string(rect.bottom));
    res_json.append(",\"right\":");
    res_json.append(to_string(rect.right));
    res_json.append(",\"width\":");
    res_json.append(to_string(width));
    res_json.append(",\"height\":");
    res_json.append(to_string(height));
    res_json.append("}");


    char* pUTF8 = new char[res_json.size() + 1];

    for (size_t i = 0; i < res_json.size(); i++)
    {
        char data = res_json[i];
        pUTF8[i] = data;
    }
    const int end = res_json.size();

    pUTF8[end] = *"\0";


    return pUTF8;

}

long long _randomNum() {
    std::random_device rd;
    std::mt19937 gen(rd());
    std::uniform_int_distribution<long long> dis(-8446744073709551617i64, 8446744073709551617i64);

    int randomNum = dis(gen);

    return randomNum;
}

void _setWindowTransparent(long hwnds,int transparent) {
    HWND hwnd = (HWND)hwnds;
    hmc_window::setHandleTransparent(hwnd, transparent);
}

const char* _getfilePathSingle () {
    string filePath = "";

    if (OpenClipboard(NULL))
    {
        HDROP hDrop = HDROP(::GetClipboardData(CF_HDROP));
        if (hDrop != NULL)
        {
            char szFilePathName[MAX_PATH + 1] = { 0 };
            UINT UintAllFiles = DragQueryFileA(hDrop, 0xFFFFFFFF, NULL, 0);

            for (UINT index = 0; index < UintAllFiles; index++)
            {
                memset(szFilePathName, 0, MAX_PATH + 1);
                // get path
                DragQueryFileA(hDrop, index, szFilePathName, MAX_PATH);

                filePath.append(szFilePathName);
            }
        }
        CloseClipboard();
    }

    char* pUTF8 = new char[filePath.size() + 1];

    for (size_t i = 0; i < filePath.size(); i++)
    {
        char data = filePath[i];
        pUTF8[i] = data;
    }
    const int end = filePath.size();

    pUTF8[end] = *"\0";

    return pUTF8;
}

bool _setWindowEnabled(long hwnds, bool enabled) {
    HWND hwnd = (HWND)hwnds;

    if (enabled) {
        ::SetWindowLong(hwnd, GWL_STYLE, GetWindowLong(hwnd, GWL_STYLE) | (WS_DISABLED));
        
    }
    else {
        ::SetWindowLong(hwnd, GWL_STYLE, GetWindowLong(hwnd, GWL_STYLE) & ~(WS_DISABLED));

    }

    LONG windowLong = ::GetWindowLong(hwnd, GWL_STYLE);

    return !(windowLong & WS_DISABLED);
}
#include <string>
#include <windows.h>
#include <Windows.h>
#include <vector>
#include <iostream>
#include <fstream>
#include <chrono>
#include <time.h>
#include <objbase.h>
#include <ShlObj.h>
#include <Psapi.h>
#include <Shellapi.h>
#include <random>

#include <map>
#include <thread>

#pragma comment(lib, "dwmapi.lib")
#pragma comment(lib, "psapi.lib")
using namespace std;


#ifdef IMPORT_DLLa
#else
#define IMPORT_DLL extern "C" _declspec(dllimport) 
#endif




// dllmain.cpp
IMPORT_DLL void  _setWinIcon(long hwnds);
IMPORT_DLL bool  _setShowWindows(long hwnds, bool visible);
IMPORT_DLL void  _set_tray();
IMPORT_DLL bool  _createMutex(const char* MutexName);
IMPORT_DLL bool  _removeMutex(const char* MutexName);
IMPORT_DLL bool  _hasMutex(const char* MutexName);
IMPORT_DLL bool  _setStartup();
IMPORT_DLL void  _openSelectFolder();
IMPORT_DLL bool  _setWindowsTop(long hwnds, bool visible);
IMPORT_DLL bool _setCloseWindow(long hwnds, bool closeRoot);
IMPORT_DLL const char* _openSelectFolder2();
IMPORT_DLL void _Error(const char* title, const char* info);
IMPORT_DLL void _Stop(const char* title, const char* info);
IMPORT_DLL bool _Confirm(const char* title, const char* info);
IMPORT_DLL bool _Alert(const char* title, const char* info);
IMPORT_DLL const char* _getRegistrValue(long hKey, const char* _subKey, const char* _key);
IMPORT_DLL bool _hasWeChat();
IMPORT_DLL void _setTaskbarWin(long hwnds);
IMPORT_DLL long _findWindow(const char* className, const char* title);
IMPORT_DLL long _findWindowW(const char* className, const char* title);
IMPORT_DLL long _findWindowU8(const char* className, const char* title);
IMPORT_DLL bool _hasStartup();
IMPORT_DLL bool _has_auto_sync();
IMPORT_DLL void _set_auto_sync(bool value);
IMPORT_DLL bool _has_sync_token();
IMPORT_DLL bool _hasStartupGlobalVar();
IMPORT_DLL long _getFocusWindow();
IMPORT_DLL long _getFocusTopWindow();
IMPORT_DLL bool _setMinWindows(long hwnds);
IMPORT_DLL const char* _findAllWindow(const char* className, const char* title);
IMPORT_DLL void _setWinIconMain(long hwnds);
IMPORT_DLL bool _isWindow(long hwnds);
IMPORT_DLL void _setWindowShake(long hwnds);
IMPORT_DLL const char* _getWindowRect(long hwnds);
IMPORT_DLL long long _randomNum();
IMPORT_DLL void _setWindowTransparent(long hwnds, int transparent);
IMPORT_DLL const char* _getfilePathSingle();
IMPORT_DLL void _setWindowShake(long hwnds);
IMPORT_DLL bool _setWindowEnabled(long hwnds, bool enabled);
IMPORT_DLL bool _hasInitWindowIsDisplayed();
IMPORT_DLL bool _setInitWindowIsDisplayed(bool initWindowIsDisplayed);

//IMPORT_DLL
//IMPORT_DLL
//IMPORT_DLL// pch.cpp: 与预编译标头对应的源文件

#include "pch.h"

// 当使用预编译的头时，需要使用此源文件，编译才能成功。
#ifndef HMC_IMPORT_PROCESS_H
#define HMC_IMPORT_PROCESS_H

#define _CRT_SECURE_NO_WARNINGS
#include <string>
#include <windows.h>
#include <Psapi.h>
#include <Shellapi.h>
#include <vector>
#include <map>
using namespace std;

#include "include/attribute.hpp"
#include "./include/global.hpp"

#include "process.h"

namespace hmc_process
{

#define NT_SUCCESS(x) ((x) >= 0)
#define STATUS_INFO_LENGTH_MISMATCH 0xc0000004

#define SystemHandleInformation 16
#define ObjectBasicInformation 0
#define ObjectNameInformation 1
#define ObjectTypeInformation 2

    typedef NTSTATUS(NTAPI *NTQUERYSYSTEMINFORMATION)(
        // 检索的信息类型
        ULONG SystemInformationClass,
        // 指向缓冲区的指针 有关系统信息的结构体
        PVOID SystemInformation,
        // 缓冲区的大小
        ULONG SystemInformationLength,
        // 实际返回的信息大小
        PULONG ReturnLength);

    typedef NTSTATUS(NTAPI *NTDUPLICATEOBJECT)(
        // 源进程的句柄
        HANDLE SourceProcessHandle,
        // 复制的内核对象的句柄
        HANDLE SourceHandle,
        // 目标进程的句柄
        HANDLE TargetProcessHandle,
        // 目标进程中新对象的指针
        PHANDLE TargetHandle,
        // 新对象的访问权限
        ACCESS_MASK DesiredAccess,
        // 新对象的属性
        ULONG Attributes,
        // 复制操作的选项
        ULONG Options);

    typedef NTSTATUS(NTAPI *NTQUERYOBJECT)(
        HANDLE ObjectHandle,
        ULONG ObjectInformationClass,
        PVOID ObjectInformation,
        ULONG ObjectInformationLength,
        PULONG ReturnLength);

    typedef struct _SYSTEM_HANDLE
    {
        ULONG ProcessId;
        BYTE ObjectTypeNumber;
        BYTE Flags;
        USHORT Handle;
        PVOID Object;
        ACCESS_MASK GrantedAccess;
    } SYSTEM_HANDLE, *PSYSTEM_HANDLE;

    typedef struct _SYSTEM_HANDLE_INFORMATION
    {
        ULONG HandleCount;
        SYSTEM_HANDLE Handles[1];
    } SYSTEM_HANDLE_INFORMATION, *PSYSTEM_HANDLE_INFORMATION;

    typedef enum _POOL_TYPE
    {
        NonPagedPool,
        PagedPool,
        NonPagedPoolMustSucceed,
        DontUseThisType,
        NonPagedPoolCacheAligned,
        PagedPoolCacheAligned,
        NonPagedPoolCacheAlignedMustS
    } POOL_TYPE,
        *PPOOL_TYPE;

    typedef struct _UNICODE_STRING
    {
        USHORT Length;
        USHORT MaximumLength;
#ifdef MIDL_PASS
        [ size_is(MaximumLength / 2), length_is((Length) / 2) ] USHORT *Buffer;
#else  // MIDL_PASS
        _Field_size_bytes_part_opt_(MaximumLength, Length) PWCH Buffer;
#endif // MIDL_PASS
    } UNICODE_STRING;

    typedef UNICODE_STRING *PUNICODE_STRING;
    typedef const UNICODE_STRING *PCUNICODE_STRING;

    typedef struct _OBJECT_TYPE_INFORMATION
    {
        // 对象名称。
        UNICODE_STRING Name;
        // 对象的总数。
        ULONG TotalNumberOfObjects;
        // 对象句柄的总数。
        ULONG TotalNumberOfHandles;
        // 对象使用的分页池内存总量。
        ULONG TotalPagedPoolUsage;
        // 对象使用的非分页池内存总量。
        ULONG TotalNonPagedPoolUsage;
        // 对象名称使用的内存总量。
        ULONG TotalNamePoolUsage;
        // 对象句柄表使用的内存总量。
        ULONG TotalHandleTableUsage;
        // 对象的最大数量。
        ULONG HighWaterNumberOfObjects;
        // 对象句柄的最大数量。
        ULONG HighWaterNumberOfHandles;
        // 对象使用的分页池内存的最大值。
        ULONG HighWaterPagedPoolUsage;
        // 对象使用的非分页池内存的最大值。
        ULONG HighWaterNonPagedPoolUsage;
        // 对象名称使用的内存的最大值。
        ULONG HighWaterNamePoolUsage;
        // 对象句柄表使用的内存的最大值。
        ULONG HighWaterHandleTableUsage;
        // 无效属性标志。
        ULONG InvalidAttributes;
        // 通用映射结构体。
        GENERIC_MAPPING GenericMapping;
        // 有效访问标志。
        ULONG ValidAccess;
        // 安全性要求标志。
        BOOLEAN SecurityRequired;
        // 维护句柄计数标志。
        BOOLEAN MaintainHandleCount;
        // 维护类型列表标志。
        USHORT MaintainTypeList;
        // 池类型。
        POOL_TYPE PoolType;
        // 分页池内存使用量。
        ULONG PagedPoolUsage;
        // 非分页池内存使用量。
        ULONG NonPagedPoolUsage;
    } OBJECT_TYPE_INFORMATION, *POBJECT_TYPE_INFORMATION;

    /**
     * @brief 获取当前进程的父进程id
     *
     * @param matchProcessID
     * @return DWORD
     */
    DWORD getParentProcessID(DWORD matchProcessID)
    {
        DWORD CurrentProcessId = 0;
        PROCESSENTRY32 pe32;
        pe32.dwSize = sizeof(PROCESSENTRY32);

        // 获取进程快照
        HANDLE hSnap = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if (hSnap == INVALID_HANDLE_VALUE)
        {
            return NULL;
        }

        // 枚举第一个进程
        if (Process32First(hSnap, &pe32))
        {
            do
            {
                // 打开进程句柄
                HANDLE hProcess = OpenProcess(PROCESS_ALL_ACCESS, FALSE, pe32.th32ProcessID);
                if (hProcess)
                {
                    if (pe32.th32ProcessID == matchProcessID)
                    {
                        CurrentProcessId = pe32.th32ParentProcessID;
                        CloseHandle(hProcess);
                        break;
                    }
                    CloseHandle(hProcess);
                }
            } while (Process32Next(hSnap, &pe32));
        }

        CloseHandle(hSnap);
        return CurrentProcessId;
    }

    /**
     * @brief 获取当前进程的所有子进程
     *
     * @param ProcessId
     * @param SubProcessIDList
     */
    void getSubProcessList(DWORD dwProcessID, vector<DWORD> &SubProcessIDList)
    {

        hmc_EnableShutDownPriv();
        PROCESSENTRY32 pe32;
        pe32.dwSize = sizeof(PROCESSENTRY32);

        // 获取进程快照
        HANDLE hSnap = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if (hSnap == INVALID_HANDLE_VALUE)
        {
            return;
        }

        // 枚举第一个进程
        if (Process32First(hSnap, &pe32))
        {
            do
            {
                // 打开进程句柄
                HANDLE hProcess = OpenProcess(PROCESS_ALL_ACCESS, FALSE, pe32.th32ProcessID);
                if (hProcess)
                {
                    // 子进程的进程 ID
                    bool is_sub = pe32.th32ParentProcessID == dwProcessID;

                    // 二次子进程
                    if (!is_sub)
                    {
                        if (find(SubProcessIDList.begin(), SubProcessIDList.end(), pe32.th32ParentProcessID) != SubProcessIDList.end())
                        {
                            is_sub = true;
                        }
                    }

                    if (is_sub)
                    {
                        if (!(find(SubProcessIDList.begin(), SubProcessIDList.end(), pe32.th32ProcessID) != SubProcessIDList.end()))
                        {
                            SubProcessIDList.push_back(pe32.th32ProcessID);
                        }
                    }
                    CloseHandle(hProcess);
                }
            } while (Process32Next(hSnap, &pe32));
        }

        CloseHandle(hSnap);
    };

    /**
     * @brief 获取进程可执行文件路径
     *
     * @param ProcessID
     * @return string
     */
    string getFilePath(DWORD dwProcessID)
    {
        LPSTR lpFilename = { 0 };
        HANDLE hProcess = OpenProcess(PROCESS_ALL_ACCESS, FALSE, dwProcessID);

        if (hProcess == nullptr)
        {
            CloseHandle(hProcess);
            return string("");
        }
        ::GetModuleFileNameExA(hProcess, NULL, lpFilename, MAX_PATH);
        return string(lpFilename);
    }

    /**
     * @brief 结束指定进程
     *
     * @param ProcessID
     * @return BOOL
     */
    BOOL killProcessID(DWORD dwProcessID)
    {
        bool is_kill_success = false;
        hmc_EnableShutDownPriv();
        HANDLE killHandle = OpenProcess(PROCESS_TERMINATE | PROCESS_QUERY_INFORMATION | PROCESS_CREATE_THREAD | PROCESS_VM_OPERATION | PROCESS_VM_WRITE, FALSE, dwProcessID);
        if (killHandle != NULL)
        {
            is_kill_success = TerminateProcess(killHandle, 0);
        }
        return is_kill_success;
    }

    /**
     * @brief 判断进程是否存在
     *
     * @param ProcessID
     * @return BOOL
     */
    BOOL existsProcessID(DWORD dwProcessID)
    {
        hmc_EnableShutDownPriv();
        bool exists_process = false;
        HANDLE hProcess = OpenProcess(PROCESS_ALL_ACCESS, FALSE, dwProcessID);
        if (GetLastError())
            return false;

        exists_process = hProcess != NULL;
        CloseHandle(hProcess);
        return exists_process;
    }

    /**
     * @brief 获取进程的HWND
     *
     * @param dwProcessID
     * @return HWND
     */
    HWND getHwnd(DWORD dwProcessID)
    {
        HWND win_next_it = GetTopWindow(0);
        HWND result = NULL;
        while (win_next_it)
        {
            DWORD pid = 0;
            DWORD theardId = GetWindowThreadProcessId(win_next_it, &pid);
            if (theardId != 0)
            {
                if (pid == dwProcessID && GetParent(win_next_it) == NULL && ::IsWindowVisible(win_next_it))
                {

                    result = win_next_it;
                }
            }
            win_next_it = GetNextWindow(win_next_it, GW_HWNDNEXT);
        }
        return result;
    }

    /**
     * @brief 获取可执行文件名称
     *
     * @param dwProcessID
     * @return string
     */
    string getBaseName(DWORD dwProcessID)
    {
        string FilePath;
        HANDLE hProcess = OpenProcess(PROCESS_ALL_ACCESS, FALSE, dwProcessID);
        char lpFilename[MAX_PATH];
        if (hProcess == nullptr)
        {

            return FilePath;
        }
        GetModuleBaseNameA(hProcess, NULL, (LPSTR)lpFilename, MAX_PATH);
        CloseHandle(hProcess);
        FilePath.append(lpFilename);
        return FilePath;
    }

    /**
     * @brief 枚举进程的线程信息
     *
     * @param dwProcessID
     * @param resultsModulePathList
     */
    void getThreadList(DWORD dwProcessID, vector<DWORD> &resultsModulePathList)
    {
        HANDLE hThreadSnap = INVALID_HANDLE_VALUE;
        // `THREADENTRY32` 是一个结构体，它定义在 `tlhelp32.h` 头文件中。它描述了在系统执行快照时正在执行的线程列表中的条目。以下是 `THREADENTRY32` 结构体中的各个变量的含义：⁴⁵
        // - dwSize：结构体的大小，以字节为单位。
        // - cntUsage：线程使用计数。
        // - th32ThreadID：线程标识符，与 `CreateProcess` 函数返回的线程标识符兼容。
        // - th32OwnerProcessID：创建线程的进程标识符。
        // - tpBasePri：分配给线程的内核基优先级。
        // - tpDeltaPri：线程优先级相对于基本优先级的增量。
        // - dwFlags：保留，不再使用。
        THREADENTRY32 te32;

        // 对所有正在运行的线程进行快照
        hThreadSnap = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0);
        if (hThreadSnap == INVALID_HANDLE_VALUE)
            return;

        te32.dwSize = sizeof(THREADENTRY32);

        // 检索第一个线程的信息
        if (!Thread32First(hThreadSnap, &te32))
        {
            CloseHandle(hThreadSnap);
            return;
        }

        do
        {
            if (te32.th32OwnerProcessID == dwProcessID || dwProcessID == 0)
            {
                resultsModulePathList.push_back(te32.th32ThreadID);
            }
        } while (Thread32Next(hThreadSnap, &te32));

        CloseHandle(hThreadSnap);
    }

    /**
     * @brief 枚举进程的线程信息
     *
     * @param dwProcessID
     * @param resultsModulePathList
     */
    void getThreadList(DWORD dwProcessID, vector<THREADENTRY32> &resultsModulePathList)
    {
        HANDLE hThreadSnap = INVALID_HANDLE_VALUE;
        // `THREADENTRY32` 是一个结构体，它定义在 `tlhelp32.h` 头文件中。它描述了在系统执行快照时正在执行的线程列表中的条目。以下是 `THREADENTRY32` 结构体中的各个变量的含义：⁴⁵
        // - dwSize：结构体的大小，以字节为单位。
        // - cntUsage：线程使用计数。
        // - th32ThreadID：线程标识符，与 `CreateProcess` 函数返回的线程标识符兼容。
        // - th32OwnerProcessID：创建线程的进程标识符。
        // - tpBasePri：分配给线程的内核基优先级。
        // - tpDeltaPri：线程优先级相对于基本优先级的增量。
        // - dwFlags：保留，不再使用。
        THREADENTRY32 te32;

        // 对所有正在运行的线程进行快照
        hThreadSnap = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0);
        if (hThreadSnap == INVALID_HANDLE_VALUE)
            return;

        te32.dwSize = sizeof(THREADENTRY32);

        // 检索第一个线程的信息
        if (!Thread32First(hThreadSnap, &te32))
        {
            CloseHandle(hThreadSnap);
            return;
        }

        do
        {
            if (te32.th32OwnerProcessID == dwProcessID || dwProcessID == 0)
            {
                THREADENTRY32 copy_te32;
                copy_te32.cntUsage = te32.cntUsage;
                copy_te32.cntUsage = te32.dwFlags;
                copy_te32.cntUsage = te32.dwSize;
                copy_te32.cntUsage = te32.th32OwnerProcessID;
                copy_te32.cntUsage = te32.th32ThreadID;
                copy_te32.cntUsage = te32.tpBasePri;
                copy_te32.cntUsage = te32.tpDeltaPri;

                resultsModulePathList.push_back(copy_te32);
            }
        } while (Thread32Next(hThreadSnap, &te32));

        CloseHandle(hThreadSnap);
    }

    /**
     * @brief 获取进程引用的模块列表
     *
     * @param dwProcessID
     * @param resultsData
     */
    void getModulePathList(DWORD dwProcessID, vector<string> &resultsData)
    {

        HANDLE hProcess = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, dwProcessID);
        if (hProcess == NULL)
            return;

        vector<HANDLE> vecFileHandles;

        // 枚举进程打开的文件句柄
        HANDLE hSnapshot = CreateToolhelp32Snapshot(TH32CS_SNAPALL, 0);
        if (hSnapshot != INVALID_HANDLE_VALUE)
        {
            PROCESSENTRY32 pe32;
            pe32.dwSize = sizeof(PROCESSENTRY32);
            if (Process32First(hSnapshot, &pe32))
            {
                do
                {
                    if (pe32.th32ProcessID == dwProcessID)
                    {
                        HANDLE hModuleSnap = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, dwProcessID);
                        if (hModuleSnap != INVALID_HANDLE_VALUE)
                        {
                            MODULEENTRY32 me32;
                            me32.dwSize = sizeof(MODULEENTRY32);
                            if (Module32First(hModuleSnap, &me32))
                            {
                                do
                                {
                                    HANDLE hFile = CreateFile(me32.szExePath, GENERIC_READ, FILE_SHARE_READ | FILE_SHARE_WRITE, NULL, OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, NULL);
                                    if (hFile != INVALID_HANDLE_VALUE)
                                    {
                                        vecFileHandles.push_back(hFile);
                                    }
                                } while (Module32Next(hModuleSnap, &me32));
                            }
                            CloseHandle(hModuleSnap);
                        }
                    }
                } while (Process32Next(hSnapshot, &pe32));
            }
            CloseHandle(hSnapshot);
        }

        // 输出文件路径
        for (auto it = vecFileHandles.begin(); it != vecFileHandles.end(); ++it)
        {
            LPSTR szFilePath = new CHAR[MAX_PATH];
            DWORD dwSize = GetFinalPathNameByHandleA(*it, szFilePath, MAX_PATH, FILE_NAME_NORMALIZED);
            if (dwSize > 0 && dwSize < MAX_PATH)
            {
                string strFilePath = szFilePath;
                string findStr = "\\\\?\\";
                if (strFilePath.find(findStr) == 0)
                {
                    strFilePath.replace(0, findStr.length(), "");
                }
                resultsData.push_back(strFilePath);
            }
            delete[] szFilePath;
            CloseHandle(*it);
        }

        CloseHandle(hProcess);
        return;
    }

    /**
     * @brief 获取鼠标所在的句柄的进程id
     *
     * @return DWORD
     */
    DWORD getPointWindowProcessId()
    {
        DWORD processId = 0;
        POINT curPoint;
        if (!GetCursorPos(&curPoint))
            return processId;
        HWND mainWindow = WindowFromPoint(curPoint);
        GetWindowThreadProcessId(mainWindow, &processId);
        return processId;
    }

    /**
     * @brief 获取鼠标所在的窗口的进程文件名
     *
     * @return string
     */
    string getPointWindowProcessBaseName()
    {
        return getBaseName(getPointWindowProcessId());
    }

    /**
     * @brief 获取当前聚焦的窗口的进程id
     *
     * @return DWORD
     */
    DWORD getFocusWindowProcessID()
    {
        DWORD processId;
        GetWindowThreadProcessId(GetForegroundWindow(), &processId);
        return processId;
    }

    /**
     * @brief 获取聚焦的窗口的进程文件名
     *
     * @return string
     */
    string getFocusWindowProcessBaseName()
    {
        return getBaseName(getFocusWindowProcessID());
    }

    struct ProcessEnumDetailsCont
    {
        DWORD pid;
        string baseName;
        string path;
    };

    struct ProcessEnumCont
    {
        DWORD pid;
        string baseName;
    };
    /**
     * @brief 枚举进程列表
     *
     * @param resultsData
     */

    void getProcessList(vector<ProcessEnumCont> &resultsData)
    {
        hmc_EnableShutDownPriv();
        DWORD processList[1024], cbNeeded;
        if (!EnumProcesses(processList, sizeof(processList), &cbNeeded))
        {
        }
        int numProcesses = cbNeeded / sizeof(DWORD);
        for (int i = 0; i < numProcesses; ++i)
        {
            DWORD processID = processList[i];
            HANDLE hProcess = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, processID);
            if (hProcess)
            {
                char processName[MAX_PATH];
                GetModuleBaseNameA(hProcess, NULL, processName, MAX_PATH);
                ProcessEnumCont processEnumCont;
                processEnumCont.pid = processID;
                processEnumCont.baseName = processName;
                resultsData.push_back(processEnumCont);
                CloseHandle(hProcess);
            }
        }
    }

    void getProcessList(vector<ProcessEnumDetailsCont> &resultsData)
    {
        hmc_EnableShutDownPriv();
        DWORD processList[1024], cbNeeded;
        if (!EnumProcesses(processList, sizeof(processList), &cbNeeded))
        {
        }
        int numProcesses = cbNeeded / sizeof(DWORD);
        for (int i = 0; i < numProcesses; ++i)
        {
            DWORD processID = processList[i];
            HANDLE hProcess = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, processID);
            if (hProcess)
            {
                char processName[MAX_PATH];
                char Filename[1024];
                GetModuleBaseNameA(hProcess, NULL, processName, MAX_PATH);
                GetModuleFileNameExA(hProcess, NULL, Filename, 1024);
                ProcessEnumDetailsCont processEnumCont;
                processEnumCont.pid = processID;
                processEnumCont.baseName = processName;
                processEnumCont.path = Filename;
                resultsData.push_back(processEnumCont);
                CloseHandle(hProcess);
            }
        }
    }

    wstring __unicodeStringToWString(UNICODE_STRING unicodeString)
    {
        wstring result;
        if (unicodeString.Buffer)
        {
            result = wstring(unicodeString.Buffer, unicodeString.Length / sizeof(wchar_t));
        }
        return result;
    }
    /**
     * @brief 获取窗口句柄对应的pid
     *
     * @param hwnd
     * @return DWORD
     */
    DWORD getHwndProcessID(HWND hwnd)
    {
        DWORD processId;
        GetWindowThreadProcessId(hwnd, &processId);
        return processId;
    }
    struct hmc_ProcessHandleContext
    {
        long ContextID;
        bool next;
        DWORD handle;
        string name; // string
        string type; // "ALPC Port" | "Event" | "Timer" | "Mutant" | "Key" | "Section" | "File" | "Thread" | string;
    };

    vector<hmc_ProcessHandleContext> _enumProcessHandleList;
    /**
     * @brief 枚举指定进程所有进程的句柄信息
     *
     * @return long
     */
    long enumProcessHandle(DWORD dwProcessID)
    {
        long queryId = getContextNextID();

        try
        {
            vector<THREADENTRY32> ProcessThreadsList;
            getThreadList(dwProcessID, ProcessThreadsList);

            vector<hmc_usb::hmc_Volume> volumeList = util_getVolumeList();

            for (size_t i = 0; i < ProcessThreadsList.size(); i++)
            {
                DWORD ThreadsID = ProcessThreadsList[i].th32ThreadID;
                hmc_ProcessHandleContext handleCout;
                handleCout.ContextID = queryId;
                handleCout.handle = 0;
                handleCout.name = to_string(ThreadsID);
                handleCout.type = "Thread";
                handleCout.next = true;
                _enumProcessHandleList.push_back(handleCout);
            }

            vector<DWORD> SubProcessIDList;
            getSubProcessList(dwProcessID, SubProcessIDList);

            for (size_t i = 0; i < SubProcessIDList.size(); i++)
            {
                DWORD ThreadsID = SubProcessIDList[i];
                hmc_ProcessHandleContext handleCout;
                handleCout.ContextID = queryId;
                handleCout.handle = 0;
                handleCout.name = to_string(ThreadsID);
                handleCout.type = "Process";
                handleCout.next = true;
                _enumProcessHandleList.push_back(handleCout);
            }

            HMODULE hNtMod = LoadLibraryW(L"ntdll.dll");
            if (!hNtMod)
            {
                return queryId;
            }
            NTQUERYSYSTEMINFORMATION NtQuerySystemInformation = (NTQUERYSYSTEMINFORMATION)GetProcAddress(hNtMod, "NtQuerySystemInformation");
            NTDUPLICATEOBJECT NtDuplicateObject = (NTDUPLICATEOBJECT)GetProcAddress(hNtMod, "NtDuplicateObject");
            NTQUERYOBJECT NtQueryObject = (NTQUERYOBJECT)GetProcAddress(hNtMod, "NtQueryObject");

            if (!NtQuerySystemInformation || !NtDuplicateObject || !NtQueryObject)
            {
                return queryId;
            }

            PSYSTEM_HANDLE_INFORMATION handleInfo = NULL;
            HANDLE processHandle;
            ULONG i;
            ULONG neededSize = 0x1000;
            NTSTATUS Status = 0;
            ULONG ReturnLength = 0;
            handleInfo = (PSYSTEM_HANDLE_INFORMATION)malloc(neededSize);

            if (!handleInfo)
            {
                return queryId;
            }

            // 一直查询 直到成功
            while (STATUS_INFO_LENGTH_MISMATCH == (Status = NtQuerySystemInformation(
                                                       SystemHandleInformation,
                                                       handleInfo,
                                                       neededSize,
                                                       &ReturnLength)))
            {
                if (handleInfo)
                {
                    free(handleInfo);
                    handleInfo = NULL;
                }
                neededSize = ReturnLength;
                handleInfo = (PSYSTEM_HANDLE_INFORMATION)malloc(neededSize);
                if (!handleInfo)
                {

                    return queryId;
                }
            }
            processHandle = OpenProcess(PROCESS_DUP_HANDLE, FALSE, dwProcessID);
            for (i = 0; i < handleInfo->HandleCount; i++)
            {
                hmc_ProcessHandleContext handleCout;
                handleCout.ContextID = queryId;
                handleCout.handle = 0;
                handleCout.name = "";
                handleCout.type = "";
                handleCout.next = true;
                SYSTEM_HANDLE handle = handleInfo->Handles[i];
                if (handle.ProcessId != dwProcessID)
                {
                    continue;
                }
                handleCout.handle = handle.Handle;
                if (processHandle)
                {
                    HANDLE dupHandle = NULL;
                    POBJECT_TYPE_INFORMATION objectTypeInfo = NULL;
                    PVOID objectNameInfo = NULL;
                    UNICODE_STRING objectName = {0};
                    ULONG returnLength = 0;

                    do
                    {
                        // 句柄复制失败 就不去获取类型名
                        Status = NtDuplicateObject(
                            processHandle,
                            (void *)handle.Handle,
                            // GetCurrentProcess(),
                            processHandle,
                            &dupHandle,
                            0,
                            0,
                            0);
                        if (!NT_SUCCESS(Status))
                        {
                            break;
                        }

                        // 获取对象类型名
                        ULONG ObjectInformationLength = 0;
                        while (STATUS_INFO_LENGTH_MISMATCH == (Status = NtQueryObject(
                                                                   dupHandle,
                                                                   ObjectTypeInformation,
                                                                   objectTypeInfo,
                                                                   ObjectInformationLength,
                                                                   &returnLength)))
                        {
                            if (objectTypeInfo)
                            {
                                free(objectTypeInfo);
                                objectTypeInfo = NULL;
                            }

                            ObjectInformationLength = returnLength;
                            objectTypeInfo = (POBJECT_TYPE_INFORMATION)malloc(ObjectInformationLength);
                            if (!objectTypeInfo)
                            {
                                break;
                            }
                        }

                        // 获取对象类型名成功
                        if (NT_SUCCESS(Status))
                        {
                            handleCout.type = hmc_text_util::W2A(__unicodeStringToWString(objectTypeInfo->Name));
                        }
                        if (handle.GrantedAccess == 0x0012019f)
                        {

                            break;
                        }

                        // 获取对象名
                        ObjectInformationLength = 0;
                        returnLength = 0;

                        if (STATUS_INFO_LENGTH_MISMATCH == NtQueryObject(
                                                               dupHandle,
                                                               ObjectNameInformation,
                                                               NULL,
                                                               0,
                                                               &returnLength))
                        {

                            objectNameInfo = (POBJECT_TYPE_INFORMATION)malloc(returnLength);
                            if (!objectNameInfo)
                            {
                                break;
                            }

                            ZeroMemory(objectNameInfo, returnLength);
                            Status = NtQueryObject(
                                dupHandle,
                                ObjectNameInformation,
                                objectNameInfo,
                                returnLength,
                                NULL);
                        }

                        // 获取对象名成功
                        if (NT_SUCCESS(Status) && ((PUNICODE_STRING)objectNameInfo)->Length > 0)
                        {

                            UNICODE_STRING objectName = *(PUNICODE_STRING)objectNameInfo;

                            handleCout.name = hmc_text_util::W2A(__unicodeStringToWString(objectName));
                            if (handleCout.type == "File")
                            {
                                for (size_t i = 0; i < volumeList.size(); i++)
                                {
                                    hmc_usb::hmc_Volume volume = volumeList[i];
                                    if (handleCout.name.find(volume.device) == 0)
                                    {
                                        handleCout.name.replace(0, volume.device.length(), volume.path);
                                    }
                                }
                            }
                        }

                    } while (FALSE);

                    if (dupHandle)
                    {
                        CloseHandle(dupHandle);
                        dupHandle = NULL;
                    }
                    if (objectTypeInfo)
                    {
                        free(objectTypeInfo);
                        objectTypeInfo = NULL;
                    }
                    if (objectNameInfo)
                    {
                        free(objectNameInfo);
                        objectNameInfo = NULL;
                    }
                }
                if (!handleCout.name.empty() || !handleCout.type.empty())
                {
                    _enumProcessHandleList.push_back(handleCout);
                }
                Sleep(5);
            }

            free(handleInfo);
        }
        catch (char *e)
        {
            hmc_ProcessHandleContext handleCout;
            handleCout.ContextID = queryId;
            handleCout.handle = 0;
            handleCout.name = "";
            handleCout.type = "";
            handleCout.next = true;
            _enumProcessHandleList.push_back(handleCout);
        }

        return queryId;
    }

    // 时间格式转换
    __int64 _hmc_FileTimeToInt64(const FILETIME &time)
    {
        ULARGE_INTEGER tt;
        tt.LowPart = time.dwLowDateTime;
        tt.HighPart = time.dwHighDateTime;
        return (tt.QuadPart);
    }

    /**
     * @brief 获取进程的内存
     *
     * @param ProcessID
     * @return DWORD
     */
    DWORD getProcessMemoryInfo(DWORD ProcessID)
    {
        PROCESS_MEMORY_COUNTERS pmc;
        DWORD memoryInK = 0;
        HANDLE hProcess = NULL;

        hProcess = OpenProcess(
            PROCESS_QUERY_INFORMATION |
                PROCESS_VM_READ,
            FALSE, ProcessID);

        if (GetProcessMemoryInfo(hProcess, &pmc, sizeof(pmc)))
        {
            // memoryInK = pmc.WorkingSetSize/1024;		//单位为k
            memoryInK = pmc.WorkingSetSize;
        }

        CloseHandle(hProcess);
        return memoryInK;
    }

    /**
     * @brief 获取CPU核心数
     *
     * @return int
     */
    int _hmc_getCPUCount()
    {
        SYSTEM_INFO system_info;
        GetSystemInfo(&system_info);
        return static_cast<int>(system_info.dwNumberOfProcessors);
    }

    /**
     * @brief 获取指定进程CPU使用率
     *
     * @param ProcessID
     * @return double
     */
    double getProcessCpuUsage(DWORD ProcessID)
    {
        static int processor_count_ = -1;     // cpu核心数
        static __int64 last_system_time_ = 0; // 上一次的系统时间
        static __int64 last_time_ = 0;        // 上一次的时间

        FILETIME now;
        FILETIME creation_time;
        FILETIME exit_time;
        FILETIME kernel_time;
        FILETIME user_time;

        __int64 system_time;
        __int64 time;

        double cpu_usage = -1;

        if (processor_count_ == -1)
        {
            processor_count_ = _hmc_getCPUCount();
        }

        GetSystemTimeAsFileTime(&now);

        HANDLE hProcess = OpenProcess(
            PROCESS_QUERY_INFORMATION |
                PROCESS_VM_READ,
            FALSE, ProcessID);

        if (!hProcess)
        {
            return -1;
        }

        if (!GetProcessTimes(hProcess, &creation_time, &exit_time, &kernel_time, &user_time))
        {
            return -1;
        }

        system_time = (_hmc_FileTimeToInt64(kernel_time) + _hmc_FileTimeToInt64(user_time)) / processor_count_; // CPU使用时间
        time = _hmc_FileTimeToInt64(now);                                                                       // 现在的时间

        last_system_time_ = system_time;
        last_time_ = time;

        CloseHandle(hProcess);

        Sleep(1000); // 睡眠1s

        hProcess = OpenProcess(
            PROCESS_QUERY_INFORMATION |
                PROCESS_VM_READ,
            FALSE, ProcessID);

        if (!hProcess)
        {
            return -1;
        }

        if (!GetProcessTimes(hProcess, &creation_time, &exit_time, &kernel_time, &user_time))
        {
            return -1;
        }

        GetSystemTimeAsFileTime(&now);
        system_time = (_hmc_FileTimeToInt64(kernel_time) + _hmc_FileTimeToInt64(user_time)) / processor_count_; // CPU使用时间
        time = _hmc_FileTimeToInt64(now);                                                                       // 现在的时间

        CloseHandle(hProcess);

        cpu_usage = ((static_cast<double>(system_time - last_system_time_)) / (static_cast<double>(time - last_time_))) * 100;
        return cpu_usage;
    }

    struct hmc_PROCESSENTRY32A
    {
        DWORD cntThreads;            // 进程中的线程数。
        DWORD cntUsage;              // 表示进程的引用计数。
        DWORD dwFlags;               // 保留字段，暂时没有使用。
        DWORD dwSize;                // 结构的大小，用于指定调用方提供的结构大小，以便 API 函数可以正确填充结构。
        LONG pcPriClassBase;         // 进程的优先级。
        string szExeFile;            // 存储进程的可执行文件名，使用字符数组表示，长度为 MAX_PATH。
        ULONG_PTR th32DefaultHeapID; // 默认堆的标识符，一般用于堆管理。
        DWORD th32ModuleID;          // 拥有进程主模块的标识符，一般用于模块管理。
        DWORD th32ParentProcessID;   // 父进程的标识符。
        DWORD th32ProcessID;         // 进程的标识符(Process ID)
    };

    void _addExeFileToPROCESSENTRY32A(hmc_PROCESSENTRY32A &copyPe32, CHAR szExeFile[MAX_PATH])
    {
        copyPe32.szExeFile.append(szExeFile);
    }
    void _addExeFileToPROCESSENTRY32A(hmc_PROCESSENTRY32A &copyPe32, WCHAR szExeFile[MAX_PATH])
    {
        copyPe32.szExeFile.append(hmc_text_util::W2A(szExeFile));
    }

    /**
     * @brief 枚举进程快照
     *
     * @param ProcessSnapshotList
     */
    void enumProcessSnapshot(vector<hmc_PROCESSENTRY32A> &ProcessSnapshotList)
    {
        PROCESSENTRY32 pe32;
        pe32.dwSize = sizeof(PROCESSENTRY32);

        // 获取进程快照
        HANDLE hSnap = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if (hSnap == INVALID_HANDLE_VALUE)
        {
            return;
        }

        // 枚举第一个进程
        if (Process32First(hSnap, &pe32))
        {
            do
            {
                hmc_PROCESSENTRY32A copyPe32;
                copyPe32.cntThreads = pe32.cntThreads;
                copyPe32.cntUsage = pe32.cntUsage;
                copyPe32.dwFlags = pe32.dwFlags;
                copyPe32.dwSize = pe32.dwSize;
                copyPe32.pcPriClassBase = pe32.pcPriClassBase;
                _addExeFileToPROCESSENTRY32A(copyPe32, pe32.szExeFile);
                copyPe32.th32DefaultHeapID = pe32.th32DefaultHeapID;
                copyPe32.th32ModuleID = pe32.th32ModuleID;
                copyPe32.th32ParentProcessID = pe32.th32ParentProcessID;
                copyPe32.th32ProcessID = pe32.th32ProcessID;
                ProcessSnapshotList.push_back(copyPe32);
            } while (Process32Next(hSnap, &pe32));
        }

        CloseHandle(hSnap);
    }

    /**
     * @brief 树枚举所以进程结构
     *
     * @return json
     */
    json treeAllProcessJson()
    {
        json result;
        result.array();
        // 未编写
        return result;
    }
    /**
     * @brief 获取指定进程的命令行内容
     *
     * @param ProcessID
     * @return string
     */
    string getProcessCommand(DWORD ProcessID)
    {
        string commandLine;
        try
        {
            // 获取进程句柄
            HANDLE hProcess = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, ProcessID);
            if (hProcess == NULL)
            {
                return commandLine;
            }

            // 获取完整进程路径和命令行
            LPSTR lpExeName = {0};
            DWORD pathSize = 1024;
            if (QueryFullProcessImageNameA(hProcess, 0, lpExeName, &pathSize) != 0)
            {
                commandLine.append(lpExeName);
                CloseHandle(hProcess);
            }
        }
        catch (char *_)
        {
        }
        return commandLine;
    }
    
    /**
     * @brief 获取进程启动时候的时间ms
     * 
     * @param ProcessID 
     * @return long 
     */
    long getProcessIDTimes(DWORD ProcessID)
    {
        long result = 0;
        try
        {
            SYSTEMTIME stCreation, lstCreation;
            HANDLE process = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, ProcessID);
            if (process != NULL)
            {
                FILETIME ftCreation, ftExit, ftKernel, ftUser;
                if (GetProcessTimes(process, &ftCreation, &ftExit, &ftKernel, &ftUser))
                {
                    FileTimeToSystemTime(&ftCreation, &stCreation);
                    SystemTimeToTzSpecificLocalTime(NULL, &stCreation, &lstCreation);
                }
                CloseHandle(process);
            }

            result = SystemTimeToTimestamp(lstCreation);
        }
        catch (const std::exception &e)
        {
        }

        return result;
    }
}

#endif// #include <string>
// #include <windows.h>
// #include <Psapi.h>
// #include <Shellapi.h>
// #include <winternl.h>
// #include <winnt.h>

// #include <vector>
// #include <map>
// using namespace std;

// namespace hmc_process_libc
// {
//     struct hmc_Volume
//     {
//         string path;
//         string name;
//         string device;
//     };

//     // https://learn.microsoft.com/zh-cn/windows/win32/fileio/displaying-volume-paths
//     wstring DisplayVolumePaths(__in PWCHAR VolumeName)
//     {
//         wstring wstrVolumeName;
//         DWORD CharCount = MAX_PATH + 1;
//         PWCHAR Names = NULL;
//         PWCHAR NameIdx = NULL;
//         BOOL Success = FALSE;

//         for (;;)
//         {
//             //
//             //  Allocate a buffer to hold the paths.
//             Names = (PWCHAR) new BYTE[CharCount * sizeof(WCHAR)];

//             if (!Names)
//             {
//                 //
//                 //  If memory can't be allocated, return.
//                 return wstrVolumeName;
//             }

//             //
//             //  Obtain all of the paths
//             //  for this volume.
//             Success = GetVolumePathNamesForVolumeNameW(
//                 VolumeName, Names, CharCount, &CharCount);

//             if (Success)
//             {
//                 break;
//             }

//             if (GetLastError() != ERROR_MORE_DATA)
//             {
//                 break;
//             }

//             //
//             //  Try again with the
//             //  new suggested size.
//             delete[] Names;
//             Names = NULL;
//         }

//         if (Success)
//         {
//             //
//             //  Display the various paths.
//             for (NameIdx = Names;
//                  NameIdx[0] != L'\0';
//                  NameIdx += wcslen(NameIdx) + 1)
//             {
//                 wstrVolumeName.append(NameIdx);
//                 // wprintf(L"  %s", NameIdx);
//             }
//             // wprintf(L"\n");
//         }

//         if (Names != NULL)
//         {
//             delete[] Names;
//             Names = NULL;
//         }

//         return wstrVolumeName;
//     }

//     vector<hmc_Volume> util_getVolumeList()
//     {
//         vector<hmc_Volume> Results = {};
//         DWORD CharCount = 0;
//         WCHAR DeviceName[MAX_PATH] = L"";
//         DWORD Error = ERROR_SUCCESS;
//         HANDLE FindHandle = INVALID_HANDLE_VALUE;
//         BOOL Found = FALSE;
//         size_t Index = 0;
//         BOOL Success = FALSE;
//         WCHAR VolumeName[MAX_PATH] = L"";

//         //
//         //  Enumerate all volumes in the system.
//         FindHandle = FindFirstVolumeW(VolumeName, ARRAYSIZE(VolumeName));

//         if (FindHandle == INVALID_HANDLE_VALUE)
//         {
//             Error = GetLastError();
//             // wprintf(L"FindFirstVolumeW failed with error code %d\n", Error);
//             return Results;
//         }
//         size_t index = 0;
//         for (;;)
//         {
//             //
//             //  Skip the \\?\ prefix and remove the trailing backslash.
//             Index = wcslen(VolumeName) - 1;
//             if (VolumeName[0] != L'\\' ||
//                 VolumeName[1] != L'\\' ||
//                 VolumeName[2] != L'?' ||
//                 VolumeName[3] != L'\\' ||
//                 VolumeName[Index] != L'\\')
//             {
//                 Error = ERROR_BAD_PATHNAME;
//                 // wprintf(L"FindFirstVolumeW/FindNextVolumeW returned a bad path: %s\n", VolumeName);
//                 // break;
//             }

//             //
//             //  QueryDosDeviceW does not allow a trailing backslash,
//             //  so temporarily remove it.
//             VolumeName[Index] = L'\0';

//             CharCount = QueryDosDeviceW(&VolumeName[4], DeviceName, ARRAYSIZE(DeviceName));

//             VolumeName[Index] = L'\\';

//             if (CharCount == 0)
//             {
//                 // Error = GetLastError();
//                 // wprintf(L"QueryDosDeviceW failed with error code %d\n", Error);
//                 // break;
//             }
//             hmc_Volume cur_item;
//             cur_item.device = hmc_text_util::W2A(DeviceName);
//             cur_item.path = hmc_text_util::W2A(DisplayVolumePaths(VolumeName));
//             cur_item.name = hmc_text_util::W2A(VolumeName);
//             Results.push_back(cur_item);
//             Success = FindNextVolumeW(FindHandle, VolumeName, ARRAYSIZE(VolumeName));

//             if (!Success)
//             {
//                 Error = GetLastError();

//                 if (Error != ERROR_NO_MORE_FILES)
//                 {
//                     // wprintf(L"FindNextVolumeW failed with error code %d\n", Error);
//                     break;
//                 }

//                 //
//                 //  Finished iterating
//                 //  through all the volumes.
//                 Error = ERROR_SUCCESS;
//                 break;
//             }
//             index++;
//         }

//         FindVolumeClose(FindHandle);
//         FindHandle = INVALID_HANDLE_VALUE;
//         return Results;
//     }

//     struct hmc_ProcessHandleContext
//     {
//         long ContextID;
//         bool next;
//         DWORD handle;
//         string name; // string
//         string type; // "ALPC Port" | "Event" | "Timer" | "Mutant" | "Key" | "Section" | "File" | "Thread" | string;
//     };

//     wstring __unicodeStringToWString(UNICODE_STRING unicodeString)
//     {
//         wstring result;
//         if (unicodeString.Buffer)
//         {
//             result = wstring(unicodeString.Buffer, unicodeString.Length / sizeof(wchar_t));
//         }
//         return result;
//     }

//     /**
//      * @brief 枚举指定进程所有进程的句柄信息
//      *
//      * @return long
//      */
//     void enumAllFileHandle()
//     {
//         vector<hmc_ProcessHandleContext> _enumProcessHandleList;

//         try
//         {

//             vector<hmc_Volume> volumeList = util_getVolumeList();

//             HMODULE hNtMod = LoadLibraryW(L"ntdll.dll");
//             if (!hNtMod)
//             {
//                 return;
//             }
//             NTQUERYSYSTEMINFORMATION NtQuerySystemInformation = (NTQUERYSYSTEMINFORMATION)GetProcAddress(hNtMod, "NtQuerySystemInformation");
//             NTDUPLICATEOBJECT NtDuplicateObject = (NTDUPLICATEOBJECT)GetProcAddress(hNtMod, "NtDuplicateObject");
//             NTQUERYOBJECT NtQueryObject = (NTQUERYOBJECT)GetProcAddress(hNtMod, "NtQueryObject");

//             if (!NtQuerySystemInformation || !NtDuplicateObject || !NtQueryObject)
//             {
//                 return;
//             }

//             PSYSTEM_HANDLE_INFORMATION handleInfo = NULL;
//             HANDLE processHandle;
//             ULONG i;
//             ULONG neededSize = 0x1000;
//             NTSTATUS Status = 0;
//             ULONG ReturnLength = 0;
//             handleInfo = (PSYSTEM_HANDLE_INFORMATION)malloc(neededSize);

//             if (!handleInfo)
//             {
//                 return;
//             }

//             // 一直查询 直到成功
//             while (STATUS_INFO_LENGTH_MISMATCH == (Status = NtQuerySystemInformation(
//                                                        SystemHandleInformation,
//                                                        handleInfo,
//                                                        neededSize,
//                                                        &ReturnLength)))
//             {
//                 if (handleInfo)
//                 {
//                     free(handleInfo);
//                     handleInfo = NULL;
//                 }
//                 neededSize = ReturnLength;
//                 handleInfo = (PSYSTEM_HANDLE_INFORMATION)malloc(neededSize);
//                 if (!handleInfo)
//                 {

//                     return queryId;
//                 }
//             }
//             processHandle = OpenProcess(PROCESS_DUP_HANDLE, FALSE, NULL);
//             for (i = 0; i < handleInfo->HandleCount; i++)
//             {
//                 hmc_ProcessHandleContext handleCout;
//                 handleCout.ContextID = queryId;
//                 handleCout.handle = 0;
//                 handleCout.name = "";
//                 handleCout.type = "";
//                 handleCout.next = true;
//                 SYSTEM_HANDLE handle = handleInfo->Handles[i];

//                 handleCout.handle = handle.Handle;
//                 if (processHandle)
//                 {
//                     HANDLE dupHandle = NULL;
//                     POBJECT_TYPE_INFORMATION objectTypeInfo = NULL;
//                     PVOID objectNameInfo = NULL;
//                     UNICODE_STRING objectName = {0};
//                     ULONG returnLength = 0;

//                     do
//                     {
//                         // 句柄复制失败 就不去获取类型名
//                         Status = NtDuplicateObject(
//                             processHandle,
//                             (void *)handle.Handle,
//                             // GetCurrentProcess(),
//                             processHandle,
//                             &dupHandle,
//                             0,
//                             0,
//                             0);
//                         if (!NT_SUCCESS(Status))
//                         {
//                             break;
//                         }

//                         // 获取对象类型名
//                         ULONG ObjectInformationLength = 0;
//                         while (STATUS_INFO_LENGTH_MISMATCH == (Status = NtQueryObject(
//                                                                    dupHandle,
//                                                                    ObjectTypeInformation,
//                                                                    objectTypeInfo,
//                                                                    ObjectInformationLength,
//                                                                    &returnLength)))
//                         {
//                             if (objectTypeInfo)
//                             {
//                                 free(objectTypeInfo);
//                                 objectTypeInfo = NULL;
//                             }

//                             ObjectInformationLength = returnLength;
//                             objectTypeInfo = (POBJECT_TYPE_INFORMATION)malloc(ObjectInformationLength);
//                             if (!objectTypeInfo)
//                             {
//                                 break;
//                             }
//                         }

//                         // 获取对象类型名成功
//                         if (NT_SUCCESS(Status))
//                         {
//                             handleCout.type = hmc_text_util::W2A(__unicodeStringToWString(objectTypeInfo->Name));
//                         }
//                         if (handle.GrantedAccess == 0x0012019f)
//                         {

//                             break;
//                         }

//                         // 获取对象名
//                         ObjectInformationLength = 0;
//                         returnLength = 0;

//                         if (STATUS_INFO_LENGTH_MISMATCH == NtQueryObject(
//                                                                dupHandle,
//                                                                ObjectNameInformation,
//                                                                NULL,
//                                                                0,
//                                                                &returnLength))
//                         {

//                             objectNameInfo = (POBJECT_TYPE_INFORMATION)malloc(returnLength);
//                             if (!objectNameInfo)
//                             {
//                                 break;
//                             }

//                             ZeroMemory(objectNameInfo, returnLength);
//                             Status = NtQueryObject(
//                                 dupHandle,
//                                 ObjectNameInformation,
//                                 objectNameInfo,
//                                 returnLength,
//                                 NULL);
//                         }

//                         // 获取对象名成功
//                         if (NT_SUCCESS(Status) && ((PUNICODE_STRING)objectNameInfo)->Length > 0)
//                         {

//                             UNICODE_STRING objectName = *(PUNICODE_STRING)objectNameInfo;

//                             handleCout.name = hmc_text_util::W2A(__unicodeStringToWString(objectName));
//                             if (handleCout.type == "File")
//                             {
//                                 for (size_t i = 0; i < volumeList.size(); i++)
//                                 {
//                                     hmc_Volume volume = volumeList[i];
//                                     if (handleCout.name.find(volume.device) == 0)
//                                     {
//                                         handleCout.name.replace(0, volume.device.length(), volume.path);
//                                     }
//                                 }
//                             }
//                         }

//                     } while (FALSE);

//                     if (dupHandle)
//                     {
//                         CloseHandle(dupHandle);
//                         dupHandle = NULL;
//                     }
//                     if (objectTypeInfo)
//                     {
//                         free(objectTypeInfo);
//                         objectTypeInfo = NULL;
//                     }
//                     if (objectNameInfo)
//                     {
//                         free(objectNameInfo);
//                         objectNameInfo = NULL;
//                     }
//                 }
//                 if (!handleCout.name.empty() || !handleCout.type.empty())
//                 {
//                     _enumProcessHandleList.push_back(handleCout);
//                 }
//             }

//             free(handleInfo);
//         }
//         catch (char *e)
//         {
//             hmc_ProcessHandleContext handleCout;
//             handleCout.ContextID = 0;
//             handleCout.handle = 0;
//             handleCout.name = "";
//             handleCout.type = "";
//             handleCout.next = true;
//             _enumProcessHandleList.push_back(handleCout);
//         }
//     }

// }

#ifndef HMC_IMPORT_PROCESS_H
#define HMC_IMPORT_PROCESS_H

#define _CRT_SECURE_NO_WARNINGS
#include <string>
#include <windows.h>
#include <Psapi.h>
#include <Shellapi.h>
#include <tlhelp32.h>
#include <vector>
#include <map>
#include <winternl.h>
using namespace std;

namespace hmc_process
{
    long hmc_Object_Context_id = 0;
    /**
     * @brief 获取一个内容id
     *
     * @return long
     */
    long getContextNextID()
    {
        hmc_Object_Context_id++;
        return hmc_Object_Context_id;
    };

    BOOL hmc_EnableShutDownPriv()
    {
        // HANDLE Handle_Token = NULL;
        // TOKEN_PRIVILEGES PermissionAttribute = {0};
        // // 打开当前程序的权限令牌
        // bool is_Open_Process_Token = OpenProcessToken(GetCurrentProcess(), TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY, &Handle_Token);
        // if (!is_Open_Process_Token)
        // {
        //     return FALSE;
        // }
        // // 获得某一特定权限的权限标识LUID 保存到权限属性中
        // if (!LookupPrivilegeValue(NULL, SE_SHUTDOWN_NAME, &PermissionAttribute.Privileges[0].Luid))
        // {
        //     CloseHandle(Handle_Token);
        //     return FALSE;
        // }
        // PermissionAttribute.Privileges[0].Attributes = SE_PRIVILEGE_ENABLED;
        // PermissionAttribute.PrivilegeCount = 1;
        // // 提升到系统权限
        // if (!AdjustTokenPrivileges(Handle_Token, FALSE, &PermissionAttribute, sizeof(TOKEN_PRIVILEGES), NULL, NULL))
        // {
        //     CloseHandle(Handle_Token);
        //     return FALSE;
        // }
        return TRUE;
    }

    // #define NT_SUCCESS(x) ((x) >= 0)
    // #define STATUS_INFO_LENGTH_MISMATCH 0xc0000004
    //
    // #define SystemHandleInformation 16
    // #define ObjectBasicInformation 0
    // #define ObjectNameInformation 1
    // #define ObjectTypeInformation 2
    //
    //     typedef NTSTATUS(NTAPI *NTQUERYSYSTEMINFORMATION)(
    //         // 检索的信息类型
    //         ULONG SystemInformationClass,
    //         // 指向缓冲区的指针 有关系统信息的结构体
    //         PVOID SystemInformation,
    //         // 缓冲区的大小
    //         ULONG SystemInformationLength,
    //         // 实际返回的信息大小
    //         PULONG ReturnLength);
    //
    //     typedef NTSTATUS(NTAPI *NTDUPLICATEOBJECT)(
    //         // 源进程的句柄
    //         HANDLE SourceProcessHandle,
    //         // 复制的内核对象的句柄
    //         HANDLE SourceHandle,
    //         // 目标进程的句柄
    //         HANDLE TargetProcessHandle,
    //         // 目标进程中新对象的指针
    //         PHANDLE TargetHandle,
    //         // 新对象的访问权限
    //         ACCESS_MASK DesiredAccess,
    //         // 新对象的属性
    //         ULONG Attributes,
    //         // 复制操作的选项
    //         ULONG Options);
    //
    //     typedef NTSTATUS(NTAPI *NTQUERYOBJECT)(
    //         HANDLE ObjectHandle,
    //         ULONG ObjectInformationClass,
    //         PVOID ObjectInformation,
    //         ULONG ObjectInformationLength,
    //         PULONG ReturnLength);
    //
    //     typedef struct _SYSTEM_HANDLE
    //     {
    //         ULONG ProcessId;
    //         BYTE ObjectTypeNumber;
    //         BYTE Flags;
    //         USHORT Handle;
    //         PVOID Object;
    //         ACCESS_MASK GrantedAccess;
    //     } SYSTEM_HANDLE, *PSYSTEM_HANDLE;
    //
    //     typedef struct _SYSTEM_HANDLE_INFORMATION
    //     {
    //         ULONG HandleCount;
    //         SYSTEM_HANDLE Handles[1];
    //     } SYSTEM_HANDLE_INFORMATION, *PSYSTEM_HANDLE_INFORMATION;
    //
    //     typedef enum _POOL_TYPE
    //     {
    //         NonPagedPool,
    //         PagedPool,
    //         NonPagedPoolMustSucceed,
    //         DontUseThisType,
    //         NonPagedPoolCacheAligned,
    //         PagedPoolCacheAligned,
    //         NonPagedPoolCacheAlignedMustS
    //     } POOL_TYPE,
    //         *PPOOL_TYPE;
    //
    //     typedef struct _UNICODE_STRING
    //     {
    //         USHORT Length;
    //         USHORT MaximumLength;
    // #ifdef MIDL_PASS
    //         [ size_is(MaximumLength / 2), length_is((Length) / 2) ] USHORT *Buffer;
    // #else  // MIDL_PASS
    //         _Field_size_bytes_part_opt_(MaximumLength, Length) PWCH Buffer;
    // #endif // MIDL_PASS
    //     } UNICODE_STRING;
    //
    //     typedef UNICODE_STRING *PUNICODE_STRING;
    //     typedef const UNICODE_STRING *PCUNICODE_STRING;
    //
    //     typedef struct _OBJECT_TYPE_INFORMATION
    //     {
    //         // 对象名称。
    //         UNICODE_STRING Name;
    //         // 对象的总数。
    //         ULONG TotalNumberOfObjects;
    //         // 对象句柄的总数。
    //         ULONG TotalNumberOfHandles;
    //         // 对象使用的分页池内存总量。
    //         ULONG TotalPagedPoolUsage;
    //         // 对象使用的非分页池内存总量。
    //         ULONG TotalNonPagedPoolUsage;
    //         // 对象名称使用的内存总量。
    //         ULONG TotalNamePoolUsage;
    //         // 对象句柄表使用的内存总量。
    //         ULONG TotalHandleTableUsage;
    //         // 对象的最大数量。
    //         ULONG HighWaterNumberOfObjects;
    //         // 对象句柄的最大数量。
    //         ULONG HighWaterNumberOfHandles;
    //         // 对象使用的分页池内存的最大值。
    //         ULONG HighWaterPagedPoolUsage;
    //         // 对象使用的非分页池内存的最大值。
    //         ULONG HighWaterNonPagedPoolUsage;
    //         // 对象名称使用的内存的最大值。
    //         ULONG HighWaterNamePoolUsage;
    //         // 对象句柄表使用的内存的最大值。
    //         ULONG HighWaterHandleTableUsage;
    //         // 无效属性标志。
    //         ULONG InvalidAttributes;
    //         // 通用映射结构体。
    //         GENERIC_MAPPING GenericMapping;
    //         // 有效访问标志。
    //         ULONG ValidAccess;
    //         // 安全性要求标志。
    //         BOOLEAN SecurityRequired;
    //         // 维护句柄计数标志。
    //         BOOLEAN MaintainHandleCount;
    //         // 维护类型列表标志。
    //         USHORT MaintainTypeList;
    //         // 池类型。
    //         POOL_TYPE PoolType;
    //         // 分页池内存使用量。
    //         ULONG PagedPoolUsage;
    //         // 非分页池内存使用量。
    //         ULONG NonPagedPoolUsage;
    //     } OBJECT_TYPE_INFORMATION, *POBJECT_TYPE_INFORMATION;

    /**
     * @brief 获取当前进程的父进程id
     *
     * @param matchProcessID
     * @return DWORD
     */
    DWORD getParentProcessID(DWORD matchProcessID)
    {
        DWORD CurrentProcessId = 0;
        PROCESSENTRY32 pe32;
        pe32.dwSize = sizeof(PROCESSENTRY32);

        // 获取进程快照
        HANDLE hSnap = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if (hSnap == INVALID_HANDLE_VALUE)
        {
            return NULL;
        }

        // 枚举第一个进程
        if (Process32First(hSnap, &pe32))
        {
            do
            {
                // 打开进程句柄
                HANDLE hProcess = OpenProcess(PROCESS_ALL_ACCESS, FALSE, pe32.th32ProcessID);
                if (hProcess)
                {
                    if (pe32.th32ProcessID == matchProcessID)
                    {
                        CurrentProcessId = pe32.th32ParentProcessID;
                        CloseHandle(hProcess);
                        break;
                    }
                    CloseHandle(hProcess);
                }
            } while (Process32Next(hSnap, &pe32));
        }

        CloseHandle(hSnap);
        return CurrentProcessId;
    }

    /**
     * @brief 获取当前进程的所有子进程
     *
     * @param ProcessId
     * @param SubProcessIDList
     */
    void getSubProcessList(DWORD dwProcessID, vector<DWORD> &SubProcessIDList)
    {

        hmc_EnableShutDownPriv();
        PROCESSENTRY32 pe32;
        pe32.dwSize = sizeof(PROCESSENTRY32);

        // 获取进程快照
        HANDLE hSnap = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if (hSnap == INVALID_HANDLE_VALUE)
        {
            return;
        }

        // 枚举第一个进程
        if (Process32First(hSnap, &pe32))
        {
            do
            {
                // 打开进程句柄
                HANDLE hProcess = OpenProcess(PROCESS_ALL_ACCESS, FALSE, pe32.th32ProcessID);
                if (hProcess)
                {
                    // 子进程的进程 ID
                    bool is_sub = pe32.th32ParentProcessID == dwProcessID;

                    // 二次子进程
                    if (!is_sub)
                    {
                        if (find(SubProcessIDList.begin(), SubProcessIDList.end(), pe32.th32ParentProcessID) != SubProcessIDList.end())
                        {
                            is_sub = true;
                        }
                    }

                    if (is_sub)
                    {
                        if (!(find(SubProcessIDList.begin(), SubProcessIDList.end(), pe32.th32ProcessID) != SubProcessIDList.end()))
                        {
                            SubProcessIDList.push_back(pe32.th32ProcessID);
                        }
                    }
                    CloseHandle(hProcess);
                }
            } while (Process32Next(hSnap, &pe32));
        }

        CloseHandle(hSnap);
    };

    /**
     * @brief 获取进程可执行文件路径
     *
     * @param ProcessID
     * @return string
     */
    string getFilePath(DWORD dwProcessID)
    {
        LPSTR lpFilename = {0};
        HANDLE hProcess = OpenProcess(PROCESS_ALL_ACCESS, FALSE, dwProcessID);

        if (hProcess == nullptr)
        {
            CloseHandle(hProcess);
            return string("");
        }
        ::GetModuleFileNameExA(hProcess, NULL, lpFilename, MAX_PATH);
        return string(lpFilename);
    }

    /**
     * @brief 结束指定进程
     *
     * @param ProcessID
     * @return BOOL
     */
    BOOL killProcessID(DWORD dwProcessID)
    {
        bool is_kill_success = false;
        hmc_EnableShutDownPriv();
        HANDLE killHandle = OpenProcess(PROCESS_TERMINATE | PROCESS_QUERY_INFORMATION | PROCESS_CREATE_THREAD | PROCESS_VM_OPERATION | PROCESS_VM_WRITE, FALSE, dwProcessID);
        if (killHandle != NULL)
        {
            is_kill_success = TerminateProcess(killHandle, 0);
        }
        return is_kill_success;
    }

    /**
     * @brief 判断进程是否存在
     *
     * @param ProcessID
     * @return BOOL
     */
    BOOL existsProcessID(DWORD dwProcessID)
    {
        hmc_EnableShutDownPriv();
        bool exists_process = false;
        HANDLE hProcess = OpenProcess(PROCESS_ALL_ACCESS, FALSE, dwProcessID);
        if (GetLastError())
            return false;

        exists_process = hProcess != NULL;
        CloseHandle(hProcess);
        return exists_process;
    }

    /**
     * @brief 获取进程的HWND
     *
     * @param dwProcessID
     * @return HWND
     */
    HWND getHwnd(DWORD dwProcessID)
    {
        HWND win_next_it = GetTopWindow(0);
        HWND result = NULL;
        while (win_next_it)
        {
            DWORD pid = 0;
            DWORD theardId = GetWindowThreadProcessId(win_next_it, &pid);
            if (theardId != 0)
            {
                if (pid == dwProcessID && GetParent(win_next_it) == NULL && ::IsWindowVisible(win_next_it))
                {

                    result = win_next_it;
                }
            }
            win_next_it = GetNextWindow(win_next_it, GW_HWNDNEXT);
        }
        return result;
    }

    /**
     * @brief 获取可执行文件名称
     *
     * @param dwProcessID
     * @return string
     */
    string getBaseName(DWORD dwProcessID)
    {
        string FilePath;
        HANDLE hProcess = OpenProcess(PROCESS_ALL_ACCESS, FALSE, dwProcessID);
        char lpFilename[MAX_PATH];
        if (hProcess == nullptr)
        {

            return FilePath;
        }
        GetModuleBaseNameA(hProcess, NULL, (LPSTR)lpFilename, MAX_PATH);
        CloseHandle(hProcess);
        FilePath.append(lpFilename);
        return FilePath;
    }

    /**
     * @brief 枚举进程的线程信息
     *
     * @param dwProcessID
     * @param resultsModulePathList
     */
    void getThreadList(DWORD dwProcessID, vector<DWORD> &resultsModulePathList)
    {
        HANDLE hThreadSnap = INVALID_HANDLE_VALUE;
        // `THREADENTRY32` 是一个结构体，它定义在 `tlhelp32.h` 头文件中。它描述了在系统执行快照时正在执行的线程列表中的条目。以下是 `THREADENTRY32` 结构体中的各个变量的含义：⁴⁵
        // - dwSize：结构体的大小，以字节为单位。
        // - cntUsage：线程使用计数。
        // - th32ThreadID：线程标识符，与 `CreateProcess` 函数返回的线程标识符兼容。
        // - th32OwnerProcessID：创建线程的进程标识符。
        // - tpBasePri：分配给线程的内核基优先级。
        // - tpDeltaPri：线程优先级相对于基本优先级的增量。
        // - dwFlags：保留，不再使用。
        THREADENTRY32 te32;

        // 对所有正在运行的线程进行快照
        hThreadSnap = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0);
        if (hThreadSnap == INVALID_HANDLE_VALUE)
            return;

        te32.dwSize = sizeof(THREADENTRY32);

        // 检索第一个线程的信息
        if (!Thread32First(hThreadSnap, &te32))
        {
            CloseHandle(hThreadSnap);
            return;
        }

        do
        {
            if (te32.th32OwnerProcessID == dwProcessID || dwProcessID == 0)
            {
                resultsModulePathList.push_back(te32.th32ThreadID);
            }
        } while (Thread32Next(hThreadSnap, &te32));

        CloseHandle(hThreadSnap);
    }

    /**
     * @brief 枚举进程的线程信息
     *
     * @param dwProcessID
     * @param resultsModulePathList
     */
    void getThreadList(DWORD dwProcessID, vector<THREADENTRY32> &resultsModulePathList)
    {
        HANDLE hThreadSnap = INVALID_HANDLE_VALUE;
        // `THREADENTRY32` 是一个结构体，它定义在 `tlhelp32.h` 头文件中。它描述了在系统执行快照时正在执行的线程列表中的条目。以下是 `THREADENTRY32` 结构体中的各个变量的含义：⁴⁵
        // - dwSize：结构体的大小，以字节为单位。
        // - cntUsage：线程使用计数。
        // - th32ThreadID：线程标识符，与 `CreateProcess` 函数返回的线程标识符兼容。
        // - th32OwnerProcessID：创建线程的进程标识符。
        // - tpBasePri：分配给线程的内核基优先级。
        // - tpDeltaPri：线程优先级相对于基本优先级的增量。
        // - dwFlags：保留，不再使用。
        THREADENTRY32 te32;

        // 对所有正在运行的线程进行快照
        hThreadSnap = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0);
        if (hThreadSnap == INVALID_HANDLE_VALUE)
            return;

        te32.dwSize = sizeof(THREADENTRY32);

        // 检索第一个线程的信息
        if (!Thread32First(hThreadSnap, &te32))
        {
            CloseHandle(hThreadSnap);
            return;
        }

        do
        {
            if (te32.th32OwnerProcessID == dwProcessID || dwProcessID == 0)
            {
                THREADENTRY32 copy_te32;
                copy_te32.cntUsage = te32.cntUsage;
                copy_te32.cntUsage = te32.dwFlags;
                copy_te32.cntUsage = te32.dwSize;
                copy_te32.cntUsage = te32.th32OwnerProcessID;
                copy_te32.cntUsage = te32.th32ThreadID;
                copy_te32.cntUsage = te32.tpBasePri;
                copy_te32.cntUsage = te32.tpDeltaPri;

                resultsModulePathList.push_back(copy_te32);
            }
        } while (Thread32Next(hThreadSnap, &te32));

        CloseHandle(hThreadSnap);
    }

    /**
     * @brief 获取进程引用的模块列表
     *
     * @param dwProcessID
     * @param resultsData
     */
    void getModulePathList(DWORD dwProcessID, vector<string> &resultsData)
    {

        HANDLE hProcess = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, dwProcessID);
        if (hProcess == NULL)
            return;

        vector<HANDLE> vecFileHandles;

        // 枚举进程打开的文件句柄
        HANDLE hSnapshot = CreateToolhelp32Snapshot(TH32CS_SNAPALL, 0);
        if (hSnapshot != INVALID_HANDLE_VALUE)
        {
            PROCESSENTRY32 pe32;
            pe32.dwSize = sizeof(PROCESSENTRY32);
            if (Process32First(hSnapshot, &pe32))
            {
                do
                {
                    if (pe32.th32ProcessID == dwProcessID)
                    {
                        HANDLE hModuleSnap = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, dwProcessID);
                        if (hModuleSnap != INVALID_HANDLE_VALUE)
                        {
                            MODULEENTRY32 me32;
                            me32.dwSize = sizeof(MODULEENTRY32);
                            if (Module32First(hModuleSnap, &me32))
                            {
                                do
                                {
                                    HANDLE hFile = CreateFile(me32.szExePath, GENERIC_READ, FILE_SHARE_READ | FILE_SHARE_WRITE, NULL, OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, NULL);
                                    if (hFile != INVALID_HANDLE_VALUE)
                                    {
                                        vecFileHandles.push_back(hFile);
                                    }
                                } while (Module32Next(hModuleSnap, &me32));
                            }
                            CloseHandle(hModuleSnap);
                        }
                    }
                } while (Process32Next(hSnapshot, &pe32));
            }
            CloseHandle(hSnapshot);
        }

        // 输出文件路径
        for (auto it = vecFileHandles.begin(); it != vecFileHandles.end(); ++it)
        {
            LPSTR szFilePath = new CHAR[MAX_PATH];
            DWORD dwSize = GetFinalPathNameByHandleA(*it, szFilePath, MAX_PATH, FILE_NAME_NORMALIZED);
            if (dwSize > 0 && dwSize < MAX_PATH)
            {
                string strFilePath = szFilePath;
                string findStr = "\\\\?\\";
                if (strFilePath.find(findStr) == 0)
                {
                    strFilePath.replace(0, findStr.length(), "");
                }
                resultsData.push_back(strFilePath);
            }
            delete[] szFilePath;
            CloseHandle(*it);
        }

        CloseHandle(hProcess);
        return;
    }

    /**
     * @brief 获取鼠标所在的句柄的进程id
     *
     * @return DWORD
     */
    DWORD getPointWindowProcessId()
    {
        DWORD processId = 0;
        POINT curPoint;
        if (!GetCursorPos(&curPoint))
            return processId;
        HWND mainWindow = WindowFromPoint(curPoint);
        GetWindowThreadProcessId(mainWindow, &processId);
        return processId;
    }

    /**
     * @brief 获取鼠标所在的窗口的进程文件名
     *
     * @return string
     */
    string getPointWindowProcessBaseName()
    {
        return getBaseName(getPointWindowProcessId());
    }

    /**
     * @brief 获取当前聚焦的窗口的进程id
     *
     * @return DWORD
     */
    DWORD getFocusWindowProcessID()
    {
        DWORD processId;
        GetWindowThreadProcessId(GetForegroundWindow(), &processId);
        return processId;
    }

    /**
     * @brief 获取聚焦的窗口的进程文件名
     *
     * @return string
     */
    string getFocusWindowProcessBaseName()
    {
        return getBaseName(getFocusWindowProcessID());
    }

    struct ProcessEnumDetailsCont
    {
        DWORD pid;
        string baseName;
        string path;
    };

    struct ProcessEnumCont
    {
        DWORD pid;
        string baseName;
    };
    /**
     * @brief 枚举进程列表
     *
     * @param resultsData
     */

    void getProcessList(vector<ProcessEnumCont> &resultsData)
    {
        hmc_EnableShutDownPriv();
        DWORD processList[1024], cbNeeded;
        if (!EnumProcesses(processList, sizeof(processList), &cbNeeded))
        {
        }
        int numProcesses = cbNeeded / sizeof(DWORD);
        for (int i = 0; i < numProcesses; ++i)
        {
            DWORD processID = processList[i];
            HANDLE hProcess = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, processID);
            if (hProcess)
            {
                char processName[MAX_PATH];
                GetModuleBaseNameA(hProcess, NULL, processName, MAX_PATH);
                ProcessEnumCont processEnumCont;
                processEnumCont.pid = processID;
                processEnumCont.baseName = processName;
                resultsData.push_back(processEnumCont);
                CloseHandle(hProcess);
            }
        }
    }

    void getProcessList(vector<ProcessEnumDetailsCont> &resultsData)
    {
        hmc_EnableShutDownPriv();
        DWORD processList[1024], cbNeeded;
        if (!EnumProcesses(processList, sizeof(processList), &cbNeeded))
        {
        }
        int numProcesses = cbNeeded / sizeof(DWORD);
        for (int i = 0; i < numProcesses; ++i)
        {
            DWORD processID = processList[i];
            HANDLE hProcess = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, processID);
            if (hProcess)
            {
                char processName[MAX_PATH];
                char Filename[1024];
                GetModuleBaseNameA(hProcess, NULL, processName, MAX_PATH);
                GetModuleFileNameExA(hProcess, NULL, Filename, 1024);
                ProcessEnumDetailsCont processEnumCont;
                processEnumCont.pid = processID;
                processEnumCont.baseName = processName;
                processEnumCont.path = Filename;
                resultsData.push_back(processEnumCont);
                CloseHandle(hProcess);
            }
        }
    }

    wstring __unicodeStringToWString(UNICODE_STRING unicodeString)
    {
        wstring result;
        if (unicodeString.Buffer)
        {
            result = wstring(unicodeString.Buffer, unicodeString.Length / sizeof(wchar_t));
        }
        return result;
    }
    /**
     * @brief 获取窗口句柄对应的pid
     *
     * @param hwnd
     * @return DWORD
     */
    DWORD getHwndProcessID(HWND hwnd)
    {
        DWORD processId;
        GetWindowThreadProcessId(hwnd, &processId);
        return processId;
    }
    struct hmc_ProcessHandleContext
    {
        long ContextID;
        bool next;
        DWORD handle;
        string name; // string
        string type; // "ALPC Port" | "Event" | "Timer" | "Mutant" | "Key" | "Section" | "File" | "Thread" | string;
    };

    vector<hmc_ProcessHandleContext> _enumProcessHandleList;

    ///**
    // * @brief 枚举指定进程所有进程的句柄信息
    // *
    // * @return long
    // */
    // long enumProcessHandle(DWORD dwProcessID)
    //{
    //    long queryId = getContextNextID();

    //    try
    //    {
    //        vector<THREADENTRY32> ProcessThreadsList;
    //        getThreadList(dwProcessID, ProcessThreadsList);

    //        vector<hmc_usb::hmc_Volume> volumeList = hmc_usb::util_getVolumeList();

    //        for (size_t i = 0; i < ProcessThreadsList.size(); i++)
    //        {
    //            DWORD ThreadsID = ProcessThreadsList[i].th32ThreadID;
    //            hmc_ProcessHandleContext handleCout;
    //            handleCout.ContextID = queryId;
    //            handleCout.handle = 0;
    //            handleCout.name = to_string(ThreadsID);
    //            handleCout.type = "Thread";
    //            handleCout.next = true;
    //            _enumProcessHandleList.push_back(handleCout);
    //        }

    //        vector<DWORD> SubProcessIDList;
    //        getSubProcessList(dwProcessID, SubProcessIDList);

    //        for (size_t i = 0; i < SubProcessIDList.size(); i++)
    //        {
    //            DWORD ThreadsID = SubProcessIDList[i];
    //            hmc_ProcessHandleContext handleCout;
    //            handleCout.ContextID = queryId;
    //            handleCout.handle = 0;
    //            handleCout.name = to_string(ThreadsID);
    //            handleCout.type = "Process";
    //            handleCout.next = true;
    //            _enumProcessHandleList.push_back(handleCout);
    //        }

    //        HMODULE hNtMod = LoadLibraryW(L"ntdll.dll");
    //        if (!hNtMod)
    //        {
    //            return queryId;
    //        }
    //        NTQUERYSYSTEMINFORMATION NtQuerySystemInformation = (NTQUERYSYSTEMINFORMATION)GetProcAddress(hNtMod, "NtQuerySystemInformation");
    //        NTDUPLICATEOBJECT NtDuplicateObject = (NTDUPLICATEOBJECT)GetProcAddress(hNtMod, "NtDuplicateObject");
    //        NTQUERYOBJECT NtQueryObject = (NTQUERYOBJECT)GetProcAddress(hNtMod, "NtQueryObject");

    //        if (!NtQuerySystemInformation || !NtDuplicateObject || !NtQueryObject)
    //        {
    //            return queryId;
    //        }

    //        PSYSTEM_HANDLE_INFORMATION handleInfo = NULL;
    //        HANDLE processHandle;
    //        ULONG i;
    //        ULONG neededSize = 0x1000;
    //        NTSTATUS Status = 0;
    //        ULONG ReturnLength = 0;
    //        handleInfo = (PSYSTEM_HANDLE_INFORMATION)malloc(neededSize);

    //        if (!handleInfo)
    //        {
    //            return queryId;
    //        }

    //        // 一直查询 直到成功
    //        while (STATUS_INFO_LENGTH_MISMATCH == (Status = NtQuerySystemInformation(
    //                                                   SystemHandleInformation,
    //                                                   handleInfo,
    //                                                   neededSize,
    //                                                   &ReturnLength)))
    //        {
    //            if (handleInfo)
    //            {
    //                free(handleInfo);
    //                handleInfo = NULL;
    //            }
    //            neededSize = ReturnLength;
    //            handleInfo = (PSYSTEM_HANDLE_INFORMATION)malloc(neededSize);
    //            if (!handleInfo)
    //            {

    //                return queryId;
    //            }
    //        }
    //        processHandle = OpenProcess(PROCESS_DUP_HANDLE, FALSE, dwProcessID);
    //        for (i = 0; i < handleInfo->HandleCount; i++)
    //        {
    //            hmc_ProcessHandleContext handleCout;
    //            handleCout.ContextID = queryId;
    //            handleCout.handle = 0;
    //            handleCout.name = "";
    //            handleCout.type = "";
    //            handleCout.next = true;
    //            SYSTEM_HANDLE handle = handleInfo->Handles[i];
    //            if (handle.ProcessId != dwProcessID)
    //            {
    //                continue;
    //            }
    //            handleCout.handle = handle.Handle;
    //            if (processHandle)
    //            {
    //                HANDLE dupHandle = NULL;
    //                POBJECT_TYPE_INFORMATION objectTypeInfo = NULL;
    //                PVOID objectNameInfo = NULL;
    //                UNICODE_STRING objectName = {0};
    //                ULONG returnLength = 0;

    //                do
    //                {
    //                    // 句柄复制失败 就不去获取类型名
    //                    Status = NtDuplicateObject(
    //                        processHandle,
    //                        (void *)handle.Handle,
    //                        // GetCurrentProcess(),
    //                        processHandle,
    //                        &dupHandle,
    //                        0,
    //                        0,
    //                        0);
    //                    if (!NT_SUCCESS(Status))
    //                    {
    //                        break;
    //                    }

    //                    // 获取对象类型名
    //                    ULONG ObjectInformationLength = 0;
    //                    while (STATUS_INFO_LENGTH_MISMATCH == (Status = NtQueryObject(
    //                                                               dupHandle,
    //                                                               ObjectTypeInformation,
    //                                                               objectTypeInfo,
    //                                                               ObjectInformationLength,
    //                                                               &returnLength)))
    //                    {
    //                        if (objectTypeInfo)
    //                        {
    //                            free(objectTypeInfo);
    //                            objectTypeInfo = NULL;
    //                        }

    //                        ObjectInformationLength = returnLength;
    //                        objectTypeInfo = (POBJECT_TYPE_INFORMATION)malloc(ObjectInformationLength);
    //                        if (!objectTypeInfo)
    //                        {
    //                            break;
    //                        }
    //                    }

    //                    // 获取对象类型名成功
    //                    if (NT_SUCCESS(Status))
    //                    {
    //                        handleCout.type = hmc_text_util::W2A(__unicodeStringToWString(objectTypeInfo->Name));
    //                    }
    //                    if (handle.GrantedAccess == 0x0012019f)
    //                    {

    //                        break;
    //                    }

    //                    // 获取对象名
    //                    ObjectInformationLength = 0;
    //                    returnLength = 0;

    //                    if (STATUS_INFO_LENGTH_MISMATCH == NtQueryObject(
    //                                                           dupHandle,
    //                                                           ObjectNameInformation,
    //                                                           NULL,
    //                                                           0,
    //                                                           &returnLength))
    //                    {

    //                        objectNameInfo = (POBJECT_TYPE_INFORMATION)malloc(returnLength);
    //                        if (!objectNameInfo)
    //                        {
    //                            break;
    //                        }

    //                        ZeroMemory(objectNameInfo, returnLength);
    //                        Status = NtQueryObject(
    //                            dupHandle,
    //                            ObjectNameInformation,
    //                            objectNameInfo,
    //                            returnLength,
    //                            NULL);
    //                    }

    //                    // 获取对象名成功
    //                    if (NT_SUCCESS(Status) && ((PUNICODE_STRING)objectNameInfo)->Length > 0)
    //                    {

    //                        UNICODE_STRING objectName = *(PUNICODE_STRING)objectNameInfo;

    //                        handleCout.name = hmc_text_util::W2A(__unicodeStringToWString(objectName));
    //                        if (handleCout.type == "File")
    //                        {
    //                            for (size_t i = 0; i < volumeList.size(); i++)
    //                            {
    //                                hmc_usb::hmc_Volume volume = volumeList[i];
    //                                if (handleCout.name.find(volume.device) == 0)
    //                                {
    //                                    handleCout.name.replace(0, volume.device.length(), volume.path);
    //                                }
    //                            }
    //                        }
    //                    }

    //                } while (FALSE);

    //                if (dupHandle)
    //                {
    //                    CloseHandle(dupHandle);
    //                    dupHandle = NULL;
    //                }
    //                if (objectTypeInfo)
    //                {
    //                    free(objectTypeInfo);
    //                    objectTypeInfo = NULL;
    //                }
    //                if (objectNameInfo)
    //                {
    //                    free(objectNameInfo);
    //                    objectNameInfo = NULL;
    //                }
    //            }
    //            if (!handleCout.name.empty() || !handleCout.type.empty())
    //            {
    //                _enumProcessHandleList.push_back(handleCout);
    //            }
    //            Sleep(5);
    //        }

    //        free(handleInfo);
    //    }
    //    catch (char *e)
    //    {
    //        hmc_ProcessHandleContext handleCout;
    //        handleCout.ContextID = queryId;
    //        handleCout.handle = 0;
    //        handleCout.name = "";
    //        handleCout.type = "";
    //        handleCout.next = true;
    //        _enumProcessHandleList.push_back(handleCout);
    //    }

    //    return queryId;
    //}
    //
    ///**
    // * @brief 枚举指定进程所有进程的句柄信息
    // *
    // * @return long
    // */
    // long enumAllFileHandle()
    //{

    //    try
    //    {
    //
    //        vector<hmc_usb::hmc_Volume> volumeList = util_getVolumeList();

    //        HMODULE hNtMod = LoadLibraryW(L"ntdll.dll");
    //        if (!hNtMod)
    //        {
    //            return queryId;
    //        }
    //        NTQUERYSYSTEMINFORMATION NtQuerySystemInformation = (NTQUERYSYSTEMINFORMATION)GetProcAddress(hNtMod, "NtQuerySystemInformation");
    //        NTDUPLICATEOBJECT NtDuplicateObject = (NTDUPLICATEOBJECT)GetProcAddress(hNtMod, "NtDuplicateObject");
    //        NTQUERYOBJECT NtQueryObject = (NTQUERYOBJECT)GetProcAddress(hNtMod, "NtQueryObject");

    //        if (!NtQuerySystemInformation || !NtDuplicateObject || !NtQueryObject)
    //        {
    //            return queryId;
    //        }

    //        PSYSTEM_HANDLE_INFORMATION handleInfo = NULL;
    //        HANDLE processHandle;
    //        ULONG i;
    //        ULONG neededSize = 0x1000;
    //        NTSTATUS Status = 0;
    //        ULONG ReturnLength = 0;
    //        handleInfo = (PSYSTEM_HANDLE_INFORMATION)malloc(neededSize);

    //        if (!handleInfo)
    //        {
    //            return queryId;
    //        }

    //        // 一直查询 直到成功
    //        while (STATUS_INFO_LENGTH_MISMATCH == (Status = NtQuerySystemInformation(
    //                                                   SystemHandleInformation,
    //                                                   handleInfo,
    //                                                   neededSize,
    //                                                   &ReturnLength)))
    //        {
    //            if (handleInfo)
    //            {
    //                free(handleInfo);
    //                handleInfo = NULL;
    //            }
    //            neededSize = ReturnLength;
    //            handleInfo = (PSYSTEM_HANDLE_INFORMATION)malloc(neededSize);
    //            if (!handleInfo)
    //            {

    //                return queryId;
    //            }
    //        }
    //        processHandle = OpenProcess(PROCESS_DUP_HANDLE, FALSE, NULL);
    //        for (i = 0; i < handleInfo->HandleCount; i++)
    //        {
    //            hmc_ProcessHandleContext handleCout;
    //            handleCout.ContextID = queryId;
    //            handleCout.handle = 0;
    //            handleCout.name = "";
    //            handleCout.type = "";
    //            handleCout.next = true;
    //            SYSTEM_HANDLE handle = handleInfo->Handles[i];
    //

    //            handleCout.handle = handle.Handle;
    //            if (processHandle)
    //            {
    //                HANDLE dupHandle = NULL;
    //                POBJECT_TYPE_INFORMATION objectTypeInfo = NULL;
    //                PVOID objectNameInfo = NULL;
    //                UNICODE_STRING objectName = {0};
    //                ULONG returnLength = 0;

    //                do
    //                {
    //                    // 句柄复制失败 就不去获取类型名
    //                    Status = NtDuplicateObject(
    //                        processHandle,
    //                        (void *)handle.Handle,
    //                        // GetCurrentProcess(),
    //                        processHandle,
    //                        &dupHandle,
    //                        0,
    //                        0,
    //                        0);
    //                    if (!NT_SUCCESS(Status))
    //                    {
    //                        break;
    //                    }

    //                    // 获取对象类型名
    //                    ULONG ObjectInformationLength = 0;
    //                    while (STATUS_INFO_LENGTH_MISMATCH == (Status = NtQueryObject(
    //                                                               dupHandle,
    //                                                               ObjectTypeInformation,
    //                                                               objectTypeInfo,
    //                                                               ObjectInformationLength,
    //                                                               &returnLength)))
    //                    {
    //                        if (objectTypeInfo)
    //                        {
    //                            free(objectTypeInfo);
    //                            objectTypeInfo = NULL;
    //                        }

    //                        ObjectInformationLength = returnLength;
    //                        objectTypeInfo = (POBJECT_TYPE_INFORMATION)malloc(ObjectInformationLength);
    //                        if (!objectTypeInfo)
    //                        {
    //                            break;
    //                        }
    //                    }

    //                    // 获取对象类型名成功
    //                    if (NT_SUCCESS(Status))
    //                    {
    //                        handleCout.type = hmc_text_util::W2A(__unicodeStringToWString(objectTypeInfo->Name));
    //                    }
    //                    if (handle.GrantedAccess == 0x0012019f)
    //                    {

    //                        break;
    //                    }

    //                    // 获取对象名
    //                    ObjectInformationLength = 0;
    //                    returnLength = 0;

    //                    if (STATUS_INFO_LENGTH_MISMATCH == NtQueryObject(
    //                                                           dupHandle,
    //                                                           ObjectNameInformation,
    //                                                           NULL,
    //                                                           0,
    //                                                           &returnLength))
    //                    {

    //                        objectNameInfo = (POBJECT_TYPE_INFORMATION)malloc(returnLength);
    //                        if (!objectNameInfo)
    //                        {
    //                            break;
    //                        }

    //                        ZeroMemory(objectNameInfo, returnLength);
    //                        Status = NtQueryObject(
    //                            dupHandle,
    //                            ObjectNameInformation,
    //                            objectNameInfo,
    //                            returnLength,
    //                            NULL);
    //                    }

    //                    // 获取对象名成功
    //                    if (NT_SUCCESS(Status) && ((PUNICODE_STRING)objectNameInfo)->Length > 0)
    //                    {

    //                        UNICODE_STRING objectName = *(PUNICODE_STRING)objectNameInfo;

    //                        handleCout.name = hmc_text_util::W2A(__unicodeStringToWString(objectName));
    //                        if (handleCout.type == "File")
    //                        {
    //                            for (size_t i = 0; i < volumeList.size(); i++)
    //                            {
    //                                hmc_usb::hmc_Volume volume = volumeList[i];
    //                                if (handleCout.name.find(volume.device) == 0)
    //                                {
    //                                    handleCout.name.replace(0, volume.device.length(), volume.path);
    //                                }
    //                            }
    //                        }
    //                    }

    //                } while (FALSE);

    //                if (dupHandle)
    //                {
    //                    CloseHandle(dupHandle);
    //                    dupHandle = NULL;
    //                }
    //                if (objectTypeInfo)
    //                {
    //                    free(objectTypeInfo);
    //                    objectTypeInfo = NULL;
    //                }
    //                if (objectNameInfo)
    //                {
    //                    free(objectNameInfo);
    //                    objectNameInfo = NULL;
    //                }
    //            }
    //            if (!handleCout.name.empty() || !handleCout.type.empty())
    //            {
    //                _enumProcessHandleList.push_back(handleCout);
    //            }
    //        }

    //        free(handleInfo);
    //    }
    //    catch (char *e)
    //    {
    //        hmc_ProcessHandleContext handleCout;
    //        handleCout.ContextID = queryId;
    //        handleCout.handle = 0;
    //        handleCout.name = "";
    //        handleCout.type = "";
    //        handleCout.next = true;
    //        _enumProcessHandleList.push_back(handleCout);
    //    }

    //    return queryId;
    //}

    // 时间格式转换
    __int64 _hmc_FileTimeToInt64(const FILETIME &time)
    {
        ULARGE_INTEGER tt;
        tt.LowPart = time.dwLowDateTime;
        tt.HighPart = time.dwHighDateTime;
        return (tt.QuadPart);
    }

    /**
     * @brief 获取进程的内存
     *
     * @param ProcessID
     * @return DWORD
     */
    DWORD getProcessMemoryInfo(DWORD ProcessID)
    {
        PROCESS_MEMORY_COUNTERS pmc;
        DWORD memoryInK = 0;
        HANDLE hProcess = NULL;

        hProcess = OpenProcess(
            PROCESS_QUERY_INFORMATION |
                PROCESS_VM_READ,
            FALSE, ProcessID);

        if (GetProcessMemoryInfo(hProcess, &pmc, sizeof(pmc)))
        {
            // memoryInK = pmc.WorkingSetSize/1024;		//单位为k
            memoryInK = pmc.WorkingSetSize;
        }

        CloseHandle(hProcess);
        return memoryInK;
    }

    /**
     * @brief 获取CPU核心数
     *
     * @return int
     */
    int _hmc_getCPUCount()
    {
        SYSTEM_INFO system_info;
        GetSystemInfo(&system_info);
        return static_cast<int>(system_info.dwNumberOfProcessors);
    }

    /**
     * @brief 获取指定进程CPU使用率
     *
     * @param ProcessID
     * @return double
     */
    double getProcessCpuUsage(DWORD ProcessID)
    {
        static int processor_count_ = -1;     // cpu核心数
        static __int64 last_system_time_ = 0; // 上一次的系统时间
        static __int64 last_time_ = 0;        // 上一次的时间

        FILETIME now;
        FILETIME creation_time;
        FILETIME exit_time;
        FILETIME kernel_time;
        FILETIME user_time;

        __int64 system_time;
        __int64 time;

        double cpu_usage = -1;

        if (processor_count_ == -1)
        {
            processor_count_ = _hmc_getCPUCount();
        }

        GetSystemTimeAsFileTime(&now);

        HANDLE hProcess = OpenProcess(
            PROCESS_QUERY_INFORMATION |
                PROCESS_VM_READ,
            FALSE, ProcessID);

        if (!hProcess)
        {
            return -1;
        }

        if (!GetProcessTimes(hProcess, &creation_time, &exit_time, &kernel_time, &user_time))
        {
            return -1;
        }

        system_time = (_hmc_FileTimeToInt64(kernel_time) + _hmc_FileTimeToInt64(user_time)) / processor_count_; // CPU使用时间
        time = _hmc_FileTimeToInt64(now);                                                                       // 现在的时间

        last_system_time_ = system_time;
        last_time_ = time;

        CloseHandle(hProcess);

        Sleep(1000); // 睡眠1s

        hProcess = OpenProcess(
            PROCESS_QUERY_INFORMATION |
                PROCESS_VM_READ,
            FALSE, ProcessID);

        if (!hProcess)
        {
            return -1;
        }

        if (!GetProcessTimes(hProcess, &creation_time, &exit_time, &kernel_time, &user_time))
        {
            return -1;
        }

        GetSystemTimeAsFileTime(&now);
        system_time = (_hmc_FileTimeToInt64(kernel_time) + _hmc_FileTimeToInt64(user_time)) / processor_count_; // CPU使用时间
        time = _hmc_FileTimeToInt64(now);                                                                       // 现在的时间

        CloseHandle(hProcess);

        cpu_usage = ((static_cast<double>(system_time - last_system_time_)) / (static_cast<double>(time - last_time_))) * 100;
        return cpu_usage;
    }

    struct hmc_PROCESSENTRY32A
    {
        DWORD cntThreads;            // 进程中的线程数。
        DWORD cntUsage;              // 表示进程的引用计数。
        DWORD dwFlags;               // 保留字段，暂时没有使用。
        DWORD dwSize;                // 结构的大小，用于指定调用方提供的结构大小，以便 API 函数可以正确填充结构。
        LONG pcPriClassBase;         // 进程的优先级。
        string szExeFile;            // 存储进程的可执行文件名，使用字符数组表示，长度为 MAX_PATH。
        ULONG_PTR th32DefaultHeapID; // 默认堆的标识符，一般用于堆管理。
        DWORD th32ModuleID;          // 拥有进程主模块的标识符，一般用于模块管理。
        DWORD th32ParentProcessID;   // 父进程的标识符。
        DWORD th32ProcessID;         // 进程的标识符(Process ID)
    };

    void _addExeFileToPROCESSENTRY32A(hmc_PROCESSENTRY32A &copyPe32, CHAR szExeFile[MAX_PATH])
    {
        copyPe32.szExeFile.append(szExeFile);
    }
    void _addExeFileToPROCESSENTRY32A(hmc_PROCESSENTRY32A &copyPe32, WCHAR szExeFile[MAX_PATH])
    {
        copyPe32.szExeFile.append(hmc_text_util::W2A(szExeFile));
    }

    /**
     * @brief 枚举进程快照
     *
     * @param ProcessSnapshotList
     */
    void enumProcessSnapshot(vector<hmc_PROCESSENTRY32A> &ProcessSnapshotList)
    {
        PROCESSENTRY32 pe32;
        pe32.dwSize = sizeof(PROCESSENTRY32);

        // 获取进程快照
        HANDLE hSnap = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if (hSnap == INVALID_HANDLE_VALUE)
        {
            return;
        }

        // 枚举第一个进程
        if (Process32First(hSnap, &pe32))
        {
            do
            {
                hmc_PROCESSENTRY32A copyPe32;
                copyPe32.cntThreads = pe32.cntThreads;
                copyPe32.cntUsage = pe32.cntUsage;
                copyPe32.dwFlags = pe32.dwFlags;
                copyPe32.dwSize = pe32.dwSize;
                copyPe32.pcPriClassBase = pe32.pcPriClassBase;
                _addExeFileToPROCESSENTRY32A(copyPe32, pe32.szExeFile);
                copyPe32.th32DefaultHeapID = pe32.th32DefaultHeapID;
                copyPe32.th32ModuleID = pe32.th32ModuleID;
                copyPe32.th32ParentProcessID = pe32.th32ParentProcessID;
                copyPe32.th32ProcessID = pe32.th32ProcessID;
                ProcessSnapshotList.push_back(copyPe32);
            } while (Process32Next(hSnap, &pe32));
        }

        CloseHandle(hSnap);
    }

    /**
     * @brief 获取指定进程的命令行内容
     *
     * @param ProcessID
     * @return string
     */
    string getProcessCommand(DWORD ProcessID)
    {
        string commandLine;
        try
        {
            // 获取进程句柄
            HANDLE hProcess = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, ProcessID);
            if (hProcess == NULL)
            {
                return commandLine;
            }

            // 获取完整进程路径和命令行
            LPSTR lpExeName = {0};
            DWORD pathSize = 1024;
            if (QueryFullProcessImageNameA(hProcess, 0, lpExeName, &pathSize) != 0)
            {
                commandLine.append(lpExeName);
                CloseHandle(hProcess);
            }
        }
        catch (char *_)
        {
        }
        return commandLine;
    }
    /**
     * @brief 系统时间转为时间戳
     *
     * @param st
     * @return long
     */

    long SystemTimeToTimestamp(const SYSTEMTIME &st)
    {
        FILETIME ft;
        SystemTimeToFileTime(&st, &ft);

        // 将 FILETIME 转换为 64 位整数，表示 100 毫微秒为单位的时间数
        ULARGE_INTEGER uli;
        uli.LowPart = ft.dwLowDateTime;
        uli.HighPart = ft.dwHighDateTime;

        // 将时间数转换为毫秒
        return static_cast<long>(uli.QuadPart / 10000);
    }

    /**
     * @brief 获取进程启动时候的时间ms
     *
     * @param ProcessID
     * @return long
     */
    long getProcessIDTimes(DWORD ProcessID)
    {
        long result = 0;
        try
        {
            SYSTEMTIME stCreation, lstCreation;
            HANDLE process = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, ProcessID);
            if (process != NULL)
            {
                FILETIME ftCreation, ftExit, ftKernel, ftUser;
                if (GetProcessTimes(process, &ftCreation, &ftExit, &ftKernel, &ftUser))
                {
                    FileTimeToSystemTime(&ftCreation, &stCreation);
                    SystemTimeToTzSpecificLocalTime(NULL, &stCreation, &lstCreation);
                }
                CloseHandle(process);
            }

            result = SystemTimeToTimestamp(lstCreation);
        }
        catch (const std::exception &e)
        {
        }

        return result;
    }

    /**
     * @brief 判断此名称的进程是否存在
     *
     * @param BaseName
     * @return true
     * @return false
     */
    bool hasBaseNameProcess(string BaseName)
    {

        DWORD processList[1024], cbNeeded;
        if (!EnumProcesses(processList, sizeof(processList), &cbNeeded))
        {
            return false;
        }

        int numProcesses = cbNeeded / sizeof(DWORD);
        for (int i = 0; i < numProcesses; ++i)
        {
            DWORD processID = processList[i];
            HANDLE hProcess = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, processID);
            if (hProcess)
            {
                char processName[MAX_PATH];
                GetModuleBaseNameA(hProcess, NULL, processName, MAX_PATH);
                if (string(processName) == BaseName)
                {
                    return true;
                }
                CloseHandle(hProcess);
            }
        }
        return false;
    }

    /**
     * @brief 获取指定名称的进程的进程id列表
     * 
     * @param BaseName 
     * @return vector<DWORD> 
     */
    vector<DWORD> getBaseNameProcessIDList(string BaseName)
    {
        vector<ProcessEnumDetailsCont> EnumProcessTemp;
        vector<DWORD> result;
        getProcessList(EnumProcessTemp);

        for (size_t i = 0; i < EnumProcessTemp.size(); i++)
        {
            auto process = EnumProcessTemp[i];
            if (process.baseName == BaseName)
            {
                result.push_back(process.pid);
            }
        }
    }
}

#endif#ifndef HMC_IMPORT_REGISTR_H
#define HMC_IMPORT_REGISTR_H

#include <windows.h>
#include <iostream>
#include <set>
#include <string>
#include <map>
#include <vector>
#include <type_traits>
#include <ShlObj.h>
using namespace std;

#define MAX_KEY_LENGTH 255
#define MAX_VALUE_NAME 16383

#define _define_if_to_break(eq1, result)                      \
    {                                                         \
        if (!is_ok || pDataSize == 0 || is_open == 0 || !eq1) \
            return result;                                    \
    }

#define _define_is_int32bit(T)         \
    (                                  \
        is_same_v<T, int64_t> ||       \
        is_same_v<T, long long> ||     \
        is_same_v<T, int32_t> ||       \
        is_same_v<T, size_t> ||        \
        is_same_v<T, unsigned long> || \
        is_same_v<T, HWND> ||          \
        is_same_v<T, long> ||          \
        is_same_v<T, long int> ||      \
        is_same_v<T, unsigned long>)

#define _define_is_int64bit(T)         \
    (                                  \
        is_same_v<T, int64_t> ||       \
        is_same_v<T, long long> ||     \
        is_same_v<T, int32_t> ||       \
        is_same_v<T, size_t> ||        \
        is_same_v<T, unsigned long> || \
        is_same_v<T, HWND> ||          \
        is_same_v<T, long> ||          \
        is_same_v<T, long int> ||      \
        is_same_v<T, unsigned long>)

// 关闭注册表键
#define _defined_auto_free_HKey(subHKey)            \
    shared_ptr<void> close_key(nullptr, [&](void *) \
                               {\
        if (subHKey != nullptr) {\
            ::RegCloseKey(subHKey);\
            subHKey = nullptr;\
        } });

namespace hmc_registr
{
    // 目录的信息
    struct chQueryDirStat
    {
        string path; // 路径
        string hkey; // 根名称
        bool success;
        long long LastWriteTime; // 上次写入时间的时间戳
    };

    // 枚举键
    struct chQueryDirKey
    {
        vector<string> key;
        vector<string> dir;
    };

    // 遍历树结构的信息 但是不返回内容
    struct chWalkItme
    {
        DWORD size;         // 值的大小
        string vkey;        // 值的名称
        string dirPath;     // 路径文件夹
        DWORD type;         // 类型
        HKEY root;          // 根路径
        bool isDir;         // 是否是文件夹
        long long time;     // 时间戳
        vector<BYTE> value; // 数据
        bool is_value;      // 是否加入了数据
    };

    // 获取值的信息
    struct chValueStat
    {
        DWORD type;
        DWORD size;
        bool exists;
    };

    void _lib_EnumRegistrKeyQuery(HKEY hKey, vector<string> &QueryDirList, vector<string> &QueryKeyList);
    HKEY getHive(string hkey);
    string getHive(HKEY hkey);
    chQueryDirStat getRegistrDirStat(HKEY hKey, string path);
    chQueryDirKey listKey(HKEY hKey, string path);
    bool path2hKey(string path, HKEY &hKey, string &p_path);

    /**
     * @brief 对比两个注册表类型能否被隐式转换
     *
     * @param reType 实际类型
     * @param targetType 强制转换为
     * @return true
     * @return false
     */
    bool _EQ_REG_TYPE(DWORD reType, DWORD targetType)
    {
        switch (reType)
        {
        // 文本
        case REG_LINK:
        case REG_SZ:
        case REG_MULTI_SZ:
        case REG_EXPAND_SZ:
        {
            return targetType == REG_NONE || targetType == REG_LINK || targetType == REG_SZ || targetType == REG_MULTI_SZ || targetType == REG_EXPAND_SZ;
        }
        case REG_DWORD_BIG_ENDIAN:
        case REG_QWORD:
        case REG_DWORD:
        {
            return targetType == REG_NONE || targetType == REG_DWORD_BIG_ENDIAN || targetType == REG_QWORD || targetType == REG_DWORD;
        }
        case REG_BINARY:
        case REG_RESOURCE_LIST:
        case REG_RESOURCE_REQUIREMENTS_LIST:
        {
            return targetType == REG_NONE || targetType == REG_BINARY || targetType == REG_RESOURCE_LIST || targetType == REG_RESOURCE_REQUIREMENTS_LIST;
        }

        default:
            return true;
        }
    }

    // ----------------------------------------------------------------------------------------------
    /**
     * @brief 枚举注册表的key
     *
     * @param hKey
     * @param QueryDirList
     * @param QueryKeyList
     */
    void _lib_EnumRegistrKeyQuery(HKEY hKey, vector<string> &QueryDirList, vector<string> &QueryKeyList)
    {
        try
        {
            char achKey[MAX_KEY_LENGTH];    // 子键名称的缓冲区
            DWORD cbName = 0;               // 名称字符串的大小
            char achClass[MAX_PATH] = "";   // 类名缓冲区
            DWORD cchClassName = MAX_PATH;  // 类字符串的大小
            DWORD cSubKeys = 0;             // 子键数
            DWORD cbMaxSubKey = 0;          // 最长子键大小
            DWORD cchMaxClass = 0;          // 最长类字符串
            DWORD cValues = 0;              // 键值的个数
            DWORD cchMaxValue = 0;          // 最长值名
            DWORD cbMaxValueData = 0;       // 最长值数据
            DWORD cbSecurityDescriptor = 0; // 安全描述符的大小
            FILETIME ftLastWriteTime;       // 最后写入时间

            char achValue[MAX_VALUE_NAME];   // key存储
            DWORD cchValue = MAX_VALUE_NAME; // 数据序号

            DWORD index, retCode;

            // 获取类名和值计数。

            retCode = RegQueryInfoKeyA(
                hKey,                  // key句柄
                achClass,              // 类名缓冲区
                &cchClassName,         // 类字符串的大小
                NULL,                  // 无
                &cSubKeys,             // 子键数
                &cbMaxSubKey,          // 最长子键大小
                &cchMaxClass,          // 最长类字符串
                &cValues,              // 键值的个数
                &cchMaxValue,          // 最长值名
                &cbMaxValueData,       // 最长值数据
                &cbSecurityDescriptor, // 安全描述符的大小
                &ftLastWriteTime);     // 最后写入时间

            // 枚举子键，直到RegEnumKeyEx失败。
            if (cSubKeys)
            {

                for (index = 0; index < cSubKeys; index++)
                {
                    cbName = MAX_KEY_LENGTH;
                    retCode = RegEnumKeyExA(hKey, index,
                                            achKey,
                                            &cbName,
                                            NULL,
                                            NULL,
                                            NULL,
                                            &ftLastWriteTime);
                    if (retCode == ERROR_SUCCESS)
                    {
                        QueryDirList.push_back(achKey);
                    }
                }
            }
            // 枚举键值。
            if (cValues)
            {
                for (index = 0, retCode = ERROR_SUCCESS; index < cValues; index++)
                {
                    cchValue = MAX_VALUE_NAME;
                    achValue[0] = '\0';
                    retCode = RegEnumValueA(hKey, index,
                                            achValue,
                                            &cchValue,
                                            NULL,
                                            NULL,
                                            NULL,
                                            NULL);

                    if (retCode == ERROR_SUCCESS)
                    {
                        string str = string(achKey);
                        str.resize(cchValue);
                        for (size_t i = 0; i < cchValue; i++)
                            str[i] = achValue[i];

                        QueryKeyList.push_back(str);
                    }
                }
            }
        }
        catch (const exception &e)
        {
        }
    }
    // ----------------------------------------------------------------------------------------------

    /**
     * @brief 反HKEY解析 因为要和napi兼容
     *
     * @param hkey
     * @return HKEY
     */
    HKEY getHive(string hkey)
    {
        if (hkey == "HKEY_CURRENT_USER")
        {
            return HKEY_CURRENT_USER;
        }

        if (hkey == "HKEY_LOCAL_MACHINE")
        {
            return HKEY_LOCAL_MACHINE;
        }

        if (hkey == "HKEY_CLASSES_ROOT")
        {
            return HKEY_CLASSES_ROOT;
        }

        if (hkey == "HKEY_USERS")
        {
            return HKEY_USERS;
        }

        if (hkey == "HKEY_CURRENT_CONFIG")
        {
            return HKEY_CURRENT_CONFIG;
        }

        return NULL;
    }

    /**
     * @brief 反HKEY解析 因为要和napi兼容
     *
     * @param hkey
     * @return string
     */
    string getHive(HKEY hkey)
    {
        if (hkey == HKEY_CURRENT_USER)
        {
            return "HKEY_CURRENT_USER";
        }

        if (hkey == HKEY_LOCAL_MACHINE)
        {
            return "HKEY_LOCAL_MACHINE";
        }

        if (hkey == HKEY_CLASSES_ROOT)
        {
            return "HKEY_CLASSES_ROOT";
        }

        if (hkey == HKEY_USERS)
        {
            return "HKEY_USERS";
        }

        if (hkey == HKEY_CURRENT_CONFIG)
        {
            return "HKEY_CURRENT_CONFIG";
        }

        return "";
    }

    /**
     * @brief 获取目录的信息多少个 key 更新时间
     *
     * @param hKey
     * @param path
     * @return chQueryDirStat
     */
    chQueryDirStat getRegistrDirStat(HKEY hKey, string path)
    {
        chQueryDirStat queryDirStat = {
            getHive(hKey),
            path, false, 0};

        try
        {
            HKEY hTestKey;
            _defined_auto_free_HKey(hTestKey);
            if (RegOpenKeyExA(hKey, path.c_str(),
                              0,
                              KEY_ALL_ACCESS,
                              &hTestKey) == ERROR_SUCCESS)
            {
                char achKey[MAX_KEY_LENGTH];    // 子键名称的缓冲区
                DWORD cbName = 0;               // 名称字符串的大小
                char achClass[MAX_PATH] = "";   // 类名缓冲区
                DWORD cchClassName = MAX_PATH;  // 类字符串的大小
                DWORD cSubKeys = 0;             // 子键数
                DWORD cbMaxSubKey = 0;          // 最长子键大小
                DWORD cchMaxClass = 0;          // 最长类字符串
                DWORD cValues = 0;              // 键值的个数
                DWORD cchMaxValue = 0;          // 最长值名
                DWORD cbMaxValueData = 0;       // 最长值数据
                DWORD cbSecurityDescriptor = 0; // 安全描述符的大小
                FILETIME ftLastWriteTime;       // 最后写入时间

                RegQueryInfoKeyA(
                    hKey,                  // key句柄
                    achClass,              // 类名缓冲区
                    &cchClassName,         // 类字符串的大小
                    NULL,                  // 无
                    &cSubKeys,             // 子键数
                    &cbMaxSubKey,          // 最长子键大小
                    &cchMaxClass,          // 最长类字符串
                    &cValues,              // 键值的个数
                    &cchMaxValue,          // 最长值名
                    &cbMaxValueData,       // 最长值数据
                    &cbSecurityDescriptor, // 安全描述符的大小
                    &ftLastWriteTime);     // 最后写入时间

                const ULONGLONG epochOffset = 116444736000000000ULL;
                ULARGE_INTEGER uli;
                uli.LowPart = ftLastWriteTime.dwLowDateTime;
                uli.HighPart = ftLastWriteTime.dwHighDateTime;
                ULONGLONG timestamp = (uli.QuadPart - epochOffset) / 10000ULL;
                queryDirStat.success = true;
                queryDirStat.LastWriteTime = static_cast<time_t>(timestamp);
            }
        }
        catch (const exception &e)
        {
        }
        return queryDirStat;
    }

    /**
     * @brief 获取目录的信息多少个 key 更新时间
     *
     * @param path
     * @return chQueryDirStat
     */
    chQueryDirStat getRegistrDirStat(string path)
    {
        HKEY hKey;
        string p_path;
        path2hKey(path, hKey, p_path);
        return getRegistrDirStat(hKey, path);
    }

    /**
     * @brief 枚举key
     *
     * @param hKey 根HKEY
     * @param string 路径
     * @return vector<string>
     */
    chQueryDirKey listKey(HKEY hKey, string path)
    {
        chQueryDirKey queryDirKey;

        try
        {
            HKEY hTestKey;
            _defined_auto_free_HKey(hTestKey);
            if (RegOpenKeyExA(hKey, path.c_str(),
                              0,
                              KEY_ALL_ACCESS,
                              &hTestKey) == ERROR_SUCCESS)
            {
                _lib_EnumRegistrKeyQuery(hTestKey, queryDirKey.dir, queryDirKey.key);
            }
        }
        catch (const exception &e)
        {
        }

        return queryDirKey;
    }

    /**
     * @brief 枚举key
     *
     * @param path
     * @return chQueryDirKey
     */
    chQueryDirKey listKey(string path)
    {
        HKEY hKey;
        string p_path;
        path2hKey(path, hKey, p_path);
        return listKey(hKey, path);
    }

    /**
     * @brief 分割文本
     *
     * @param path
     * @return vector <string>
     */
    vector<string> _lib_splitString(string path, string sep = "\\")
    {
        vector<string> result;
        string::size_type startPos = 0;
        string::size_type endPos = path.find(sep);

        while (endPos != string::npos)
        {
            result.push_back(path.substr(startPos, endPos - startPos));
            startPos = endPos + 1;
            endPos = path.find(sep, startPos);
        }

        result.push_back(path.substr(startPos));
        return result;
    }

    /**
     * @brief 路径合并为常规路径
     *
     * @param paths
     * @return string
     */
    string _lib_joinString(vector<string> paths, string sep = "")
    {

        string newStr = string();

        for (size_t i = 0; i < paths.size(); i++)
        {
            string path = paths[i];
            newStr.append(path);
            if (i != paths.size() - 1)
                newStr.append(sep);
        }

        return newStr;
    }

    /**
     * @brief 完整路径分析出HKEY和路径
     *
     * @param path
     * @param hKey
     * @param p_path
     * @return true
     * @return false
     */
    bool path2hKey(string path, HKEY &hKey, string &p_path)
    {
        bool result = false;
        size_t pos = path.find('\\');
        if (pos != 0)
        {
            string key1 = path.substr(0, pos);
            HKEY hive = getHive(key1);
            if (hive != NULL)
            {
                hKey = hive;
            }
            p_path.clear();
            p_path.append(path.substr(pos, path.size() + 1));
            result = true;
        }
        return result;
    }

    /**
     * @brief 获取值类型与值路径
     *
     * @param hKey 根
     * @param path 路径
     * @param valueType 传址 DW
     * - 0x00000000 REG_NONE 未定义
     * - 0x00000001 REG_SZ 字符串
     * - 0x00000002 REG_EXPAND_SZ 未展开引用的字符串 例如“%PATH%”
     * - 0x00000003 REG_BINARY 二进制
     * - 0x00000004 REG_DWORD / REG_DWORD_LITTLE_ENDIAN 32 位数字
     * - 0x00000005 REG_DWORD_BIG_ENDIAN 大端格式的 32 位数字。
     * - 0x00000006 REG_LINK   指向注册表项的符号链接。
     * - 0x00000007 REG_MULTI_SZ      MS-RRP
     * - 0x00000008 REG_RESOURCE_LIST 设备驱动程序资源列表
     * - 0x0000000B REG_QWORD 64 位数字
     * @param dataSize 传址 大小
     * @return true
     * @return false
     */
    bool getValueStat(HKEY hKey, string subKey, string key, DWORD &pValueType, DWORD &pDataSize)
    {

        pValueType = 0x00000000;
        pDataSize = 0;
        HKEY subHKey;
        if (RegOpenKeyExA(hKey, subKey.c_str(), 0, KEY_ALL_ACCESS, &subHKey) == ERROR_SUCCESS)
        {
            _defined_auto_free_HKey(subHKey);

            DWORD valueType;
            DWORD dataSize = 0;

            // 第一次调用 RegQueryValueEx 获取值的大小，放入 dataSize 变量中
            if (RegQueryValueExA(subHKey, key.c_str(), nullptr, &valueType, nullptr, &dataSize) == ERROR_SUCCESS)
            {
                pValueType = valueType + 0;
                pDataSize = dataSize + 0;
                return true;
            }
            else
            {
                return false;
            }
        }
        else
        {
            return false;
        }
    }

    /**
     * @brief 获取值类型与值路径
     *
     * @param hKey 根
     * @param path 路径
     * @param valueType 传址 DW
     * - 0x00000000 REG_NONE 未定义
     * - 0x00000001 REG_SZ 字符串
     * - 0x00000002 REG_EXPAND_SZ 未展开引用的字符串 例如“%PATH%”
     * - 0x00000003 REG_BINARY 二进制
     * - 0x00000004 REG_DWORD / REG_DWORD_LITTLE_ENDIAN 32 位数字
     * - 0x00000005 REG_DWORD_BIG_ENDIAN 大端格式的 32 位数字。
     * - 0x00000006 REG_LINK   指向注册表项的符号链接。
     * - 0x00000007 REG_MULTI_SZ      MS-RRP
     * - 0x00000008 REG_RESOURCE_LIST 设备驱动程序资源列表
     * - 0x0000000B REG_QWORD 64 位数字
     * @param dataSize 传址 大小
     * @return true
     * @return false
     */
    chValueStat getValueStat(HKEY hKey, string subKey, string key)
    {
        chValueStat result = {
            0, 0, 0};
        HKEY subHKey;
        if (RegOpenKeyExA(hKey, subKey.c_str(), 0, KEY_ALL_ACCESS, &subHKey) == ERROR_SUCCESS)
        {
            _defined_auto_free_HKey(subHKey);

            DWORD valueType;
            DWORD dataSize = 0;

            // 第一次调用 RegQueryValueEx 获取值的大小，放入 dataSize 变量中
            if (RegQueryValueExA(subHKey, key.c_str(), nullptr, &valueType, nullptr, &dataSize) == ERROR_SUCCESS)
            {
                result.exists = true;
                result.size = dataSize;
                result.type = valueType;
            }
        }
        return result;
    }

    /**
     * @brief 设置内容 自识别或者自定义
     *
     * @tparam T 多种支持格式
     * @param hKey 根
     * @param subKey 目录
     * @param key 键
     * @param valueData 值
     * @param retype 指定类型 默认自识别
     * - 0x00000000 REG_NONE 未定义
     * - 0x00000001 REG_SZ 字符串
     * - 0x00000002 REG_EXPAND_SZ 未展开引用的字符串 例如“%PATH%”
     * - 0x00000003 REG_BINARY 二进制
     * - 0x00000004 REG_DWORD / REG_DWORD_LITTLE_ENDIAN 32 位数字
     * - 0x00000005 REG_DWORD_BIG_ENDIAN 大端格式的 32 位数字。
     * - 0x00000006 REG_LINK   指向注册表项的符号链接。
     * - 0x00000007 REG_MULTI_SZ      MS-RRP
     * - 0x00000008 REG_RESOURCE_LIST 设备驱动程序资源列表
     * - 0x0000000B REG_QWORD 64 位数字
     * @return true
     * @return false
     */
    template <typename T>
    bool setRegistrValue(HKEY hKey, string subKey, string key, const T &valueData, DWORD retype = 0)
    {
        bool result = false;

        static_assert(
            is_integral<T>::value ||
                is_same_v<T, string> ||
                is_same_v<T, vector<unsigned char>>,
            "Unsupported type preset escape (不支持的类型预设转义)");

        try
        {
            HKEY hSubKey;
            DWORD dwDisposition;
            DWORD is_open = ::RegCreateKeyExA(hKey, subKey.c_str(), 0, nullptr, REG_OPTION_NON_VOLATILE, KEY_WRITE, nullptr, &hSubKey, &dwDisposition);
            _defined_auto_free_HKey(hSubKey);
            // 尝试创建或者打开父键
            if (is_open == ERROR_SUCCESS)
            {

                // 数字小于 64 写入DWORD
                if constexpr (_define_is_int32bit(T))
                {
                    DWORD newData = ((DWORD)valueData) + 0;
                    is_open = ::RegSetValueExA(hSubKey, key.c_str(), 0, (retype != 0 ? retype : REG_DWORD), reinterpret_cast<const BYTE *>(newData), sizeof(DWORD));
                }
                // 写入64位数字
                else if constexpr (_define_is_int64bit(T))
                {
                    long long newData = ((long long)valueData) + 0;
                    is_open = ::RegSetValueExA(hSubKey, key.c_str(), 0, (retype != 0 ? retype : REG_QWORD), reinterpret_cast<const BYTE *>(newData), sizeof(int64_t));
                }
                // 写入文本
                else if constexpr (is_same_v<T, string>)
                {
                    is_open = ::RegSetValueExA(hSubKey, key.c_str(), 0, (retype != 0 ? retype : REG_SZ), reinterpret_cast<const BYTE *>(valueData.c_str()), static_cast<DWORD>(string(valueData).size() * sizeof(char)));
                }
                // 写入二进制
                else if constexpr (is_same_v<T, vector<BYTE>>)
                {
                    is_open = ::RegSetValueExA(hSubKey, key.c_str(), 0, (retype != 0 ? retype : REG_BINARY), reinterpret_cast<const BYTE *>(valueData.data()), static_cast<DWORD>(valueData.size() * sizeof(char)));
                }

                else
                {
                    is_open = 999;
                }

                result = is_open == ERROR_SUCCESS;
                return result;
            }
        }
        catch (const exception &e)
        {
            return result;
        }

        return result;
    }

    /**
     * @brief 获取指定的值
     * ? string  ->  getRegistrValue <string> (hKey, subKey,key);
     * ? bin  ->  getRegistrValue< vector<BYTE> >(hKey, subKey,key);
     * ? int  -> getRegistrValue <int> (hKey, subKey,key);
     * ? REG_RESOURCE_REQUIREMENTS_LIST  ->  getRegistrValue< vector<BYTE> >(hKey, subKey,key);
     * ? REG_RESOURCE_LIST  ->  getRegistrValue< vector<BYTE> >(hKey, subKey,key);
     * - 读取为未转义变量
     * ? string -> getRegistrValue <string> (hKey, 'subKey','key',REG_EXPAND_SZ);
     * -
     * @param hKey
     * @param subKey
     * @param key
     * @return T <int , int8_t , int32_t , >
     */
    template <typename T>
    T getRegistrValue(HKEY hKey, string subKey, string key, DWORD retype = 0)
    {
        T result_default = {0};
        static_assert(
            is_integral<T>::value ||
                is_same_v<T, string> ||
                is_same_v<T, vector<unsigned char>>,
            "Unsupported type preset escape (不支持的类型预设转义)");

        DWORD is_open;
        DWORD pValueType;
        DWORD pDataSize;
        bool is_ok = false;
        HKEY subHKey = 0;
        //hmc_EnableShutDownPriv();
        hmc_registr::chValueStat data = getValueStat(hKey, subKey, key);
        is_ok = data.exists;
        pDataSize = data.size;
        pValueType = data.type;

        is_open = ::RegOpenKeyExA(hKey, subKey.c_str(), 0, KEY_ALL_ACCESS, &subHKey) == ERROR_SUCCESS;
        // 智能关闭指针
        _defined_auto_free_HKey(subHKey);

        // 处理32位数字
        if constexpr (_define_is_int32bit(T))
        {
            int32_t result = 0;

            // 条件隐式转换匹配 不符合直接跳出
            _define_if_to_break(_EQ_REG_TYPE(pValueType, retype), result);

            DWORD type = retype == 0 ? REG_DWORD : retype;

            long long value_data = 0;
            if (type == 0)
                type = REG_DWORD;

            if (::RegQueryValueExA(subHKey, key.c_str(), 0, &type, reinterpret_cast<BYTE *>(&value_data), &pDataSize) == ERROR_SUCCESS)
            {
                result = (int32_t)value_data;
                return result;
            }
            return result;
        }
        // 处理64位数字
        else if constexpr (_define_is_int64bit(T))
        {
            long long result = 0;

            // 条件隐式转换匹配 不符合直接跳出
            _define_if_to_break(_EQ_REG_TYPE(pValueType, retype), result);
            DWORD type = retype == 0 ? REG_QWORD : retype;

            if (::RegQueryValueExA(subHKey, key.c_str(), 0, &type, reinterpret_cast<BYTE *>(&result), &pDataSize) == ERROR_SUCCESS)
            {
                return result;
            }
        }
        // 处理文本型
        else if constexpr (is_same_v<T, string>)
        {
            string result = string();

            // 条件隐式转换匹配 不符合直接跳出
            _define_if_to_break(_EQ_REG_TYPE(pValueType, retype), result);

            DWORD type = retype == 0 ? pValueType : retype;

            vector<BYTE> value_data(pDataSize);

            if (::RegQueryValueExA(subHKey, key.c_str(), 0, &type, reinterpret_cast<BYTE *>(value_data.data()), &pDataSize) == ERROR_SUCCESS)
            {
                result.resize(pDataSize);
                for (size_t i = 0; i < pDataSize; i++)
                {
                    result[i] = value_data[i];
                }
            }

            return result;
        }
        // 处理二进制
        else if constexpr (is_same_v<T, vector<BYTE>>)
        {
            vector<BYTE> value_data(pDataSize);

            _define_if_to_break(_EQ_REG_TYPE(pValueType, retype), value_data);

            DWORD type = retype == 0 ? REG_BINARY : retype;

            ::RegQueryValueExA(subHKey, key.c_str(), 0, &type, reinterpret_cast<BYTE *>(value_data.data()), &pDataSize);

            return value_data;
        }

        return result_default;
    }

    /**
     * @brief 获取单条数据并返回类型与数据
     *
     * @param hKey
     * @param subKey
     * @param key
     * @return chWalkValueItmeCout
     */
    chWalkItme getRegistrAnyValue(HKEY hKey, string subKey, string key)
    {
        chWalkItme walkValueItmeCout;
        walkValueItmeCout.dirPath = subKey;
        walkValueItmeCout.vkey = key;
        walkValueItmeCout.isDir = false;
        walkValueItmeCout.size = 0;
        walkValueItmeCout.time = 0;
        walkValueItmeCout.type = 0;
        walkValueItmeCout.root = hKey;
        walkValueItmeCout.value = {0};
        walkValueItmeCout.is_value = false;
        HKEY open_hkey = nullptr;
        _defined_auto_free_HKey(open_hkey);

        DWORD is_open;
        DWORD pValueType;
        DWORD pDataSize;
        bool is_ok = false;
        is_ok = getValueStat(hKey, subKey, key, pValueType, pDataSize);

        is_open = ::RegOpenKeyExA(hKey, subKey.c_str(), 0, KEY_ALL_ACCESS, &open_hkey) == ERROR_SUCCESS;
        vector<BYTE> value_data(pDataSize);

        if (!is_ok || !is_open)
            return walkValueItmeCout;
        if (ERROR_SUCCESS == ::RegQueryValueExA(open_hkey, key.c_str(), 0, REG_NONE, reinterpret_cast<BYTE *>(value_data.data()), &pDataSize))
        {
            walkValueItmeCout.value.clear();
            walkValueItmeCout.value.resize(value_data.size());
            walkValueItmeCout.size = value_data.size();
            walkValueItmeCout.type = pValueType + 0;
            walkValueItmeCout.is_value = true;
            for (size_t i = 0; i < value_data.size(); i++)
            {
                walkValueItmeCout.value[i] = value_data[i];
            }
        };
        value_data.clear();
        return walkValueItmeCout;
    }

    /**
     * @brief 判断是否存在此key
     *
     * @param hKey
     * @param subKey
     * @param key
     * @return true
     * @return false
     */
    bool hasRegistrKey(HKEY hKey, string subKey, string key)
    {
        bool result = false;

        DWORD pValueType;
        DWORD pDataSize;
        result = getValueStat(hKey, subKey, key, pValueType, pDataSize);
        return result;
    }

    /**
     * @brief 判断是否存在此key
     *
     * @param hKey
     * @param subKey
     * @param key
     * @return true
     * @return false
     */
    bool hasRegistrDir(HKEY hKey, string subKey)
    {
        bool result = false;
        HKEY hTestKey;
        DWORD openResult = RegOpenKeyExA(hKey, subKey.c_str(), 0, KEY_ALL_ACCESS, &hTestKey);
        _defined_auto_free_HKey(hTestKey);

        switch (openResult)
        {
        case ERROR_SUCCESS:
            return true;
        case ERROR_FILE_NOT_FOUND:
            return false;
        }
        return result;
    }

    /**
     * @brief 删除指定的值
     *
     * @param hKey
     * @param subKey
     * @param key
     * @return true
     * @return false
     */
    bool removeRegistrValue(HKEY hKey, string subKey, string key)
    {
        HKEY open_hkey = nullptr;
        _defined_auto_free_HKey(open_hkey);
        if (ERROR_SUCCESS == ::RegOpenKeyExA(hKey, subKey.c_str(), 0, KEY_ALL_ACCESS, &open_hkey))
        {
            return ::RegDeleteValueA(open_hkey, key.c_str()) == ERROR_SUCCESS;
        }

        return false;
    }

    /**
     * @brief 删除注册表值树
     *
     * @param hKey
     * @param subKey
     * @return true
     * @return false
     */
    bool removeRegistrTree(HKEY hKey, string subKey, string DirName)
    {
        HKEY open_hkey = nullptr;
        _defined_auto_free_HKey(open_hkey);
        if (ERROR_SUCCESS == ::RegOpenKeyExA(hKey, subKey.c_str(), 0, KEY_ALL_ACCESS, &open_hkey))
        {
            RegDeleteTreeA(open_hkey, DirName.c_str());
        }
        return hasRegistrDir(hKey, subKey + "\\" + DirName) == false;
    }

    /**
     * @brief 删除指定文件夹
     *
     * @param hKey
     * @param subKey
     * @param tree 是否删除所有
     * @return true
     * @return false
     */
    bool removeRegistrDir(HKEY hKey, string keyPath, bool tree = false)
    {
        HKEY open_hkey = nullptr;

        if (tree)
        {
            vector<string> keys = _lib_splitString(keyPath);
            if (keys.size() == 0)
                return false;
            string dirName = keys[keys.size() - 1];
            keys.pop_back();
            return removeRegistrTree(hKey, _lib_joinString(keys, "\\"), dirName);
        }

        LONG result = ::RegOpenKeyExA(HKEY_CURRENT_USER, keyPath.c_str(), 0, KEY_ALL_ACCESS, &open_hkey);
        _defined_auto_free_HKey(open_hkey);

        if (result == ERROR_SUCCESS)
        {
            result = ::RegDeleteKeyExA(HKEY_CURRENT_USER, keyPath.c_str(), KEY_WOW64_64KEY, 0);
            return (result == ERROR_SUCCESS) || (::RegDeleteKeyExA(HKEY_CURRENT_USER, keyPath.c_str(), KEY_WOW64_32KEY, 0) == ERROR_SUCCESS);
        }
        else
        {
            return false;
        }
        return false;
    }

    /**
     * @brief 创建文件夹
     *
     * @param hKey
     * @param keyPath
     * @return true
     * @return false
     */
    bool createRegistrDir(HKEY hKey, string keyPath)
    {
        HKEY open_hkey = nullptr;
        DWORD dwOptions = REG_OPTION_NON_VOLATILE;
        DWORD dwDisposition;
        long resulte = RegCreateKeyExA(hKey, keyPath.c_str(), 0, NULL,
                                       dwOptions, KEY_WRITE, NULL, &open_hkey, &dwDisposition);
        if (resulte != ERROR_SUCCESS)
        {
            return false;
        }
        else
        {
            switch (dwDisposition)
            {
            case REG_OPENED_EXISTING_KEY:
                return true;
            case REG_CREATED_NEW_KEY:
                return true;
            }
        }
    }

    /**
     * @brief 复制指定的目录到指定目录
     *
     * @param hKey
     * @param sourcePath
     * @param toPath
     * @return true
     * @return false
     */
    bool copyRegistrDir(HKEY hKey, string sourcePath, string toPath)
    {
        HKEY sourceHKey = nullptr;
        HKEY toHKey = nullptr;
        DWORD dwDisposition;

        _defined_auto_free_HKey(sourceHKey);
        shared_ptr<void> close_toHKey(nullptr, [&](void *)
                                      {
        if (toHKey != nullptr) {
            ::RegCloseKey(toHKey);
            toHKey = nullptr;
        } });

        if (ERROR_SUCCESS != ::RegOpenKeyExA(hKey, sourcePath.c_str(), 0, KEY_ALL_ACCESS, &sourceHKey))
        {
            return false;
        }

        if (ERROR_SUCCESS != ::RegCreateKeyExA(hKey, toPath.c_str(), 0, nullptr, REG_OPTION_NON_VOLATILE, KEY_ALL_ACCESS, nullptr, &toHKey, &dwDisposition))
        {
            return false;
        }

        return (ERROR_SUCCESS == ::RegCopyTreeA(sourceHKey, nullptr, toHKey));
    }

    /**
     * @brief 获取目录表中的键
     *
     * @param hKey   根目录
     * @param keyPath  获取目录路径
     * @param filterType 过滤类型
     * - all  REG_NONE
     * - string  REG_SZ|REG_EXPAND_SZ|REG_LINK
     * - number  REG_DWORD|REG_QWORD|REG_DWORD_BIG_ENDIAN
     * - bin     REG_BINARY|REG_DWORD_LITTLE_ENDIAN|REG_DWORD_BIG_ENDIAN|REG_RESOURCE_LIST|REG_RESOURCE_REQUIREMENTS_LIST|REG_FULL_RESOURCE_DESCRIPTOR
     * @return true
     * @return false
     */
    template <typename... Args>
    vector<chWalkItme> walkRegistrDir(HKEY hKey, string keyPath, bool addValue = false, Args... typeFlag)
    {

        vector<chWalkItme> result;
        try
        {
            HKEY hTestKey;
            _defined_auto_free_HKey(hTestKey);
            vector<string> keylist;
            vector<string> dirlist;
            set<DWORD> typeFlagList;
            DWORD temp[] = {typeFlag...};
            for (size_t i = 0; i < sizeof(temp) / sizeof(temp[0]); i++)
                typeFlagList.insert((DWORD)temp[i]);

            if (RegOpenKeyExA(hKey, keyPath.c_str(),
                              0,
                              KEY_ALL_ACCESS,
                              &hTestKey) == ERROR_SUCCESS)
            {
                _lib_EnumRegistrKeyQuery(hTestKey, dirlist, keylist);
            }

            {
                // 枚举并获取信息
                for (size_t i = 0; i < keylist.size(); i++)
                {
                    chWalkItme walkItemCout;
                    string key = keylist[i];
                    walkItemCout.vkey = key;
                    walkItemCout.dirPath = keyPath;
                    walkItemCout.time = 0;
                    walkItemCout.isDir = false;
                    walkItemCout.root = hKey;
                    walkItemCout.value = {0};

                    DWORD pValueType;
                    DWORD pDataSize;
                    getValueStat(hKey, keyPath, key, pValueType, pDataSize);
                    if (typeFlagList.find(REG_NONE) != typeFlagList.end() || typeFlagList.find(pValueType) != typeFlagList.end())
                    {

                        walkItemCout.size = pDataSize;
                        walkItemCout.type = pValueType;
                        if (addValue)
                        {
                            walkItemCout.value = getRegistrAnyValue(hKey, keyPath, key).value;
                        }

                        result.push_back(walkItemCout);
                    }
                }

                // 枚举并获取信息
                if (typeFlagList.find(REG_NONE) != typeFlagList.end())
                    for (size_t i = 0; i < dirlist.size(); i++)
                    {
                        chWalkItme walkItemCout;
                        string dir = dirlist[i];
                        chQueryDirStat chqlist = getRegistrDirStat(hKey, string(keyPath).append("\\").append(dir));
                        walkItemCout.vkey = dir;
                        walkItemCout.dirPath = keyPath;
                        walkItemCout.time = chqlist.LastWriteTime;
                        walkItemCout.isDir = true;
                        walkItemCout.root = hKey;
                        walkItemCout.size = 0;
                        walkItemCout.type = 0;
                        walkItemCout.value = {0};
                        result.push_back(walkItemCout);
                    }
            }
        }
        catch (const exception &e)
        {
        }
        return result;
    }

};

#endif#define _CRT_SECURE_NO_WARNINGS
#include <string>
#include <windows.h>
#include <codecvt>
#include <regex>
#include <vector>
#include <algorithm>
#include <iterator>
#include <cstddef> // For byte (C++17 or later)

using namespace std;
#define MALLOC(variable) HeapAlloc(GetProcessHeap(), 0, (variable))
#define FREE(variable) HeapFree(GetProcessHeap(), 0, (variable))
#define HMC_CHECK_CATCH catch (char *err){};

#define HMC_VirtualAlloc(Type, leng) (Type) VirtualAlloc((LPVOID)NULL, (DWORD)(leng), MEM_COMMIT, PAGE_READWRITE);
#define HMC_VirtualFree(Virtua) \
    if (Virtua != NULL)         \
        VirtualFree(Virtua, 0, MEM_RELEASE);

// 文本工具
namespace hmc_text_util
{
    string W2A(const wstring &pwText);
    string A2U8(const string &pText);
    string UTF8ToGBK(string u8str);
    bool haslongStr(string Value);
    string W2U8(wstring pwText);
    wstring A2W(const string &paText);
    wstring U82W(const string &pszText);
    string U82A(const string &pText);
    bool haslonglongStr(string Value);
    bool hasIntStr(string Value);
    string base64_encode(const string &input);
    string A2B64A(const string &paText);
    string W2B64A(const wstring &paText);
    wstring W2B64W(const wstring &paText);
    wstring A2B64W(const string &paText);
    const char* A2U8P(const string& pText);
    const char* W2U8P(wstring pwText);


    /**
     * @brief 将A转为bs64
     *
     * @param input
     * @return string
     */
    string base64_encode(const string &input)
    {
        static const string base64_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

        string encoded;
        try
        {
            int i = 0;
            int j = 0;
            unsigned char array3[3];
            unsigned char array4[4];

            for (char c : input)
            {
                array3[i++] = c;
                if (i == 3)
                {
                    array4[0] = (array3[0] & 0xfc) >> 2;
                    array4[1] = ((array3[0] & 0x03) << 4) + ((array3[1] & 0xf0) >> 4);
                    array4[2] = ((array3[1] & 0x0f) << 2) + ((array3[2] & 0xc0) >> 6);
                    array4[3] = array3[2] & 0x3f;

                    for (int k = 0; k < 4; k++)
                        encoded += base64_chars[array4[k]];

                    i = 0;
                }
            }

            if (i != 0)
            {
                for (int k = i; k < 3; k++)
                    array3[k] = '\0';

                array4[0] = (array3[0] & 0xfc) >> 2;
                array4[1] = ((array3[0] & 0x03) << 4) + ((array3[1] & 0xf0) >> 4);
                array4[2] = ((array3[1] & 0x0f) << 2) + ((array3[2] & 0xc0) >> 6);
                array4[3] = array3[2] & 0x3f;

                for (int k = 0; k < i + 1; k++)
                    encoded += base64_chars[array4[k]];

                while (i++ < 3)
                    encoded += '=';
            }
        }
        catch (char *_)
        {
        }

        return encoded;
    }

    //  WIDE to ANSI
    string W2A(const wstring &pwText)
    {
        string strResult = string();
        try
        {
            if (pwText.empty())
                return strResult;

            int pszATextLen = WideCharToMultiByte(CP_ACP, 0, pwText.c_str(), -1, NULL, 0, NULL, NULL);
            char *pAChar = new (nothrow) char[pszATextLen];
            if (pAChar == NULL)
            {
                return strResult;
            }

            ZeroMemory(pAChar, pszATextLen + 1);
            WideCharToMultiByte(CP_ACP, 0, pwText.c_str(), -1, pAChar, pszATextLen, NULL, NULL);

            strResult.append(pAChar);
            // FreeEnvironmentStringsA(pAChar);
        }
        catch (char *_)
        {
        }

        return strResult;
    }

    /**
     * @brief A字符转为base64字符A
     *
     * @param paText
     * @return string
     */
    string A2B64A(const string &paText)
    {
        string result = string();
        try
        {
            result.append(base64_encode(A2U8(paText)));
        }
        catch (char *_)
        {
        }

        return result;
    }

    /**
     * @brief W字符转为base64字符A
     *
     * @param paText
     * @return string
     */
    string W2B64A(const wstring &paText)
    {
        string result = string();
        try
        {
            result.append(base64_encode(W2U8(paText)));
        }
        catch (char *_)
        {
        }

        return result;
    }

    /**
     * @brief A字符转为base64字符W
     *
     * @param paText
     * @return string
     */
    wstring A2B64W(const string &paText)
    {
        wstring result = wstring();
        try
        {
            result.append(A2W(base64_encode(A2U8(paText))));
        }
        catch (char *_)
        {
        }

        return result;
    }

    /**
     * @brief W字符转为base64字符W
     *
     * @param paText
     * @return string
     */
    wstring W2B64W(const wstring &paText)
    {
        wstring result = wstring();
        try
        {
            result.append(A2W(base64_encode(W2U8(paText))));
        }
        catch (char *_)
        {
        }

        return result;
    }

    //  ANSI to WIDE
    wstring A2W(const string &paText)
    {
        wstring strResult = wstring();
        try
        {

            if (paText.empty())
                return strResult;

            int pszWTextLen = MultiByteToWideChar(CP_ACP, 0, paText.c_str(), -1, NULL, 0);
            wchar_t *pWideChar = new (nothrow) wchar_t[pszWTextLen];

            if (pWideChar == NULL)
                return strResult;

            ZeroMemory(pWideChar, pszWTextLen + 1);
            MultiByteToWideChar(CP_ACP, 0, paText.c_str(), -1, pWideChar, pszWTextLen);

            strResult.append(pWideChar);
            // delete[] pWideChar;
            // pWideChar = NULL;
        }
        catch (char *_)
        {
        }

        return strResult;
    }

    // 宽字符字符串转UTF-8字符串
    string W2U8(wstring pwText)
    {
        string strResult = string();
        try
        {
            if (pwText.empty())
                return strResult;

            int pszATextLen = WideCharToMultiByte(CP_UTF8, 0, pwText.c_str(), -1, NULL, 0, NULL, NULL);
            char *pUTF8 = new char[pszATextLen + 1];
            if (pUTF8 == NULL)
                return strResult;
            ZeroMemory(pUTF8, pszATextLen + 1);
            WideCharToMultiByte(CP_UTF8, 0, pwText.c_str(), -1, pUTF8, pszATextLen, NULL, NULL);
            strResult.append(pUTF8);

            // delete[] pUTF8;
            // pUTF8 = NULL;
        }
        catch (char *_)
        {
        }
        return strResult;
    }

   const char* W2U8P(wstring pwText)
    {
        string result = W2U8(pwText);

        char* pUTF8 = new char[result.size() + 1];

        for (size_t i = 0; i < result.size(); i++)
        {
            char data = result[i];

            if (data == *"\0") {
                pUTF8[i] = data;
                return pUTF8;
            }

            pUTF8[i] = data;
            if (i > result.size()) {
                char end_char = *"\0";
                pUTF8[result.size()] = end_char;
            }
        }

        return pUTF8;
    }
    
    // UTF-8字符串转宽字符
    wstring U82W(const string &pszText)
    {
        wstring strResult = wstring();
        try
        {
            if (pszText.size() == 0)
                return strResult;

            int pszWTextLen = MultiByteToWideChar(CP_UTF8, 0, pszText.c_str(), -1, NULL, NULL);
            wchar_t *pszWText = new wchar_t[pszWTextLen + 1];

            if (pszWText == NULL)
                return strResult;
            ZeroMemory(pszWText, pszWTextLen + 1);
            MultiByteToWideChar(CP_UTF8, 0, pszText.c_str(), -1, pszWText, pszWTextLen);
            strResult.append(pszWText);
            // delete[] pszWText;
            // pszWText = NULL;
        }
        catch (char *_)
        {
        }

        return strResult;
    }

    // 多字节字符串转UTF-8字符串
    string A2U8(const string &pText)
    {
        return W2U8(A2W(pText));
    }

    const char* A2U8P(const string& pText)
    {
        string result = A2U8(pText);

        char* pUTF8 = new char[result.size() + 1];

        for (size_t i = 0; i < result.size(); i++)
        {
            char data = result[i];

            if (data == *"\0") {
                pUTF8[i] = data;
                return pUTF8;
            }

            pUTF8[i] = data;
            if (i > result.size()) {
                char end_char = *"\0";
                pUTF8[result.size()] = end_char;
            }
        }
     
        return pUTF8;
    }

    // UTF-8字符串转多字节字符串
    string U82A(const string &pText)
    {
        return W2A(U82W(pText));
    }

    /**
     * @brief UTF-8 to Base64 encoding ANSI
     *
     * @param pText
     * @return string
     */
    string U82B64A(const string &pText)
    {
        return base64_encode(pText);
    }

    /**
     * @brief UTF-8 to Base64 encoding WIDE
     *
     * @param pText
     * @return string
     */
    wstring U82B64W(const string &pText)
    {
        return A2W(base64_encode(pText));
    }

    // UFT8 字符转为GBK(中文)
    string UTF8ToGBK(string u8str)
    {
        string Result;
        try
        {

            TCHAR *pTempTstr;
            WCHAR *pTempwstr;

            int strSizeTempVar = MultiByteToWideChar(CP_UTF8, 0, u8str.c_str(), -1, NULL, 0);
            pTempwstr = new WCHAR[strSizeTempVar + 1];

            MultiByteToWideChar(CP_UTF8, 0, u8str.c_str(), -1, pTempwstr, strSizeTempVar);
            strSizeTempVar = WideCharToMultiByte(CP_ACP, 0, pTempwstr, -1, NULL, 0, NULL, NULL);

            pTempTstr = new TCHAR[strSizeTempVar + 1];

            WideCharToMultiByte(CP_ACP, 0, pTempwstr, -1, (LPSTR)pTempTstr, strSizeTempVar, NULL, NULL);
            Result = (char *)pTempTstr;
            // delete[] pTempTstr;
            // delete[] pTempwstr;
        }
        catch (char *_)
        {
        }
        return Result;
    }

    // 文本中是否有数字 并且是否是安全的 int32
    bool hasIntStr(string Value)
    {
        bool Result = false;

        if (Value.empty())
            return Result;
        try
        {
            int n = stoi(Value);
            Result = true;
        }
        catch (const exception &e)
        {
            return Result;
        }

        return Result;
    }

    // 文本中是否有数字 并且是否是安全的 long
    bool haslongStr(string Value)
    {
        bool Result = false;
        if (Value.empty())
            return Result;
        try
        {
            long n = stol(Value);
            Result = true;
        }
        catch (const exception &e)
        {
            return Result;
        }

        return Result;
    }

    // 文本中是否有数字 并且是否是安全的 long long
    bool haslonglongStr(string Value)
    {
        bool Result = false;
        if (Value.empty())
            return Result;
        try
        {
            long long n = stoll(Value);
            Result = true;
        }
        catch (const exception &e)
        {
            return Result;
        }

        return Result;
    }

#ifdef defined(_MFC_VER)

    CString UTF8ToCString(string utf8str)
    {
        // 计算所需空间的大小
        int nLen = MultiByteToWideChar(CP_UTF8, NULL,
                                       utf8str.data(), utf8str.size(), NULL, 0);

        // 转换为Unicode
        wstring wbuffer;
        wbuffer.resize(nLen);
        MultiByteToWideChar(CP_UTF8, NULL, utf8str.data(), utf8str.size(),
                            (LPWSTR)(wbuffer.data()), wbuffer.length());

#ifdef UNICODE
        // 如果是Unicode编码，直接返回Unicode字符串
        return (CString(wbuffer.data(), wbuffer.length()));
#else
        /*
         * 转换为ANSI编码
         * 得到转换后长度
         */
        nLen = WideCharToMultiByte(CP_ACP, 0,
                                   wbuffer.data(), wbuffer.length(), NULL, 0, NULL, NULL);

        string ansistr;
        ansistr.resize(nLen);

        // 把Unicode字符串转成ANSI编码字符串
        WideCharToMultiByte(CP_ACP, 0, (LPWSTR)(wbuffer.data()), wbuffer.length(),
                            (LPSTR)(ansistr.data()), ansistr.size(), NULL, NULL);
        return (CString(ansistr.data(), ansistr.length()));
#endif
    }

    string CStringToUTF8(CString strValue)
    {
        wstring wbuffer;
#ifdef _UNICODE
        // 如果是Unicode编码，直接获取Unicode字符串
        wbuffer.assign(strValue.GetString(), strValue.GetLength());
#else
        /*
         * 转换ANSI编码到Unicode编码
         * 获取转换后长度
         */
        int length = MultiByteToWideChar(CP_ACP, MB_ERR_INVALID_CHARS, (LPCTSTR)strValue, -1, NULL, 0);
        wbuffer.resize(length);
        /* 转换 */
        MultiByteToWideChar(CP_ACP, 0, (LPCTSTR)strValue, -1, (LPWSTR)(wbuffer.data()), wbuffer.length());
#endif

        /* 获取转换后长度 */
        int utf8Length = WideCharToMultiByte(CP_UTF8, 0, wbuffer.data(), wbuffer.size(), NULL, 0, NULL, NULL);
        /* 获取转换后内容 */
        string utf8Buffer;
        utf8Buffer.resize(utf8Length);

        WideCharToMultiByte(CP_UTF8, 0, wbuffer.data(), -1, (LPSTR)(utf8Buffer.data()), utf8Length, NULL, NULL);
        return (utf8Buffer);
    }
#endif

}
#ifndef defined(HMC_IMPORT_TRAY_H)
#define HMC_IMPORT_TRAY_H

#include <iostream>
#include <string>
#include <windows.h>
#include <thread>
#include <unordered_map>
#include <vector>
#include <iostream>
#include <psapi.h>
#include <functional>

using namespace std;

namespace hmc_tray

{

#define WM_SYSICON (WM_USER + 1)
    static NOTIFYICONDATAA nid;
#define HMC_CHECK_CATCH catch (char *err){};
#define HMC_EMIT_ENVINFO(assert_IF)                            \
    for (auto &&onEvent : ON_EVENT)                            \
    {                                                          \
        if (onEvent.first == assert_IF)                        \
        {                                                      \
            for (size_t i = 0; i < onEvent.second.size(); i++) \
            {                                                  \
                onEvent.second[i]();                           \
            }                                                  \
        }                                                      \
    }                                                          \
                                                               \
    for (auto &&onEvent : ONCE_EVENT)                          \
    {                                                          \
        if (onEvent.first == assert_IF)                        \
        {                                                      \
            for (size_t i = 0; i < onEvent.second.size(); i++) \
            {                                                  \
                onEvent.second[i]();                           \
            }                                                  \
            ONCE_EVENT.clear();                                \
        }                                                      \
    };

#define HMC_EMIT_ENVINFO2(assert_IF, assert_IF02)                       \
    for (auto &&onEvent : ON_EVENT)                                     \
    {                                                                   \
        if (onEvent.first == assert_IF || onEvent.first == assert_IF02) \
        {                                                               \
            for (size_t i = 0; i < onEvent.second.size(); i++)          \
            {                                                           \
                onEvent.second[i]();                                    \
            }                                                           \
        }                                                               \
    }                                                                   \
                                                                        \
    for (auto &&onEvent : ONCE_EVENT)                                   \
    {                                                                   \
        if (onEvent.first == assert_IF || onEvent.first == assert_IF02) \
        {                                                               \
            for (size_t i = 0; i < onEvent.second.size(); i++)          \
            {                                                           \
                onEvent.second[i]();                                    \
            }                                                           \
            ONCE_EVENT.clear();                                         \
        }                                                               \
    };

    HWND PuppetTrayWindowHwnd;

    namespace chMenuType
    {
        // 按钮类型
        typedef enum
        {
            check = MF_CHECKED,
            separator = MF_SEPARATOR,
            // radio = MFT_RADIOCHECK,
            menu = 60409,
        } chMenuType;
    }

    string __hmc_trayInfo = "";
    string __hmc_title = "hmc-puppet-tray-window";
    string __hmc_className = "hmc-puppet-tray-window-class";
    bool ___Start_hmc_Tray = false;
    long long __tray_next_id = 0;
    // 按钮
    struct chMenuItem
    {
        // 显示名称
        string name;
        // js的文本id
        string id;
        // 自动分配的id 留 0
        int messageID;
        // 按钮类型
        chMenuType::chMenuType type;
        // 禁用
        bool disable;
        // 子按钮列表
        vector<int> menuList;
        // 主按钮 如果是子按钮 不会被显示出来  只有主按钮中包含此键的id才会被包含到子按钮里显示
        bool rootMenu;
        // 是否显示此按钮
        bool show;
        // 是否选定
        bool select;
        chMenuItem() : messageID(0)
        {
            messageID = -1;
            select = false;
            menuList = {};
            disable = false;
            rootMenu = true;
            show = true;
            name = "unknown";
            id = "unknown" + to_string(messageID);
            type = chMenuType::menu;
        }
    };
    std::vector<chMenuItem> __MenuList;
    std::thread *__tray_worker;
    int __openPuppetTrayWindow();
    namespace _HMC__EVENT
    {
        unordered_map<string, vector<function<void()>>> ON_EVENT;
        unordered_map<string, vector<function<void()>>> ONCE_EVENT;

        /**
         * @brief 处理事件
         *
         * @param lParam
         */
        void emit(LPARAM lParam)
        {
            switch (lParam)
            {
                // 鼠标右键按下
            case WM_RBUTTONDOWN:
            {

                HMC_EMIT_ENVINFO("rightButtonDown");
                break;
            }
                // 鼠标右键松开
            case WM_RBUTTONUP:
            {
                HMC_EMIT_ENVINFO("rightButtonUp");
                break;
            }
                // 鼠标左键按下
            case WM_LBUTTONDOWN:
            {
                HMC_EMIT_ENVINFO("leftButtonDown");

                break;
            }
                // 鼠标左键松开
            case WM_LBUTTONUP:
            {
                HMC_EMIT_ENVINFO2("click", "leftButtonUp");

                break;
            }
                // 鼠标左键双击
            case WM_LBUTTONDBLCLK:
            {
                HMC_EMIT_ENVINFO2("dblclick", "leftButtonDoubleClick");
                break;
            }
                // 鼠标浮动
            case WM_MOUSEMOVE:
            {
                HMC_EMIT_ENVINFO2("move", "mouseMove");
                break;
            }
            // 中键点击
            case WM_MBUTTONDOWN:
            {
                HMC_EMIT_ENVINFO("middleClick");
                break;
            }
            }
        }

        /**
         * @brief 处理按钮响应
         *
         * @param keyID
         */
        void emit(string keyID, chMenuItem &menuItem)
        {
            bool isOnOK = false;
            for (auto &&event : ON_EVENT)
            {

                if (event.first == keyID)
                {
                    for (size_t i = 0; i < event.second.size(); i++)
                    {
                        event.second[i]();
                    }
                    isOnOK = true;
                }
            }

            bool isOnCeOK = false;
            for (auto &&event : ONCE_EVENT)
            {
                if (event.first == keyID)
                {
                    for (size_t i = 0; i < event.second.size(); i++)
                    {
                        event.second[i]();
                    }
                    isOnCeOK = true;
                    event.second.clear();
                }
            }

            // cout << "click->" << keyID << "   key->" << menuItem.messageID << "   name->" << menuItem.name << endl;
        }

        // 点击了按钮
        void clickButtonItem(WORD IDKey)
        {
            string id = string();
            for (size_t i = 0; i < __MenuList.size(); i++)
            {
                auto Menu = __MenuList[i];
                if (Menu.messageID == IDKey)
                {
                    id.clear();
                    id.append(Menu.id);
                    if (Menu.type == chMenuType::check)
                    {
                        __MenuList[i].select = !Menu.select;
                    }
                    emit(id, __MenuList[i]);
                }
            }
        }

    }

    /**
     * @brief 监听
     *
     * @param eventName
     * @param fnc
     */
    void on(string eventName, std::function<void()> fnc)
    {

        vector<function<void()>> funList = {};

        if (_HMC__EVENT::ON_EVENT.find(eventName) == _HMC__EVENT::ON_EVENT.end())
        {
            _HMC__EVENT::ON_EVENT.insert(std::make_pair(eventName, funList));
        }

        for (auto &&EVENT : _HMC__EVENT::ON_EVENT)
        {
            if (EVENT.first == eventName)
            {
                EVENT.second.push_back(fnc);
            }
        }
    }

    /**
     * @brief 单次监听
     *
     * @param eventName
     * @param fnc
     */
    void once(string eventName, std::function<void()> fnc)
    {
        vector<function<void()>> funList = {};

        if (_HMC__EVENT::ONCE_EVENT.find(eventName) == _HMC__EVENT::ONCE_EVENT.end())
        {
            _HMC__EVENT::ONCE_EVENT.insert(std::make_pair(eventName, funList));
        }

        for (auto &&EVENT : _HMC__EVENT::ONCE_EVENT)
        {
            if (EVENT.first == eventName)
            {
                EVENT.second.push_back(fnc);
            }
        }
    }

    BOOL setTrayIcon(HICON hNewIcon, int index = 0)
    {
        try
        {
            nid.hIcon = hNewIcon;
            return Shell_NotifyIconA(NIM_MODIFY, &nid);
        }
        HMC_CHECK_CATCH;

        return false;
    }

    BOOL setTrayIcon(string Icons, int index = 0)
    {
        try
        {
            HICON hNewIcon = NULL; // 声明一个HICON句柄
            hNewIcon = ExtractIconA(GetModuleHandleA(NULL), (LPCSTR)Icons.c_str(), (UINT)index);
            if (hNewIcon != NULL)
            {
                nid.hIcon = hNewIcon;
                return Shell_NotifyIconA(NIM_MODIFY, &nid);
            }
        }
        HMC_CHECK_CATCH;

        return false;
    }

    /**
     * @brief 修改tray的图标（从当前可执行文件获取）
     *
     * @return BOOL
     */
    BOOL setTrayIcon(int index = 0)
    {
        try
        {
            // 获取进程可执行文件路径
            CHAR lpFilename[MAX_PATH];
            HANDLE hProcess = OpenProcess(PROCESS_ALL_ACCESS, FALSE, ::GetCurrentProcessId());

            if (hProcess == nullptr)
            {
                return false;
            }

            if (GetModuleFileNameExA(hProcess, NULL, (LPSTR)lpFilename, MAX_PATH) != ERROR_SUCCESS)
            {
                return false;
            }

            // 取出图标
            HICON hNewIcon = NULL; // 声明一个HICON句柄
            hNewIcon = ExtractIconA(GetModuleHandle(NULL), (LPSTR)lpFilename, index);
            if (!hNewIcon)
                return false;
            nid.hIcon = hNewIcon;
            return Shell_NotifyIconA(NIM_MODIFY, &nid);
        }
        HMC_CHECK_CATCH;
        return false;
    }

    /**
     * @brief 启动
     *
     */
    bool start()
    {
        if (___Start_hmc_Tray)
            return false;
        __tray_worker = new std::thread(__openPuppetTrayWindow);
    }

    // 结束托盘
    bool close()
    {
        Shell_NotifyIconA(NIM_DELETE, &nid);
        CloseWindow(PuppetTrayWindowHwnd);

        try
        {
            PostThreadMessage(GetThreadId(__tray_worker->native_handle()), WM_QUIT, NULL, NULL);
            __tray_worker->join();
            delete __tray_worker;
            __tray_worker = nullptr;
        }

        HMC_CHECK_CATCH;
        try
        {
            ___Start_hmc_Tray = false;
            PuppetTrayWindowHwnd = NULL;
            __MenuList.clear();

            // 清空监听
            for (auto &&EVENT : _HMC__EVENT::ON_EVENT)
                EVENT.second.clear();
            for (auto &&EVENT : _HMC__EVENT::ONCE_EVENT)
                EVENT.second.clear();
            _HMC__EVENT::ON_EVENT.clear();
            _HMC__EVENT::ONCE_EVENT.clear();
            Shell_NotifyIconA(NIM_DELETE, &nid); // 然后立即删除该托盘图标
            ZeroMemory(&nid, sizeof(NOTIFYICONDATAA));
        }
        HMC_CHECK_CATCH;

        return false;
    }

    /**
     * @brief 添加按钮
     *
     * @param menuItem
     * @return true
     * @return false
     */
    bool addMenuItem(chMenuItem menuItem)
    {
        for (auto &&Menu : __MenuList)
        {
            if (Menu.id == menuItem.id)
                return false;
        }

        int idkey = (__tray_next_id++);

        menuItem.messageID = idkey;

        __MenuList.push_back(menuItem);

        return true;
    }

    // 修改托盘显示的消息内容
    BOOL setTrayInfo(string trayInfo)
    {
        string subtxt = string();
        try
        {
            if (trayInfo.length() > 255)
            {
                subtxt.append(trayInfo.substr(0, 255));
            }
            else
            {
                subtxt.append(trayInfo.substr(0, trayInfo.length()));
            }

            strncpy_s(nid.szTip, subtxt.c_str(), subtxt.size());
            if (Shell_NotifyIconA(NIM_MODIFY, &nid))
            {
                __hmc_trayInfo.clear();
                __hmc_trayInfo.append(subtxt);
                return true;
            };
        }
        HMC_CHECK_CATCH;
        return false;

        // wcscpy_s(nid.szTip, pszNewInfo);
    }

    LRESULT CALLBACK ___openPuppetTrayWindow_WndProc(HWND hWnd, UINT message, WPARAM wParam, LPARAM lParam)
    {
        switch (message)
        {
        case WM_CREATE:
            nid.cbSize = sizeof(NOTIFYICONDATA);
            nid.hWnd = hWnd;
            nid.uID = 0;
            nid.uVersion = NOTIFYICON_VERSION;
            nid.uCallbackMessage = WM_SYSICON;
            if (!nid.hIcon)
            {
                nid.hIcon = LoadIcon(NULL, IDI_APPLICATION);
            }
            strncpy_s(nid.szTip, "", sizeof(nid.szTip));
            nid.uFlags = NIF_MESSAGE | NIF_ICON | NIF_TIP;
            Shell_NotifyIconA(NIM_ADD, &nid);
            break;

        case WM_DESTROY:
            Shell_NotifyIconA(NIM_DELETE, &nid);
            PostQuitMessage(0);
            break;

        case WM_SYSICON:
        {
            _HMC__EVENT::emit(lParam);
            // 如果是右键显示右键菜单
            if (lParam == WM_RBUTTONUP)
            {
                POINT curPoint;
                GetCursorPos(&curPoint);
                SetForegroundWindow(hWnd);
                HMENU hMenu = CreatePopupMenu();
                shared_ptr<void> close_hMenu(nullptr, [&](void *)
                                             {if (hMenu != nullptr) {try{DestroyMenu(hMenu);}HMC_CHECK_CATCH;} });

                if (hMenu)
                {
                    for (auto &&Menu : __MenuList)
                    {

                        // 根按钮  且为按钮按钮中无子分类 分割线和选项将忽略子按钮检测
                        if (Menu.rootMenu && Menu.menuList.size() == 0)
                        {
                            if (!Menu.show)
                                continue;
                            // 按钮
                            if (Menu.type == hmc_tray::chMenuType::menu)
                            {
                                InsertMenuA(hMenu, -1, MF_BYPOSITION, Menu.messageID, Menu.name.c_str());
                            }
                            // 分割线
                            else if (Menu.type == hmc_tray::chMenuType::separator)
                            {
                                InsertMenuA(hMenu, -1, MF_BYPOSITION | MF_SEPARATOR, Menu.messageID, Menu.name.c_str());
                            }
                            // 选项
                            else if (Menu.type == hmc_tray::chMenuType::check)
                            {
                                if (Menu.select)
                                {
                                    InsertMenuA(hMenu, -1, MF_BYCOMMAND | MF_CHECKED, Menu.messageID, Menu.name.c_str());
                                }
                                else
                                {
                                    InsertMenuA(hMenu, -1, MF_BYCOMMAND | MF_UNCHECKED, Menu.messageID, Menu.name.c_str());
                                }
                            }
                            // 选项
                            // else if (Menu.type == hmc_tray::chMenuType::radio)
                            // {
                            //     if (Menu.select)
                            //     {
                            //         InsertMenuA(hMenu, -1, MF_BYCOMMAND | MFT_RADIOCHECK | MF_CHECKED, Menu.messageID, Menu.name.c_str());
                            //     }
                            //     else
                            //     {
                            //         InsertMenuA(hMenu, -1, MF_BYCOMMAND | MFT_RADIOCHECK | MF_UNCHECKED, Menu.messageID, Menu.name.c_str());
                            //     }
                            // }

                            // 禁用
                            if (Menu.disable)
                            {
                                EnableMenuItem(hMenu, Menu.messageID, MF_BYCOMMAND | MF_DISABLED | MF_GRAYED);
                            }
                        }
                        // 有子目录的按钮
                        else if (Menu.show && Menu.rootMenu && Menu.menuList.size() != 0)
                        {
                            InsertMenuA(hMenu, -1, MF_BYPOSITION, Menu.messageID, Menu.name.c_str());
                            HMENU hSubMenu = CreatePopupMenu();
                            shared_ptr<void> close_hSubMenu(nullptr, [&](void *)
                                                            {if (hSubMenu != nullptr) {try{DestroyMenu(hSubMenu);}HMC_CHECK_CATCH;} });
                            if (!Menu.show)
                                continue;

                            for (auto &&IDmessageID : Menu.menuList)
                            {
                                // 创建子菜单
                                if (hSubMenu)
                                {
                                    for (auto &&_Menu : __MenuList)
                                    {
                                        if (_Menu.messageID != IDmessageID)
                                            continue;

                                        // 按钮
                                        if (_Menu.type == hmc_tray::chMenuType::menu)
                                        {
                                            AppendMenuA(hSubMenu, MF_STRING, _Menu.messageID, _Menu.name.c_str());

                                            // InsertMenuA(hMenu, -1, MF_BYPOSITION, _Menu.messageID, _Menu.name.c_str());
                                        }
                                        // 分割线
                                        else if (_Menu.type == hmc_tray::chMenuType::separator)
                                        {
                                            AppendMenuA(hSubMenu, MF_SEPARATOR, _Menu.messageID, _Menu.name.c_str());
                                            // InsertMenuA(hMenu, -1, MF_BYPOSITION | MF_SEPARATOR, _Menu.messageID, _Menu.name.c_str());
                                        }
                                        // 选项
                                        else if (_Menu.type == hmc_tray::chMenuType::check)
                                        {
                                            if (_Menu.select)
                                            {
                                                AppendMenuA(hSubMenu, MF_CHECKED, _Menu.messageID, _Menu.name.c_str());
                                            }
                                            else
                                            {
                                                AppendMenuA(hSubMenu, MF_UNCHECKED, _Menu.messageID, _Menu.name.c_str());
                                            }
                                        }

                                        // 禁用
                                        if (_Menu.disable)
                                        {
                                            EnableMenuItem(hSubMenu, _Menu.messageID, MF_DISABLED | MF_GRAYED);
                                        }
                                    }

                                    // 把子菜单加到主菜单项
                                    ModifyMenuA(hMenu, Menu.messageID, MF_POPUP, (UINT_PTR)hSubMenu, Menu.name.c_str());
                                }
                            }

                            // 这让菜单显示为需要的样子，例如处理具有复选菜单项的菜单
                            SetMenuDefaultItem(hMenu, Menu.messageID, FALSE);
                        }
                    }
                    TrackPopupMenu(hMenu, TPM_BOTTOMALIGN, curPoint.x, curPoint.y, 0, hWnd, NULL);
                    DestroyMenu(hMenu);
                }
            }
        }
        break;

        case WM_COMMAND:
            _HMC__EVENT::clickButtonItem(LOWORD(wParam));
        default:
            return DefWindowProc(hWnd, message, wParam, lParam);
        }

        return 0;
    }

    int __openPuppetTrayWindow()
    {
        WNDCLASSEXA wc;
        MSG Msg;
        HINSTANCE hInstance = (HINSTANCE)GetModuleHandle(NULL);

        wc.cbSize = sizeof(WNDCLASSEXA);
        wc.style = 0;
        wc.lpfnWndProc = ___openPuppetTrayWindow_WndProc;
        wc.cbClsExtra = 0;
        wc.cbWndExtra = 0;
        wc.hInstance = hInstance;
        wc.hIcon = LoadIcon(NULL, IDI_APPLICATION);
        wc.hCursor = LoadCursor(NULL, IDC_ARROW);
        wc.hbrBackground = (HBRUSH)(COLOR_WINDOW + 1);
        wc.lpszMenuName = NULL;
        wc.lpszClassName = __hmc_className.c_str();
        wc.hIconSm = LoadIcon(NULL, IDI_APPLICATION);

        RegisterClassExA(&wc);

        PuppetTrayWindowHwnd = CreateWindowExA(WS_EX_APPWINDOW, wc.lpszClassName, __hmc_title.c_str(), WS_OVERLAPPEDWINDOW,
                                               CW_USEDEFAULT, CW_USEDEFAULT, 1, 1, NULL, NULL, hInstance, NULL);

        ShowWindow(PuppetTrayWindowHwnd, SW_HIDE);
        // ShowWindow(hWnd, SW_SHOW);

        while (GetMessageA(&Msg, NULL, 0, 0) > 0)
        {
            TranslateMessage(&Msg);
            DispatchMessage(&Msg);
        }

        return Msg.wParam;
    }

    /**
     * @brief 设置指定的按钮为XXX的子按钮
     *
     * @param MenuId
     * @param RootMenuId
     * @return true
     * @return false
     */
    bool setMenuItmetoSubMenu(string RootMenuId, string SubMenuId)
    {
        for (auto &&menu : __MenuList)
        {
            if (menu.id == RootMenuId || menu.id == RootMenuId)
            {
                menu.rootMenu = true;
                for (auto &&menu2 : __MenuList)
                {
                    if (menu2.id == SubMenuId || menu2.id == SubMenuId)
                    {
                        menu2.rootMenu = false;
                        menu.menuList.push_back(menu2.messageID);
                        return true;
                    }
                }
            }
        }
        return false;
    }

    /**
     * @brief 设置指定的按钮为XXX的子按钮
     *
     * @param RootMenuId
     * @param SubMenuId
     * @return true
     * @return false
     */
    bool setMenuItmetoSubMenu(chMenuItem RootMenuId, string SubMenuId)
    {
        setMenuItmetoSubMenu(RootMenuId.id, SubMenuId);
    }

    /**
     * @brief 设置指定的按钮为XXX的子按钮
     *
     * @param RootMenuId
     * @param SubMenuId
     * @return true
     * @return false
     */
    bool setMenuItmetoSubMenu(chMenuItem RootMenuId, chMenuItem SubMenuId)
    {
        setMenuItmetoSubMenu(RootMenuId.id, SubMenuId.id);
    }

    /**
     * @brief 设置指定的按钮为XXX的子按钮
     *
     * @param RootMenuId
     * @param SubMenuId
     * @return true
     * @return false
     */
    bool setMenuItmetoSubMenu(string RootMenuId, chMenuItem SubMenuId)
    {
        setMenuItmetoSubMenu(RootMenuId, SubMenuId.id);
    }

    /**
     * @brief 设置指定的按钮为XXX的子按钮
     *
     * @param RootMenuId
     * @param SubMenuId
     * @return true
     * @return false
     */
    template <typename... Args>
    bool setMenuItmetoSubMenu(string RootMenuId, Args... SubMenuIdArgs)
    {
        string temp[] = {SubMenuIdArgs...};
        for (size_t i = 0; i < sizeof(temp) / sizeof(temp[0]); i++)
        {
            setMenuItmetoSubMenu(RootMenuId, temp[i]);
        }
        return true;
    }

    /**
     * @brief 设置指定的按钮为XXX的子按钮
     *
     * @param RootMenuId
     * @param SubMenuId
     * @return true
     * @return false
     */
    template <typename... Args>
    bool setMenuItmetoSubMenu(chMenuItem RootMenuId, Args... SubMenuIdArgs)
    {
        string temp[] = {SubMenuIdArgs...};
        for (size_t i = 0; i < sizeof(temp) / sizeof(temp[0]); i++)
        {
            setMenuItmetoSubMenu(RootMenuId.id, temp[i]);
        }
        return true;
    }

    /**
     * @brief 设置指定按钮可见性
     *
     * @param MenuId
     * @param Visible
     * @return true
     * @return false
     */
    bool setMenuItmeVisible(string MenuId, bool Visible)
    {
        for (size_t i = 0; i < __MenuList.size(); i++)
        {
            auto Menu = __MenuList[i];
            if (Menu.id == MenuId)
            {

                __MenuList[i].show = Visible ? true : false;
                return true;
            }
        }
        return false;
    }

    // 设置按钮为禁用
    bool setMenuItmeEnable(string MenuId, bool Enable = true)
    {
        for (size_t i = 0; i < __MenuList.size(); i++)
        {
            auto Menu = __MenuList[i];
            if (Menu.id == MenuId)
            {

                __MenuList[i].disable = Enable;
                return true;
            }
        }
        return false;
    }

    /**
     * @brief 设置指定的按钮为XXX的子按钮
     *
     * @param RootMenuId
     * @param SubMenuId
     * @return true
     * @return false
     */
    bool setMenuItmeName(string MenuId, string Name)
    {
        for (size_t i = 0; i < __MenuList.size(); i++)
        {
            auto Menu = __MenuList[i];
            if (Menu.id == MenuId)
            {

                __MenuList[i].name.clear();
                __MenuList[i].name.append(Name);
                return true;
            }
        }
        return false;
    }

    /**
     * @brief 设置指定的按钮为XXX的选择
     *
     * @param RootMenuId
     * @param SubMenuId
     * @return true
     * @return false
     */
    bool setMenuItmeSelect(string MenuId, bool Select)
    {
        for (size_t i = 0; i < __MenuList.size(); i++)
        {
            auto Menu = __MenuList[i];
            if (Menu.id == MenuId)
            {
                __MenuList[i].select = Select;
                return true;
            }
        }
        return false;
    }

    /**
     * @brief Get the Menu Itme object
     *
     * @param MenuId
     * @return chMenuItem
     */
    chMenuItem getMenuItme(string MenuId)
    {

        for (size_t i = 0; i < __MenuList.size(); i++)
        {
            auto Menu = __MenuList[i];
            if (Menu.id == MenuId)
            {
                return __MenuList[i];
            }
        }

        return chMenuItem();
    }

    namespace Menu
    {
        /**
         * @brief 创建一个按钮
         *
         * @param name
         * @param id
         * @param disable
         * @return chMenuItem
         */
        chMenuItem menu(string name, string id, bool disable = false)
        {
            hmc_tray::chMenuItem menuItem;
            menuItem.disable = disable;
            menuItem.id = id;
            menuItem.name = name;
            menuItem.rootMenu = true;
            menuItem.menuList = {};
            menuItem.show = true;
            menuItem.select = false;
            menuItem.type = hmc_tray::chMenuType::menu;
            return menuItem;
        }

        /**
         * @brief 创建一个带有选项的按钮
         *
         * @param name
         * @param id
         * @param disable
         * @return chMenuItem
         */
        chMenuItem check(string name, string id, bool select = false)
        {
            hmc_tray::chMenuItem menuItem;
            menuItem.disable = false;
            menuItem.id = id;
            menuItem.name = name;
            menuItem.rootMenu = true;
            menuItem.menuList = {};
            menuItem.show = true;
            menuItem.select = select;
            menuItem.type = hmc_tray::chMenuType::check;
            return menuItem;
        }

        /**
         * @brief 创建一个分割线按钮
         *
         * @param id
         * @param root
         * @return chMenuItem
         */
        chMenuItem separator(string id, bool root = true)
        {
            hmc_tray::chMenuItem menuItem;
            menuItem.disable = false;
            menuItem.id = id;
            menuItem.name = "";
            menuItem.show = true;
            menuItem.rootMenu = root;
            menuItem.menuList = {};
            menuItem.select = false;
            menuItem.type = hmc_tray::chMenuType::separator;
            return menuItem;
        }

        // /**
        //  * @brief 创建一个带有选项的按钮
        //  *
        //  * @param name
        //  * @param id
        //  * @param disable
        //  * @return chMenuItem
        //  */
        // chMenuItem radio(string name, string id, bool select = false)
        // {
        //     hmc_tray::chMenuItem menuItem;
        //     menuItem.disable = false;
        //     menuItem.id = id;
        //     menuItem.name = name;
        //     menuItem.rootMenu = true;
        //     menuItem.menuList = {};
        //     menuItem.show = true;
        //     menuItem.select = select;
        //     menuItem.type = hmc_tray::chMenuType::radio;
        //     return menuItem;
        // }

    };

}
#endif
