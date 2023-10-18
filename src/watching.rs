#![allow(warnings, unused)]

use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::{
    collections::HashMap,
    path::Path,
    sync::atomic::Ordering,
    sync::Arc,
    sync::{atomic::AtomicUsize, OnceLock},
    thread,
};

use crate::{
    global_var::{self, push_vec_string},
    global_var_util,
    handle_dat,
    // handle_dat::{self, push_console_message},
    push_map_vec_bind_variable,
    util::{self, str_eq_ostr, str_eq_str, Sleep},
    wh_mod,
};
// use lazy_static::lazy_static;
use std::collections::HashSet;
use std::sync::Mutex;
use std::sync::MutexGuard;
use crate::console_log;
// lazy_static! {
//     static ref WARCHER_CHANGE_LIST: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
//     static ref WARCHER_PATH_LIST: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
// }

static mut WARCHER_CHANGE_LIST: Option<HashSet<String>> = Option::None;
static WARCHER_CHANGE_LIST_BIND: AtomicUsize = AtomicUsize::new(0);

static mut WARCHER_PATH_LIST: Option<HashSet<String>> = Option::None;
static WARCHER_PATH_LIST_BIND: AtomicUsize = AtomicUsize::new(0);

static VARIABLE_INITIALIZE_WATCH_PATH_TOKEN: OnceLock<bool> = OnceLock::new();

fn initialize_watch_path_token() {
    if *(VARIABLE_INITIALIZE_WATCH_PATH_TOKEN
        .get()
        .unwrap_or_else(|| &false))
    {
        return;
    }
    unsafe {
        if WARCHER_CHANGE_LIST.is_none() {
            WARCHER_CHANGE_LIST.replace(HashSet::new());
        }
    }

    unsafe {
        if WARCHER_PATH_LIST.is_none() {
            WARCHER_PATH_LIST.replace(HashSet::new());
        }
    }

    VARIABLE_INITIALIZE_WATCH_PATH_TOKEN.set(true);
}

// 添加变化路径令牌 (有此路径在全局变量中才会被处理 否则会在更新时候直接停止 )
pub fn insert_watch_path_token(path: String) {
    initialize_watch_path_token();
    // let mut lazy_value = WARCHER_PATH_LIST.lock().unwrap();
    // lazy_value.insert(path.clone());
    // println!("insert_watch_path_token-> {}", path);
    // drop(lazy_value);
    // push_map_vec_bind_variable!(WARCHER_PATH_LIST,WARCHER_PATH_LIST_BIND,);

    let mutex = Arc::new(Mutex::new(&WARCHER_PATH_LIST_BIND));
    mutex.lock();
    let the_value: usize = WARCHER_PATH_LIST_BIND.load(Ordering::SeqCst);
    unsafe {
        if let Some(item) = WARCHER_PATH_LIST.as_mut() {
            item.insert(path);
        }
        println!("insert_watch_path_token -> {:?}",&WARCHER_PATH_LIST.as_mut().unwrap().len());
    };

    WARCHER_PATH_LIST_BIND.store(the_value + 1, Ordering::SeqCst);
    drop(mutex);
}

// 判断此令牌是否有效
pub fn has_watch_path_token(path: String) -> bool {
    initialize_watch_path_token();
    let mut token = false;
    // let mut lazy_value = WARCHER_PATH_LIST.lock().unwrap();
    // for value in lazy_value.iter() {
    //     if (str_eq_str(value.to_string(), path.clone())) {
    //         token = true;
    //     }
    // }
    // drop(lazy_value);

    let mutex = Arc::new(Mutex::new(&WARCHER_PATH_LIST_BIND));
    mutex.lock();
    let the_value: usize = WARCHER_PATH_LIST_BIND.load(Ordering::SeqCst);
    unsafe {
        if let Some(item) = WARCHER_PATH_LIST.as_mut() {
            for value in item.iter() {
                if (str_eq_str(value.to_string(), path.clone())) {
                    token = true;
                }
            }
        }
    };

    WARCHER_PATH_LIST_BIND.store(the_value + 1, Ordering::SeqCst);
    drop(mutex);

    return token;
}

