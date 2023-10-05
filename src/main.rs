#![allow(warnings, unused)]

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

use std::sync::atomic::{AtomicBool, AtomicI32, AtomicUsize, Ordering,AtomicI64};
use std::sync::{Arc, Condvar, Mutex,RwLock};

use crate::{util::{str_eq_str, Sleep}, libWxIkunPlus::{hasWeChatWin, hasWeChat}};

mod atomic_util;
mod global_var;
mod global_var_util;
mod gui;
mod gui_detect_config;
mod gui_drag_scan;
mod gui_hotspot;
mod gui_imge;
mod gui_manage_item;
mod gui_select_user_base;
mod gui_text_control;
mod handle_dat;
mod libWxIkunPlus;
mod util;
mod watching;
mod wh_mod;

const APP_MUTEX_NAME: &str = "ikun::^_^::wx_auto_export_image:^_^::end";

static SYNC_TOKEN:AtomicBool = AtomicBool::new(false);
static SYNC_IMMED_TOKEN:AtomicBool = AtomicBool::new(false);
static APP_STARTUP:AtomicBool = AtomicBool::new(false);


// mod wh_util;
fn main() -> Result<()> {
    // 处理命令行
    handle_dat::handle_commandLine();

    // 拒绝软件重复启动
    if (libWxIkunPlus::hasMutex(APP_MUTEX_NAME.to_owned())) {
        libWxIkunPlus::error(
            "启动失败".to_owned(),
            "已经存在了一个 <WxAutoExIm> 进程".to_owned(),
        );
        process::exit(0);
    } else {
        libWxIkunPlus::createMutex(APP_MUTEX_NAME.to_owned());
    }

    // let get_wxid_acc = wh_mod::convert::get_wxid_name(wh_mod::convert::get_user_data_path().unwrap(),wh_mod::convert::get_user_id2());
    // println!("{:?}",get_wxid_acc);
    // let get_wxid_acc = wh_mod::convert::get_wxid_name(wh_mod::convert::get_user_data_path().unwrap(),wh_mod::convert::get_user_id1());
    // println!("{:?}",get_wxid_acc);
    // println!("hasWeChat->  {:?}",libWxIkunPlus::hasWeChat());

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

    thread::spawn(move || {
        if (!wh_mod::convert::is_developer()) {
            if (!libWxIkunPlus::has_auto_sync()) {
                let mut err_name = if !hasWeChatWin() {"WX尚未登录" } else if !hasWeChat() {"WX进程未找到并且WX窗口是伪装的"} else {"用户未启用同步"};
                console_log!(format!("[同步暂停] 因为：{}", err_name));

                loop {
                    if (libWxIkunPlus::has_auto_sync()||libWxIkunPlus::has_sync_token()) {
                        break;
                    }
                    Sleep(5000);
                }
              
            }
        }

        if (wh_mod::convert::is_developer()) {
            console_log!(format!("[同步]{}", "自动同步已启用 因为开发者模式有效"));
        } else {
            console_log!(format!("[同步]{}", "自动同步已启用"));
        }

        // 启动文件系统日志模式
        thread::spawn(move || {
            watching::initialize_file_system_Change();
        });

        // 启动后全量扫描
        thread::spawn(move || {
            util::Sleep(global_var::get_u64_or("SLEEP_SCAN_ALL", 30000));

            let conn: Connection =
                Connection::open("ikun_user_data.db").expect("无法 创建/打开 数据库");
            handle_dat::initialize_table(&conn);
            let _ = handle_dat::handle_walk_pictures(&conn);
            let _ = conn.close();
        });
    });

    // 阻塞进程状态可用 进程不退出 以及与cpp的数据更新
    loop {
        if(set_bool!(SYNC_IMMED_TOKEN,libWxIkunPlus::has_sync_token())){
            if get_bool!(SYNC_IMMED_TOKEN){
                console_log!(format!("[用户] 立即全部扫描"));
                thread::spawn(move || {        
                    let conn: Connection =
                        Connection::open("ikun_user_data.db").expect("无法 创建/打开 数据库");
                    handle_dat::initialize_table(&conn);
                    let _ = handle_dat::handle_walk_pictures(&conn);
                    let _ = conn.close();
                });
            }
            // println!("has_sync_token 值已经更新");
        };
        
        if(set_bool!(SYNC_TOKEN,libWxIkunPlus::has_auto_sync())){
            // println!("has_auto_sync 值已经更新");
            if(libWxIkunPlus::has_auto_sync()){
                console_log!(format!("[用户] 自动同步开启"));
            }else{
                console_log!(format!("[用户] 自动同步关闭"));
            }
        };

        if(set_bool!(APP_STARTUP,libWxIkunPlus::hasStartupGlobalVar())){
            // println!("自启动值已经更新");
            
        };

        // println!("主线程");
        Sleep(150);
    }

    Ok(())
}
