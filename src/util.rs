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
use std::time::SystemTime;
use chrono::{DateTime, Local};
use crate::{util};

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

#[derive(Debug)]
pub struct TextSize{
    // 文本内存大小
    pub str_size:usize,
    // 在内存中的大小
    pub bytes_size:usize,
    // 大于3字节的utf8文本的文本数量
    pub utf8_len:usize,
    // 英文 / 符号 的数量
    pub ansi_len:usize,
    // 文本数量
    pub all_len:usize,
    // 预测宽度
    pub prediction_len:usize,
    // 文本数组
    // pub chars:Vec<char>
}

impl Clone for TextSize {
    fn clone(&self) -> Self {
        TextSize {
            str_size: self.str_size.clone(),
            bytes_size: self.bytes_size.clone(),
            utf8_len: self.utf8_len.clone(),
            ansi_len: self.ansi_len.clone(),
            all_len: self.all_len.clone(),
            prediction_len: self.prediction_len.clone(),
            // chars: self.chars.clone(),
        }
    }
}

impl TextSize {
    pub fn strings (data:&str) -> Vec<String> {
        let mut result: Vec<String> =Vec::new();

        for value in data.chars() {
            result.push(value.to_string());
        }

        return result;
    }
    pub fn chars(data:&str) -> Vec<char> {
        let mut result :Vec<char> =Vec::new();
        for value in data.chars() {
            result.push(value);
        }
        result
    }
}

// 获取文本的
pub fn text_size (data:&str) -> TextSize {
    let mut result = TextSize{
        str_size: 0,
        utf8_len: 0,
        ansi_len: 0,
        all_len: 0,
        prediction_len:0,
        bytes_size:data.as_bytes().len(),
        // chars:Vec::new(),
    };

    let mut name_len = 0;

    for value in data.chars() {
        if value.len_utf8()>2 {
            result.utf8_len+=1;
            result.prediction_len+=2;

        }else {
            result.ansi_len+=1;
            result.prediction_len+=1;
        }

        // result.chars.push(value);
        result.all_len+=1;
    }


    result.str_size = data.len();

    result
}

// 文本消敏 按照比例  开头0.25（25%） 结尾 0.52 (%52)  中间的 23% 将被替换为 mask
pub fn masks_percentage(_input: &str , front_percentage: f32, back_percentage: f32, mask: &str) -> String {
    let input = util::TextSize::strings(_input);

    let total_len = input.len();
    let mask_start = (total_len as f32 * front_percentage).round() as usize;
    let mask_end = (total_len as f32 * back_percentage).round() as usize;
    if total_len <= mask_start + mask_end {
        return input.join("");
    }
    let mut masked = input[0..mask_start].join("");
    masked.push_str(&mask.repeat(total_len - mask_start - mask_end));
    masked.push_str(&input[total_len - mask_end..].join(""));
    masked
}

pub fn mask(_input: &str, left: usize, right: usize, mask: &str) -> String {
    let input = util::TextSize::strings(_input);
    let total_len = input.len();
    if total_len <= left + right {
        return input.join("");
    }
    let mut masked = input[0..left].join("");
    masked.push_str(&mask.repeat(total_len - left - right));
    masked.push_str(&input[total_len - right..].join(""));
    masked
}


pub trait OverloadedAnyStr {
    // 转换为文本
    fn to_string(&self) -> Option<String>;
    // 如果转换失败返回空文本
    fn to_string_default(&self) -> String;
    // 如果转化失败则返回此文本
    fn to_string_or(&self,or_str:String) ->String;

}

macro_rules! over_loaded_any_option_convert_fun {
    ($funRes:expr,$or:expr)=>{
        if let Some(data) = $funRes {
            data
        }else {
            $or
        }
    }
}

impl OverloadedAnyStr for Path {
    fn to_string(&self) -> Option<String> {
        Some(self.to_string_lossy().to_string())
    }
    fn to_string_default(&self) -> String {
        self.to_string_lossy().to_string()
    }
    fn to_string_or(&self,or_str:String) -> String {
        self.to_string_lossy().to_string()
    }

}

impl OverloadedAnyStr for &Path {
    fn to_string(&self) -> Option<String> {
        Some(self.to_string_lossy().to_string())
    }
    fn to_string_default(&self) -> String {
        self.to_string_lossy().to_string()
    }
    fn to_string_or(&self,or_str:String) -> String {
        self.to_string_lossy().to_string()
    }

}

