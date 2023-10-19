#![allow(warnings, unused)]

pub(crate) mod convert;
pub(crate) mod watch_path;
pub(crate) mod config;
mod mobile_screenshot;

use crate::wh_mod::convert::detect_image_format;
use chrono::{DateTime, Local};
// use lazy_static::lazy_static;
use std::collections::HashSet;
use std::{fs, path};
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{collections::hash_map::HashMap, ptr};
use std::{
    sync::atomic::Ordering,
    sync::Arc,
    sync::MutexGuard,
    sync::{atomic::AtomicUsize, OnceLock},
};
use std::ffi::OsStr;
use crate::util;

// lazy_static! {
//     static ref WALK_ATTACH_FILE_LIST: Mutex<HashMap<String, Vec<PathBuf>>> = Mutex::new(HashMap::new());
// }

static mut WALK_ATTACH_FILE_LIST: Option<HashMap<String, Vec<PathBuf>>> = Option::None;
static WALK_ATTACH_FILE_LIST_BIND: AtomicUsize = AtomicUsize::new(0);
static VARIABLE_INITIALIZE_WALK_ATTACH_FILE_LIST: OnceLock<bool> = OnceLock::new();

fn initialize_watch_path_token() {
    if *(VARIABLE_INITIALIZE_WALK_ATTACH_FILE_LIST
        .get()
        .unwrap_or_else(|| &false))
    {
        return;
    }
    unsafe {
        if WALK_ATTACH_FILE_LIST.is_none() {
            WALK_ATTACH_FILE_LIST.replace(HashMap::new());
        }
    }
    VARIABLE_INITIALIZE_WALK_ATTACH_FILE_LIST.set(true);
}

// 清理历史枚举记录
pub fn gc_walk_attach_file_list() {
    initialize_watch_path_token();
    let mutex = Arc::new(Mutex::new(&WALK_ATTACH_FILE_LIST_BIND));
    mutex.lock();
    let the_value: usize = WALK_ATTACH_FILE_LIST_BIND.load(Ordering::SeqCst);
    unsafe {
        if let Some(lazy_value) = WALK_ATTACH_FILE_LIST.as_mut() {
            lazy_value.clear();
        }
    }

    WALK_ATTACH_FILE_LIST_BIND.store(the_value + 1, Ordering::SeqCst);
    drop(mutex);
    // let mut lazy_value = WALK_ATTACH_FILE_LIST.lock().unwrap();
    // lazy_value.clear();
    // drop(lazy_value);
}

// 获取历史枚举列表
pub fn get_walk_attach_file_history() -> HashMap<String, Vec<PathBuf>> {
    initialize_watch_path_token();
    let mut result = HashMap::new();

    let mutex = Arc::new(Mutex::new(&WALK_ATTACH_FILE_LIST_BIND));
    mutex.lock();
    let the_value: usize = WALK_ATTACH_FILE_LIST_BIND.load(Ordering::SeqCst);
    unsafe {
        println!("get_walk_attach_file -> WALK_ATTACH_FILE_LIST -> {:?}",&WALK_ATTACH_FILE_LIST.as_mut().unwrap().len());

        if let Some(lazy_value) = WALK_ATTACH_FILE_LIST.as_mut() {
            for (key, value) in lazy_value.iter() {
                result.insert(key.to_string(), value.clone());
            }
        }
        println!("get_walk_attach_file -> WALK_ATTACH_FILE_LIST -> {:?}",&WALK_ATTACH_FILE_LIST.as_mut().unwrap().len());

    }

    WALK_ATTACH_FILE_LIST_BIND.store(the_value + 1, Ordering::SeqCst);
    drop(mutex);

    result
}

#[derive(Debug)]
pub struct WxFsRetrievalStruct {
    pub path: String,
    pub path2arr: Vec<String>,
    pub root: String,
    pub user_data: String,
    pub wxid: String,
    pub attach_id: String,
    pub attach_path: String,
    pub month: String,
    pub is_thumbnail: bool,
    pub is_dat: bool,
}

