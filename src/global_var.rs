// once Cell
/*
use once_cell::sync::OnceCell;

// 每次全局扫描时间
static SLEEP_SCAN_ALL: OnceLock<u64> = OnceLock::new();



pub fn sleep_scan_all()-> u64  {
    return *SLEEP_SCAN_ALL.get().unwrap_or_else(||&10000);
}



pub fn set_sleep_scan_all (value:u64) -> u64 {
    match  SLEEP_SCAN_ALL.set(value) {
        Ok(_)=>value.clone(),
        Err(_)=>10000 as u64
    }
}

*/

// lazy_static!
/**


// use std::cell::OnceCell;
use lazy_static::lazy_static;
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::sync::OnceLock;

use std::cell::LazyCell;


lazy_static! {
    static ref SLEEP_SCAN_ALL: Mutex<u64> = Mutex::new(10000);
}

 pub fn get_sleep_scan_all() -> u64 {
    let mut value: u64 = 10000;

    let mut lazy_value: MutexGuard<u64> = SLEEP_SCAN_ALL.lock().unwrap();
    value = *lazy_value;

    drop(lazy_value);
    return value;
}

pub fn set_sleep_scan_all(value: u64) -> u64 {
    // let mut default_value: u64 = 10000;

    let mut lazy_value: MutexGuard<u64> = SLEEP_SCAN_ALL.lock().unwrap();
    *lazy_value = value;

    // drop(lazy_value);

    return value;
}


 */

/**


 #![feature(lazy_cell)]

 use std::sync::OnceLock;
 use std::cell::LazyCell;

 static SLEEP_SCAN_ALL: LazyCell<u64> = LazyCell::new(10000);


 pub fn get_sleep_scan_all() -> u64 {
    return SLEEP_SCAN_ALL;
}

pub fn set_sleep_scan_all(value: u64) -> u64 {
    SLEEP_SCAN_ALL = value ;
}


 */
use lazy_static::lazy_static;
use rusqlite::Connection;
// use once_cell::sync::OnceCell;
// use std::cell::LazyCell;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::sync::OnceLock;

use crate::console_log;

lazy_static! {
    static ref VARIABLE_U64: Mutex<HashMap<String, u64>> = Mutex::new(HashMap::new());
    static ref VARIABLE_STRING: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    static ref VARIABLE_I32: Mutex<HashMap<String, i32>> = Mutex::new(HashMap::new());
    static ref VARIABLE_I128: Mutex<HashMap<String, i128>> = Mutex::new(HashMap::new());
    static ref VARIABLE_BOOL: Mutex<HashMap<String, bool>> = Mutex::new(HashMap::new());
    static ref VARIABLE_VEC_STRING: Mutex<HashMap<String, Vec<String>>> =
        Mutex::new(HashMap::new());
    static ref VARIABLE_VEC_I32: Mutex<HashMap<String, Vec<i32>>> = Mutex::new(HashMap::new());
    static ref VARIABLE_VEC_EXPORT_DIR_ITME: Mutex<Vec<ExportDirItme>> = Mutex::new(Vec::new());
}

static VARIABLE_INITIALIZE: OnceLock<bool> = OnceLock::new();
static INITIALIZE_GET_EXPORT_DIR_TIME_LIST: OnceLock<bool> = OnceLock::new();

// 初始化值 需要提前赋值的需要在这边赋值
fn initialize() {
    if *(VARIABLE_INITIALIZE.get().unwrap_or_else(|| &false)) {
        return;
    }
    // global_var::set_bool("user::config::check_button_the_month",false);
    // global_var::set_bool("user::config::check_button_source",false);
    // global_var::set_bool("user::config::check_button_thumbnail",false);
    // global_var::set_str("user::config::input_select_dir","".to_string());
    // global_var::set_i32("user::config::select_user_thumbnail_obj",-1);

    let mut lazy_value = VARIABLE_U64.lock().unwrap();
    match lazy_value.insert("SLEEP_SCAN_ALL".to_owned(), 30000) {
        Some(_) => {}
        None => {}
    };

    drop(lazy_value);

    let mut lazy_value = VARIABLE_I32.lock().unwrap();
    match lazy_value.insert("user::config::select_user_thumbnail_obj".to_owned(), -1) {
        Some(_) => {}
        None => {}
    };

    drop(lazy_value);

    let mut lazy_value = VARIABLE_BOOL.lock().unwrap();
    let value_list = [
        "user::config::check_button_the_month",
        "user::config::check_button_source",
        "user::config::check_button_thumbnail",
        "gui::open::gui_detect_config",
        "gui::open::gui_drag_scan",
        "gui::open::handle_dat",
        "gui::open::manage_item"

    ];
    for value in value_list {
        match lazy_value.insert(value.parse().unwrap(), false) {
            Some(_) => {}
            None => {}
        };
    }

    drop(lazy_value);


    let mut lazy_value = VARIABLE_STRING.lock().unwrap();
    let value_list = [
        "user::config::input_select_dir",
        "user::config::user_select_path",
        "user::config::user_select_wxid",
        "user::config::walk_drag_path"
    ];
    for value in value_list {
        match lazy_value.insert(value.parse().unwrap(), String::new()) {
            Some(_) => {}
            None => {}
        };
    }

    drop(lazy_value);

    // println!("Initializing -> {}", VARIABLE_INITIALIZE.get().is_some());
    VARIABLE_INITIALIZE.set(true);
}