impl OverloadedAnyStr for &PathBuf {
    fn to_string(&self) -> Option<String> {
        Some(self.to_string_lossy().to_string())
    }
    fn to_string_default(&self) -> String {
        self.to_string_lossy().to_string()
    }
    fn to_string_or(&self,or_str:String) -> String {
        self.to_string_lossy().to_string()
    }

}

impl OverloadedAnyStr for PathBuf {
    fn to_string(&self) -> Option<String> {
        Some(self.to_string_lossy().to_string())
        // None
    }
    fn to_string_default(&self) -> String {
        self.to_string_lossy().to_string()
    }
    fn to_string_or(&self,or_str:String) -> String {
        self.to_string_lossy().to_string()
    }

}

impl OverloadedAnyStr for &str {
    fn to_string(&self) -> Option<String> {
        Some(format!("{}",self))
    }
    fn to_string_default(&self) -> String {
        format!("{}",self)
    }
    fn to_string_or(&self,or_str:String) -> String {
        format!("{}",self)
    }

}

impl OverloadedAnyStr for String {
    fn to_string(&self) -> Option<String> {
        Some(format!("{}",self))
    }
    fn to_string_default(&self) -> String {
        format!("{}",self)
    }
    fn to_string_or(&self,or_str:String) -> String {
        format!("{}",self)
    }
}

impl OverloadedAnyStr for &String {
    fn to_string(&self) -> Option<String> {
        Some(format!("{}",self))
    }
    fn to_string_default(&self) -> String {
        format!("{}",self)
    }
    fn to_string_or(&self,or_str:String) -> String {
        format!("{}",self)
    }

}

impl OverloadedAnyStr for &OsStr {
    fn to_string(&self) -> Option<String> {
        if let Some(str) = self.to_str() {
           return  Some(format!("{}",str));
        }
        // Some(format!("{}",self))
        None
    }
    fn to_string_default(&self) -> String {
        if let Some(str) = self.to_str() {
            return format!("{}",str);
        }
        String::new()
    }
    fn to_string_or(&self,or_str:String) -> String {
        if let Some(str) = self.to_str() {
            return format!("{}",str);
        }
        or_str
    }
}

impl OverloadedAnyStr for &&OsStr {
    fn to_string(&self) -> Option<String> {
        if let Some(str) = self.to_str() {
            return  Some(format!("{}",str));
        }
        // Some(format!("{}",self))
        None
    }
    fn to_string_default(&self) -> String {
        if let Some(str) = self.to_str() {
            return format!("{}",str);
        }
        String::new()
    }
    fn to_string_or(&self,or_str:String) -> String {
        if let Some(str) = self.to_str() {
            return format!("{}",str);
        }
        or_str
    }
}

impl OverloadedAnyStr for Option<String> {
    fn to_string(&self) -> Option<String> {
        self.clone()
    }
    fn to_string_default(&self) -> String {
        if let Some(str) = self {
            return format!("{}",str);
        }
        String::new()
    }
    fn to_string_or(&self,or_str:String) -> String {
        if let Some(str) = self {
            return format!("{}",str);
        }
        or_str
    }
}

impl OverloadedAnyStr for Option<&str> {
    fn to_string(&self) -> Option<String> {
        if let Some(str) = self {
            return Some(format!("{}",str));
        }
        None
    }
    fn to_string_default(&self) -> String {
        if let Some(str) = self {
            return format!("{}",str);
        }
        String::new()
    }
    fn to_string_or(&self,or_str:String) -> String {
        if let Some(str) = self {
            return format!("{}",str);
        }
        or_str
    }
}

impl OverloadedAnyStr for Option<&OsStr> {
    fn to_string(&self) -> Option<String> {
        if let Some(str) = self {
           return  Some( str.to_string_lossy().to_string())
        }
        None
    }
    fn to_string_default(&self) -> String {
        if let Some(str) = self {
            return format!("{}",str.to_string_lossy().to_string());
        }
        String::new()
    }
    fn to_string_or(&self,or_str:String) -> String {
        if let Some(str) = self {
            return format!("{}",str.to_string_lossy().to_string());
        }
        or_str
    }
}