pub fn split_path<T: util::OverloadedAnyStr >(input: T) -> Vec<String> {
    let mut input_path: String=input.to_string_default();
    let mut path2arr: Vec<String> = Vec::new();
    let mut str: String = String::new();
    // 按照 \\ / 分割路径
    for char in input_path.chars() {
        if char.to_string().bytes().eq("\\".to_string().bytes())
            || char.to_string().bytes().eq("/".to_string().bytes())
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

/**
 * 格式化wx位置路径
 * path : D:\...\weixin\WeChat Files\wxid_...\FileStorage\MsgAttach\attach_id\any\2023-08\*.dat
 */
pub fn wx_parse_path(wx_path: String) -> WxFsRetrievalStruct {
    let mut wx_fs_retrieval_struct = WxFsRetrievalStruct {
        path: "".to_string(),
        path2arr: vec![],
        root: "".to_string(),
        user_data: "".to_string(),
        wxid: "".to_string(),
        attach_id: "".to_string(),
        month: "".to_string(),
        is_thumbnail: false,
        is_dat: false,
        attach_path: String::new(),
    };

    let mut path2arr: Vec<String> = Vec::new();

    path2arr = split_path(wx_path.clone());
    if let Some(last) = path2arr.last() {
        wx_fs_retrieval_struct.is_thumbnail = last.to_ascii_lowercase().contains("_t.dat");
    }

    if path2arr.len() > 1 {
        // 基础路径
        wx_fs_retrieval_struct.path = path2arr.join("\\");
        wx_fs_retrieval_struct.path2arr = path2arr.to_vec();
        let path_parse = Path::new(wx_fs_retrieval_struct.path.as_str());

        // 判断后缀名为dat 不区分大小写
        wx_fs_retrieval_struct.is_dat = path_parse
            .extension()
            .unwrap_or_else(|| "".as_ref())
            .to_string_lossy()
            .to_ascii_lowercase()
            .bytes()
            .eq("dat".bytes());

        // 根目录
        wx_fs_retrieval_struct.root = String::from(path2arr.get(0).unwrap());

        //解析 wx_id msg (置信度为 最后出现的一次)

        let mut the_path_next_name_list: Vec<String> = Vec::new();
        let mut msg_attach: Vec<String> = Vec::new();
        let mut wx_id = String::new();
        let mut user_data: Vec<String> = Vec::new();
        let mut path2buff = "".bytes();
        let mut len: usize = 0;
        for for_path_data in path2arr.to_vec() {
            let to_ascii_lowercase = for_path_data.to_ascii_lowercase();
            path2buff = to_ascii_lowercase.bytes();
            the_path_next_name_list.push(for_path_data.clone());

            // magat
            if path2buff.clone().eq("msgattach".bytes()) {
                for the_path in the_path_next_name_list.clone() {
                    msg_attach.push(the_path);
                }
                if path2arr.len() > len + 1 {
                    wx_fs_retrieval_struct.attach_id = path2arr[len + 1].clone();
                }
            }

            // user data root
            if path2buff.clone().eq("wechat files".bytes()) {
                for the_path in the_path_next_name_list.clone() {
                    user_data.push(the_path);
                }
            }

            // wx id
            if let Some(item) = get_wx_user_store(wx_path.as_str()) {
                wx_id =item.wxid;
            }
       
            // 判断是否是日期 2023-05
            if (for_path_data.contains("202") && for_path_data.len() > 5) {
                if (for_path_data.find("-") == Some(4)) {
                    let mut is_not_month = false;
                    let eq_data = "0123456789-";

                    for char in for_path_data.chars() {
                        if is_not_month {
                            break;
                        };

                        let mut eq_for = false;
                        for eq_char in eq_data.chars() {
                            if char.to_string().bytes().eq(eq_char.to_string().bytes()) {
                                eq_for = true;
                            }
                        }

                        if !eq_for {
                            is_not_month = true;
                        }
                    }

                    if (!is_not_month) {
                        wx_fs_retrieval_struct.month = for_path_data;
                    }
                }
            }

            len = len + 1;
        }

        wx_fs_retrieval_struct.wxid = wx_id;
        wx_fs_retrieval_struct.attach_path = msg_attach.join("\\");
        wx_fs_retrieval_struct.user_data = user_data.join("\\");
    }

    if wx_fs_retrieval_struct.wxid.is_empty() {
         // wx id
         if let Some(item) = get_wx_user_store(wx_path.as_str()) {
            wx_fs_retrieval_struct.wxid =item.wxid;
        }

    }

    wx_fs_retrieval_struct
}

#[derive(Debug)]
pub struct AccountItem {
    pub id: String,
    pub update_time: SystemTime,
    pub time_str: String,
}

/**
 * 从文件名中获取 account_id （如果有）
 * path : D:\...\weixin\WeChat Files\
 */
pub fn wx_account_id(path: PathBuf) -> AccountItem {
    let modified_list = sort_modified_dir_meta(path.as_path());
    let mut id = String::new();
    let mut up_time = UNIX_EPOCH;

    for read_name in modified_list {
        let file_name = read_name
            .dir
            .file_name()
            .unwrap_or_else(|| "".as_ref())
            .to_string_lossy();

        let time = read_name.metadata.modified().unwrap();

        if let Ok(time) = read_name.metadata.modified() {
            if up_time < time {
                up_time = time;
            }
        }

        if file_name.contains("account_") {
            let account_id = file_name.replace("account_", "");
            id = account_id;
        }
    }

    let modified_datetime: DateTime<Local> = up_time.into();
    let formatted_time = modified_datetime.format("%Y-%m-%d %H:%M:%S").to_string();

    return AccountItem {
        id,
        update_time: up_time,
        time_str: formatted_time,
    };
}

#[derive(Debug)]
pub struct WxReadWxid {
    pub account_id: String,
    pub wxid: String,
    pub update_time: SystemTime,
    pub update_time_str: String,
    pub attach: PathBuf,
    pub user_root: PathBuf,
}


#[derive(Debug)]
pub struct WxUserStoreInfo {
    pub wxid: String,
    pub user_root: PathBuf,
}

pub fn get_wx_user_store<T: util::OverloadedAnyStr >(input: T) -> Option<WxUserStoreInfo>{
    let mut wx_user_store_info = WxUserStoreInfo{
        wxid: String::new(),
        user_root: PathBuf::new(),
    };

    let match_feature = vec![
        vec!["FileStorage","MsgAttach"],
        vec!["config","AccInfo.dat"],
        vec!["Msg"],
    ];
    let mut split_paths = split_path(input.to_string_default());
    let mut new_split_paths = Vec::new();

    // D:\usersData\...\WeChat Files\{wxid}
    for (index,value) in split_paths.iter().enumerate() {
       
        if value.as_bytes().eq("WeChat Files".as_bytes()) {
            new_split_paths.push("WeChat Files".to_string());

            if let Some(wxid) = split_paths.get(index+1usize) {
                wx_user_store_info.user_root = PathBuf::from(new_split_paths.join("\\"));

                new_split_paths.push(wxid.clone());
                wx_user_store_info.wxid = wxid.clone();
            }
            else{
                return None;
            }
            break;
        }
        new_split_paths.push(value.clone());
    }

    let mut the_match = true;
    
    for value in match_feature {
        let path = format!("{}/{}",new_split_paths.join("/"),value.join("/"));
        the_match = path::PathBuf::from(path).exists();
        if !the_match{
           return  None;
        }
    }

    
    Some(wx_user_store_info)
}

//  D:\usersData\...\WeChat Files\
pub fn wx_search_store_root (wx_root: &str) -> Vec<String> {
    let mut result = Vec::new();
    let match_feature = vec![
        vec!["FileStorage","MsgAttach"],
        vec!["config","AccInfo.dat"],
        vec!["Msg"],
    ];
        
        if let Ok(read_dir) = fs::read_dir(wx_root) {
           for DirEntry in read_dir {
            //  D:\usersData\...\WeChat Files\{the}
            if let Ok(entry) = DirEntry {
              let mut path = entry.path();
              let mut the_match = true;

                for feature in match_feature.to_vec() {
                    let feature_path = path.clone().join(feature.join("\\"));
                    the_match = feature_path.exists();
                    if !the_match{
                        continue;
                    }
                }
                
                if the_match {
                    let mut split_paths = split_path(path);
                    result.push(split_paths.join("\\"));
                }

            }
           }
            
        }

    result
}

/**
 * 获取id
 * path : D:\...\weixin\WeChat Files\
 */
pub fn wx_read_root_wxid(wx_root: &Path) -> Vec<WxReadWxid> {
    let mut wx_read_item_list: Vec<WxReadWxid> = Vec::new();

    // 获取基础路径信息
    for dir in read_dir(wx_root) {
        let dir_path = dir.path();
           // wx id
           if let Some(stor) = get_wx_user_store(&dir_path) {

            let item = WxReadWxid {
                account_id: "".to_string(),
                wxid: stor.wxid,
                update_time: UNIX_EPOCH,
                update_time_str: "".to_string(),
                attach: dir_path.join("FileStorage\\MsgAttach"),
                user_root: dir_path,
            };
            wx_read_item_list.push(item);
             }

    }

    // 读取更新时间和wxid(如果有)
    for index in 0..wx_read_item_list.len() {
        let get_wx_account_id = wx_account_id(wx_read_item_list[index].user_root.clone());
        wx_read_item_list[index].account_id = get_wx_account_id.id.clone();
        wx_read_item_list[index].update_time = get_wx_account_id.update_time.clone();
        wx_read_item_list[index].update_time_str = get_wx_account_id.time_str.clone();
    }

    wx_read_item_list
}

/**
 * 排序出最近更新的文件夹
 * path : any
 */
pub fn sort_modified_dir(dir_path: &Path) -> Vec<std::path::PathBuf> {
    let mut res: Vec<std::path::PathBuf> = Vec::new();
    let data = sort_modified_dir_meta(dir_path);
    for index in 0..data.len() {
        res.push(data[index].dir.clone());
    }
    res
}

#[derive(Debug)]
pub struct TmpMetadata {
    pub metadata: fs::Metadata,
    pub dir: std::path::PathBuf,
}

/**
 * 排序出最近更新的文件夹
 * path : any
 */
pub fn sort_modified_dir_meta(dir_path: &Path) -> Vec<TmpMetadata> {
    let mut modified_dir_list = Vec::new();
    let mut temp_metadata_list: Vec<TmpMetadata> = Vec::new();

    // 获取root 下的所有文件夹
    match fs::read_dir(dir_path) {
        Ok(rade_dir) => {
            for entry in rade_dir {
                let entry_value = entry.unwrap();

                // 获取文件属性表
                match entry_value.metadata() {
                    Ok(metadata) => {
                        let path = entry_value.path();
                        if path.is_dir() {
                            let obj = TmpMetadata {
                                metadata,
                                dir: entry_value.path(),
                            };
                            temp_metadata_list.push(obj);
                        }
                    }
                    Err(_) => {}
                }
            }
        }
        Err(_) => {}
    }

    temp_metadata_list.sort_by(|a, b| {
        a.metadata
            .modified()
            .unwrap()
            .cmp(&b.metadata.modified().unwrap())
    });

    for temp_metadata in temp_metadata_list {
        modified_dir_list.push(temp_metadata);
    }

    modified_dir_list
}

#[derive(Debug)]
pub struct TmpMetaDataf {
    pub metadata: fs::Metadata,
    pub file: std::path::PathBuf,
}
pub fn sort_modified_file_meta(dir_path: &Path) -> Vec<TmpMetaDataf> {
    let mut modified_dir_list = Vec::new();
    let mut temp_metadata_list: Vec<TmpMetaDataf> = Vec::new();

    // 获取root 下的所有文件夹
    match fs::read_dir(dir_path) {
        Ok(rade_dir) => {
            for entry in rade_dir {
                let entry_value = entry.unwrap();

                // 获取文件属性表
                match entry_value.metadata() {
                    Ok(metadata) => {
                        let path = entry_value.path();
                        if path.is_file() {
                            let obj = TmpMetaDataf {
                                metadata,
                                file: entry_value.path(),
                            };
                            temp_metadata_list.push(obj);
                        }
                    }
                    Err(_) => {}
                }
            }
        }
        Err(_) => {}
    }

    temp_metadata_list.sort_by(|a, b| {
        a.metadata
            .modified()
            .unwrap()
            .cmp(&b.metadata.modified().unwrap())
    });

    for temp_metadata in temp_metadata_list {
        modified_dir_list.push(temp_metadata);
    }

    modified_dir_list
}

/**
 * 读取文件夹下的路径
 * path : any
 */
pub fn read_dir(dir_path: &Path) -> Vec<fs::DirEntry> {
    let mut read_dir_list = Vec::new();

    match fs::read_dir(dir_path) {
        Ok(rade_dir) => {
            for entry in rade_dir {
                match entry {
                    Ok(entry) => read_dir_list.push(entry),
                    Err(_) => {}
                }
            }
        }
        Err(_) => {}
    }
    read_dir_list
}

#[derive(Debug)]
pub struct AttachThumbnail {
    pub is_thumbnail: bool,
    pub is_source: bool,
    // exists_source:bool,
    // exists_thumbnail: bool,
    pub thumbnail_path: String,
    // source_path: String,
    pub thumbnail: Vec<u8>,
    pub thumbnail_type: String,
    pub imag_id: String,
    pub attach_id: String,
}

impl Clone for AttachThumbnail {
    fn clone(&self) -> Self {
        AttachThumbnail {
            is_thumbnail: self.is_thumbnail.clone(),
            is_source: self.is_source.clone(),
            thumbnail_path: self.thumbnail_path.clone(),
            thumbnail: self.thumbnail.clone(),
            thumbnail_type: self.thumbnail_type.clone(),
            imag_id: self.imag_id.clone(),
            attach_id: self.attach_id.clone(),
        }
    }
}

/**
 * 获取指定 id 下的按照修改时间排序的 缩略图
 * path : D:\...\weixin\WeChat Files\wxid_...\FileStorage\MsgAttach\AttachID\
 */
pub fn read_attach_buff_thumbnail_data(dir_path: &PathBuf, thumbnail_len: usize,) -> Vec<AttachThumbnail> {
    let mut attach_thumbnail_list: Vec<AttachThumbnail> = Vec::new();
    let mut new_path = Path::new(dir_path).join("Thumb");
    let mut new_path_images = Path::new(dir_path).join("Image");

    if new_path.is_dir() {
        let modified_datetime: DateTime<Local> = SystemTime::now().into();
        let formatted_time: String = modified_datetime.format("%Y-%m").to_string();
        let mut new_path = new_path.join(formatted_time.clone());
        let mut new_path_images = new_path_images.join(formatted_time.clone());

        if new_path.is_dir() {
            let mut file_list = sort_modified_file_meta(&new_path);
            let mut path_list = Vec::new();
            for (index, file) in file_list.iter().rev().enumerate() {
                if index > thumbnail_len {
                    break;
                }
                path_list.push(file.file.clone());
            }
            drop(file_list);

            for path in path_list {
                let path_str = path.to_string_lossy().to_string();
                let name_str = path
                    .file_name()
                    .unwrap_or_else(|| (&"").as_ref())
                    .to_string_lossy()
                    .to_string();
                let path_str_imag =
                    Path::new(new_path_images.as_path()).join(name_str.replace("_t.dat", ".dat"));

                let wx_parse = wx_parse_path(path_str.clone());

                let mut attach_thumbnail = AttachThumbnail {
                    // 1
                    thumbnail_path: path_str.clone(),
                    // 1
                    is_thumbnail: true,
                    // 1
                    is_source: false,
                    // 1
                    // exists_source: path_str_imag.is_file(),
                    //
                    // source_path:path_str_imag.to_string_lossy().to_string(),
                    // 1
                    thumbnail: convert::convert_dat_images_buff(path.clone())
                        .unwrap_or_else(|_| Vec::new()),
                    // 1
                    thumbnail_type: String::new(),
                    // 1
                    attach_id: wx_parse.attach_id.clone(),
                    // 1
                    imag_id: name_str.replace("_t.dat", "").replace(".dat", ""),
                    // 1
                    // exists_thumbnail: true,
                };

                // 获取缩略图的格式
                match detect_image_format(&attach_thumbnail.thumbnail.clone()) {
                    Some(format) => attach_thumbnail.thumbnail_type = format.to_string(),
                    None => println!("Unknown image format."),
                }

                attach_thumbnail_list.push(attach_thumbnail);
            }
        }
    }

    attach_thumbnail_list
}

/**
 * 读取attach 下的最后更新N个 thumbnail (与read_attach_buff_thumbnail_data 不一样的是此函数一次性获取的是多个人的)
 * path : D:\...\weixin\WeChat Files\wxid_...\FileStorage\MsgAttach
 */
pub fn read_attach_buff_thumbnail_list(dir_path: &Path, len: usize, extract_len: usize, ) -> Vec<AttachThumbnail> {
    let mut attach_thumbnail_list: Vec<AttachThumbnail> = Vec::new();

    let sort_modified_list = sort_modified_dir_meta(dir_path);

    let n = if len >= sort_modified_list.len() {
        sort_modified_list.len()
    } else {
        len
    };

    for (i, sort_modified) in sort_modified_list.iter().rev().enumerate() {
        if i > n {
            break;
        }
        let dir_path = sort_modified.dir.clone();
        let read_attach_list = read_attach_buff_thumbnail_data(&dir_path, extract_len);

        for read_attach in read_attach_list {
            attach_thumbnail_list.push(read_attach);
        }
    }

    attach_thumbnail_list
}

/**
 * v2 版本将排序所有文件 而不是文件夹
 * 读取attach 下的最后更新N个 thumbnail (与read_attach_buff_thumbnail_data 不一样的是此函数一次性获取的是多个人的)
 * path : D:\...\weixin\WeChat Files\wxid_...\FileStorage\MsgAttach
 */
pub fn read_attach_buff_thumbnail_list_v2(dir_path: &Path, len: usize, extract_len: usize, ) -> Vec<AttachThumbnail> {
    let mut attach_thumbnail_list: Vec<AttachThumbnail> = Vec::new();

    let modified_datetime: DateTime<Local> = SystemTime::now().into();
    let formatted_time: String = modified_datetime.format("%Y-%m").to_string();

    let mut all_tmp_meta_data_file = Vec::new();

    if let Ok(dir_dir) = fs::read_dir(dir_path) {

        // D:\...\weixin\WeChat Files\wxid_...\FileStorage\MsgAttach\{attid}

        for dir_entry in dir_dir {
            if let Ok(entry) = dir_entry {

                // D:\...\weixin\WeChat Files\wxid_...\FileStorage\MsgAttach\{attid}\Thumb\2023-10

                let for_path = entry.path().join("Thumb").join(formatted_time.as_str());
                if for_path.is_dir() {
                    let mut c = sort_modified_file_meta(for_path.as_path());
                    if let Some(c) = c.pop() {
                        all_tmp_meta_data_file.push(c);
                    }
                }

            }
        }
    }

    // 排序创建时间
    all_tmp_meta_data_file.sort_by(|a, b| {
        let mut a_created = UNIX_EPOCH;
        let mut b_created = UNIX_EPOCH;

        if let Result::Ok(create) = a.metadata.created() {
            a_created = create;
        }

        if let Result::Ok(create) = b.metadata.created() {
            b_created = create;
        }

        a_created.cmp(&b_created)
    });

    all_tmp_meta_data_file.reverse();

    println!("all_tmp_meta_data_file-->{}",all_tmp_meta_data_file.len());
    // 取出五个任务
    for index in 0..len {
        if let Some(meta) = all_tmp_meta_data_file.get(index) {
            if let Some(str) = meta.file.to_str() {
                let dir_path = wx_parse_path(str.to_string());
                // println!("dir_path-> {:?}",&dir_path);

                let read_attach_list = read_attach_buff_thumbnail_data(&PathBuf::from(dir_path.attach_path.as_str()).join(dir_path.attach_id.as_str()), extract_len);

                println!("read_attach_list ->[{}] {}",dir_path.attach_path,read_attach_list.len(),);

                for read_attach in read_attach_list {
                    attach_thumbnail_list.push(read_attach);
                }
            }
        }

    }

    attach_thumbnail_list
}


fn visit_dirs(dir: &Path) -> Vec<PathBuf> {
    let mut path_list: Vec<PathBuf> = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    for visit_dir in visit_dirs(&path) {
                        path_list.push(visit_dir);
                    }
                } else {
                    path_list.push(path);
                }
            }
        }
    }
    path_list
}

