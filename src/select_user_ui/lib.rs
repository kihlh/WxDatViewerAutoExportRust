use chrono::Local;
use rusqlite::Connection;

use crate::{atomic_util, get_arc_bind_variable, global_var, gui_detect_config, gui_drag_scan, gui_hotspot, gui_imge::{self, ImgPreview}, gui_text_control, handle_dat, libWxIkunPlus::getFocusTopWindow, read_rw_lazy_lock, read_rw_lock, set_arc_bind_variable, set_arc_bind_variable_insert, set_arc_bind_variable_vec_clear, set_arc_bind_variable_vec_replace_data, util::{str_eq_str, Sleep}, wh_mod::{self, AttachThumbnail}, write_rw_lock, write_rw_lock_insert, gui_util, libWxIkunPlus};

use std::{
    sync::atomic::{AtomicUsize, Ordering},
    sync::{Arc, Condvar, Mutex, RwLock},
    thread, time::UNIX_EPOCH, collections::HashMap, path::{Path, PathBuf},
};
use std::sync::atomic::AtomicBool;
use crate::select_user_ui::WINDOW_CLASS_NAME;

static HAS_SELECT_USER_WINDOW_NORMAL: AtomicBool = AtomicBool::new(false);

// 图片预览 全局变量
static mut IMG_PREVIEW_LIST: Vec<gui_util::img::ImgPreview> = Vec::new();
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
    // thread::spawn(move || {
        let conn: Connection = Connection::open("ikun_user_data.db").unwrap();
        handle_dat::initialize_table(&conn);

        match conn.execute(
            "DELETE FROM user_remark WHERE wxid = ?1 AND attach_id = ?2",
            [wxid.clone(), attach_id.clone()],
        ) {
            Ok(updated) => {
            }
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
            Ok(_) => {
                gui_util::message::sub_message(libWxIkunPlus::findWindow(WINDOW_CLASS_NAME,""),gui_util::message::IconType::Success,"当前别名备注已经更新",5000u64);

            }
            Err(err) => {
                gui_util::message::sub_message(libWxIkunPlus::findWindow(WINDOW_CLASS_NAME,""),gui_util::message::IconType::Failure,format!("别名更新失败 因为-> {:?}",err).as_str(),5000u64);

            }
        }

        conn.close();
    // });
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

// 更新进视图
pub fn update_thumbnail_preview_list()  {
        // 取出缩略图列表 并将其缩减到5条以内
        let mut thumbnail_list = {

            let mut thumbnail_list =
                get_arc_bind_variable!(THUMBNAIL_LIST, THUMBNAIL_LIST_BIND).to_vec();
            let mut atid_list: HashMap<String, AttachThumbnail> = HashMap::new();
    
            for value in thumbnail_list {
                let key = value.attach_id.clone();
                let mut oid_created = UNIX_EPOCH;
                let mut new_created = UNIX_EPOCH;
    
                // oid create time
                if let Some(thumbnail) = atid_list.get(&key) {
                    if let Ok(metadata) = std::fs::metadata(thumbnail.thumbnail_path.clone()) {
                        if let Result::Ok(create) = metadata.created() {
                            oid_created = create;
                        }
                    }
                }
    
                // new create time
                if let Ok(metadata) = std::fs::metadata(value.thumbnail_path.clone()) {
                    if let Result::Ok(create) = metadata.created() {
                        new_created = create;
                    }
                }
    
                // 按照创建时间判断是否更新视图
                if (new_created > oid_created) {
                    atid_list.insert(value.attach_id.clone(), value);
                }
            }
    
    
            let mut thumbnail_list: Vec<AttachThumbnail> = Vec::new();
    
            for (key, value) in atid_list {
                thumbnail_list.push(value);
            }
    
            // 排序
            thumbnail_list.sort_by(|a, b| {
                let mut a_created = UNIX_EPOCH;
                let mut b_created = UNIX_EPOCH;
    
                if let Ok(metadata) = std::fs::metadata(a.thumbnail_path.clone()) {
                    if let Result::Ok(create) = metadata.created() {
                        a_created = create;
                    }
                }
    
                if let Ok(metadata) = std::fs::metadata(b.thumbnail_path.clone()) {
                    if let Result::Ok(create) = metadata.created() {
                        b_created = create;
                    }
                }
    
                a_created.cmp(&b_created)
            });
    
            let mut new_thumbnail_list = Vec::new();
    
            thumbnail_list.reverse();
            for value in thumbnail_list {
                if (new_thumbnail_list.len() > 5 - 1) {
                    break;
                }
                new_thumbnail_list.push(value);
            }
    
            new_thumbnail_list
        };

        set_arc_bind_variable!(THUMBNAIL_LIST, THUMBNAIL_LIST_BIND, thumbnail_list.to_vec());
    
    
        // 更新到视图中
        let thumbnail_list = get_arc_bind_variable!(THUMBNAIL_LIST, THUMBNAIL_LIST_BIND);
    
        // 锁定缩略图更新
        let mutex = Arc::new(Mutex::new(&THUMBNAIL_LIST_BIND));
        mutex.lock();
    
        let img_preview_list = get_arc_bind_variable!(IMG_PREVIEW_LIST, IMG_PREVIEW_LIST_BIND);
    
        let (width, height) = (75, 75);

        if thumbnail_list.is_empty(){
            gui_util::message::sub_message(libWxIkunPlus::findWindow(WINDOW_CLASS_NAME,""),gui_util::message::IconType::Warning,"没有发现图片列表 可以找开发者反馈",5000u64);
        }

        // 更新到视图中  
        for index in 0..img_preview_list.len() {
            if let Some(mut img_preview) = img_preview_list.get(index) {
                if let Some(thumbnail) = thumbnail_list.get(index) {
                    img_preview.clone().from_data(
                        thumbnail.thumbnail.clone(),
                        -1,
                        -1,
                        width - 2,
                        height - 2,
                    );
                } else {
                    img_preview.clone().from_data(
                        include_bytes!("./src/not.png").to_vec(),
                        -1,
                        -1,
                        width - 2,
                        height - 2,
                    );
                }
            }
        }

        drop(mutex);

}

