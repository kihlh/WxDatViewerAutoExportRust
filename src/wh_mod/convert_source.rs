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
// 对指定的文件进行异或算法 以此获取解码的文件
pub fn convert_bat_images(input_path: PathBuf, mut output_path: PathBuf) -> io::Result<String> {

    let buff =  convert_dat_images_buff(output_path.clone())?;
    match detect_image_format(&buff.clone()) {
        Some(format) => {
            output_path.set_extension(format);
            let mut out_dir = Path::new(&output_path).join("..");

            if !out_dir.exists() {
                fs::create_dir_all(out_dir)?;
            }

            fs::write(&output_path, &buff)?;
        }
        None => println!("Unknown image format."),
    }

    Ok(output_path.display().to_string())
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

// 获取活动用户信息
#[derive(Debug)]
pub struct WxActiveUser {
    // 获取用户名
    pub accinfo: Option<WxidNameAccinfo>,
    // wxid
    pub user_wxid: String,
    // 根数据目录 D:\...\WeChat Files
    pub user_root: String,
    // 数据目录 D:\...\WeChat Files\%wxid%
    pub user_data: String
}

// 获取活动用户的路径
pub fn get_active_user(user_root: &str) -> Vec<WxActiveUser> {
    let mut active_users = Vec::new();
    let mut get_size = libWxIkunPlus::findAllWindow("WeChatMainWndForPC", "").len();
    let mut read_dir_list: Vec<fs::DirEntry> = Vec::new();

    // 读取包含 wxid_ 的文件夹
    if let Ok(read_dir) = fs::read_dir(user_root) {
        for dir in read_dir {
            if let Ok(dir) = dir {
                let string_lossy = format!("{:?}", dir.file_name());

                if string_lossy.contains("wxid_") {
                    read_dir_list.push(dir);
                }
            }
        }
    }
    
    let mut vec_wxid_list:Vec<PathBuf> = Vec::new();
    
    for value in read_dir_list {
        // 通过高更新率的文件判断出最后修改时间
        let config_path = value.path().join("config");

        let mut read_file_list = Vec::new();

        if let Ok(read_dir) = fs::read_dir(config_path) {
            for dir in read_dir {
                if let Ok(read_file) = dir {
                    read_file_list.push(read_file);
                }
            }
        }

        //按照修改时间排序
        read_file_list.sort_by(|a, b| {
            let mut a_created = std::time::UNIX_EPOCH;
            let mut b_created = std::time::UNIX_EPOCH;

            if let Ok(metadata) = a.metadata() {
                if let Result::Ok(create) = metadata.modified() {
                    a_created = create;
                }
            }

            if let Ok(metadata) = b.metadata() {
                if let Result::Ok(create) = metadata.modified() {
                    b_created = create;
                }
            }

            a_created.cmp(&b_created)
        });
        read_file_list.reverse();

        if !read_file_list.is_empty() {
            vec_wxid_list.push(read_file_list[0].path());
        }   

    }

    // 排序出根目录的路径
    vec_wxid_list.sort_by(|a, b| {
        let mut a_created = std::time::UNIX_EPOCH;
        let mut b_created = std::time::UNIX_EPOCH;

        if let Ok(metadata) = a.metadata() {
            if let Result::Ok(create) = metadata.modified() {
                a_created = create;
            }
        }

        if let Ok(metadata) = b.metadata() {
            if let Result::Ok(create) = metadata.modified() {
                b_created = create;
            }
        }

        a_created.cmp(&b_created)
    });

    vec_wxid_list.reverse();

    for value in vec_wxid_list.to_vec() {
        if active_users.len() >= get_size &&!is_developer(){
            break;
        }
     
       let parse_path = wh_mod::wx_parse_path(format!("{}",value.to_str().unwrap()));
        
        if !parse_path.user_data.is_empty()&&!parse_path.wxid.is_empty() {
            active_users.push(WxActiveUser{
                accinfo: get_wxid_name(parse_path.user_data.clone(),parse_path.wxid.clone()),
                user_wxid: parse_path.wxid.clone(),
                user_root: parse_path.user_data.clone(),
                user_data: format!("{}\\{}",parse_path.user_data.clone(),parse_path.wxid.clone()),
            });  
        }
        
      

    //    println!("parse_path-> {:?}",&parse_path);
    }
   
    // println!("read_file_list ->  {:?}",&vec_wxid_list);

    active_users
}

