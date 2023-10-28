#![allow(warnings, unused)]

use crate::{libWxIkunPlus, set_bool, APP_DB_NAME};
use chrono::Local;
use core::sync::atomic::Ordering;
use std::arch::x86_64::CpuidResult;
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
use glob::glob;
use hotwatch::{
    blocking::{Flow, Hotwatch},
    EventKind,
};
use magic_crypt::MagicCryptTrait;
use msgbox::IconType;
use rusqlite::{params, Connection, Result};
use serde_json::json;
use std::collections::hash_map::DefaultHasher;
use std::mem::transmute;
use std::sync::Mutex;
use std::sync::{atomic::AtomicBool, Arc};
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
use crate::console_log;
use crate::{
    get_bool, global_var, global_var_util,
    util::{self, str_eq_str, Sleep},
    wh_mod,
};
static HANDLE_DAT_ING: AtomicBool = AtomicBool::new(false);

// 初始化数据库表头
pub fn initialize_table(conn: &Connection) {
    match conn.execute(
        "CREATE TABLE IF NOT EXISTS msg_attach_export (
            id    INTEGER PRIMARY KEY,
            time  TEXT NOT NULL,
            name  TEXT NOT NULL,
            user_name	TEXT NOT NULL,
            ext TEXT NOT NULL,
            input  TEXT NOT NULL UNIQUE,
            ouput  TEXT NOT NULL,
            message TEXT NOT NULL
        );
        ",
        (), // empty list of parameters.
    ) {
        Ok(_) => {}
        Err(err) => eprint!("{}", err),
    };

    match conn.execute(
        "CREATE TABLE IF NOT EXISTS msg_attach_failure (
            id    INTEGER PRIMARY KEY,
            time  TEXT NOT NULL,
            user_name	TEXT NOT NULL,
            name  TEXT NOT NULL,
            ext TEXT NOT NULL,
            input  TEXT NOT NULL,
            ouput  TEXT NOT NULL,
            message TEXT NOT NULL
        );
        ",
        (), // empty list of parameters.
    ) {
        Ok(_) => {}
        Err(err) => eprint!("{}", err),
    };

    match conn.execute(
        "CREATE TABLE IF NOT EXISTS export_dir_path (
            id    INTEGER PRIMARY KEY,
            time  TEXT NOT NULL,
            name  TEXT NOT NULL,
            path TEXT NOT NULL UNIQUE,
            ouput TEXT NOT NULL,
            rename	TEXT,
            thumbnail	BLOB
        );
        ",
        (), // empty list of parameters.
    ) {
        Ok(_) => {}
        Err(err) => eprint!("{}", err),
    };

    match conn.execute(
        "CREATE TABLE IF NOT EXISTS 'user_wx_root_history' (
             time	TEXT,
             path	TEXT UNIQUE,
             name	TEXT
         );",
        (), // empty list of parameters.
    ) {
        Ok(_) => {}
        Err(err) => eprint!("{}", err),
    };

    match conn.execute(
        "CREATE UNIQUE INDEX IF NOT EXISTS 'attach_export_ok' ON 'msg_attach_export' ('input');",
        (), // empty list of parameters.
    ) {
        Ok(_) => {}
        Err(err) => eprint!("{}", err),
    };


    match conn.execute(
        "CREATE TABLE IF NOT EXISTS 'user_remark' (
            time	TEXT,
            wxid	TEXT,
            attach_id	TEXT,
            remark_name TEXT
         );",
        (), // empty list of parameters.
    ) {
        Ok(_) => {}
        Err(err) => eprint!("{}", err),
    };
    make_header_key("export_dir_path","version","NUMERIC");
    make_header_key("export_dir_path","thumbnail","BLOB");
}

