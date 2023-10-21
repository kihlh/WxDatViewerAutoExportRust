use crate::util::str_eq_ostr;
use crate::{console_log, global_var_util, gui_util, handle_dat, libWxIkunPlus, wh_mod};
use chrono::Local;
use glob::glob;
use rusqlite::{params, Connection, Result};
use serde_json::json;
use serde_json::Value as Json;
use toml::Value as Toml;
use crate::global_var;
use crate::gui_main_ui::THE_WIN_CLASS_NAME;
use crate::config;

pub struct AppVersionInfo {}

macro_rules! get_the_hwnd {
    ($class_id:expr) => {{
        let mut _hwnd = 0;
        for _ in 0..8 {
            _hwnd = libWxIkunPlus::findWindow($class_id, "");
            if !libWxIkunPlus::isWindow(_hwnd) {
                _hwnd = 0;
            } else {
                break;
            }
            fltk::app::sleep(0.020);
        }
        _hwnd as i128
    }};
    () => {
        get_the_hwnd!(THE_WIN_CLASS_NAME)
    };
}

fn toml2json(toml: Toml) -> Json {
    match toml {
        Toml::String(s) => Json::String(s),
        Toml::Integer(i) => Json::Number(i.into()),
        Toml::Float(f) => {
            let n = serde_json::Number::from_f64(f).expect("float infinite and nan not allowed");
            Json::Number(n)
        }
        Toml::Boolean(b) => Json::Bool(b),
        Toml::Array(arr) => Json::Array(arr.into_iter().map(toml2json).collect()),
        Toml::Table(table) => {
            Json::Object(table.into_iter().map(|(k, v)| (k, toml2json(v))).collect())
        }
        Toml::Datetime(dt) => Json::String(dt.to_string()),
    }
}

pub fn get_app_version_info() -> Json {
    const APP_VERSION: &str = include_str!("../../../Cargo.toml");
    // println!("toml2json-> {:?}",toml2json(APP_VERSION));

    match APP_VERSION.parse() {
        Ok(toml) => {
            let json = toml2json(toml);
            return json;
        }
        Err(error) => println!("failed to parse TOML: {}", error),
    }

    json!("")
}

// 获取初始化文本
pub fn get_init_text() -> String {
    let mut result = String::new();
    let mut sync_type = String::new();
    let mut build_name = if config::is_build_52pojie() {
        "52破解专版"
    } else {
        "开源版"
    };
    let version_info = get_app_version_info();
    let version = (version_info["package"]["version"]).as_str().unwrap();

    if !config::is_developer() {
        result.push_str(
            format!(
                r#"作者 @Ikun 软件开源协议 GPL 3.0 (但是并不包含解码算法)
        当前版本：{} ({})
        本软件 是免费软件 如果付费请维权退款
        本软件只供节约自己另存为图片时间，禁止用于其他用途
        "#,
                version, build_name
            )
            .replace("  ", "")
            .as_str(),
        );
    } else {
        result.push_str(("初始化成功 [开发者模式]"));
    }

    if libWxIkunPlus::has_auto_sync() {
        result.push_str(format!("\n[用户] 自动同步开启").as_str());
    } else if config::is_developer() {
        result.push_str("\n[同步] 自动同步已启用 因为开发者模式有效");
    } else {
        result.push_str("\n[同步] 自动同步关闭");
    }

    result
}