/**
 * 匹配出 imag id
 */
pub fn walk_file(dir_path: &Path,run_tx: mpsc::Sender<(String, Vec<PathBuf>)>,img_id: String,) -> HashMap<String, Vec<PathBuf>> {
    // let mut file_list: Vec<PathBuf> = Vec::new();
    let mut wk_list: HashMap<String, Vec<PathBuf>> = HashMap::new();

    let (tx, rx) = mpsc::channel();

    let dir_root = read_dir(dir_path);
    let mut max_len = dir_root.len();

    for dir in dir_root {
        let tx = tx.clone();
        let paths = dir.path();
        let key = paths.to_string_lossy().to_string();

        thread::spawn(move || {
            let mut path_list: Vec<PathBuf> = visit_dirs(paths.as_path());
            tx.send((key, path_list));
            drop(tx);
            return;
        });
    }

    drop(tx);
    let mut index = 0;
    while index < 5 {
        let (key, data_vec) = rx.recv().unwrap_or_else(|_| (String::new(), Vec::new()));
        if key.is_empty() {
            index = index + 1;
            continue;
        }

        // 如果有img id
        if !img_id.is_empty() {
            for data in data_vec.to_vec() {
                if data
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .contains(img_id.clone().as_str())
                {
                    let key = format!("img_id::{}", img_id);
                    run_tx.send((key.clone(), vec![data.clone()]));
                    wk_list.insert(key, vec![data.clone()]);
                }
            }
        }
        run_tx.send((key.clone(), data_vec.clone()));
        wk_list.insert(key, data_vec);
    }

    // // set_walk_attach_file(wk_list.clone());
    // let mut lazy_value = WALK_ATTACH_FILE_LIST.lock().unwrap();

    // for (key, value) in wk_list.iter() {
    //     match lazy_value.insert(key.to_string(), value.clone()) {
    //         std::option::Option::Some(_) => {}
    //         // std::option::Option::None()=>{

    //         // }
    //         _ => {}
    //     };
    // }

    // drop(lazy_value);
    initialize_watch_path_token();
    let mutex = Arc::new(Mutex::new(&WALK_ATTACH_FILE_LIST_BIND));
    mutex.lock();
    let the_value: usize = WALK_ATTACH_FILE_LIST_BIND.load(Ordering::SeqCst);
    unsafe {
        println!("walk_file -> WALK_ATTACH_FILE_LIST -> {:?}",&WALK_ATTACH_FILE_LIST.as_mut().unwrap().len());
        println!("walk_file -> wk_list -> {:?}",wk_list.len());

        if let Some(lazy_value) = &WALK_ATTACH_FILE_LIST {
            for (key, value) in wk_list.iter() {
             
             if let Some(item) = WALK_ATTACH_FILE_LIST.as_mut() {
                    let key = key.to_string();
                    let push_value: Vec<PathBuf> = value.clone();

                    item.insert(key, push_value);
                };
            }
        }
        println!("walk_file -> WALK_ATTACH_FILE_LIST -> {:?}",&WALK_ATTACH_FILE_LIST.as_mut().unwrap().len());

    }

    WALK_ATTACH_FILE_LIST_BIND.store(the_value + 1, Ordering::SeqCst);
    drop(mutex);

    return wk_list;
}

