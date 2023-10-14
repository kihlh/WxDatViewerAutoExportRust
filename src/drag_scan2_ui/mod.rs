#![allow(warnings, unused)]

use crate::{get_arc_bind_variable, get_bool, get_option_arc_bind_variable, get_option_arc_bind_variable_or, global_var, gui_util, inject_fltk_theme, libWxIkunPlus, select_user_ui, set_arc_bind_variable, set_arc_bind_variable_string_replace_data, set_bool, set_item_id, set_option_arc_bind_variable, wh_mod};
use fltk::enums::{Color, FrameType};
use fltk::window::DoubleWindow;
use fltk::{prelude::*, *};
use fltk_theme::{color_themes, ColorTheme, SchemeType, ThemeType, WidgetScheme, WidgetTheme};
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Local};

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
      wh_mod::gc_walk_attach_file_list();
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
                    push_message(format!("[扫描]当前：{:?}",file_name).as_str(),false);
                }

                for path in paths {
                    let resolve_path = path.to_string_lossy();
                    if resolve_path.contains(imag_id_copy2.as_str()) {
                        let start = get_option_arc_bind_variable_or!(STATUS_TIME,STATUS_TIME_BIND,std::time::Instant::now());
                        let att_info = wh_mod::wx_parse_path(resolve_path.to_string());

                        let mut input_data = format!("用户<{}> [已选定] 用时: {:?}", att_info.attach_id, start.elapsed());
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
                    let start = get_option_arc_bind_variable_or!(STATUS_TIME,STATUS_TIME_BIND,std::time::Instant::now());
                    let mut input_data = format!("扫描结束【未找到】 用时: {:?}", start.elapsed());

                    push_message(input_data.as_str(),true);

                    println!("{}", &input_data);
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
            if !has_window() {return };

           progress_bar_preview.hide();

            let start = get_option_arc_bind_variable!(STATUS_TIME,STATUS_TIME_BIND);

            if let Some(start) = start {
                std::thread::sleep(std::time::Duration::from_millis(300u64));
                if !has_window() {return };

                buf.remove(0,buf.length());
                let walk_drag_path = global_var::get_string_default("user::config::walk_drag_path");
                if walk_drag_path.is_empty() {
                    let mut input_data = format!("扫描结束 用时约为: {:?} ", start.elapsed());
                    buf.append(input_data.as_str());
                }
                else{
                    let att_info = wh_mod::wx_parse_path(walk_drag_path.to_string());
                    let mut input_data = format!("ID<{}> [已选定] 用时约为: {:?}", att_info.attach_id , start.elapsed());
                    buf.append(input_data.as_str());
                }
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
    // 窗口已经被初始化 而且句柄有效
    if has_window() {
        return Option::None;
    }

    let mut win: DoubleWindow = fltk::window::DoubleWindow::new(0, 0, 600,360, "扫描图源用户").center_screen();
    let mut rect = libWxIkunPlus::getWindowRect(libWxIkunPlus::findWindow(select_user_ui::WINDOW_CLASS_NAME,""));
    win.set_pos(rect.left,rect.top);

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
                                    buf.append(temp_imag_id.as_str());
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
            buf.append(temp_imag_id.as_str());
            copy_progress_bar.show();

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

    }

    libWxIkunPlus::setWinIcon(get_the_hwnd!());
    libWxIkunPlus::setWinTop(get_the_hwnd!(),true);

    Some(win)
}