// 移除此路径的变化令牌
pub fn remove_watch_path_token(path: String) -> bool {
    initialize_watch_path_token();
    let mut token = false;
    // let mut lazy_value = WARCHER_PATH_LIST.lock().unwrap();
    // let values: Vec<String> = lazy_value.iter().cloned().collect();
    // for value in values {
    //     if str_eq_str(value.to_string(), path.clone()) {
    //         token = true;
    //         lazy_value.retain(|x| x != &value);
    //     }
    // }
    // drop(lazy_value);

    let mutex = Arc::new(Mutex::new(&WARCHER_PATH_LIST_BIND));
    mutex.lock();
    let the_value: usize = WARCHER_PATH_LIST_BIND.load(Ordering::SeqCst);
    unsafe {
        if let Some(item) = &WARCHER_PATH_LIST {
            for value in item.iter() {
                if (str_eq_str(value.to_string(), path.clone())) {
                    token = true;
                    WARCHER_PATH_LIST.as_mut().unwrap().retain(|x| x != value);
                }
            }
        }
    };

    WARCHER_PATH_LIST_BIND.store(the_value + 1, Ordering::SeqCst);
    drop(mutex);

    return has_watch_path_token(path);
}

// 获取所有令牌
fn get_all_watch_token_path() -> Vec<String> {
    initialize_watch_path_token();
    let mut token_path_list = Vec::new();
    // let mut lazy_value = WARCHER_PATH_LIST.lock().unwrap();

    // for value in lazy_value.iter() {
    //     token_path_list.push(value.to_string());
    // }
    // drop(lazy_value);

    let mutex = Arc::new(Mutex::new(&WARCHER_PATH_LIST_BIND));
    mutex.lock();
    let the_value: usize = WARCHER_PATH_LIST_BIND.load(Ordering::SeqCst);
    unsafe {
        if let Some(item) = WARCHER_PATH_LIST.as_mut() {
            for value in item.iter() {
                token_path_list.push(value.to_string());
            }
        }
    };

    WARCHER_PATH_LIST_BIND.store(the_value + 1, Ordering::SeqCst);
    drop(mutex);

    token_path_list
}

// 更新路径令牌
fn update_watch_path_token(initialize: bool) {
    initialize_watch_path_token();
    let export_dir_itme_list: Vec<global_var_util::ExportDirItme> =
        global_var_util::get_export_dir_itme_list();

    // 添加新的已有token
    if (initialize) {
        for value in export_dir_itme_list.clone() {
            insert_watch_path_token(value.path.to_string());
        }
    }

    let mut token_path_list = get_all_watch_token_path();

    // 开始处理 需要的添加
    for value in export_dir_itme_list.clone() {
        let mut token = false;
        let path = value.path.clone();

        // 判断是否存在
        for value in token_path_list.clone() {
            if (str_eq_str(path.clone(), value)) {
                token = true;
            }
        }

        // 需要添加
        if (!token) {
            insert_watch_path_token(path.clone());
            // watch(path.clone(),value);
            let handle = thread::spawn(move || {
                if let Err(e) = watch(path.as_str(), value.clone()) {
                    console_log!(format!(
                        "[错误] 注册[ {} ] 失败，因为 -> {}",
                        value.name,
                        e.to_string()
                    ));
                }
                console_log!(format!("[状态] 开始注册此路径日志变化 [ {} ] ", value.name));
            });
        }

        // println!("{} -> {}", path.clone(), has_watch_path_token(path.clone()));
    }

    // 处理需要移除
    let mut token_path_list = get_all_watch_token_path();

    for path in token_path_list {
        let mut token = false;

        // 判断是否存在
        for value in export_dir_itme_list.clone() {
            if (str_eq_str(path.clone(), value.path.clone())) {
                token = true;
            }
        }
        if (!token) {
            remove_watch_path_token(path.clone());
        }
        // println!("{} -> {}",path,token);
    }
}