#[derive(Debug)]
pub struct Dat2VarParseMeta {
    pub attach_id: String,
    pub attach_dir: String,
    // pub format_dir : String,
    pub is_the_month: bool,
    pub is_thumbnail: bool,
    pub is_source: bool,
    pub is_all: bool,
    pub is_sync:bool,
    pub is_video:bool,
    pub rename_rule: String,
    format_path_list: Vec<std::path::PathBuf>,
}

impl Dat2VarParseMeta {
    // 获取此可变体格式的格式化后的路径列表
    pub fn format(&mut self) -> Vec<std::path::PathBuf> {
        if (self.format_path_list.len() != 0) {
            return self.format_path_list.clone();
        }
        let mut path_list: Vec<std::path::PathBuf> = Vec::new();
        let attach_dir = Path::new(self.attach_dir.as_str());
        let the_month = chrono::Local::now().format("%Y-%m").to_string();

        if (attach_dir.exists()) {
            // 限定本月
            if (self.is_the_month) {
                if self.is_thumbnail {
                    let mut thumbnail_path = Path::new(attach_dir.clone())
                        .join("Thumb")
                        .join(the_month.clone());
                    path_list.push(thumbnail_path.clone());
                }
                if self.is_source {
                    let mut thumbnail_path = Path::new(attach_dir.clone())
                        .join("Image")
                        .join(the_month.clone());
                    path_list.push(thumbnail_path.clone());
                }
            }
            // 有限定缩略图和原图要求
            else if self.is_thumbnail || self.is_source {
                if self.is_thumbnail {
                    let mut thumbnail_path = Path::new(attach_dir.clone()).join("Thumb");
                    path_list.push(thumbnail_path.clone());
                }
                if self.is_source {
                    let mut thumbnail_path = Path::new(attach_dir.clone()).join("Image");
                    path_list.push(thumbnail_path.clone());
                }
            }
            // 没有声明 则全部
            else {
                path_list.push(attach_dir.to_path_buf());
            }
        }

        for path in path_list.clone() {
            self.format_path_list.push(path);
        }

        return path_list;
    }

