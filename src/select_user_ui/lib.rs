use chrono::Local;
use rusqlite::Connection;

use crate::{
    get_arc_bind_variable, global_var, gui_detect_config, gui_drag_scan, gui_hotspot,
    gui_imge::{self, ImgPreview},
    gui_text_control, handle_dat,
    libWxIkunPlus::getFocusTopWindow,
    read_rw_lazy_lock, read_rw_lock, set_arc_bind_variable, set_arc_bind_variable_insert,
    util::{str_eq_str, Sleep},
    wh_mod::{self, AttachThumbnail},
    write_rw_lock, write_rw_lock_insert,
};
use std::{
    sync::atomic::{AtomicUsize, Ordering},
    sync::{Arc, Condvar, Mutex, RwLock},
    thread,
};

static REQUEST_RECV: AtomicUsize = AtomicUsize::new(0);
// static mut static_var:Vec<ImgPreview> = Vec::new();
// static mut static_atomic :AtomicUsize = AtomicUsize::new(0);

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
    thread::spawn(move || {
        let conn: Connection = Connection::open("ikun_user_data.db").unwrap();
        handle_dat::initialize_table(&conn);

        match conn.execute(
            "DELETE FROM user_remark WHERE wxid = ?1 AND attach_id = ?2",
            [wxid.clone(), attach_id.clone()],
        ) {
            Ok(updated) => {}
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
            Ok(_) => {}
            Err(err) => {}
        }

        conn.close();
    });
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
