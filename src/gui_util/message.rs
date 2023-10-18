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

