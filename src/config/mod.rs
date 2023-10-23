#![allow(warnings, unused)]

use serde_json::json;
use std::hint;
use std::io::BufReader;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicI64, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, RwLock, OnceLock};
mod lib;
use crate::atomic_util::{add_usize,set_usize, get_usize};
use crate::{atomic_util, get_bool,set_bool, libWxIkunPlus};
// 配置常量

// 启用开发者模式（此处为false 配置文件中的开发者模式将被忽略）
pub(crate) const APP_ENABLE_DEVELOPER: bool = true;
// 编译到52破解版本
const APP_BUILD_52POJIE: bool = false;

const APP_STORE_NAME: &str = "config.json";
const APP_STORE_DIR: &str = "./";
const APP_SOFTWARE_UPDATE_DETECTION: [&str;3] = ["https://raw.githubusercontent.com/kihlh/WxDatViewerAutoExportRust/master/version.json","https://vip.123pan.cn/1816369032/assets/WxAutoExIm/version.json","https://x-1300389275.cos.ap-shanghai.myqcloud.com/assets/WxAutoExIm/version.json"];


// 并发线程数
static CONFIG_THREAD_COUNT: AtomicUsize = AtomicUsize::new(0);
// 实时响应
static CONFIG_AUTO_ACTION: AtomicBool = AtomicBool::new(true);
// 全局扫描
static CONFIG_GLOBAL_SCAN: AtomicBool = AtomicBool::new(true);
// 添加后立即扫描
static CONFIG_SCAN_ADDING: AtomicBool = AtomicBool::new(false);
// 添加后立即添加到日志扫描
static CONFIG_SCAN_LOG_ADDING: AtomicBool = AtomicBool::new(true);
// 授权联网
static CONFIG_NETWORKING: AtomicBool = AtomicBool::new(false);
// 忽略重启
static CONFIG_IGNORE_MUTUAL: AtomicBool = AtomicBool::new(false);
// 任务配置保留
static CONFIG_PRESERVE_CONFIG: AtomicBool = AtomicBool::new(false);
// 任务配置保留
static CONFIG_CREATE_CONT: AtomicBool = AtomicBool::new(false);
// 任务连续创建
static CONFIG_PRESERVE_LIST: AtomicBool = AtomicBool::new(false);
// 日志输出
static CONFIG_CONSOLE_LOG: AtomicBool = AtomicBool::new(false);
// 开发者模式
static CONFIG_DEVELOPER: AtomicBool = AtomicBool::new(true);
// 数据消敏
static CONFIG_SHOW_MASK: AtomicBool = AtomicBool::new(true);
// 演示模式
static CONFIG_DOME_PREVIEW: AtomicBool = AtomicBool::new(false);
// 日志输出文件
static CONFIG_LOG_OUTPUT_FILE: AtomicBool = AtomicBool::new(false);
// 选择对象时候同时显示预览十张
static CONFIG_SHOW_OBJECT_PREVIEW: AtomicBool = AtomicBool::new(false);
// 不显示设置按钮
static CONFIG_HIDE_SETING_BUTTON: AtomicBool = AtomicBool::new(false);

// 每次修改都会导致配置id+1
static CONFIG_ID: AtomicUsize = AtomicUsize::new(0);

// 已经初始化
static VARIABLE_INITIALIZE: OnceLock<bool> = OnceLock::new();


#[derive(Debug)]
pub struct Config {
    // 并发线程数
    pub ThreadCount: usize,
    // 实时响应
    pub AutoAction: bool,
    // 全局扫描
    pub GlobalScan: bool,
    // 添加后立即扫描
    pub ScanAdding: bool,
    // 添加后立即添加到日志扫描
    pub ScanLogAdding: bool,
    // 授权联网
    pub Networking: bool,
    // 配置不存储
    pub IgnoreMutual: bool,
    // 任务配置保留
    pub PreserveConfig: bool,
    // 任务配置保留
    pub CreateCont: bool,
    // 任务连续创建
    pub PreserveList: bool,
    // 日志输出
    pub ConsoleLog: bool,
    // 开发者模式
    pub Developer: bool,
    // 数据消敏
    pub ShowMask: bool,
    // 演示模式
    pub DomePreview: bool,
    // 日志输出文件
    pub LogOutputFile: bool,
    // 立即预览选择对象
    pub ShowObjectPreview: bool,
    // 隐藏设置按钮
    pub HideSetingButton:bool,
}