    // 判断此路径是否属于此 可变体的路径
    pub fn exists(&mut self, eq_path: String) -> bool {
        let format_list = self.format();
        let eq_path = split_path(eq_path);

        let mut is_exists = false;
        for format in format_list {
            let split_format_path = split_path(format.to_string_lossy().to_string());
            let mut index: usize = 0;
            let mut match_path: bool = true;
            for eq_path in eq_path.clone() {
                if (split_format_path.len() <= index) {
                    break;
                }
                if !split_format_path[index].as_bytes().eq(eq_path.as_bytes()) {
                    match_path = false;
                }
                index = index + 1;
            }
            if match_path {
                is_exists = true;
            }
            if is_exists {
                break;
            }
        }
        is_exists
    }

    // 获取可变命名路径的实际需要生成的路径
    pub fn get_rename_output(&mut self) -> String{
        let mut result = self.rename_rule.clone();
        let mut the_data = HashMap::new();
        let time_info =util::get_time_info();

        the_data.insert("<现在>",time_info.time);
        the_data.insert("<年>",time_info.years);
        the_data.insert("<月>",time_info.month);
        the_data.insert("<日>",time_info.day);
        the_data.insert("<时>",time_info.hour);
        the_data.insert("<分>",time_info.minutes);
        // the_data.insert("<别名>","");
        // the_data.insert("<任务名>","");
        let mut mk_time_years = time_info.time_years;
        the_data.insert("<创建月>",mk_time_years);

        // let mut _type = "图片";
        // if self.is_video {
        //     _type = "视频";
        // }
        // else if self.is_thumbnail {
        //     _type = "缩略图";
        // }
        // // else if self.is_source {
        // //     _type = "手机截图";
        // // }
        // else {
        //     _type = "图片";
        // }

        // the_data.insert("<类型>",_type.to_string());
        the_data.insert("<哈希>",self.attach_id.clone());


        for (key,data) in the_data {
            result = result.replace(key,data.as_str());
        }

        result
    }
    pub fn writeFile(exp_dir:&str,var2dat_path:&str,buff:&[u8]) {

    }
}