// 添加进数据库
pub fn push_sql_export_dir_path(name: &str, export_dir: &str, task_command: &str) {
    if !eq_next() {
        libWxIkunPlus::stop(
            "错误".to_owned(),
            "当前未发现wx进程或者未登录 拒绝提供添加".to_owned(),
        );
        return;
    }
    if name.is_empty() {
        console_log!(format!("\n[错误] 没有名称"));
        gui_util::sub_message(get_the_hwnd!(), gui_util::IconType::Failure, "[错误] 没有名称", 3500u64);
        return;
    }

    if export_dir.is_empty() {
        console_log!(format!("\n[错误] 没有设置导出到的路径"));
        gui_util::sub_message(get_the_hwnd!(), gui_util::IconType::Failure, "[错误] 没有设置导出到的路径", 3500u64);
        return;
    }

    if task_command.is_empty() {
        console_log!(format!("\n[错误] 没有任务命令"));
        gui_util::sub_message(get_the_hwnd!(), gui_util::IconType::Failure, "[错误] 没有任务命令", 3500u64);
        return;
    }

    let conn: Connection = Connection::open("ikun_user_data.db").unwrap();

    handle_dat::initialize_table(&conn);
    match conn.execute(
        "INSERT INTO export_dir_path (name,time,path,ouput) values (?1, ?2, ?3, ?4)",
        [
            name,
            Local::now().format("%Y-%m-%d").to_string().as_str(),
            task_command,
            export_dir,
        ],
    ) {
        Ok(_) => {
            console_log!(format!("\n[存储] 添加成功"));
            gui_util::sub_message(
                get_the_hwnd!(),
                gui_util::IconType::Success,
                "添加成功",
                3500u64,
            );
        }
        Err(err) => {
            if (str_eq_ostr(
                err.to_string(),
                "UNIQUE constraint failed: export_dir_path.path",
            )) {
                console_log!(format!("\n[错误] 添加失败 因为-> {}", "当前任务已经存在"));
                gui_util::sub_message(
                    get_the_hwnd!(),
                    gui_util::IconType::Warning,
                    "当前任务已经存在",
                    3500u64,
                );
            } else {
                gui_util::sub_message(
                    get_the_hwnd!(),
                    gui_util::IconType::Failure,
                    "任务添加失败",
                    3500u64,
                );
            }
        }
    }

    conn.close();
    global_var_util::update_export_dir_itme_list();
}

pub fn eq_next() -> bool {
    (config::is_developer()
        || (libWxIkunPlus::hasWeChat() && libWxIkunPlus::hasWeChatWin()))
}

// 测试
pub fn test_task(name: &str, export_dir: &str, task_command: &str) {
    let mut path_dir = wh_mod::parse_dat2var_path(format!("{}", task_command));

    if name.is_empty() {
        console_log!(format!("\n[警告] 没有名称"));
    }

    if export_dir.is_empty() {
        console_log!(format!("\n[警告] 没有设置导出到的路径"));
    }

    if task_command.is_empty() {
        console_log!(format!("\n[警告] 没有任务命令"));
    }

    if let Ok(metadata) = std::fs::metadata(path_dir.attach_dir.clone()) {
        if (!metadata.is_dir()) {
            console_log!(format!("\n[错误] dat目录文件夹 不是文件夹"));
            gui_util::sub_message(
                get_the_hwnd!(),
                gui_util::IconType::Failure,
                "dat目录文件夹 不是文件夹",
                3500u64,
            );
            return;
        }

        console_log!(format!("\n[测试] 正在扫描当前文件夹存在的dat图片"));
        console_log!(format!(
            "\n[测试] 处理范围: 仅本月:{}   缩略图:{}   原图:{}   视频:{}   同步:{}   全部:{}   ",
            bool_to_str(path_dir.is_the_month),
            bool_to_str(path_dir.is_thumbnail),
            bool_to_str(path_dir.is_source),
            bool_to_str(path_dir.is_video),
            bool_to_str(path_dir.is_sync),
            bool_to_str(path_dir.is_all)
        ));

        let pattern = format!(
            "{}",
            std::path::Path::new(&path_dir.attach_dir.clone())
                .join("**/*.dat")
                .display()
                .to_string()
        );

        let mut index = 0;

        console_log!(format!("\n[测试] 开始扫描 “{}” 中的dat文件", pattern));

        for entry in glob(&pattern).unwrap() {
            index = index + 1;
        }

        console_log!(format!(
            "\n[测试] 在 “{}” \n中发现了 [{}] 个dat文件",
            pattern, index
        ));
        gui_util::sub_message(
            get_the_hwnd!(),
            gui_util::IconType::Success,
            "测试成功",
            3500u64,
        );

        return;
    }
    console_log!(format!(
        "\n[错误] dat目录文件夹 无法被读取",
    ));
    gui_util::sub_message(
        get_the_hwnd!(),
        gui_util::IconType::Failure,
        "dat目录文件夹 打开失败",
        3500u64,
    );
}

fn bool_to_str (b:bool) -> &'static str {
    if b {"是"} else { "否" }
}
