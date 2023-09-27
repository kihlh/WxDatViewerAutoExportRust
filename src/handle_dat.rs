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
use serde_json::json;
use std::collections::hash_map::DefaultHasher;
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

use crate::{console_log, global_var, util::{self, str_eq_str, Sleep}, wh_mod};

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
            ouput TEXT NOT NULL
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

// 向log台发送信息
pub fn push_console_message(message: String) {
    let mut user_key = "ikun_user_auto_console_info";

    match env::var(user_key) {
        Ok(varStr) => {
            let mut new_value = String::new();
            new_value.push_str(&varStr);
            new_value.push_str(";;;");
            new_value.push_str(&message);
            env::set_var(user_key, new_value);
        }
        Err(_) => {
            env::set_var(user_key, message);
        }
    }
}

// 获取log数据
pub fn get_console_message() -> String {
    // let mut user_key = "ikun_user_auto_console_info";

    // match env::var(user_key) {
    //     Ok(varStr) => {
    //         let new_str = format!("\n{}", varStr.replace(";;;", "\n"));
    //         env::set_var(user_key, "");
    //         return new_str;
    //     }
    //     Err(_) => {
    //         return "".to_owned();
    //     }
    // }

    global_var::retrieve_vec_str("console_log").join("\n")
}

// 处理图像 (所有)
pub fn handle_walk_pictures(conn: &Connection) -> Result<()> {
    if (!util::getVarBooleanValue("ikun_user_auto_disable_sync".to_owned())) {
        let mut items_dir_list: Vec<global_var::ExportDirItme> =
            global_var::update_export_dir_itme_list();
        let mut handle_end_size = 0;
        let mut handle_all_size = 0;

        console_log!(format!(
            "开始全量扫描 -> 需要处理的文件夹有:{}",
            items_dir_list.len()
        ));

        for item_path in items_dir_list.iter() {
            let mut path_item = wh_mod::parse_dat_path(item_path.path.clone());

            // 深度枚举
            let pattern = format!(
                "{}",
                Path::new(&path_item.attach_dir)
                    .join("**/*.dat")
                    .display()
                    .to_string()
            );

            if (util::getVarBooleanValue("ikun_user_auto_disable_sync".to_owned())) {
                break;
            };

            // 处理路径
            for entry in glob(&pattern).unwrap() {
                let path = entry.unwrap().display().to_string();
                let base = Path::new(&path).file_name().unwrap().to_str().unwrap();

                let ouput_path = format!(
                    "{}\\{}@{}",
                    &item_path.ouput.as_str(),
                    Local::now().format("%Y-%m-%d_%H_%M_%S_%3f").to_string(),
                    base
                );

                let contains_value = conn.query_row(
                    &format!(
                        "SELECT 1 FROM msg_attach_export WHERE input LIKE '%{}%';",
                        path
                    ),
                    [],
                    |row| row.get::<_, i32>(0),
                );

                let mut has_push = false;

                Sleep(150);

                match contains_value {
                    Ok(1) => has_push = false,
                    Ok(_) => has_push = true,
                    Err(err) => {
                        if util::str_eq_str(
                            format!("{}", err),
                            "Query returned no rows".to_string(),
                        ) {
                            has_push = true;
                            // eprintln!("{}", "已存在");
                        } else {
                            has_push = false;
                        }

                        // eprintln!("{}", err)
                    }
                }
                handle_all_size = handle_all_size + 1;
                if has_push {
                    let bat_path =  path.clone();
                    if path_item.exists(bat_path.clone()) {
                        match wh_mod::convert::convert_bat_images((&bat_path.clone()).into(), ouput_path.clone().into()) {
                            Ok(path2) => {
                                // println!("{}", path);
                                // println!("{}", &ouputPath);
                                let itme: MsgAttachExport = MsgAttachExport {
                                    id: 0,
                                    time: Local::now().format("%Y-%m-%d").to_string(),
                                    name: base.to_owned(),
                                    user_name: item_path.name.to_owned(),
                                    ouput: path2.clone(),
                                    ext: util::path_extension_str(&path2),
                                    input: path.to_owned(),
                                    message: "successful".to_owned(),
                                };

                                console_log!(format!(
                                "[已处理]  {:?} -> {}",
                                itme.user_name, itme.input
                            ));
                                handle_end_size = handle_end_size + 1;
                                // push_console_message(format!(
                                //     "[已处理]  {:?} -> {}",
                                //     itme.user_name, itme.input
                                // ));
                                // println!("[push]  {:?} -> {}", itme.user_name, itme.input);

                                let _ = &conn.execute(
                                    "INSERT INTO msg_attach_export (time,name,ext,input,ouput,message,user_name) values (?1, ?2, ?3, ?4 ,?5, ?6, ?7)",
                                    [itme.time,itme.name,itme.ext,itme.input,itme.ouput,itme.message,itme.user_name],
                                )?;
                            }
                            Err(err) => {
                                let itme: MsgAttachExport = MsgAttachExport {
                                    id: 0,
                                    time: Local::now().format("%Y-%m-%d").to_string(),
                                    user_name: item_path.name.to_owned(),
                                    name: base.to_owned(),
                                    ouput: "".to_owned(),
                                    ext: "".to_owned(),
                                    input: path.to_owned(),
                                    message: err.to_string().to_owned(),
                                };
                                console_log!(format!("[失败]  {:?} -> {}", itme.user_name, itme.input));
                                let _ = &conn.execute(
                                    "INSERT INTO msg_attach_failure (time,name,ext,input,ouput,message,user_name) values (?1, ?2, ?3, ?4 ,?5, ?6, ?7)",
                                    [itme.time,itme.name,itme.ext,itme.input,itme.ouput,itme.message,itme.user_name],
                                )?;
                            }
                        }
                    }
                }
            }
        }

        console_log!(format!(
            "[全量扫描] 处理完成共计{}条 本次添加了-> [{}]",
            handle_all_size, handle_end_size
        ));
    } else {
        push_console_message(format!("[处理]  取消处理 -> 已被禁用"));
        println!("取消处理 -> 已被禁用");
    };
    Ok(())
}