#[derive(Debug)]
pub enum CONFIG_KEY {
    // 并发线程数
     ThreadCount,
    // 实时响应
     AutoAction,
    // 全局扫描
     GlobalScan,
    // 添加后立即扫描
     ScanAdding,
    // 添加后立即添加到日志扫描
     ScanLogAdding,
    // 授权联网
     Networking,
    // 配置不存储
     IgnoreMutual,
    // 任务配置保留
     PreserveConfig,
    // 任务连续创建
     CreateCont,
    // 列表保留（不GC）
     PreserveList,
    // 日志输出
     ConsoleLog,
    // 开发者模式
     Developer,
    // 数据消敏
     ShowMask,
    // 演示模式
     DomePreview,
    // 日志输出文件
     LogOutputFile,
     // 选定对象后显示最近10张
     ShowObjectPreview,
    //  隐藏设置按钮
     HideSetingButton,
 }

 
pub fn initialize_config() -> Config {

    let mut config = Config {
        ThreadCount: atomic_util::get_usize(&CONFIG_THREAD_COUNT),
        AutoAction: get_bool!(CONFIG_AUTO_ACTION),
        GlobalScan: get_bool!(CONFIG_GLOBAL_SCAN),
        HideSetingButton:get_bool!(CONFIG_HIDE_SETING_BUTTON),
        ScanAdding: get_bool!(CONFIG_SCAN_ADDING),
        ShowObjectPreview:get_bool!(CONFIG_SHOW_OBJECT_PREVIEW),
        ScanLogAdding: get_bool!(CONFIG_SCAN_LOG_ADDING),
        Networking: get_bool!(CONFIG_NETWORKING),
        IgnoreMutual: get_bool!(CONFIG_IGNORE_MUTUAL),
        PreserveConfig: get_bool!(CONFIG_PRESERVE_CONFIG),
        CreateCont: get_bool!(CONFIG_CREATE_CONT),
        PreserveList: get_bool!(CONFIG_PRESERVE_LIST),
        ConsoleLog: get_bool!(CONFIG_CONSOLE_LOG),
        Developer: is_developer(),
        ShowMask: get_bool!(CONFIG_DOME_PREVIEW)||get_bool!(CONFIG_SHOW_MASK),
        DomePreview: get_bool!(CONFIG_DOME_PREVIEW),
        LogOutputFile: get_bool!(CONFIG_LOG_OUTPUT_FILE),
    };

    if *(VARIABLE_INITIALIZE.get().unwrap_or_else(|| &false)) || APP_BUILD_52POJIE {
        return config;
    }


    let mut config_path  = std::path::PathBuf::from(APP_STORE_DIR).join(APP_STORE_NAME);

    if config_path.exists() {
        if let Ok(file) =  std::fs::read_to_string(config_path) {
            let mut data: serde_json::Value = serde_json::from_str(file.as_str()).unwrap_or_else(|_|json!({}));
            config.ThreadCount = data["ThreadCount"].as_i64().unwrap_or_else(||0i64) as usize;
            config.AutoAction = data["AutoAction"].as_bool().unwrap_or_else(||false);
            config.GlobalScan = data["GlobalScan"].as_bool().unwrap_or_else(||false);
            config.HideSetingButton = data["HideSetingButton"].as_bool().unwrap_or_else(||false);
            config.ScanAdding = data["ScanAdding"].as_bool().unwrap_or_else(||false);
            config.ScanLogAdding = data["ScanLogAdding"].as_bool().unwrap_or_else(||false);
            config.Networking = data["Networking"].as_bool().unwrap_or_else(||false);
            config.ShowObjectPreview = data["ShowObjectPreview"].as_bool().unwrap_or_else(||false);
            config.IgnoreMutual = data["IgnoreMutual"].as_bool().unwrap_or_else(||false);
            config.PreserveConfig = data["PreserveConfig"].as_bool().unwrap_or_else(||false);
            config.CreateCont = data["CreateCont"].as_bool().unwrap_or_else(||false);
            config.PreserveList = data["PreserveList"].as_bool().unwrap_or_else(||false);
            config.ConsoleLog = data["ConsoleLog"].as_bool().unwrap_or_else(||false);
            config.Developer = data.get("Developer").unwrap_or_else(||&json!(false)).as_bool().unwrap();
            config.ShowMask = data["ShowMask"].as_bool().unwrap_or_else(||false);
            config.DomePreview = data["DomePreview"].as_bool().unwrap_or_else(||false);
            config.LogOutputFile = data["LogOutputFile"].as_bool().unwrap_or_else(||false);

        }
       

    }

    set_bool!(CONFIG_AUTO_ACTION,config.AutoAction);
    set_bool!(CONFIG_GLOBAL_SCAN,config.GlobalScan);
    set_bool!(CONFIG_SCAN_ADDING,config.ScanAdding);
    set_bool!(CONFIG_HIDE_SETING_BUTTON,config.HideSetingButton);
    set_bool!(CONFIG_SHOW_OBJECT_PREVIEW,config.ShowObjectPreview);
    set_bool!(CONFIG_SCAN_LOG_ADDING,config.ScanLogAdding);
    set_bool!(CONFIG_NETWORKING,config.Networking);
    set_bool!(CONFIG_IGNORE_MUTUAL,config.IgnoreMutual);
    set_bool!(CONFIG_PRESERVE_CONFIG,config.PreserveConfig);
    set_bool!(CONFIG_CREATE_CONT,config.CreateCont);
    set_bool!(CONFIG_PRESERVE_LIST,config.PreserveList);
    set_bool!(CONFIG_CONSOLE_LOG,config.ConsoleLog);
    set_bool!(CONFIG_DEVELOPER,config.Developer);
    set_bool!(CONFIG_SHOW_MASK,config.ShowMask);
    set_bool!(CONFIG_DOME_PREVIEW,config.DomePreview);
    set_bool!(CONFIG_LOG_OUTPUT_FILE,config.LogOutputFile);
    set_usize(&CONFIG_THREAD_COUNT , config.ThreadCount);
    
    
    VARIABLE_INITIALIZE.set(true);

    config
}