// 获取数字的推断宏
macro_rules! get_numebr_value {
    ($key:expr,$type:ty,$default_values:expr,$lazy_data_hash:expr) => {
        initialize();
        let mut default_value: $type = $default_values;

        let mut lazy_value = $lazy_data_hash.lock().unwrap();

        let keys: String = $key.to_string();
        if (!lazy_value.clone().contains_key(&keys.clone())) {
            lazy_value.clone().insert(keys.clone(), default_value);
        }
        let value: $type = *lazy_value.clone().get(&keys).unwrap();

        drop(lazy_value);
        return value;
    };
}

// 获取数组的推断宏
macro_rules! get_vec_value {
    ($key:expr,$type:ty,$default_values:expr,$lazy_data_hash:expr) => {
        initialize();
        let mut default_value: $type = $default_values;

        let mut lazy_value = $lazy_data_hash.lock().unwrap();

        let keys: String = $key.to_string();
        if (!lazy_value.clone().contains_key(&keys.clone())) {
            lazy_value.clone().insert(keys.clone(), default_value);
        }
        let value: $type = lazy_value.clone().get(&keys).unwrap().to_vec();

        drop(lazy_value);
        return value;
    };
}

// 获取文本值的推断宏
macro_rules! get_string_value {
    ($key:expr,$type:ty,$default_values:expr,$lazy_data_hash:expr) => {
        initialize();
        let mut default_value: $type = $default_values;

        let mut lazy_value = $lazy_data_hash.lock().unwrap();

        let keys: String = $key.to_string();
        if (!lazy_value.clone().contains_key(&keys.clone())) {
            lazy_value.clone().insert(keys.clone(), default_value);
        }
        let value: $type = lazy_value.clone().get(&keys).unwrap().to_string();

        drop(lazy_value);
        return value;
    };
}

// 获取字符串的全局值
pub fn get_str(key: &str) -> String {
    get_string_value!(key, String, String::from(""), VARIABLE_STRING);
}

// 获取数字的全局值
pub fn get_u64(key: &str) -> u64 {
    get_numebr_value!(key, u64, 0 as u64, VARIABLE_U64);
}

// 获取数字的全局值
pub fn get_i32(key: &str) -> i32 {
    get_numebr_value!(key, i32, 0 as i32, VARIABLE_I32);
}

// 获取数字的全局值
pub fn get_i128(key: &str) -> i128 {
    get_numebr_value!(key, i128, 0 as i128, VARIABLE_I128);
}

// 获取布尔的全局值
pub fn get_bool(key: &str) -> bool {
    get_numebr_value!(key, bool, false, VARIABLE_BOOL);
}

// 获取 文本数组 的全局值
pub fn get_vec_str(key: &str) -> Vec<String> {
    get_vec_value!(
        key,
        Vec<String>,
        Vec::new() as Vec<String>,
        VARIABLE_VEC_STRING
    );
}

// 获取 数字数组 的全局值
pub fn get_vec_i32(key: &str) -> Vec<i32> {
    get_vec_value!(key, Vec<i32>, Vec::new() as Vec<i32>, VARIABLE_VEC_I32);
}

// 设置值的宏
macro_rules! set_any_value {
    ($key:expr,$value:expr,$g_value_hash:expr) => {
        initialize();
        let mut result: bool = false;

        let mut lazy_value = $g_value_hash.lock().unwrap();

        match lazy_value.insert($key.to_owned(), $value) {
            Some(_) => result = true,
            None => {}
        };

        drop(lazy_value);

        return result;
    };
}

// 设置全局值
pub fn set_u64(key: &str, value: u64) -> bool {
    set_any_value!(key, value, VARIABLE_U64);
}

// 设置全局值
pub fn set_str(key: &str, value: String) -> bool {
    set_any_value!(key, value, VARIABLE_STRING);
}

// 设置全局值
pub fn set_i32(key: &str, value: i32) -> bool {
    set_any_value!(key, value, VARIABLE_I32);
}

// 设置全局值
pub fn set_i128(key: &str, value: i128) -> bool {
    set_any_value!(key, value, VARIABLE_I128);
}

