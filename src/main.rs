#![allow(
    dead_code,
    unused_imports,
    unused_parens,
    unused_variables,
    unused_mut,
    unused_must_use,
    unused_assignments,
    non_snake_case,
    unreachable_code
)]
// #![windows_subsystem = "windows"]

use chrono::Local;
use glob::glob;
use hotwatch::{
    blocking::{Flow, Hotwatch},
    EventKind,
};
use libc::c_void;
use rusqlite::{params, Connection, Result};

use fltk::app::handle;
use fltk::button::Button;
use fltk::draw::font;
use fltk::enums::{Cursor, Event, Font, LabelType};
use fltk::frame::Frame;
use fltk::group::Group;
use fltk::input::{InputType, IntInput};
use fltk::text::TextDisplay;
use fltk::{enums::Color, enums::FrameType};
use fltk::{prelude::*, window::Window, *};
use magic_crypt::MagicCryptTrait;
use msgbox::IconType;
use notify::{RecursiveMode, Watcher};
use serde_json::json;
use std::mem::transmute;
use std::sync::mpsc;
use std::sync::mpsc::channel;
use std::{collections::hash_map::DefaultHasher, ptr};
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
use winapi::{
    shared::winerror::ERROR_ALREADY_EXISTS,
    um::{errhandlingapi::GetLastError, handleapi::INVALID_HANDLE_VALUE, synchapi::CreateMutexW},
};

use crate::util::{str_eq_str, Sleep};

mod global_var;
mod gui;
mod gui_hotspot;
mod gui_imge;
mod handle_dat;
mod libWxIkunPlus;
mod gui_manage_item;
mod gui_select_user_base;
mod util;
mod watching;
mod wh_mod;
mod gui_drag_scan;
mod gui_detect_config;
mod gui_text_control;
mod atomic_util;

// mod wh_util;
fn main() -> Result<()> {
    // 处理命令行
    handle_dat::handle_commandLine();

    // 窗口部分
    thread::spawn(move || {
        // 创建窗口
        let appMain = app::App::default();

        let mut mainItme = gui::mianWindow(true);

        match mainItme {
            Ok(window_item) => {
                // 处理窗口对象
            }
            Err(err) => {
                // 处理错误
                eprintln!("Error: {:?}", err);
            }
        }
    
        appMain.run().unwrap();
    });

    println!("程序初始化成功");

    // 用CPP注册托盘
    libWxIkunPlus::set_tray();

    // 启动文件系统日志模式
    thread::spawn(move || {
        watching::initialize_file_system_Change();
    });

    // 启动后全量扫描
    thread::spawn(move || {
        util::Sleep(global_var::get_u64("SLEEP_SCAN_ALL"));
        let conn: Connection =
            Connection::open("ikun_user_data.db").expect("无法 创建/打开 数据库");
        handle_dat::initialize_table(&conn);
        let _ = handle_dat::handle_walk_pictures(&conn);
        let _ = conn.close();
    });

    loop {
        Sleep(5500);
    }

    Ok(())
}
