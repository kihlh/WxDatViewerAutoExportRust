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
mod handle_dat;
mod libWxIkunPlus;
mod util;
mod watching;
mod wh_mod;

mod gui_util;

#[path = "GUI/gui_rename_ui/mod.rs"]
mod gui_rename_ui;

#[path = "GUI/gui_drag_scan2_ui/mod.rs"]
mod gui_drag_scan2_ui;

#[path = "GUI/gui_select_user_ui/mod.rs"]
mod gui_select_user_ui;

#[path = "GUI/gui_main_ui/mod.rs"]
mod gui_main_ui;

#[path = "GUI/gui_detect_config_ui/mod.rs"]
mod gui_detect_config_ui;

#[path = "trash/gui_manage_item.rs"]
mod gui_manage_item;

#[path = "GUI/gui_donation_ui/mod.rs"]
mod gui_donation_ui;

const APP_MUTEX_NAME: &str = "ikun::^_^::wx_auto_export_image:^_^::end";

// static SYNC_TOKEN:AtomicBool = AtomicBool::new(false);
// static SYNC_IMMED_TOKEN:AtomicBool = AtomicBool::new(false);
// static APP_STARTUP:AtomicBool = AtomicBool::new(false);

// let start = Instant::now();
// libWxIkunPlus::findWindow("WeChatMainWndForPC","微信");
// 代码执行时间: 54.1µs
// libWxIkunPlus::findAllWindow("WeChatMainWndForPC","微信");
// 代码执行时间: 72.5087ms
// libWxIkunPlus::findAllWindow("WeChatMainWndForPC","");
// 代码执行时间: 1.0077ms
// println!("代码执行时间: {:?}", start.elapsed());

// mod wh_util;
fn main() -> Result<()> {
    
    // 处理命令行
    handle_dat::handle_commandLine();

    // 拒绝软件重复启动
    if (!wh_mod::config::is_developer()){

    if (libWxIkunPlus::hasMutex(APP_MUTEX_NAME.to_owned())) {
        libWxIkunPlus::error(
            "启动失败".to_owned(),
            "已经存在了一个 <WxAutoExIm> 进程".to_owned(),
        );
        process::exit(0);
    } else {
        libWxIkunPlus::createMutex(APP_MUTEX_NAME.to_owned());
    }

   }else {
        if (libWxIkunPlus::hasMutex(APP_MUTEX_NAME.to_owned())) {
        if !libWxIkunPlus::confirm(
            "进程存在".to_owned(),
            "(开发者模式) 已经存在了一个 <WxAutoExIm> 进程 是否继续启动".to_owned(),
        ) {
            process::exit(0);
        }
        }
    }


    // 窗口部分
    thread::spawn(move || {
        // 创建窗口
        let appMain = app::App::default();
        // gui_donation_ui::main_init();
        gui_main_ui::main_init();
        appMain.run().unwrap();
    });

    println!("程序初始化成功");

    // 用CPP注册托盘
    libWxIkunPlus::set_tray();

    thread::spawn(move || {
        if (!wh_mod::config::is_developer()) {
            if (!libWxIkunPlus::has_auto_sync()) {
                let mut err_name = if !hasWeChatWin() {"WX尚未登录" } else if !hasWeChat() {"WX进程未找到"} else {"用户未启用同步"};
                console_log!(format!("[同步暂停] 因为：{}", err_name));

                loop {
                    if (libWxIkunPlus::has_auto_sync()||libWxIkunPlus::has_sync_token()) {
                        break;
                    }
                    Sleep(5000);
                }

            }
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
        if(libWxIkunPlus::has_sync_token()){
                console_log!(format!("[用户] 立即全部扫描"));
                thread::spawn(move || {
                    let conn: Connection =
                        Connection::open("ikun_user_data.db").expect("无法 创建/打开 数据库");
                    handle_dat::initialize_table(&conn);
                    let _ = handle_dat::handle_walk_pictures(&conn);
                    let _ = conn.close();
                });
        };
        //
        // if(set_bool!(APP_STARTUP,libWxIkunPlus::hasStartupGlobalVar())){
        //     // println!("自启动值已经更新");
        //
        // };

        // println!("主线程");
        Sleep(150);
    }

    Ok(())
}
