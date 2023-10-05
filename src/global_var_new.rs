
use lazy_static::lazy_static;
use rusqlite::Connection;
// use once_cell::sync::OnceCell;
// use std::cell::LazyCell;
use std::collections::HashMap;

use std::sync::atomic::{AtomicBool, AtomicI32, AtomicI64, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, RwLock,OnceLock,MutexGuard};

use crate::console_log;
use crate::get_arc_bind_variable;
use crate::set_arc_bind_variable;
use crate::set_map_arc_bind_variable_insert;

// lazy_static! {
//     static ref VARIABLE_U64: Mutex<HashMap<String, u64>> = Mutex::new(HashMap::new());
//     static ref VARIABLE_STRING: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
//     static ref VARIABLE_I32: Mutex<HashMap<String, i32>> = Mutex::new(HashMap::new());
//     static ref VARIABLE_I128: Mutex<HashMap<String, i128>> = Mutex::new(HashMap::new());
//     static ref VARIABLE_BOOL: Mutex<HashMap<String, bool>> = Mutex::new(HashMap::new());
//     static ref VARIABLE_VEC_STRING: Mutex<HashMap<String, Vec<String>>> =
//         Mutex::new(HashMap::new());
//     static ref VARIABLE_VEC_I32: Mutex<HashMap<String, Vec<i32>>> = Mutex::new(HashMap::new());
//     static ref VARIABLE_VEC_EXPORT_DIR_ITME: Mutex<Vec<ExportDirItme>> = Mutex::new(Vec::new());
// }

// 哈希表

static mut VARIABLE_U64: HashMap<String, u64> = HashMap::new();
static mut VARIABLE_STRING: HashMap<String, String> = HashMap::new();
static mut VARIABLE_I32: HashMap<String, i32> = HashMap::new();
static mut VARIABLE_I128: HashMap<String, i128> = HashMap::new();
static mut VARIABLE_BOOL: HashMap<String, bool>= HashMap::new();
static mut VARIABLE_VEC_STRING: HashMap<String, Vec<String>>= HashMap::new();
static mut VARIABLE_VEC_I32: HashMap<String, Vec<i32>> = HashMap::new();



static mut VARIABLE_VEC_EXPORT_DIR_ITME: Vec<ExportDirItme> = Vec::new();


// 原子锁 一对一
static VARIABLE_U64_BIND: AtomicUsize = AtomicUsize::new(0);
static VARIABLE_STRING_BIND: AtomicUsize = AtomicUsize::new(0);
static VARIABLE_I32_BIND: AtomicUsize = AtomicUsize::new(0);
static VARIABLE_I128_BIND: AtomicUsize = AtomicUsize::new(0);
static VARIABLE_BOOL_BIND: AtomicUsize = AtomicUsize::new(0);
static VARIABLE_VEC_STRING_BIND: AtomicUsize = AtomicUsize::new(0);
static VARIABLE_VEC_I32_BIND: AtomicUsize = AtomicUsize::new(0);
static VARIABLE_VEC_EXPORT_DIR_ITME_BIND: AtomicUsize = AtomicUsize::new(0);

// 全局变量 原子锁

// 结束

static VARIABLE_INITIALIZE: OnceLock<bool> = OnceLock::new();
static INITIALIZE_GET_EXPORT_DIR_TIME_LIST: OnceLock<bool> = OnceLock::new();