impl OverloadedAnyStr for std::fs::DirEntry {
    fn to_string(&self) -> Option<String> {
        self.to_string()
    }
    fn to_string_default(&self) -> String {
        if let Some(str) =  self.to_string() {
            return str;
        }
        String::new()
    }
    fn to_string_or(&self,or_str:String) -> String {
        if let Some(str) =  self.to_string() {
            return str;
        }
        or_str
    }
}

impl OverloadedAnyStr for &std::fs::DirEntry {
    fn to_string(&self) -> Option<String> {
        self.to_string()
    }
    fn to_string_default(&self) -> String {
        if let Some(str) =  self.to_string() {
            return str;
        }
        String::new()
    }
    fn to_string_or(&self,or_str:String) -> String {
        if let Some(str) =  self.to_string() {
            return str;
        }
        or_str
    }
}

impl OverloadedAnyStr for Option<std::fs::DirEntry> {
    fn to_string(&self) -> Option<String> {
        if let Some(item) = self {
            return  item.to_string()
        }
        None
    }
    fn to_string_default(&self) -> String {
        if let Some(item) = self {
            
            if let Some(item) = item.to_string() {
                return item ;
            }  
        }
        String::new()
    }
    fn to_string_or(&self,or_str:String) -> String {
        if let Some(item) = self {
            
            if let Some(item) = item.to_string() {
                return item ;
            }  
        }
        or_str
    }
}


use std::os::raw::c_char;
use std::ffi::{CStr, CString};

impl OverloadedAnyStr for *const c_char {
    fn to_string(&self) -> Option<String> {
        let c_str = unsafe { CStr::from_ptr(self.clone()) };
        let bytes = c_str.to_bytes();
        Some(String::from_utf8_lossy(bytes).into_owned())
    }
    fn to_string_default(&self) -> String {
        let c_str = unsafe { CStr::from_ptr(self.clone()) };
        let bytes = c_str.to_bytes();
        String::from_utf8_lossy(bytes).into_owned()
    }
    fn to_string_or(&self,or_str:String) -> String {
        let c_str = unsafe { CStr::from_ptr(self.clone()) };
        let bytes = c_str.to_bytes();
        String::from_utf8_lossy(bytes).into_owned()

    }
}

pub fn to_string<T: util::OverloadedAnyStr >(input: T) -> Option<String> {
    input.to_string()
}

pub fn to_string_default<T: util::OverloadedAnyStr >(input: T) -> String {
    input.to_string_default()
}

pub fn to_string_or<T: util::OverloadedAnyStr >(input: T, or_str: String) -> String {
    input.to_string_or(or_str)
}


pub struct time_info {
    // 年
    pub years:String,
    // 月
    pub month:String,
    // 日
    pub day:String,
    // 时
    pub hour:String,
    // 分
    pub minutes:String,
    // 今天
    pub time:String,
    // 创建月
    pub time_years:String
}

pub fn get_time_info () ->time_info {
    let mut result:time_info =time_info{
        // 年
        years: String::new(),
        // 月
        month: String::new(),
        // 天
        day: String::new(),
        // 小时
        hour: String::new(),
        // 分钟
        minutes: String::new(),
        // 今天
        time: String::new(),
        // 2023-10
        time_years:String::new()
    };

    let time = SystemTime::now();
    let modified_datetime: DateTime<Local> = time.into();
    result.time = modified_datetime.format("%Y-%m-%d").to_string();
    result.years = modified_datetime.format("%Y").to_string();
    result.month = modified_datetime.format("%m").to_string();
    result.day = modified_datetime.format("%d").to_string();
    result.hour = modified_datetime.format("%H").to_string();
    result.minutes = modified_datetime.format("%M").to_string();
    result.time_years = modified_datetime.format("%Y-%m").to_string();

    result
}

pub fn load_thumbnai_data (open:&str) -> Result< Vec<u8> ,image::ImageError>{
    let img = image::open(open)?;
    let resized_img = img.resize_exact(128, 128, image::imageops::FilterType::CatmullRom);
    let mut output = std::io::Cursor::new(Vec::new());
    resized_img.write_to(&mut output, image::ImageFormat::Png)?;
    return Ok(output.into_inner());

    Ok(Vec::new())
} 