pub fn config() -> Config {
 Config {
        ThreadCount: atomic_util::get_usize(&CONFIG_THREAD_COUNT),
        AutoAction: get_bool!(CONFIG_AUTO_ACTION),
        GlobalScan: get_bool!(CONFIG_GLOBAL_SCAN),
        HideSetingButton:get_bool!(CONFIG_HIDE_SETING_BUTTON),
        ScanAdding: get_bool!(CONFIG_SCAN_ADDING),
        ShowObjectPreview: get_bool!(CONFIG_SHOW_OBJECT_PREVIEW),
        ScanLogAdding: get_bool!(CONFIG_SCAN_LOG_ADDING),
        Networking: get_bool!(CONFIG_NETWORKING),
        IgnoreMutual: get_bool!(CONFIG_IGNORE_MUTUAL),
        PreserveConfig: get_bool!(CONFIG_PRESERVE_CONFIG),
        CreateCont: get_bool!(CONFIG_CREATE_CONT),
        PreserveList: get_bool!(CONFIG_PRESERVE_LIST),
        ConsoleLog: get_bool!(CONFIG_CONSOLE_LOG),
        Developer: is_developer(),
        ShowMask: get_bool!(CONFIG_SHOW_MASK),
        DomePreview: get_bool!(CONFIG_DOME_PREVIEW),
        LogOutputFile: get_bool!(CONFIG_LOG_OUTPUT_FILE),
    }
}

pub fn config_id ()->usize{
    get_usize(&CONFIG_ID)
}