// 初始化值 需要提前赋值的需要在这边赋值
fn initialize() {
    if *(VARIABLE_INITIALIZE.get().unwrap_or_else(|| &false)) {
        return;
    }

    set_map_arc_bind_variable_insert!(
        VARIABLE_U64,
        VARIABLE_U64_BIND,
        String::from("SLEEP_SCAN_ALL"),
        30000
    );
    set_map_arc_bind_variable_insert!(
        VARIABLE_I32,
        VARIABLE_I32_BIND,
        String::from("user::config::select_user_thumbnail_obj"),
        -1
    );

    // 初始化bool预设值
    let value_list = [
        "user::config::check_button_the_month",
        "user::config::check_button_source",
        "user::config::check_button_thumbnail",
        "gui::open::gui_detect_config",
        "gui::open::gui_drag_scan",
        "gui::open::handle_dat",
        "gui::open::manage_item",
    ];
    for value in value_list {
        set_map_arc_bind_variable_insert!(
            VARIABLE_BOOL,
            VARIABLE_BOOL_BIND,
            value.parse().unwrap(),
            false
        );
    }

    // 初始化string 预设值
    let value_list = [
        "user::config::input_select_dir",
        "user::config::user_select_path",
        "user::config::user_select_wxid",
        "user::config::walk_drag_path",
    ];
    for value in value_list {
        for value in value_list {
            set_map_arc_bind_variable_insert!(
                VARIABLE_STRING,
                VARIABLE_STRING_BIND,
                value.parse().unwrap(),
                String::new()
            );
        }
    }

    VARIABLE_INITIALIZE.set(true);
}

#[macro_export]
macro_rules! get_arc_map_bind_variable {
    ($key:expr ,$static_var: expr,$static_atomic: expr) => {{
        let mutex = Arc::new(Mutex::new(&$static_atomic));
        mutex.lock();
        let the_value: usize = $static_atomic.load(Ordering::SeqCst);
        $static_atomic.store(the_value + 1, Ordering::SeqCst);

        let data = unsafe { $static_var.get($key) };
        drop(mutex);
        data
    }};
}


// 获取字符串的全局值
pub fn get_str(key: &str) -> String {
    if let Some(data) = get_arc_map_bind_variable!(key,VARIABLE_STRING,VARIABLE_STRING_BIND) {
        return data.to_string() ;
    }else{
        return String::new();
    }
}

// 获取数字的全局值
pub fn get_u64(key: &str) -> u64 {
 if let Some(data) = get_arc_map_bind_variable!(key,VARIABLE_U64,VARIABLE_U64_BIND) {
        return data.clone() ;
    }
    
    0 as u64
}

// 获取数字的全局值
pub fn get_i32(key: &str) -> i32 {
    // get_numebr_value!(key, i32, 0 as i32, VARIABLE_I32);
    if let Some(data) = get_arc_map_bind_variable!(key,VARIABLE_I32,VARIABLE_I32_BIND) {
        return data.clone() ;
    }
    
    0.to_owned()
}

// 获取数字的全局值
pub fn get_i128(key: &str) -> i128 {
    // get_numebr_value!(key, i128, 0 as i128, VARIABLE_I128);
    if let Some(data) = get_arc_map_bind_variable!(key,VARIABLE_I128,VARIABLE_I128_BIND) {
        return data.clone() ;
    }
    
    0.to_owned()
}

// 获取布尔的全局值
pub fn get_bool(key: &str) -> bool {
    // get_numebr_value!(key, bool, false, VARIABLE_BOOL);
    if let Some(data) = get_arc_map_bind_variable!(key,VARIABLE_BOOL,VARIABLE_BOOL_BIND) {
        return data.clone() ;
    }
    
    false
}

// 获取 文本数组 的全局值
pub fn get_vec_str(key: &str) -> Vec<String> {
    // get_vec_value!(
    //     key,
    //     Vec<String>,
    //     Vec::new() as Vec<String>,
    //     VARIABLE_VEC_STRING
    // );
    if let Some(data) = get_arc_map_bind_variable!(key,VARIABLE_VEC_STRING,VARIABLE_VEC_STRING_BIND) {
        return data.clone() ;
    }
    
    Vec::new()
}

// 获取 数字数组 的全局值
pub fn get_vec_i32(key: &str) -> Vec<i32> {
    // get_vec_value!(key, Vec<i32>, Vec::new() as Vec<i32>, VARIABLE_VEC_I32);
    if let Some(data) = get_arc_map_bind_variable!(key,VARIABLE_VEC_I32,VARIABLE_VEC_I32_BIND) {
        return data.clone() ;
    }
    
    Vec::new()
}