pub fn has_header_key(table_name:&str,key:&str) -> bool {
    let mut result = false;

    let conn: Connection = match Connection::open(APP_DB_NAME) {
        Ok(conn) => conn,
        Err(e) => {
            console_log!(format!("[格式判断] 数据库内部错误因为 ->  {:?}", e));
            return result;
        }
    };


    if let Ok(mut stmt) = conn.prepare("
        SELECT 
            CASE WHEN COUNT(*) > 0 THEN 1 ELSE 0 END AS has_key_column
        FROM 
           pragma_table_info(?1)
        WHERE
            name = ?2
     ") {
       
        let has_key_column: bool = stmt.query_row(rusqlite::params![table_name,key], |row| row.get(0)).unwrap_or_else(|_|{false});
        result=has_key_column
    }
    conn.close();
    result 
}

pub fn make_header_key(table_name:&str,key:&str,type_name:&str) -> bool{
    let mut result = false;

    let conn: Connection = match Connection::open(APP_DB_NAME) {
        Ok(conn) => conn,
        Err(e) => {
            console_log!(format!("[配置格式] 数据库内部错误因为 ->  {:?}", e));
            return result;
        }
    };

      if let Ok(mut stmt) = conn.prepare("
        SELECT 
            CASE WHEN COUNT(*) > 0 THEN 1 ELSE 0 END AS has_key_column
        FROM 
           pragma_table_info(?1)
        WHERE
            name = ?2
     ") {
       
        let has_key_column: bool = stmt.query_row(rusqlite::params![table_name,key], |row| row.get(0)).unwrap_or_else(|_|{false});
        if has_key_column {
            return false;
        }
    }

    if let Ok(mut stmt) = conn.execute(&format!("ALTER TABLE '{}' ADD COLUMN {} {};",table_name,key,type_name),rusqlite::params![]) {
        result=true;
    }
   
    conn.close();
    result 
}


#[derive(Debug)]
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

// 获取log数据
pub fn get_console_message() -> String {
    global_var::retrieve_vec_string("console_log").join("\n")
}

// 处理图像 (所有)
pub fn handle_walk_pictures() -> Result<()> {
   
    let mut items_dir_list: Vec<global_var_util::ExportTaskItem> =
        global_var_util::update_export_task_item_list();
        handle_walk_pictures_from_vec(items_dir_list)?;

    Ok(())
}