// 设置全局值
pub fn set_bool(key: &str, value: bool) -> bool {
    set_any_value!(key, value, VARIABLE_BOOL);
}

// 设置全局值
pub fn set_vec_i32(key: &str, value: Vec<i32>) -> bool {
    set_any_value!(key, value, VARIABLE_VEC_I32);
}

// 设置全局值
pub fn set_vec_str(key: &str, value: Vec<String>) -> bool {
    set_any_value!(key, value, VARIABLE_VEC_STRING);
}

// 获取全局值存在 的宏
macro_rules! has_vec_key {
    ($key:expr,$lazy_data_hash:expr) => {
        initialize();
        let mut result: bool = false;
        let mut lazy_value = $lazy_data_hash.lock().unwrap();
        result = lazy_value.contains_key($key);
        drop(lazy_value);
        return result;
    };
}

// 获取全局值存在
pub fn has_vec_str(key: &str) -> bool {
    has_vec_key!(key, VARIABLE_VEC_STRING);
}

// 获取全局值存在
pub fn has_vec_i32(key: &str) -> bool {
    has_vec_key!(key, VARIABLE_VEC_I32);
}

// 获取全局值存在
pub fn has_str(key: &str) -> bool {
    has_vec_key!(key, VARIABLE_STRING);
}

// 获取全局值存在
pub fn has_i32(key: &str) -> bool {
    has_vec_key!(key, VARIABLE_I32);
}

// 获取全局值存在
pub fn has_vec_u64(key: &str) -> bool {
    has_vec_key!(key, VARIABLE_U64);
}

// 获取全局值存在
pub fn has_vec_i128(key: &str) -> bool {
    has_vec_key!(key, VARIABLE_I128);
}

// 获取全局值存在
pub fn has_vec_bool(key: &str) -> bool {
    has_vec_key!(key, VARIABLE_BOOL);
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

    return set_vec_str(key, vec_data);
}

// 添加数组 内容
pub fn insert_vec_i32(key: &str, args: &[i32]) -> bool {
    if (!has_vec_i32(key)) {
        set_vec_i32(key, Vec::new());
    }

    let mut vec_data = get_vec_i32(key);
    for value in args {
        vec_data.push(*value);
    }

    return set_vec_i32(key, vec_data);
}

// 取出数组中的所有数字 并且清空
pub fn retrieve_vec_i32(key: &str) -> Vec<i32> {
    let mut result = Vec::new();

    if (!has_vec_i32(key)) {
        set_vec_i32(key, Vec::new());
    }

    let mut default_value: i32 = 0;

    let mut lazy_value = VARIABLE_VEC_I32.lock().unwrap();

    let keys: String = key.to_string();

    let mut default_value: Vec<i32> = Vec::new();

    let mut value = lazy_value
        .clone()
        .get(&keys)
        .unwrap_or_else(|| &default_value)
        .to_vec();

    for value in value.clone() {
        result.push(value);
    }

    &value.clear();

    lazy_value.insert(key.to_owned(), value);

    drop(lazy_value);

    return result;
}

// 取出数组中的所有文本 并且清空
pub fn retrieve_vec_str(key: &str) -> Vec<String> {
    let mut result = Vec::new();

    if (!has_vec_i32(key)) {
        set_vec_i32(key, Vec::new());
    }

    let mut default_value: i32 = 0;

    let mut lazy_value = VARIABLE_VEC_STRING.lock().unwrap();

    let keys: String = key.to_string();

    let mut default_value: Vec<String> = Vec::new();

    let mut value = lazy_value
        .clone()
        .get(&keys)
        .unwrap_or_else(|| &default_value)
        .to_vec();

    for value in value.clone() {
        result.push(value);
    }

    &value.clear();

    lazy_value.insert(key.to_owned(), value);

    drop(lazy_value);

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

    let mut lazy_value = VARIABLE_VEC_EXPORT_DIR_ITME.lock().unwrap();
    let mut oid_len: i32 = lazy_value.len().try_into().unwrap();

    lazy_value.clear();

    for value in itme_list.clone() {
        lazy_value.push(value);
    }

    drop(lazy_value);

    let itme_list_len: i32 = itme_list.len().try_into().unwrap();

    if (oid_len != itme_list_len) {
        insert_vec_str(
            "console_log",
            &[format!(
                "[数据更新] 用户任务列表数量 -> {} 数量{}",
                itme_list_len,
                itme_list_len - oid_len
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
    let mut lazy_value = VARIABLE_VEC_EXPORT_DIR_ITME.lock().unwrap();

    for value in lazy_value.clone() {
        itme_list.push(value);
    }

    drop(lazy_value);
    return itme_list;
}