// // 设置值的宏
// macro_rules! set_any_value {
//     ($key:expr,$value:expr,$g_value_hash:expr) => {
//         initialize();
//         let mut result: bool = false;

//         let mut lazy_value = $g_value_hash.try_lock().unwrap();

//         match lazy_value.insert($key.to_owned(), $value) {
//             Some(_) => result = true,
//             None => {}
//         };

//         drop(lazy_value);

//         return result;
//     };
// }

#[macro_export]
macro_rules! set_map_arc_bind_variable_insert{
    ($static_var: expr,$static_atomic: expr,$key:expr,$value:expr)=>{{
        let mutex = Arc::new(Mutex::new(&$static_atomic));
        mutex.lock();
        let the_value:usize = $static_atomic.load(Ordering::SeqCst);
        $static_atomic.store(the_value+1, Ordering::SeqCst);
    
        unsafe{
            // $static_var = $value; 
            match *$static_var.insert($key, $value) {
                Some(_) => {}
                None => {}
            };
            *$static_var.insert($key, $value);
        }
        
        drop(mutex);}
    }
}


// 设置全局值
pub fn set_u64(key: &str, value: u64) {
    // set_any_value!(key, value, VARIABLE_U64);
    set_map_arc_bind_variable_insert!(
        VARIABLE_U64,
        VARIABLE_U64_BIND,
        format!("{}",key),
        value
    );
    
}

// 设置全局值
pub fn set_str(key: &str, value: String) {
    // set_any_value!(key, value, VARIABLE_STRING);
    set_map_arc_bind_variable_insert!(
        VARIABLE_STRING,
        VARIABLE_STRING_BIND,
        format!("{}",key),
        value
    );
}

// 设置全局值
pub fn set_i32(key: &str, value: i32) {
    // set_any_value!(key, value, VARIABLE_I32);
    set_map_arc_bind_variable_insert!(
        VARIABLE_I32,
        VARIABLE_I32_BIND,
        format!("{}",key),
        value
    );
}

// 设置全局值
pub fn set_i128(key: &str, value: i128) {
    // set_any_value!(key, value, VARIABLE_I128);
    set_map_arc_bind_variable_insert!(
        VARIABLE_I128,
        VARIABLE_I128_BIND,
        format!("{}",key),
        value
    );
}

// 设置全局值
pub fn set_bool(key: &str, value: bool) {
    // set_any_value!(key, value, VARIABLE_BOOL);
    set_map_arc_bind_variable_insert!(
        VARIABLE_BOOL,
        VARIABLE_BOOL_BIND,
        format!("{}",key),
        value
    );
}

// 设置全局值
pub fn set_vec_i32(key: &str, value: Vec<i32>) {
    // set_any_value!(key, value, VARIABLE_VEC_I32);
    set_map_arc_bind_variable_insert!(
        VARIABLE_VEC_I32,
        VARIABLE_VEC_I32_BIND,
        format!("{}",key),
        value
    );
}

// 设置全局值
pub fn set_vec_str(key: &str, value: Vec<String>) {
    // set_any_value!(key, value, VARIABLE_VEC_STRING);
    set_map_arc_bind_variable_insert!(
        VARIABLE_VEC_STRING,
        VARIABLE_VEC_STRING_BIND,
        format!("{}",key),
        value
    );
}

// 获取全局值存在 的宏
#[macro_export]
macro_rules! has_arc_map_bind_variable_key {
    ($key:expr ,$static_var: expr,$static_atomic: expr) => {{
        let mut result = false;
        let mutex = Arc::new(Mutex::new(&$static_atomic));
        mutex.lock();
        let the_value: usize = $static_atomic.load(Ordering::SeqCst);
        $static_atomic.store(the_value + 1, Ordering::SeqCst);
        
        let data = unsafe { 
            if let Some(item) = $static_var.get($key) {
                result = true;
            }
         };
        drop(mutex);
        
       return result;
    }};
}