pub fn handle_pictures_itme(
    pic_path: String,
    ouput_dir: String,
    expor_itme: global_var::ExportDirItme,
) -> Result<()> {
    let mut buf = false;

    let base = Path::new(&pic_path).file_name().unwrap().to_str().unwrap();

    let ouput_path = format!(
        "{}\\{}@{}",
        &ouput_dir.as_str(),
        Local::now().format("%Y-%m-%d_%H_%M_%S_%3f").to_string(),
        base
    );

    let conn: Connection = Connection::open("ikun_user_data.db")?; //.expect("无法 创建/打开 数据库");

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
                // eprintln!("{}", "已存在");
            } else {
                has_push = false;
            }

            // eprintln!("{}", err)
        }
    }

    Ok(if has_push {
        match wh_mod::convert::convert_bat_images((&pic_path).into(), ouput_path.clone().into()) {
            Ok(path2) => {
                // println!("{}", path);
                // println!("{}", &ouputPath);
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

                console_log!(format!("[已处理]  {:?} -> {}", itme.user_name, itme.input));

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
                    ouput: "".to_owned(),
                    ext: "".to_owned(),
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
        env::set_var("K9V7OKIIMR1E1_theInitializationWindowIsDisplayed", "true");
    }

    if args.len() > 1 {
        assert_eq!(args.len(), 4 , "添加 处理的文件夹需要传入3个参数 [ 用户别名  用户图片路径  用户图片存储位置 ] 当前传入为 {} ",args.len());
    }
    // push export_dir_path

    if args.len() == 4 {
        let conn: Connection =
            Connection::open("ikun_user_data.db").expect("无法 创建/打开 数据库");
        initialize_table(&conn);

        let itme: global_var::ExportDirItme = global_var::ExportDirItme {
            name: args[1].to_owned(),
            id: 0,
            time: Local::now().format("%Y-%m-%d").to_string(),
            path: args[2].to_owned(),
            ouput: args[3].to_owned(),
        };

        match conn.execute(
            "INSERT INTO export_dir_path (name,time,path,ouput) values (?1, ?2, ?3, ?4)",
            [itme.name, itme.time, itme.path, itme.ouput],
        ) {
            Ok(_) => {
                eprintln!("创建成功");
            }
            Err(err) => {
                eprintln!("创建失败 因为=> {} ", err)
            }
        }
        process::exit(0);
    }
    Ok(())
}