// 设置配置值
pub fn set_config <T:lib::LoadConfigValue > (config: CONFIG_KEY , value:T ) -> bool {
    
    //initialize_config();

    if APP_BUILD_52POJIE {
        return  false;
    }
    macro_rules! set_config_bool {
        ($CONFIG:expr)=>{
            set_bool!($CONFIG, value.to_bool());
            for index in 0..5 {
                if(!get_bool!($CONFIG).eq(&value.to_bool())){
                    set_bool!($CONFIG, value.to_bool());
                }else{
                    break;
                }
            }

            if(!get_bool!($CONFIG).eq(&value.to_bool())){
                return false;
            }
        }
    }

    macro_rules! set_config_usize {
        ($CONFIG:expr)=>{
            set_usize(&$CONFIG, value.to_usize());
            for index in 0..5 {
                if(!atomic_util::get_usize(&$CONFIG).eq(&value.to_usize())){
                    set_usize(&$CONFIG, value.to_usize());
                    
                }else{
                    break;
                }
            }
            
            if(!atomic_util::get_usize(&$CONFIG).eq(&value.to_usize())){
                return false;
            }
        }
    }

    match config {
        CONFIG_KEY::ThreadCount=> {set_config_usize!(CONFIG_THREAD_COUNT);}
        CONFIG_KEY::AutoAction=> {set_config_bool!(CONFIG_AUTO_ACTION);}
        CONFIG_KEY::GlobalScan=> {set_config_bool!(CONFIG_GLOBAL_SCAN);}
        CONFIG_KEY::HideSetingButton=> {set_config_bool!(CONFIG_HIDE_SETING_BUTTON);}
        CONFIG_KEY::ScanAdding=> {set_config_bool!(CONFIG_SCAN_ADDING);}
        CONFIG_KEY::HideSetingButton=> {set_config_bool!(CONFIG_HIDE_SETING_BUTTON);}
        CONFIG_KEY::ShowObjectPreview=>{set_config_bool!(CONFIG_SHOW_OBJECT_PREVIEW);}
        CONFIG_KEY::ScanLogAdding=> {set_config_bool!(CONFIG_SCAN_LOG_ADDING);}
        CONFIG_KEY::Networking=> {set_config_bool!(CONFIG_NETWORKING);}
        CONFIG_KEY::IgnoreMutual=> {set_config_bool!(CONFIG_IGNORE_MUTUAL);}
        CONFIG_KEY::PreserveConfig=> {set_config_bool!(CONFIG_PRESERVE_CONFIG);}
        CONFIG_KEY::CreateCont=> {set_config_bool!(CONFIG_CREATE_CONT);}
        CONFIG_KEY::PreserveList=> {set_config_bool!(CONFIG_PRESERVE_LIST);}
        CONFIG_KEY::ConsoleLog=> {set_config_bool!(CONFIG_CONSOLE_LOG);}
        CONFIG_KEY::Developer=> {set_config_bool!(CONFIG_DEVELOPER);}
        CONFIG_KEY::ShowMask=> {set_config_bool!(CONFIG_SHOW_MASK);}
        CONFIG_KEY::DomePreview=> {set_config_bool!(CONFIG_DOME_PREVIEW);}
        CONFIG_KEY::LogOutputFile=> {set_config_bool!(CONFIG_LOG_OUTPUT_FILE);}
    }

    add_usize(&CONFIG_ID, 1);
   
    true
}

pub fn get_config_bool (config: CONFIG_KEY) -> bool {
    //initialize_config();
    match config {
        CONFIG_KEY::ThreadCount=> {return  false;}
        CONFIG_KEY::AutoAction=> {get_bool!(CONFIG_AUTO_ACTION)}
        CONFIG_KEY::GlobalScan=> {get_bool!(CONFIG_GLOBAL_SCAN)}
        CONFIG_KEY::ScanAdding=> {get_bool!(CONFIG_SCAN_ADDING)}
        CONFIG_KEY::HideSetingButton=> {get_bool!(CONFIG_HIDE_SETING_BUTTON)}
        CONFIG_KEY::ShowObjectPreview=>{get_bool!(CONFIG_SHOW_OBJECT_PREVIEW)}
        CONFIG_KEY::ScanLogAdding=> {get_bool!(CONFIG_SCAN_LOG_ADDING)}
        CONFIG_KEY::Networking=> {get_bool!(CONFIG_NETWORKING)}
        CONFIG_KEY::IgnoreMutual=> {get_bool!(CONFIG_IGNORE_MUTUAL)}
        CONFIG_KEY::PreserveConfig=> {get_bool!(CONFIG_PRESERVE_CONFIG)}
        CONFIG_KEY::CreateCont=> {get_bool!(CONFIG_CREATE_CONT)}
        CONFIG_KEY::PreserveList=> {get_bool!(CONFIG_PRESERVE_LIST)}
        CONFIG_KEY::ConsoleLog=> {get_bool!(CONFIG_CONSOLE_LOG)}
        CONFIG_KEY::Developer=> {get_bool!(CONFIG_DEVELOPER)}
        CONFIG_KEY::ShowMask=> {get_bool!(CONFIG_SHOW_MASK)}
        CONFIG_KEY::DomePreview=> {get_bool!(CONFIG_DOME_PREVIEW)}
        CONFIG_KEY::LogOutputFile=> {get_bool!(CONFIG_LOG_OUTPUT_FILE)}
    }
}

