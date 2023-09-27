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

macro_rules! Sleep {
    ($b:expr) => {{
        std::thread::sleep(std::time::Duration::from_millis($b));
    }};
}

pub fn Sleep(ms: u64) {
    Sleep!(ms);
}

/**
 * 文本是否相等
 */
pub fn str_eq_str(str: String, eq_str: String) -> bool {
    if (str.len() != eq_str.len()) {
        return false;
    };

    // 转为二进制再判断
    let str_buf = str.as_bytes();
    let eq_str_buf = eq_str.as_bytes();
    return str_buf.eq(eq_str_buf);
}

/**
 * 文本是否相等
 */
pub fn str_eq_ostr(str: String, eq_str: &str) -> bool {
    return str_eq_str(str, String::from(eq_str));
}

// 从环境变量中获取布尔值
pub fn getVarBooleanValue(key: String) -> bool {
    let dataStr = env::var(key).unwrap_or_else(|_| String::from("false"));
    str_eq_ostr(dataStr, "true")
}

// 在环境变量中设置布尔值
pub fn setVarBooleanValue(key: String, valua: bool) {
    let mut data = "false";

    if (valua) {
        data = "true";
    }

    env::set_var(key, data)
}

// OsStr 转String
pub fn os_str_to_str(os_str: &OsStr) -> String {
    let string_buff: String = os_str.to_string_lossy().into_owned();
    return string_buff;
}

// Path 转String
pub fn path_to_str(path_str: &Path) -> String {
    path_str.display().to_string()
}

// 获取路径的ext
pub fn path_extension(path_str: &Path) -> String {
    path_str
        .extension()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned()
}

// 获取路径的ext
pub fn path_extension_str(path_str: &String) -> String {
    let path_str_buff = Path::new(path_str);
    path_str_buff
        .extension()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned()
}