// 获取全局值存在
pub fn has_vec_str(key: &str) -> bool {
    has_arc_map_bind_variable_key!(key, VARIABLE_VEC_STRING,VARIABLE_VEC_STRING_BIND);
}

// 获取全局值存在
pub fn has_vec_i32(key: &str) -> bool {
    has_arc_map_bind_variable_key!(key, VARIABLE_VEC_I32,VARIABLE_VEC_I32_BIND);
}

// 获取全局值存在
pub fn has_str(key: &str) -> bool {
    has_arc_map_bind_variable_key!(key, VARIABLE_STRING,VARIABLE_STRING_BIND);
}

// 获取全局值存在
pub fn has_i32(key: &str) -> bool {
    has_arc_map_bind_variable_key!(key, VARIABLE_I32,VARIABLE_I32_BIND);
}

// 获取全局值存在
pub fn has_vec_u64(key: &str) -> bool {
    has_arc_map_bind_variable_key!(key, VARIABLE_U64,VARIABLE_U64_BIND);
}

// 获取全局值存在
pub fn has_vec_i128(key: &str) -> bool {
    has_arc_map_bind_variable_key!(key, VARIABLE_I128,VARIABLE_I128_BIND);
}

// 获取全局值存在
pub fn has_vec_bool(key: &str) -> bool {
    has_arc_map_bind_variable_key!(key, VARIABLE_BOOL,VARIABLE_BOOL_BIND);
}

// 添加数组 内容
pub fn insert_vec_str(key: &str, args: &[String]) -> bool {
    if (!has_vec_str(key)) {
        set_vec_str(key, Vec::new());
    }

    let mut vec_data = get_vec_str(key);
    for value in args {
        vec_data.push(value.to_string());
    }

    // println!("{}",get_vec_str("_watch_path_").join("\n"));

     set_vec_str(key, vec_data);

     return has_vec_str(key);
}

// // 添加数组 内容
// pub fn insert_vec_i32(key: &str, args: &[i32]) -> bool {
//     if (!has_vec_i32(key)) {
//         set_vec_i32(key, Vec::new());
//     }

//     let mut vec_data = get_vec_i32(key);
//     for value in args {
//         vec_data.push(*value);
//     }

//     return set_vec_i32(key, vec_data);
// }

// // 取出数组中的所有数字 并且清空
// pub fn retrieve_vec_i32(key: &str) -> Vec<i32> {
//     let mut result = Vec::new();

//     if (!has_vec_i32(key)) {
//         set_vec_i32(key, Vec::new());
//     }

//     let mut default_value: i32 = 0;

//     let mut lazy_value = VARIABLE_VEC_I32.try_lock().unwrap();

//     let keys: String = key.to_string();

//     let mut default_value: Vec<i32> = Vec::new();

//     let mut value = lazy_value
//         .clone()
//         .get(&keys)
//         .unwrap_or_else(|| &default_value)
//         .to_vec();

//     for value in value.clone() {
//         result.push(value);
//     }

//     &value.clear();

//     lazy_value.insert(key.to_owned(), value);

//     drop(lazy_value);

//     return result;
// }

// 取出数组中的所有文本 并且清空
pub fn retrieve_vec_str(key: &str) -> Vec<String> {
    let mut result = Vec::new();

    if (!has_vec_i32(key)) {
        set_vec_i32(key, Vec::new());
    }

    let mut default_value: i32 = 0;
   
    let mutex = Arc::new(Mutex::new(&VARIABLE_VEC_STRING_BIND));
    mutex.lock();
    let the_value: usize =VARIABLE_VEC_STRING_BIND.load(Ordering::SeqCst);
    VARIABLE_VEC_STRING_BIND.store(the_value + 1, Ordering::SeqCst);

    let data = unsafe { 
        if let Some(item) = VARIABLE_VEC_STRING.get(&format!("{}",key)) {
            for value in item.to_vec() {
                result.push(value);
            }
        }
        // *VARIABLE_VEC_STRING.insert(format!("{}",key), Vec::new());
     };
    drop(mutex);

    return result;
}