// 处理图像 (选择)
pub fn handle_walk_pictures_from_vec(items_dir_list:Vec<global_var_util::ExportTaskItem>) -> Result<()> {
   
    let conn: Connection = match Connection::open(APP_DB_NAME) {
        Ok(conn) => conn,
        Err(e) => {
            console_log!(format!("[处理图像] 数据库内部错误因为 ->  {:?}", e));
            return Ok(());
        }
    };

    if get_bool!(HANDLE_DAT_ING) {
        console_log!(format!("[处理]  取消处理 -> 任务重复"));
        return Ok(());
    }

    if !libWxIkunPlus::has_auto_sync() {
        console_log!(format!("[处理]  取消处理 -> 总同步开关要求禁用"));
        return Ok(());
    }

    set_bool!(HANDLE_DAT_ING, true);

    let mut handle_end_size = 0;
    let mut handle_all_size = 0;

    console_log!(format!(
        "开始全量扫描 -> 需要处理的文件夹有:{}",
        items_dir_list.len()
    ));

    for task_item in items_dir_list.iter() {
        let mut path_item = wh_mod::parse_dat2var_path(task_item.path.clone());

        if !task_item.is_sync() {
            console_log!(format!("[同步取消(全量)]{}",&task_item.name));
            continue;
        }

        // 深度枚举
        let pattern = format!(
            "{}",
            Path::new(&path_item.attach_dir)
                .join("**/*.dat")
                .display()
                .to_string()
        );

        if (!libWxIkunPlus::has_auto_sync()) {
            console_log!("[跳出] 因为用户关闭自动同步 已经退出扫描".to_string());

            break;
        };

        console_log!(format!("[开始] 当前正在处理-> {}",&pattern));
        let mut push_break_len:usize = 0;

        // 处理路径
        for entry in glob(&pattern).unwrap() {
            let path = entry.unwrap().display().to_string();
            let base = Path::new(&path).file_name().unwrap().to_str().unwrap();

            let contains_value = conn.query_row(
                &format!(
                    "SELECT 1 FROM msg_attach_export WHERE input LIKE '%{}%';",
                    path
                ),
                [],
                |row| row.get::<_, i32>(0),
            );

            let mut has_push = false;
            
            Sleep(25);

            match contains_value {
                Ok(1) => has_push = false,
                Ok(_) => has_push = true,
                Err(err) => {
                    if util::str_eq_str(format!("{}", err), "Query returned no rows".to_string()) {
                        has_push = true;
                    } else {
                        has_push = false;
                    }
                }
            }
            handle_all_size = handle_all_size + 1;

            if (!libWxIkunPlus::has_auto_sync()) {
                console_log!("[跳出] 因为用户关闭自动同步 已经退出扫描".to_string());
                break;
            };

            if has_push {
                let bat_path = path.clone();
                if path_item.exists(bat_path.clone()) {
                    Sleep(150);


                    // match wh_mod::convert::convert_bat_images(  (&bat_path.clone()).into(), ouput_path.clone().into(),) 
                    match  wh_mod::Dat2VarParseMeta::writeFile(&bat_path, task_item.clone())
                    {
                        Ok(path2) => {
                            let itme: MsgAttachExport = MsgAttachExport {
                                id: 0,
                                time: Local::now().format("%Y-%m-%d").to_string(),
                                name: base.to_owned(),
                                user_name: task_item.name.to_owned(),
                                ouput: path2.clone(),
                                ext: util::path_extension_str(&path2),
                                input: path.to_owned(),
                                message: "successful".to_owned(),
                            };

                           
                            handle_end_size = handle_end_size+1;
                            
                            // 前十条直接显示处理的路径
                            if handle_end_size < 10 {
                                console_log!(format!(
                                    "[已处理(全)]  {:?} -> {}",
                                    itme.user_name, itme.input
                                ));
                            }else{
                                
                                if handle_end_size==15{
                                    console_log!(format!("[扩展(全)]  本次的处理数量可能比较多 已经停止界面日志输出 改为每25条更新一次")); 
                                }

                                if handle_end_size % 25 == 0 {
                                    console_log!(format!("[已处理(全)]  当前已处理{}条",handle_end_size)); 
                                }
                            }

                            let _ = &conn.execute(
                                    "INSERT INTO msg_attach_export (time,name,ext,input,ouput,message,user_name) values (?1, ?2, ?3, ?4 ,?5, ?6, ?7)",
                                    [itme.time,itme.name,itme.ext,itme.input,itme.ouput,itme.message,itme.user_name],
                                )?;
                        }
                        Err(err) => {
                            let itme: MsgAttachExport = MsgAttachExport {
                                id: 0,
                                time: Local::now().format("%Y-%m-%d").to_string(),
                                user_name: task_item.name.to_owned(),
                                name: base.to_owned(),
                                ouput: "".to_owned(),
                                ext: "".to_owned(),
                                input: path.to_owned(),
                                message: err.to_string().to_owned(),
                            };
                            console_log!(format!("[失败]  {:?} -> {} 因为->{:?}", itme.user_name, itme.input,err.to_string()));
                            let _ = &conn.execute(
                                    "INSERT INTO msg_attach_failure (time,name,ext,input,ouput,message,user_name) values (?1, ?2, ?3, ?4 ,?5, ?6, ?7)",
                                    [itme.time,itme.name,itme.ext,itme.input,itme.ouput,itme.message,itme.user_name],
                                )?;
                        }
                    }
                }
            }else{
                push_break_len= push_break_len+1;

                if push_break_len < 10 {
                    console_log!(format!("[跳过重复]  {:?}",&task_item.ouput));
                }else{
                    
                    if push_break_len==15{
                        console_log!(format!("[扩展]  本次的处理重复的文件可能有点多 已经停止界面日志输出 改为每25条更新一次")); 
                    }

                    if push_break_len % 25 == 0 {
                        console_log!(format!("[已跳过]  当前已跳过重复的文件{}条",push_break_len)); 
                    }
                }
            }
        }

        console_log!(format!( "[扫描] 当前步骤完成共计{}条 本次添加了-> [{}] 跳过 [{}]" , handle_all_size, handle_end_size,push_break_len));

    }

    set_bool!(HANDLE_DAT_ING, false);
    conn.close();
    Ok(())
}


