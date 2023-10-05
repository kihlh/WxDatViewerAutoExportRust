#![allow(warnings, unused)]

use std::collections::hash_map::DefaultHasher;
use std::io::Read;
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

use libc::getenv;
use std::string::FromUtf8Error;

use crate::{libWxIkunPlus, wh_mod};

// 判断文件头(魔术数字)
pub fn detect_image_format(data: &[u8]) -> Option<&'static str> {
    if data.len() < 4 {
        return None; // 数据不够长，无法识别
    }

    match &data[0..4] {
        [0x89, 0x50, 0x4E, 0x47] => Some("png"),
        [0xFF, 0xD8, _, _] => Some("jpg"),
        [0x47, 0x49, 0x46, 0x38] => Some("gif"),
        [0x52, 0x49, 0x46, 0x46]
            if data.len() >= 12 && &data[8..12] == [0x57, 0x45, 0x42, 0x50] =>
        {
            Some("webp")
        }
        _ => None, // 未知格式
    }
}

// !请注意  异或解密算法是不开源的部分
// 对指定的文件进行异或算法 以此获取解码的文件
pub fn convert_bat_images(input_path: PathBuf, mut ouput_path: PathBuf) -> io::Result<String> {
    let content = fs::read(&input_path)?;

    // let ** = content[0];
    // let ** = content[1];

    // let ** = ** ^ 0x66666;
    // let ** = ** ^ 0x66666;
    // let ** = ** ^ 0x66666;
    // let ** = ** ^ 0x66666;
    // let ** = ** ^ 0x66666;
    // let ** = ** ^ 0x66666;

    // let mut vi = ** ^ 0x66666;

    // if ** == ** {
    //     vi = **;
    // } else if ** == ** {
    //     vi = **;
    // } else if ** == ** {
    //     vi = **;
    // }

    // let buff: Vec<u8> = content.iter().map(|br| br ^ vi).collect();

    let buff: Vec<u8> = Vec::new();
    match detect_image_format(&buff.clone()) {
        Some(format) => {
            ouput_path.set_extension(format);
            let mut out_dir = Path::new(&ouput_path).join("..");

            if !out_dir.exists() {
                fs::create_dir_all(out_dir)?;
            }

            fs::write(&ouput_path, &buff)?;
        }
        None => println!("Unknown image format."),
    }

    Ok(ouput_path.display().to_string())
}

// !请注意  异或解密算法是不开源的部分
// 对指定的文件进行异或算法 以此获取解码的文件
pub fn convert_dat_images_buff(input_path: PathBuf) -> io::Result<Vec<u8>> {
    let content = fs::read(&input_path)?;

    // let ** = content[0];
    // let ** = content[1];

    // let ** = ** ^ 0x66666;
    // let ** = ** ^ 0x66666;
    // let ** = ** ^ 0x66666;
    // let ** = ** ^ 0x66666;
    // let ** = ** ^ 0x66666;
    // let ** = ** ^ 0x66666;

    // let mut ** = ** ^ 0x66666;

    // if ** == ** {
    //     vi = **;
    // } else if ** == ** {
    //     vi = **;
    // } else if ** == ** {
    //     vi = **;
    // }

    // let buff: Vec<u8> = content.iter().map(|br| br ^ vi).collect();
   
    let buff: Vec<u8> = Vec::new();
    Ok(buff)
}

// 按照换行分割文本
pub fn split_line(input_path: String) -> Vec<String> {
    let mut path2arr: Vec<String> = Vec::new();
    let mut str: String = String::new();
    // 按照 \\ / 分割路径
    for char in input_path.chars() {
        if char.to_string().bytes().eq("\n".to_string().bytes())
            || char.to_string().bytes().eq("\r".to_string().bytes())
        {
            path2arr.push(str.clone());
            str.clear();
            continue;
        }
        str.push(char);
    }

    // 最后一个文本
    if str.len() != 0 {
        path2arr.push(str.clone());
    }

    let mut path2arr_filter = Vec::new();

    for path2 in path2arr {
        if (!path2.is_empty()) {
            path2arr_filter.push(path2);
        }
    }

    path2arr_filter
}

// 获取用户数据根文件夹
pub fn get_user_data_path() -> Option<String> {
    // ! 这部分也是不开源的部分
    Option::None
}

// 获取用户名
#[derive(Debug)]
pub struct WxidNameAccinfo {
    // 头像url
    pub avatar: String,
    // 名称
    pub name: String,
    // wxid
    pub wx_id: String,
}

// 获取指定数据的用户名
pub fn get_wxid_name(user_data_path: String, wxid: String) -> Option<WxidNameAccinfo> {
    // ! 这部分也是不开源的部分
    Option::None
}

// 开发者用户id
pub fn get_user_id1() -> String {
    // ! 这部分也是不开源的部分
    return String::new();
}

// 开发者用户id
pub fn get_user_id2() -> String {
    // ! 这部分也是不开源的部分
    return String::new();
}

// 判断当前是否处于开发者模式
pub fn is_developer() -> bool {
    // ! 这部分也是不开源的部分

    false
}