pub fn store_config() -> bool {
    //initialize_config();
    if APP_BUILD_52POJIE {
        libWxIkunPlus::stop("存在BUG 作者正在处理中。。。", "配置值多处链式关联   (有启用导致违规 不启用导致软件奔溃) 的可能 \n禁用部分选项，或与非逻辑判断错误将可能导致软件无法正常工作或者奔溃 \n存在无法避免的问题，请使用默认值！\n作者会尽快解决此问题");
        return  false;
    }

    let mut data: serde_json::Value = json!({ });
    let mut config_dir  = std::path::PathBuf::from(APP_STORE_DIR);
    let  config = config();
    // println!("[store_config]config->{:?}",&config);

    data["ThreadCount"] = serde_json::Value::Number(config.ThreadCount.into());
    data["AutoAction"] = serde_json::Value::Bool(config.AutoAction);
    data["GlobalScan"] = serde_json::Value::Bool(config.GlobalScan);
    data["HideSetingButton"] = serde_json::Value::Bool(config.HideSetingButton);
    data["ScanAdding"] = serde_json::Value::Bool(config.ScanAdding);
    data["ScanLogAdding"] = serde_json::Value::Bool(config.ScanLogAdding);
    data["Networking"] = serde_json::Value::Bool(config.Networking);
    data["IgnoreMutual"] = serde_json::Value::Bool(config.IgnoreMutual);
    data["PreserveConfig"] = serde_json::Value::Bool(config.PreserveConfig);
    data["CreateCont"] = serde_json::Value::Bool(config.CreateCont);
    data["PreserveList"] = serde_json::Value::Bool(config.PreserveList);
    data["ConsoleLog"] = serde_json::Value::Bool(config.ConsoleLog);
    data["Developer"] = serde_json::Value::Bool(config.Developer);
    data["ShowMask"] = serde_json::Value::Bool(config.ShowMask);
    data["DomePreview"] = serde_json::Value::Bool(config.DomePreview);
    data["LogOutputFile"] = serde_json::Value::Bool(config.LogOutputFile);
    // // println!("[store_config]config->{:#?}",&data);

    if config_dir.exists() {
        if !config_dir.is_dir() {
            return  false ;
        }
        if let Err(item) = std::fs::create_dir_all(config_dir.clone()) {
            return false; 
        }
    }

    let pretty = serde_json_fmt::JsonFormat::pretty()
    .indent_width(Some(4))
    .ascii(true)
    .format_to_string(&data);

    if let Ok(pretty) = pretty {
        
        if let Err(item) = std::fs::write(config_dir.join(APP_STORE_NAME),pretty.as_bytes()) {
            return false; 
        }
    
    }else{
        return false; 
    }


    true
}

// 判断当前是否处于开发者模式
pub fn is_developer() -> bool {
    //initialize_config();
    // println!("is_developer()->{}",!APP_BUILD_52POJIE && APP_ENABLE_DEVELOPER && get_bool!(CONFIG_DEVELOPER));
    !APP_BUILD_52POJIE && APP_ENABLE_DEVELOPER && get_bool!(CONFIG_DEVELOPER)

}

// 编译版本是 52破解专版
pub fn is_build_52pojie() -> bool {
    //initialize_config();
    APP_BUILD_52POJIE
}

// 是否对显示的数据进行消敏
pub fn is_show_mask() -> bool {
    //initialize_config();
    is_show_dome() || get_bool!(CONFIG_SHOW_MASK)
}

// 是否在选择对象后自动显示最近十张照片
pub fn is_click_open_preview() -> bool {
    //initialize_config();
    get_bool!(CONFIG_SHOW_OBJECT_PREVIEW)
}

// 演示模式
pub fn is_show_dome() -> bool {
    //initialize_config();
    get_bool!(CONFIG_DOME_PREVIEW)
}