// 处理更新的文件的路径
fn change_watch_path(path: String, exp_itme: global_var_util::ExportDirItme) {
    initialize_watch_path_token();

    let mutex = Arc::new(Mutex::new(&WARCHER_CHANGE_LIST_BIND));
    mutex.lock();
    let the_value: usize = WARCHER_CHANGE_LIST_BIND.load(Ordering::SeqCst);
    unsafe {
        if let Some(lazy_value) = WARCHER_CHANGE_LIST.as_mut() {
            // let mut lazy_value = WARCHER_CHANGE_LIST.lock().unwrap();

            if (lazy_value.insert(path.clone())) {
                // console_log!(format!("[检测] 检测到文件更新-> {}", path.clone()));
                Sleep(500);
                handle_dat::handle_pictures_itme(
                    path.clone(),
                    exp_itme.ouput.clone(),
                    exp_itme.clone(),
                );
            }

            if (lazy_value.len() > 1000) {
                let mut new_lazy_value_list = Vec::new();

                for value in lazy_value.iter() {
                    new_lazy_value_list.push(value.clone());
                }
                lazy_value.clear();

                let mut leng = new_lazy_value_list.len();

                loop {
                    let itme = new_lazy_value_list.get(leng);
                    if (itme.is_some()) {
                        lazy_value.insert(itme.unwrap().to_string());
                    }
                    leng = leng + 1;
                    if (leng > 500) {
                        break;
                    };
                }
                new_lazy_value_list.clear();
            }
        }
    };

    WARCHER_CHANGE_LIST_BIND.store(the_value + 1, Ordering::SeqCst);
    drop(mutex);
}

// 启动文件系统日志模式
pub fn initialize_file_system_Change() {
    let mut warcher_len = 0;
    let mut itme_list = global_var_util::update_export_dir_itme_list();

    update_watch_path_token(true);
    for value in itme_list.clone() {
        let handle = thread::spawn(move || {
            if let Err(e) = watch(value.path.as_str(), value.clone()) {
                console_log!(format!(
                    "[错误] 注册[ {} ] 失败，因为 -> {}",
                    value.name,
                    e.to_string()
                ));
            }
        });
    }

    console_log!(format!(
        "[状态] 已经注册文件系统日志变化检测 共 [ {} ] 个文件夹",
        itme_list.len()
    ));

    thread::spawn(move || loop {
        Sleep(500);
        update_watch_path_token(false);
    });
}

// 对文件夹个体进行处理
pub fn watch<P: AsRef<Path>>(
    path: P,
    exp_item: global_var_util::ExportDirItme,
) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    let dir_path = path.as_ref().display().to_string();
    let mut dat_parse_meta = wh_mod::parse_dat2var_path(dir_path.clone());

    println!("dir_path-> {}", dir_path.clone());
    watcher.watch(
        &Path::new(dat_parse_meta.attach_dir.as_str()),
        RecursiveMode::Recursive,
    )?;

    for res in rx {
        // 检测更新令牌
        if (!has_watch_path_token(dir_path.clone())) {
            console_log!(format!(
                "[结束处理] {} 令牌结束 已经取消 ",
                dir_path.clone()
            ));
            return Ok(());
        }

        match res {
            Ok(event) => {
                for value in event.clone().paths {
                    let mut paths = value.clone().display().to_string();
                    let mut ext = util::path_extension(&value);

                    // 是文件 后缀是dat 更新方式是修改
                    if (value.is_file()
                        && event.clone().kind.is_modify()
                        && str_eq_str("dat".to_owned(), ext.clone()))
                    {
                        if (dat_parse_meta.exists(paths.clone())) {
                            change_watch_path(paths.clone(), exp_item.clone());
                        }
                    }
                }
            }
            Err(e) => {
                console_log!(format!("[报错] 检测文件更新错误 因为-> {}", e.to_string()));
            }
        }
    }

    Ok(())
}