impl Clone for Dat2VarParseMeta {
    fn clone(&self) -> Self {
        Dat2VarParseMeta {
            attach_id: self.attach_id.clone(),
            attach_dir: self.attach_dir.clone(),
            is_the_month: self.is_the_month.clone(),
            is_thumbnail: self.is_thumbnail.clone(),
            is_source: self.is_source.clone(),
            format_path_list: self.format_path_list.clone(),
            is_all: self.is_all.clone(),
            is_video:self.is_video.clone(),
            is_sync: self.is_sync.clone(),
            rename_rule:self.rename_rule.clone(),
        }
    }
}

/**
 * 解析可变化路径
 */
pub fn parse_dat2var_path<T: util::OverloadedAnyStr >(input: T) -> Dat2VarParseMeta {
    // D:\usersData\weixin\WeChat Files/wxid_y.....1/FileStorage/MsgAttach/99e.......d..f,the_month,source,thumbnail
    let mut dat_parse_meta = Dat2VarParseMeta {
        attach_id: "".to_string(),
        attach_dir: "".to_string(),
        rename_rule:"".to_string(),
        // format_dir: "".to_string(),
        is_the_month: false,
        is_thumbnail: false,
        is_sync:false,
        is_video:false,
        is_source: false,
        is_all: false,
        format_path_list: Vec::new(),
    };
    
    let mut path_dir = input.to_string_default();
    let mut path_list = Vec::new();
    let binding = split_path(path_dir).join("\\");
    let lines: Vec<&str> = binding.split('*').collect();

    if (lines.is_empty()) {
        return dat_parse_meta;
    }

    for line in lines {
        let line_f = format!("{}", line);
        if (line_f.is_empty()) {
            continue;
        }
        if line_f.as_bytes().eq("the_month".as_bytes()) {
            dat_parse_meta.is_the_month = true;
        }
        if line_f.as_bytes().eq("source".as_bytes()) {
            dat_parse_meta.is_source = true;
        }
        if line_f.as_bytes().eq("thumbnail".as_bytes()) {
            dat_parse_meta.is_thumbnail = true;
        }
        
        if line_f.as_bytes().eq("video".as_bytes()) {
            dat_parse_meta.is_video = true;
        }

        if line_f.as_bytes().eq("Sync".as_bytes()) {
            dat_parse_meta.is_sync = true;
        }
        
        if line_f.contains("rename_rule=") {
            dat_parse_meta.rename_rule = line_f.to_string().replace("rename_rule=","");
        }

        dat_parse_meta.is_all = !dat_parse_meta.is_thumbnail.clone()
            && dat_parse_meta.is_source.clone()
            && dat_parse_meta.is_the_month.clone()&& dat_parse_meta.is_video.clone();

        path_list.push(line_f);
    }

    if let Some(attach_dir) = path_list.get(0) {
        dat_parse_meta.attach_dir = format!("{}",attach_dir);
    }

    if let Some(attach_id) = split_path(dat_parse_meta.attach_dir.clone()).pop() {
        dat_parse_meta.attach_id=attach_id;
    }
    
    return dat_parse_meta;
}


pub fn resolve_path(path: String) -> String {
    return split_path(path).join("\\");
}




// 自动按照设置获取显示的文本的消敏
pub fn get_show_mask_text <T: util::OverloadedAnyStr >(input: T) -> String {
    if config::is_show_mask() {
        format!("{}",util::get_mask_text(input.to_string_default().as_str()))
    }else {
        input.to_string_default()
    }
}
