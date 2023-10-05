#![allow(warnings, unused)]

use crate::{
    get_arc_bind_variable, global_var, gui_detect_config, gui_drag_scan, gui_hotspot, gui_imge,
    gui_text_control, handle_dat, libWxIkunPlus, read_rw_lazy_lock, read_rw_lock,
    set_arc_bind_variable, set_arc_bind_variable_insert,
    util::{str_eq_str, Sleep},
    wh_mod::{self, AttachThumbnail},
    write_rw_lock, write_rw_lock_insert,
};
use chrono::Local;
use fltk::{
    app::{self, handle},
    button::{self, Button},
    dialog,
    draw::font,
    enums::{self, Color, Cursor, Event, Font, FrameType, LabelType},
    frame,
    frame::Frame,
    group::{self, Flex, Group},
    image,
    input::{self, InputType, IntInput},
    prelude::*,
    text::TextDisplay,
    tree,
    window::{self, DoubleWindow, Window},
};
use fltk_table::{SmartTable, TableOpts};
use fltk_theme::{color_themes, ColorTheme, SchemeType, ThemeType, WidgetScheme, WidgetTheme};
use once_cell::sync::Lazy;
use rusqlite::Connection;
use std::sync::{mpsc, MutexGuard};
use std::time::{SystemTime, UNIX_EPOCH};

use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::ops::Deref;
use std::ptr::null;
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::{
    fs,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    process,
    process::{Command, Stdio},
    thread,
};

use crate::gui_imge::ImgPreview;
use crate::libWxIkunPlus::closeWindow;
use crate::watching::insert_watch_path_token;
use arc_swap::ArcSwap;
use fltk::draw::{height, width};
use fltk::image::PngImage;
// use lazy_static::lazy_static;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::OnceLock;
use winapi::um::winnt::LPWSTR;
use winapi::um::winuser::{CloseWindow, SetActiveWindow};

static REQUEST_RECV: AtomicUsize = AtomicUsize::new(0);
// static mut static_var:Vec<ImgPreview> = Vec::new();
// static mut static_atomic :AtomicUsize = AtomicUsize::new(0);

// 使用原子锁代替线程锁 避免死锁

// 图片预览 全局变量
static mut IMG_PREVIEW_LIST: Vec<ImgPreview> = Vec::new();
static IMG_PREVIEW_LIST_BIND: AtomicUsize = AtomicUsize::new(0);

// 缩略图 全局变量
static mut THUMBNAIL_LIST: Vec<wh_mod::AttachThumbnail> = Vec::new();
static THUMBNAIL_LIST_BIND: AtomicUsize = AtomicUsize::new(0);

// wxid 全局变量
static mut WX_ID: String = String::new();
static WX_ID_BIND: AtomicUsize = AtomicUsize::new(0);

// 用户wx存储位置 全局变量
static mut USER_PATH: String=String::new();
static USER_PATH_BIND: AtomicUsize = AtomicUsize::new(0);

// 使用原子锁代替线程锁 避免死锁
// fn set_IMG_PREVIEW_LIST(value:Vec<ImgPreview>){

//     set_arc_bind_variable!(THUMBNAIL_LIST,THUMBNAIL_LIST_BIND,value);

//     get_arc_bind_variable!(IMG_PREVIEW_LIST,IMG_PREVIEW_LIST_BIND);

// }

// thread_local! {
// // static WX_ID_ARC: RwLock<Arc<String>> = RwLock::new(Arc::new(String::new()));
// static USER_PATH_ARC: RwLock<Arc<String>> = RwLock::new(Arc::new(String::new()));
// // static THUMBNAIL_LIST_ARC: RwLock<Arc<Vec<wh_mod::AttachThumbnail>>> = RwLock::new(Arc::new(Vec::new()));
// // static IMG_PREVIEW_LIST_ARC: RwLock<Arc<Vec<ImgPreview>>> = RwLock::new(Arc::new(Vec::new()));


// }

// thread_local! {
//         static IMG_PREVIEW_LIST_ARCLAZY: ArcSwap< Vec<ImgPreview> > = ArcSwap::from_pointee(Vec::new().into());
// }

// 设置背景为图片（主视图）
fn setInterfaceBackgroundImage(appMainWin: &mut window::DoubleWindow) -> Frame {
    let background_image =
        // image::SvgImage::from_data(include_str!("./assets/select_user_base/main.svg"))
        image::PngImage::from_data(include_bytes!("./assets/select_user_base/main.png"))
            .expect("set main icon error");
    let mut frame = Frame::default().with_size(600, 0).center_of(appMainWin);
    frame.set_frame(FrameType::EngravedBox);
    frame.set_image(Some(background_image));

    return frame;
}