pub fn handle_pictures_itme(pic_path: String,ouput_dir: String,expor_itme: global_var_util::ExportTaskItem,) -> Result<()> {
    let mut buf = false;
    if !libWxIkunPlus::has_auto_sync() {
        console_log!(format!("[跳过] 因为同步被禁用 {} 被跳过",&pic_path));
      return Ok(());
    }

    if !expor_itme.is_sync() {
        // console_log!(format!("[同步取消]{}",&expor_itme.name));
        return Ok(());
    }


    let base = Path::new(&pic_path).file_name().unwrap().to_str().unwrap();

    // let ouput_path = format!(
    //     "{}\\{}@{}",
    //     &ouput_dir.as_str(),
    //     Local::now().format("%Y-%m-%d_%H_%M_%S_%3f").to_string(),
    //     base
    // );

   
    let conn: Connection = Connection::open(APP_DB_NAME)?; //.expect("无法 创建/打开 数据库");

    let contains_value = conn.query_row(
        &format!(
            "SELECT 1 FROM msg_attach_export WHERE input LIKE '%{}%';",
            pic_path
        ),
        [],
        |row| row.get::<_, i32>(0),
    );

    let mut has_push = false;

    match contains_value {
        Ok(1) => has_push = false,
        Ok(_) => has_push = true,
        Err(err) => {
            if util::str_eq_str(format!("{}", err), "Query returned no rows".to_string()) {
                has_push = true;
                // //*eprintln!*("{}", "已存在");
            } else {
                has_push = false;
            }

            // //*eprintln!*("{}", err)
        }
    }

    Ok(if has_push {

        // match wh_mod::convert::convert_bat_images((&pic_path).into(), ouput_path.clone().into()) {
        match wh_mod::Dat2VarParseMeta::writeFile(&pic_path, expor_itme.clone()) {
            Ok(path2) => {
                

                let itme: MsgAttachExport = MsgAttachExport {
                    id: 0,
                    time: Local::now().format("%Y-%m-%d").to_string(),
                    name: base.to_owned(),
                    user_name: expor_itme.name.to_owned(),
                    ouput: path2.clone(),
                    ext: util::path_extension_str(&path2),
                    input: pic_path.to_owned(),
                    message: "successful".to_owned(),
                };

                console_log!(format!("[已处理(*)]  {:?} -> {}", itme.user_name, itme.input));

                let _ = &conn.execute(
            "INSERT INTO msg_attach_export (time,name,ext,input,ouput,message,user_name) values (?1, ?2, ?3, ?4 ,?5, ?6, ?7)",
            [itme.time,itme.name,itme.ext,itme.input,itme.ouput,itme.message,itme.user_name],
        )?;
            }
            Err(err) => {
                let itme: MsgAttachExport = MsgAttachExport {
                    id: 0,
                    time: Local::now().format("%Y-%m-%d").to_string(),
                    user_name: expor_itme.name.to_owned(),
                    name: base.to_owned(),
                    ouput: String::new(),
                    ext: String::new(),
                    input: pic_path.to_owned(),
                    message: err.to_string().to_owned(),
                };
                let _ = &conn.execute(
            "INSERT INTO msg_attach_failure (time,name,ext,input,ouput,message,user_name) values (?1, ?2, ?3, ?4 ,?5, ?6, ?7)",
            [itme.time,itme.name,itme.ext,itme.input,itme.ouput,itme.message,itme.user_name],
        )?;
            }
        }
    })
}

// 处理命令行
pub fn handle_commandLine() -> Result<()> {
    let args: std::env::Args = std::env::args();
    let mut args: Vec<String> = Vec::new();
    let mut is_show = true;

    std::env::args().for_each(|value| {
        if (str_eq_str(value.clone(), "-startup".to_owned())) {
            is_show = false;
        } else {
            args.push(value);
        }
    });

    if (is_show) {
        libWxIkunPlus::setInitWindowIsDisplayed(true);
        // env::set_var("K9V7OKIIMR1E1_theInitializationWindowIsDisplayed", "true");
    }

    Ok(())
}