// 初始化五张图片到视图
pub fn initialize_thumbnail_preview(user_root: &str,wxid: &str){
    
    let msg_attach_dir = PathBuf::from(format!("{}\\{}\\FileStorage\\MsgAttach",user_root,wxid).as_str());

    println!("msg_attach_dir-> {:?}",msg_attach_dir);

    let mut read_imag_list = wh_mod::read_attach_buff_thumbnail_list_v2(&msg_attach_dir.as_path(), 5, 1);

    // v2 没有内容就是说明这个库可能修改时间有误 尝试用v1 获取
    if read_imag_list.is_empty() {
        read_imag_list = wh_mod::read_attach_buff_thumbnail_list(&msg_attach_dir.as_path(), 5, 1);
    }

    set_arc_bind_variable_vec_replace_data!(THUMBNAIL_LIST,THUMBNAIL_LIST_BIND,read_imag_list);
    update_thumbnail_preview_list();

}

// 开始获取更新
pub fn initialize_watch_path_puppet(path: String) {
    std::thread::spawn(move ||{
        // 启动日志检测模式
        let (tx, rx) = std::sync::mpsc::channel();

        let wh_id = wh_mod::watch_path::watch_path_puppet(path.clone(), tx);
        println!("copy_path_wake-> {}", path.clone());
        while wh_id == wh_mod::watch_path::get_the_id() {
            if let Result::Ok(data) = rx.recv() {
                let path = data.join("..").join("..").join("..");
                let data_list = wh_mod::read_attach_buff_thumbnail_data(&path, 1);
                // write_rw_lock_insert!(THUMBNAIL_LIST_ARC, data_list.to_vec());
                set_arc_bind_variable_insert!(
                                THUMBNAIL_LIST,
                                THUMBNAIL_LIST_BIND,
                                data_list.to_vec()
                            );

                if (data_list.len() > 0) {
                   update_thumbnail_preview_list();
                }
            }
        }

    });
}

macro_rules! gc_select_user_ui {
   ()=>{
        if !atomic_util::get_bool(&HAS_SELECT_USER_WINDOW_NORMAL){
                println!("[gc] initialize_gc_select_user_ui");
                wh_mod::watch_path::un_next_exits();
                global_var::set_string("user::config::user_select_path", String::new());
                global_var::set_string("user::config::user_select_wxid", String::new());
                set_arc_bind_variable_vec_clear!(THUMBNAIL_LIST,THUMBNAIL_LIST_BIND);
                set_arc_bind_variable_vec_clear!(IMG_PREVIEW_LIST,IMG_PREVIEW_LIST_BIND);
                println!("[gc] initialize_gc_select_user_ui ok! ");
                return;
            }
   }
}

// 自动在窗口销毁时候自动清理
pub fn initialize_gc_select_user_ui(hwnd:i128){

    if atomic_util::get_bool(&HAS_SELECT_USER_WINDOW_NORMAL){
        return;
    }

    atomic_util::set_bool(&HAS_SELECT_USER_WINDOW_NORMAL, true);
    
    thread::spawn(move ||{
        loop {
            if !atomic_util::get_bool(&HAS_SELECT_USER_WINDOW_NORMAL)||!libWxIkunPlus::isWindow(hwnd){
                println!("[gc] initialize_gc_select_user_ui");
                atomic_util::set_bool(&HAS_SELECT_USER_WINDOW_NORMAL,false);
                wh_mod::watch_path::un_next_exits();
                global_var::set_string("user::config::user_select_path", String::new());
                global_var::set_string("user::config::user_select_wxid", String::new());
                set_arc_bind_variable_vec_clear!(THUMBNAIL_LIST,THUMBNAIL_LIST_BIND);
                set_arc_bind_variable_vec_clear!(IMG_PREVIEW_LIST,IMG_PREVIEW_LIST_BIND);
                println!("[gc] initialize_gc_select_user_ui ok! ");
                return;
            }
            Sleep(150);
        }
    });
}

// 绑定视图5张的显示控件
pub fn initialize_img_preview_list (img_preview_list:&Vec<gui_util::img::ImgPreview>){
    use std::sync::{Arc, Condvar, Mutex, RwLock};
    let mutex = Arc::new(Mutex::new(&IMG_PREVIEW_LIST_BIND));
    mutex.lock();
    let the_value: usize = IMG_PREVIEW_LIST_BIND.load(Ordering::SeqCst);
    unsafe {
        IMG_PREVIEW_LIST.clear();
        for value in img_preview_list {
            IMG_PREVIEW_LIST.push(value.clone())

        }
    }
    IMG_PREVIEW_LIST_BIND.store(the_value + 1, Ordering::SeqCst);

    drop(mutex);

    // set_arc_bind_variable_vec_replace_data!(IMG_PREVIEW_LIST,IMG_PREVIEW_LIST_BIND,img_preview_list);
}

// GC掉大部分高内存的存储
pub fn gc_select_user_ui(){
    atomic_util::set_bool(&HAS_SELECT_USER_WINDOW_NORMAL,false);
    gc_select_user_ui!();
}

// 获取缩略图绑定列表
pub fn get_thumbnail_list() -> Vec<AttachThumbnail> {
    get_arc_bind_variable!(THUMBNAIL_LIST,THUMBNAIL_LIST_BIND).clone()
}

