#![allow(dropping_references)]

use crate::util::{str_eq_str, Sleep};
use crate::{console_log, global_var, util};
use lazy_static::lazy_static;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{mpsc, OnceLock};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

static WATCH_PATH_ID: AtomicUsize = AtomicUsize::new(0);

lazy_static! {
    static ref WATCH_NEXT_EXITS: Mutex<bool> = Mutex::new(false);
}

struct TmepMetadata {
    pub metadata: fs::Metadata,
    pub dir: std::path::PathBuf,
}

pub fn has_next_exits() -> bool {
    let mut result: bool = false;

    let mut lazy_value = WATCH_NEXT_EXITS.lock().unwrap();

    result = *lazy_value;
    drop(lazy_value);

    result
}

pub fn un_next_exits() -> bool {
    let mut result: bool = false;

    let mut lazy_value = WATCH_NEXT_EXITS.lock().unwrap();

    *lazy_value = false;
    drop(lazy_value);

    result
}
pub fn initialize_next_exits() -> bool {
    let mut result: bool = false;

    let mut lazy_value = WATCH_NEXT_EXITS.lock().unwrap();

    *lazy_value = true;
    drop(lazy_value);

    result
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

        let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();
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

        for res in rx {
            // 需要处理的任务已经更新了 释放 为什么不用un 因为会误操作其他的

            match res {
                Ok(event) => {
                    for value in event.clone().paths {
                        let mut paths = value.clone().display().to_string();
                        let mut ext = util::path_extension(&value);

                        // 是文件 后缀是dat 更新方式是修改
                        if (value.is_file()
                            && (event.clone().kind.is_modify())
                            && str_eq_str("dat".to_owned(), ext.clone()))
                        {
                            send_main_tx.send(value.clone());
                            println!("is_modify [is_modify] -> {:?}  id ->  {}", value.clone(),watch_puppet_id.clone());
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