// 用户任务
pub struct ExportDirItme {
    pub id: i32,
    pub time: String,
    pub name: String,
    pub path: String,
    pub ouput: String,
}

impl Clone for ExportDirItme {
    fn clone(&self) -> Self {
        ExportDirItme {
            id: self.id.clone(),
            time: self.time.clone(),
            name: self.name.clone(),
            path: self.path.clone(),
            ouput: self.ouput.clone(),
        }
    }
}

// 从数据库中获取用户任务
fn get_export_dir_path_itme_sql_lib(
    conn: &Connection,
) -> Result<Vec<ExportDirItme>, rusqlite::Error> {
    let mut result: Vec<ExportDirItme> = Vec::new();
    let mut stmt = conn.prepare("SELECT id, time, name, path, ouput  FROM export_dir_path")?;

    let cats = stmt.query_map([], |row| {
        Ok(ExportDirItme {
            id: row.get(0)?,
            time: row.get(1)?,
            name: row.get(2)?,
            path: row.get(3)?,
            ouput: row.get(4)?,
        })
    })?;

    for cat in cats {
        let paths = cat?;
        result.push(paths);
    }

    Ok(result)
}

// 获取用户任务 （刷新）
pub fn update_export_dir_itme_list() -> Vec<ExportDirItme> {
    let mut itme_list: Vec<ExportDirItme> = Vec::new();

    let conn: Connection = match Connection::open("ikun_user_data.db") {
        Ok(conn) => conn,
        Err(e) => {
            insert_vec_str(
                "console_log",
                &[format!("[用户任务] 数据库内部错误因为 ->  {:?}", e)],
            );
            return Vec::new();
        }
    };

    match get_export_dir_path_itme_sql_lib(&conn) {
        Ok(data) => {
            for value in data {
                itme_list.push(value);
            }
        }
        Err(e) => {}
    }

    match conn.close() {
        Ok(_) => {}
        Err(e) => {
            insert_vec_str(
                "console_log",
                &[format!("[用户任务] 数据库断开错误因为 ->  {:?}", e)],
            );
        }
    };

        
    // let mut lazy_value = VARIABLE_VEC_EXPORT_DIR_ITME.try_lock().unwrap();
    // let mut oid_len: i32 = lazy_value.len().try_into().unwrap();

    // lazy_value.clear();

    // for value in itme_list.clone() {
    //     lazy_value.push(value);
    // }

    // drop(lazy_value);
    let oid_leng = get_arc_bind_variable!(VARIABLE_VEC_EXPORT_DIR_ITME,VARIABLE_VEC_EXPORT_DIR_ITME_BIND).len();

    set_arc_bind_variable!(VARIABLE_VEC_EXPORT_DIR_ITME,VARIABLE_VEC_EXPORT_DIR_ITME_BIND,itme_list.to_vec());

    let itme_list_len: usize = itme_list.len();

    if (oid_leng != itme_list_len) {
        insert_vec_str(
            "console_log",
            &[format!(
                "[数据更新] 用户任务列表数量 -> {} 数量{}",
                itme_list_len,
                itme_list_len as i32 - oid_leng as i32
            )],
        );
    }

    return itme_list;
}

// 获取用户任务 （不刷新 除非为0）
pub fn get_export_dir_itme_list() -> Vec<ExportDirItme> {
    if !*(INITIALIZE_GET_EXPORT_DIR_TIME_LIST
        .get()
        .unwrap_or_else(|| &false))
    {
        INITIALIZE_GET_EXPORT_DIR_TIME_LIST.set(true);
        return update_export_dir_itme_list();
    }

    let mut itme_list: Vec<ExportDirItme> = Vec::new();
    
    for value in  get_arc_bind_variable!(VARIABLE_VEC_EXPORT_DIR_ITME,VARIABLE_VEC_EXPORT_DIR_ITME_BIND){
        itme_list.push(value.clone());
    }

    return itme_list;
}