// 按钮基本都是以坐标判断的
struct PointExistHasmap {
    // 显示手型
    existCursor: bool,
}

// 判断鼠标坐标是否在此元素内
fn getFormPointSpace(x: i32, y: i32) -> PointExistHasmap {
    // 输入: x,y,w,y
    macro_rules! check_point_in_space {
        ($xx:expr, $yy:expr, $width:expr, $height:expr) => {
            x > $xx && x < $xx + $width && y > $yy && y < $yy + $height
        };
    }
    let existCursor = false;
    // IMG_PREVIEW_LIST_KEEP;
    let mut point_exist_hasmap = PointExistHasmap { existCursor: false };

    point_exist_hasmap.existCursor = existCursor;

    return point_exist_hasmap;
}

pub struct SelectUserBaseMain {
    // 选择的路径
    pub select_path: Vec<String>,
    // pub win: DoubleWindow,
    pub thumbnail: bool,
    // 是否导出原图
    pub source: bool,
    // 是否仅限本月
    pub the_month: bool,
    // 数据库位置
    pub database: String,
}

impl Clone for SelectUserBaseMain {
    fn clone(&self) -> Self {
        SelectUserBaseMain {
            select_path: self.select_path.clone(),
            // win:self.win.clone(),
            thumbnail: self.thumbnail.clone(),
            source: self.source.clone(),
            the_month: self.the_month.clone(),
            database: self.database.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}

struct UpdateJpgItme {
    index: usize,
    jpg: image::JpegImage,
    path: String,
}

struct PicturePreviewItem {
    // pub main_id:String,
    pub picture_id: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl PicturePreviewItem {
    pub fn get_picture(&self) -> Frame {
        let mut frame: Frame = app::widget_from_id(self.picture_id.as_str()).unwrap();
        frame
    }

    // 判断鼠标是否在当前元素
    pub fn existPoint(&self, x: i32, y: i32) -> bool {
        return x > self.x.clone()
            && x < self.x.clone() + self.width.clone()
            && y > self.y.clone()
            && y < self.y.clone() + self.height.clone();
    }
}

fn get_next_id() -> usize {
    let mut id: usize = 0;
    let mutex = Arc::new(Mutex::new(&REQUEST_RECV));
    mutex.lock();
    id = REQUEST_RECV.fetch_add(1, Ordering::Relaxed);
    drop(mutex);
    println!("id-> {}", id);
    id
}

fn push_wx_user_table(select_path: String, user_name: String) {
    // let mut lazy_value = USER_PATH.lock().unwrap();

    // if lazy_value.contains(select_path.as_str()) {
    //     return;
    // }

    // *lazy_value = select_path.clone();
    // drop(lazy_value);
    // if read_rw_lock!(USER_PATH_ARC, String::new()).contains(select_path.as_str()) {
    //     return;
    // }
    if(get_arc_bind_variable!(USER_PATH,USER_PATH_BIND).contains(&select_path)){
        return;
    }
    // write_rw_lock!(USER_PATH_ARC, select_path.clone());
    set_arc_bind_variable!(USER_PATH,USER_PATH_BIND,select_path.clone());

    
    thread::spawn(|| {
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
struct UserWxRootHistory {
    time: String,
    path: String,
    name: String,
}
fn get_wx_user_history_path() -> Result<UserWxRootHistory, rusqlite::Error> {
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

fn update_preview_main() {
    // println!(
    //     "[preview_main] IMG_PREVIEW_LIST_ARC-> {}",
    //     read_rw_lock!(IMG_PREVIEW_LIST_ARC, Vec::new()).len()
    // );
    // // println!(
    // //     "[preview_main] IMG_PREVIEW_LIST_ARCSWAP-> {}",
    // //     IMG_PREVIEW_LIST_ARCLAZY.with().len()
    // // );

    // println!(
    //     "[preview_main] IMG_PREVIEW_LIST_ARCSWAP-> {}",
    //     read_rw_lazy_lock!(IMG_PREVIEW_LIST_ARCLAZY).len()
    // );

    // println!(
    //     "[preview_main]THUMBNAIL_LIST-> {}",
    //     get_arc_bind_variable!(IMG_PREVIEW_LIST,IMG_PREVIEW_LIST_BIND).len()
    // );

    println!(
        "[preview-main]IMG_PREVIEW_LIST-> {}",
        get_arc_bind_variable!(IMG_PREVIEW_LIST, IMG_PREVIEW_LIST_BIND).len()
    );
    println!(
        "[preview-main]THUMBNAIL_LIST-> {}",
        get_arc_bind_variable!(THUMBNAIL_LIST, THUMBNAIL_LIST_BIND).len()
    );

    // 取出缩略图列表 并将其缩减到5条以内
    let mut thumbnail_list = {
        // let mut thumbnail_list = read_rw_lock!(THUMBNAIL_LIST_ARC, Vec::new()).to_vec();
        let mut thumbnail_list =
            get_arc_bind_variable!(THUMBNAIL_LIST, THUMBNAIL_LIST_BIND).to_vec();
        let mut atid_list: HashMap<String, AttachThumbnail> = HashMap::new();

        for value in thumbnail_list {
            let key = value.attach_id.clone();
            let mut oid_created = UNIX_EPOCH;
            let mut new_created = UNIX_EPOCH;

            // oid create time
            if let Some(thumbnail) = atid_list.get(&key) {
                if let Ok(metadata) = fs::metadata(thumbnail.thumbnail_path.clone()) {
                    if let Result::Ok(create) = metadata.created() {
                        oid_created = create;
                    }
                }
            }

            // new create time
            if let Ok(metadata) = fs::metadata(value.thumbnail_path.clone()) {
                if let Result::Ok(create) = metadata.created() {
                    new_created = create;
                }
            }

            // 按照创建时间判断是否更新视图
            if (new_created > oid_created) {
                atid_list.insert(value.attach_id.clone(), value);
            }
        }

        println!("atid_list size -> {}", atid_list.len());

        let mut thumbnail_list: Vec<AttachThumbnail> = Vec::new();

        for (key, value) in atid_list {
            thumbnail_list.push(value);
        }

        thumbnail_list.sort_by(|a, b| {
            let mut a_created = UNIX_EPOCH;
            let mut b_created = UNIX_EPOCH;

            if let Ok(metadata) = fs::metadata(a.thumbnail_path.clone()) {
                if let Result::Ok(create) = metadata.created() {
                    a_created = create;
                }
            }

            if let Ok(metadata) = fs::metadata(b.thumbnail_path.clone()) {
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

    // println!("[328] thumbnail_list-> {}",thumbnail_list.len());

    // write_rw_lock!(THUMBNAIL_LIST_ARC, thumbnail_list.to_vec());
    set_arc_bind_variable!(THUMBNAIL_LIST, THUMBNAIL_LIST_BIND, thumbnail_list.to_vec());

    // println!(
    //     "[preview_main_end] IMG_PREVIEW_LIST_ARC-> {}",
    //     read_rw_lock!(IMG_PREVIEW_LIST_ARC, Vec::new()).len()
    // );
    // println!(
    //     "[preview_main_end] IMG_PREVIEW_LIST_ARCSWAP-> {}",
    //     read_rw_lazy_lock!(IMG_PREVIEW_LIST_ARCLAZY).len()
    // );
    // println!(
    //     "[preview_main_end]THUMBNAIL_LIST-> {}",
    //     get_arc_bind_variable!(IMG_PREVIEW_LIST,IMG_PREVIEW_LIST_BIND).len()
    // );

    // 更新到视图中
    let thumbnail_list = get_arc_bind_variable!(THUMBNAIL_LIST, THUMBNAIL_LIST_BIND);

    // 锁定缩略图更新
    let mutex = Arc::new(Mutex::new(&THUMBNAIL_LIST_BIND));
    mutex.lock();

    let img_preview_list = get_arc_bind_variable!(IMG_PREVIEW_LIST, IMG_PREVIEW_LIST_BIND);

    let (width, height) = (75, 75);

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
            }
        }
    }
    drop(mutex);
}

// 开始获取更新
fn initialize_watch_path_puppet(path: String) {
    let mut frame: Frame = app::widget_from_id("watch_path_user_name").unwrap();
    let mut btn_next: Frame = app::widget_from_id("gui::btn_text_recent_pictures").unwrap();
    let copy_path = path.clone();
    let watch_path = path.clone();
    // println!("initialize_watch_path_puppet-> {}",path.clone());

    let path_list = wh_mod::sort_modified_dir(path.clone().as_ref());
    let mut has_wx = false;
    let mut match_wxid_len = 0;
    let mut wxid = String::new();
    for path in path_list.clone() {
        if (path.to_string_lossy().contains("wxid_")) {
            match_wxid_len = match_wxid_len + 1;
        }

        if (path.to_string_lossy().contains("wxid_")
            || path.to_string_lossy().contains("Applet")
            || path.to_string_lossy().contains("All Users"))
        {
            has_wx = true;
        }
    }

    if (!has_wx) {
        dialog::alert_default("此路径不是有效的wx的根目录");
        return;
    }

    if (match_wxid_len >= 1) {
        for for_path in path_list.clone() {
            let file_name = for_path
                .file_name()
                .unwrap_or_else(|| "".as_ref())
                .to_string_lossy()
                .to_string();

            if file_name.contains("wxid_") {
                wxid = file_name.clone();
                // let mut lazy_value = WX_ID.lock().unwrap();
                // *lazy_value = file_name.clone();
                // drop(lazy_value);

                // write_rw_lock!(WX_ID_ARC, file_name.clone());
                set_arc_bind_variable!(WX_ID,WX_ID_BIND,file_name.clone());

                // let get_wxid_acc = wh_mod::convert::get_wxid_name(format!("{}",for_path.to_str()),wxid.clone());
                if let Some(get_wxid_acc) = wh_mod::convert::get_wxid_name(
                    format!("{}", for_path.to_str().unwrap_or_else(|| &"")),
                    wxid.clone(),
                ) {
                    frame.set_label(&format!("{}  [ {} ]", file_name, get_wxid_acc.name));
                } else {
                    frame.set_label(
                        format!("{}  [ {} ]", file_name, wh_mod::wx_account_id(for_path).id)
                            .as_str(),
                    );
                };
                // 显示到ui
                // frame.set_label(
                //     format!("{}  [ {} ]", file_name,get_wxid_acc /*wh_mod::wx_account_id(for_path).id).as_str()*/,
                // );
                frame.redraw();
                btn_next.set_label("检测");
                // btn_next.redraw();
                // let attach_path = Path::new(copy_path.as_str());
                push_wx_user_table(path.clone(), file_name);
                global_var::set_string("user::config::user_select_wxid", wxid.clone());

                let copy_path = format!("{}/{}/FileStorage/MsgAttach", copy_path.as_str(), wxid);
                let copy_path_wake = format!("{}", watch_path);

                // 取得缩略图
                thread::spawn(move || {
                    // 扫描最近文件夹
                    let path = Path::new(copy_path.as_str());
                    let imag = wh_mod::read_attach_buff_thumbnail_list(path, 5, 1);

                    let mut data_list = Vec::new();

                    //
                    for imag in imag {
                        println!("{}", imag.thumbnail_path.clone());
                        data_list.push(imag);
                    }
                    // *lazy_value = data_list.clone();
                    // drop(lazy_value);

                    // write_rw_lock_insert!(THUMBNAIL_LIST_ARC, data_list.to_vec());
                    // let mut oid_thumbnail_list = get_arc_bind_variable!(THUMBNAIL_LIST,THUMBNAIL_LIST_BIND).to_vec();

                    // for value in data_list {
                    //     oid_thumbnail_list.push(value);
                    // }

                    set_arc_bind_variable_insert!(
                        THUMBNAIL_LIST,
                        THUMBNAIL_LIST_BIND,
                        data_list.to_vec()
                    );

                    // println!("read_rw_lock!(THUMBNAIL_LIST_ARC).unwrap().len() ->  {}",read_rw_lock!(THUMBNAIL_LIST_ARC).unwrap().len());

                    if (data_list.len() > 0) {
                        update_preview_main();
                    }

                    // 释放 并已更新

                    // 启动日志检测模式
                    let (tx, rx) = std::sync::mpsc::channel();

                    let wh_id = wh_mod::watch_path::watch_path_puppet(copy_path_wake.clone(), tx);
                    println!("copy_path_wake-> {}", copy_path_wake.clone());
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
                                update_preview_main();
                            }
                        }
                    }
                    // wh_mod::watch_path
                });

                break;
            }
        }
    } else {
        frame.set_label("存在多个用户 请手动发送图片确认...");
        frame.redraw();
        btn_next.set_label("刷新");
        btn_next.redraw();
    }

    // frame.set_label("开始扫描中...");
}

fn wx_ready_initialize_open_preview_main_up(path: String) {
    let path_list = wh_mod::sort_modified_dir(path.clone().as_ref());

    for path in path_list {
        println!("{:?}", path);
        // let path_list = sort_modified_dir(path.as_ref());
        //
        // for path in path_list {
        //
        // }
    }
}

// 创建选择的窗口
pub fn mian_window() -> SelectUserBaseMain {
    if (global_var::get_bool_default("gui::open::handle_dat")) {
        if let Some(mut wins) =
            app::widget_from_id("gui::DoubleWindow::handle_dat::main") as Option<DoubleWindow>
        {
            wins.show();
            wins.set_visible_focus();
        }
        let mut select_user_base: SelectUserBaseMain = SelectUserBaseMain {
            // win:DoubleWindow::new(0, 0, 600, 531, "Ikun导出"),
            select_path: Vec::new(),
            thumbnail: false,
            source: false,
            the_month: false,
            database: String::new(),
        };

        return select_user_base;
    }

    global_var::set_bool("gui::open::handle_dat", true);

    // 设置主题
    let theme = ColorTheme::new(color_themes::BLACK_THEME);
    let widget_theme = WidgetTheme::new(ThemeType::HighContrast);
    widget_theme.apply();
    let widget_scheme = WidgetScheme::new(SchemeType::Aqua);
    widget_scheme.apply();
    theme.apply();

    let mut select_user_base: SelectUserBaseMain = SelectUserBaseMain {
        // win:DoubleWindow::new(0, 0, 600, 531, "Ikun导出"),
        select_path: Vec::new(),
        thumbnail: false,
        source: false,
        the_month: false,
        database: String::new(),
    };

    let mut win: DoubleWindow = DoubleWindow::new(0, 0, 600, 470, "任务创建向导").center_screen();
    win.set_color(Color::from_rgb(24, 24, 24));
    win.set_border(false);
    win.set_id("gui::DoubleWindow::handle_dat::main");

    // fltk::app::set_scrollbar_size(3);
    // fltk::app::set_selection_color(24, 24, 24);
    setInterfaceBackgroundImage(&mut win);
    // 背景已经绘制

    let mut btn_exit_imag = gui_imge::create_Images(
        &mut win,
        540,
        15,
        37,
        37,
        image::PngImage::from_data(include_bytes!("./assets/select_user_base/exit.png"))
            .expect("set exit_imag_btn icon error"),
        "btn_exit_select".to_owned(),
    );

    let mut flex = group::Flex::default()
        .with_size(395 + 60, 30)
        .row()
        .center_of_parent();

    flex.set_pos(75, 396);

    macro_rules! show_cursor {
        ($show_cursor_itme:expr) => {
            let mut new_win = win.clone();
            $show_cursor_itme.handle({
                move |win, ev| match ev {
                    enums::Event::Move => {
                        new_win.set_cursor(Cursor::Hand);
                        true
                    }
                    enums::Event::Leave => {
                        new_win.set_cursor(Cursor::Default);
                        true
                    }
                    _ => false,
                }
            });
        };
    }

    let mut check_button_thumbnail = button::CheckButton::default().with_label("保存缩略图");
    let mut check_button_source = button::CheckButton::default().with_label("保存原图(如果有)");
    let mut check_button_the_month = button::CheckButton::default().with_label("只保存本月图片");

    check_button_thumbnail.set_callback(|win| {
        global_var::set_bool("user::config::check_button_thumbnail", win.is_checked());
    });

    check_button_source.set_callback(|win| {
        global_var::set_bool("user::config::check_button_source", win.is_checked());
    });

    check_button_the_month.set_callback(|win| {
        global_var::set_bool("user::config::check_button_the_month", win.is_checked());
    });

    show_cursor!(check_button_thumbnail);
    show_cursor!(check_button_source);
    show_cursor!(check_button_the_month);

    flex.end();

    let mut user_name = Frame::new(59, 207, 269, 37, "尚未找到更新图片的用户");
    user_name.set_id("watch_path_user_name");
    user_name.set_label_size(12);

    let mut text_fall_pictures = Frame::new(364, 218, 85, 15, "通过拽入获取");
    text_fall_pictures.set_label_size(12);

    let mut text_recent_pictures = Frame::new(495, 218, 30, 15, "扫描");
    text_recent_pictures.set_label_size(12);
    text_recent_pictures.set_id("gui::btn_text_recent_pictures");

    let mut text_title01 = Frame::new(30, 96, 190, 21, "请选择 WX文件的保存位置 *");
    text_title01.set_label_size(12);
    text_title01.set_label_color(Color::from_rgb(105, 105, 105));

    let mut text_title02 = Frame::new(
        6,
        96 + 70,
        490,
        21,
        "选择用户*    [ 涉及用户数据安全   仅限已登录中的用户 并且能够收发的用户 ]",
    );
    text_title02.set_label_size(12);
    text_title02.set_label_color(Color::from_rgb(105, 105, 105));

    let mut text_title03 = Frame::new(
        6 + 50,
        96 + 70 + 90,
        490,
        21,
        "选择最近对象*  （ 如果不存在请随意发送一张的图片给对方 [不能是表情]  更新后约5秒显示 ） ",
    );
    text_title03.set_label_size(12);
    text_title03.set_label_color(Color::from_rgb(105, 105, 105));
    text_title03.set_id("gui::gui_select_user_base::text_title03");

    thread::spawn(move || {
        let mut title = text_title03.clone();
        while global_var::get_bool_default("gui::open::handle_dat") {
            // if let Some (mut title) = app::widget_from_id("gui::gui_select_user_base::text_title03") as  Option<Frame> {
            let data = global_var::get_string_default("user::config::user_select_path");
            let id = global_var::get_i32_default("user::config::select_user_thumbnail_obj");

            if (data.is_empty()) {
                title.set_label("选择最近对象*  （ 如果不存在请随意发送一张的图片给对方 [不能是表情]  更新后约5秒显示 ） ");
            } else {
                title.set_label(
                    format!(
                        "已选定 ：[ {} ] {}  [再次点击取消]",
                        (if id == -2 {
                            "拖拽".to_string()
                        } else {
                            id.to_string()
                        }),
                        data
                    )
                    .as_str(),
                );
            }

            title.resize(6 + 50, 96 + 70 + 90, 490, 21);

            // }
            Sleep(500);
        }
        println!("gui_select_user_base 线程退出");
    });
    // 刷新最近图片
    let mut btn_recent_pictures = gui_hotspot::create_hotspot(480, 204, 62, 39);
    // 通过拖拽确认用户
    let mut btn_fall_pictures_path = gui_hotspot::create_hotspot(334, 203, 125, 42);
    // 打开微信文件所在位置
    let mut btn_open_select_dir = gui_hotspot::create_hotspot(515, 123, 39, 35);
    // 输入文件夹路径
    let mut input_select_dir = input::Input::new(50, 127, 450 - 5, 27, "");
    input_select_dir.set_id("gui::input_select_dir");

    if let Ok(history) = get_wx_user_history_path() {
        let paths = history.path;
        input_select_dir.set_value(paths.as_str());
        global_var::set_string("user::config::input_select_dir", paths);

        // initialize_watch_path_puppet(paths);
    }
    if (input_select_dir.value().is_empty()) {
        if let Some(paths) = wh_mod::convert::get_user_data_path() {
            input_select_dir.set_value(paths.as_str());
            global_var::set_string("user::config::input_select_dir", paths);
        }
    }

    // 输入文件夹路径 的热区
    let mut input_select_dir_hotspot = gui_hotspot::create_hotspot(50, 127, 450 - 5, 27);

    // initialize_preview_main
    let mut preview_main_point_list = [
        [71, 296, 75, 75],
        [167, 296, 75, 75],
        [263, 296, 75, 75],
        [359, 296, 75, 75],
        [455, 296, 75, 75],
    ];
    let mut preview_main_hotspot_01 = gui_hotspot::create_hotspot(76, 303, 73, 73);
    let mut preview_main_hotspot_02 = gui_hotspot::create_hotspot(169, 303, 73, 73);
    let mut preview_main_hotspot_03 = gui_hotspot::create_hotspot(264, 303, 73, 73);
    let mut preview_main_hotspot_04 = gui_hotspot::create_hotspot(355, 303, 73, 73);
    let mut preview_main_hotspot_05 = gui_hotspot::create_hotspot(447, 303, 73, 73);

    let mut preview_main = Vec::new();
    for index in 0..preview_main_point_list.len() {
        let point = preview_main_point_list[index];
        let [x, y, width, height] = point;
        // let id = format!("gui::preview_main::index::{}",index);
        // let idc = id.as_str();
        let mut preview = ImgPreview::new(x, y, width, height, "gui::preview_main::index::");
        // -1,-1,width-2,height-2 每个边 向内缩进1像素 使其有1px描边的效果
        preview.from_data(
            include_bytes!("./assets/select_user_base/not.png").to_vec(),
            -1,
            -1,
            width - 2,
            height - 2,
        );
        preview_main.push(preview);
    }
    let mut preview = ImgPreview::new(455, 296, 75, 75, "gui::preview_main::index::user_select");
    preview.preview.hide();
    preview_main.push(preview);

    println!("preview_main size-> {}", preview_main.len());

    // // 写入到全局变量中
    // write_rw_lock!(IMG_PREVIEW_LIST_ARC, preview_main.to_vec());
    // // write_rw_lock!(IMG_PREVIEW_LIST_ARCLAZY,preview_main.to_vec().into());

    // // *IMG_PREVIEW_LIST_ARCLAZY.write().unwrap() = preview_main.to_vec().into();
    // // IMG_PREVIEW_LIST_ARCLAZY.store(preview_main.to_vec().into());

    set_arc_bind_variable!(
        IMG_PREVIEW_LIST,
        IMG_PREVIEW_LIST_BIND,
        preview_main.to_vec()
    );

    // // get_arc_bind_variable!(IMG_PREVIEW_LIST,IMG_PREVIEW_LIST_BIND);
    // println!(
    //     "[win-main]IMG_PREVIEW_LIST_ARC size-> {}",
    //     read_rw_lock!(IMG_PREVIEW_LIST_ARC, Vec::new()).len()
    // );
    // println!(
    //     "[win-main]IMG_PREVIEW_LIST_ARCSWAP-> {}",
    //     read_rw_lazy_lock!(IMG_PREVIEW_LIST_ARCLAZY).len()
    // );
    // println!(
    //     "[win-main]THUMBNAIL_LIST-> {}",
    //     get_arc_bind_variable!(IMG_PREVIEW_LIST,IMG_PREVIEW_LIST_BIND).len()
    // );

    println!(
        "[win-main]IMG_PREVIEW_LIST-> {}",
        get_arc_bind_variable!(IMG_PREVIEW_LIST, IMG_PREVIEW_LIST_BIND).len()
    );
    println!(
        "[win-main]THUMBNAIL_LIST-> {}",
        get_arc_bind_variable!(THUMBNAIL_LIST, THUMBNAIL_LIST_BIND).len()
    );

    let mut preview_tips = ImgPreview::new(165, 18, 27, 27, "gui::preview_main::next::tips");
    preview_tips.from_data(
        include_bytes!("./assets/select_user_base/tips.png").to_vec(),
        0,
        0,
        27,
        27,
    );

    let mut title_tips =
        gui_text_control::TextControl::new(46, 18, 330, 27, 12, "帮助".to_string(), [84, 84, 84]);
    // end

    win.end();
    win.show();

    win.handle({
        let mut x = 0;
        let mut y = 0;
        let mut point_exist_hasmap = getFormPointSpace(x, y);
        let mut has_show = false;
        // let mut move_
        move |win, ev| match ev {
            enums::Event::Show => {
                win.set_visible_focus();
                libWxIkunPlus::setwinVisible(win.raw_handle() as i128, true);
                println!("hwnd -> :  {}", win.raw_handle() as i128);
                true
            }

            enums::Event::Push => {
                // 处理 check 组件
                macro_rules! check_select_click {
                    ($check_select:expr) => {
                        $check_select.existPoint(x, y)
                        // check_select_source.set_value(global_var);
                    };
                }

                //  打开文件夹选择器
                if check_select_click!(btn_open_select_dir) {
                    let user_select_path = libWxIkunPlus::openSelectFolder2();

                    if user_select_path.len() > 1 {
                        initialize_watch_path_puppet(user_select_path.clone());
                        user_name.set_label("开始扫描...");
                        input_select_dir
                            .clone()
                            .set_value(user_select_path.clone().as_str());
                        global_var::set_string(
                            "user::config::input_select_dir",
                            user_select_path.clone(),
                        );
                    }
                }

                if btn_recent_pictures.existPoint(x, y) {}

                // 刷新按钮
                if btn_recent_pictures.existPoint(x, y) {
                    if text_recent_pictures.label().contains("检测") {
                        if (check_button_the_month.is_checked()
                            || check_button_source.is_checked()
                            || check_button_thumbnail.is_checked())
                            && global_var::get_i32_or("user::config::select_user_thumbnail_obj",-1) != -1
                        {
                        } else {
                            gui_detect_config::main_window();
                            // title_tips.set_label("帮助:[未选存储内容]".to_string());
                        }
                    }
                }

                // 通过拖拽获取
                if (btn_fall_pictures_path.existPoint(x, y)) {
                    let value = input_select_dir.value();

                    if (value.len() > 3) {
                        match fs::metadata(Path::new(value.as_str())) {
                            Ok(meta) => {
                                user_name.set_label("开始扫描...");
                                initialize_watch_path_puppet(value.clone());
                            }
                            Err(err) => {
                                let choice = dialog::alert_default(&*format!(
                                    "输入的路径存在错误 错误-> {:?}",
                                    err
                                ));
                            }
                        }
                        gui_drag_scan::main_window();
                    } else {
                        gui_detect_config::main_window();
                        let choice = dialog::alert_default(&*format!(
                            "输入的路径存在错误 错误-> 没有选择WX路径"
                        ));
                    }
                }

                // 选择 或者刷新
                if (btn_recent_pictures.existPoint(x, y)
                    && !text_recent_pictures.label().contains("检测"))
                {
                    let value = input_select_dir.value();
                    global_var::set_string("user::config::input_select_dir", value.clone());
                    if (value.len() > 3) {
                        match fs::metadata(Path::new(value.as_str())) {
                            Ok(meta) => {
                                user_name.set_label("开始扫描...");
                                initialize_watch_path_puppet(value.clone());
                            }
                            Err(err) => {
                                let choice = dialog::alert_default(&*format!(
                                    "输入的路径存在错误 错误-> {:?}",
                                    err
                                ));
                            }
                        }
                    } else {
                        let choice = dialog::alert_default(&*format!("输入的路径过短或者不存在"));
                    }
                }

                // 关闭
                if (btn_exit_imag.existPoint(x, y)) {
                    libWxIkunPlus::closeWindow(win.raw_handle() as i128, true);

                    // write_rw_lock!(THUMBNAIL_LIST_ARC, Vec::new());
                    // write_rw_lock!(IMG_PREVIEW_LIST_ARC,Vec::new());
                    set_arc_bind_variable!(THUMBNAIL_LIST, THUMBNAIL_LIST_BIND, Vec::new());
                    set_arc_bind_variable!(IMG_PREVIEW_LIST, IMG_PREVIEW_LIST_BIND, Vec::new());

                    // 终止更新检测
                    wh_mod::watch_path::un_next_exits();
                    global_var::set_i32("user::config::select_user_thumbnail_obj", -1);
                    global_var::set_string("user::config::user_select_path", String::new());
                    global_var::set_string("user::config::user_select_wxid", String::new());
                    global_var::set_bool("gui::open::handle_dat", false);
                }

                if preview_tips.existPoint(x, y) {
                    gui_detect_config::main_window();
                }

                macro_rules! select_user_preview {
                    ($select_user_preview:expr,$id:expr) => {
                        if ($select_user_preview.existPoint(x, y)) {
                            let select_id =
                                global_var::get_i32_or("user::config::select_user_thumbnail_obj",-1);

                            if (select_id == $id) {
                                global_var::set_string(
                                    "user::config::user_select_path",
                                    "".to_string(),
                                );
                                global_var::set_i32("user::config::select_user_thumbnail_obj", -1);
                            }

                            if (select_id != $id) {
                                let mut str_path = String::new();
                                let thumbnail_list = get_arc_bind_variable!(THUMBNAIL_LIST,THUMBNAIL_LIST_BIND);
                                
                                if let Some(item) = thumbnail_list.get($id - 1) {
                                    str_path = item.attach_id.clone();

                                    println!("[select_user_preview] -> {} [{}] ",&str_path,&$id);

                                    global_var::set_string("user::config::user_select_path", str_path);
                                    global_var::set_i32("user::config::select_user_thumbnail_obj", $id);
                                }
                            }
                        }
                    };
                }
                // app::grab().expect("msg")
                select_user_preview!(preview_main_hotspot_01, 1);
                select_user_preview!(preview_main_hotspot_02, 2);
                select_user_preview!(preview_main_hotspot_03, 3);
                select_user_preview!(preview_main_hotspot_04, 4);
                select_user_preview!(preview_main_hotspot_05, 5);

                true
            }

            enums::Event::Move => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;

                macro_rules! move_show_cursor {
                    ($check_select:expr) => {
                        $check_select.existPoint(x, y)
                    };
                }

                if move_show_cursor!(btn_exit_imag)
                    || move_show_cursor!(preview_main_hotspot_01)
                    || move_show_cursor!(preview_main_hotspot_02)
                    || move_show_cursor!(preview_main_hotspot_03)
                    || move_show_cursor!(preview_main_hotspot_04)
                    || move_show_cursor!(preview_main_hotspot_05)
                    || move_show_cursor!(btn_recent_pictures)
                    || move_show_cursor!(btn_fall_pictures_path)
                    || move_show_cursor!(btn_open_select_dir)
                    || preview_tips.existPoint(x, y)
                {
                    win.clone().set_cursor(Cursor::Hand);
                } else {
                    win.clone().set_cursor(Cursor::Default);
                }

                true
            }

            // enums::Event:
            enums::Event::Drag => {
                if y < 69 {
                    win.clone()
                        .set_pos(app::event_x_root() - x, app::event_y_root() - y);
                }

                true
            }
            _ => false,
        }
    });

    win.set_visible_focus();

    select_user_base
}
