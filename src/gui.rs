// #![allow(
//     dead_code,
//     unused_imports,
//     unused_parens,
//     unused_variables,
//     unused_mut,
//     unused_must_use,
//     unused_assignments,
//     non_snake_case,
//     unreachable_code,
//     unused_macros,
//     unused_unsafe
// )]
#![allow(warnings, unused)]

// #![windows_subsystem = "windows"]

use chrono::Local;
use glob::glob;
use hotwatch::{
    blocking::{Flow, Hotwatch},
    EventKind,
};
// GleamThinDownBox -> 53
// GtkRoundUpFrame -> 46
// RoundDownBox -> 23
// RoundUpBox -> 22
// RShadowBox -> 19
// EngravedFrame -> 12
// 12 19 22 23 46 53(buj)

use libc::c_void;
use rusqlite::{params, Connection, Result};

use fltk::draw::font;
use fltk::enums::{Cursor, Event, Font, LabelType};
use fltk::frame::Frame;
use fltk::group::Group;
use fltk::input::{Input, InputType, IntInput};
use fltk::text::TextDisplay;
use fltk::{
    app::handle,
    text::{TextBuffer, TextEditor},
};
use fltk::{button::Button, enums::Align, window::DoubleWindow};
use fltk::{enums::Color, enums::FrameType};
use fltk::{prelude::*, window::Window, *};
use magic_crypt::MagicCryptTrait;
use msgbox::IconType;
use serde_json::json;
use std::collections::{hash_map::DefaultHasher, HashMap};
use std::mem::transmute;
use fltk_theme::ColorTheme;
use fltk_theme::color_themes;
use fltk_theme::WidgetTheme;
use fltk_theme::ThemeType;
use fltk_theme::WidgetScheme;
use fltk_theme::SchemeType;

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

use crate::{atomic_util, global_var, handle_dat, libWxIkunPlus::{self, setTaskbarWin}, gui_manage_item, gui_select_user_base, util::{self, str_eq_ostr, str_eq_str, Sleep}, wh_mod::convert::{convert_bat_images}, gui_drag_scan, wh_mod, console_log, gui_imge, global_var_util, get_bool, APP_STARTUP};
use crate::wh_mod::parse_dat_path;

use std::sync::atomic::{AtomicBool, AtomicI32, AtomicUsize, Ordering,AtomicI64};
use std::sync::{Arc, Condvar, Mutex,RwLock};

struct MainTheme {
    /**主背景颜色 */
    background: Color,
    /**次背景*/
    backgroundMain: Color,
    /**顶部文字和logo */
    logo: Color,
    /**卡片文本成功 */
    cardSuccessText: Color,
    /**卡片文本失败 */
    cardFailureText: Color,
    /**卡片文本 */
    cardText: Color,
    /**卡片描边 */
    cardStroke: Color,
    /**分割线 */
    cuttingLine: Color,
    /** 底部三个按钮的颜色*/
    botBtnColor: Color,
    /** 底部三个按钮的图标颜色*/
    botBtnIconColor: Color,
    // null
    not: Color,
}

// 统一在这里定义主题颜色
fn getMainTheme() -> MainTheme {
    let mut mainTheme: MainTheme = MainTheme {
        /**主背景颜色 */
        background: Color::rgb_color(24, 24, 24),
        /**次背景*/
        backgroundMain: Color::rgb_color(17, 17, 17),
        /**顶部文字和logo */
        logo: Color::rgb_color(122, 120, 120),
        /**卡片文本成功 */
        cardSuccessText: Color::rgb_color(99, 138, 99),
        /**卡片文本失败 */
        cardFailureText: Color::rgb_color(189, 79, 79),
        /**卡片文本 */
        cardText: Color::rgb_color(255, 255, 255),
        /**卡片描边 */
        cardStroke: Color::rgb_color(46, 46, 46),
        /**分割线 */
        cuttingLine: Color::rgb_color(38, 38, 38),
        /** 底部三个按钮的颜色*/
        botBtnColor: Color::rgb_color(0, 0, 0),
        /** 底部三个按钮的图标颜色*/
        botBtnIconColor: Color::rgb_color(125, 125, 125),
        not: Color::from_u32(0),
    };
    return mainTheme;
}

// 设置界面主题
pub fn setMainTheme() {
    let mut mainTheme: MainTheme = getMainTheme();
    app::set_background_color(24, 24, 24);
    // app::set_fonts("name");
    app::set_frame_shadow_width(0);
    app::set_frame_color(mainTheme.not);
    app::set_background2_color(17, 17, 17);
    app::set_foreground_color(17, 17, 17);
    app::set_selection_color(17, 17, 17);
    // app::set_frame_type2(old_frame, new_frame);

    app::set_frame_type(FrameType::NoBox);
    app::set_inactive_color(24, 24, 24);
    app::set_frame_border_radius_max(0);
    app::set_frame_type2(FrameType::BorderBox, FrameType::NoBox);
    app::set_visible_focus(false);
    app::set_frame_shadow_width(0);
    app::swap_frame_type(FrameType::NoBox);
    app::set_menu_linespacing(0);
    app::set_scrollbar_size(0);
}

// 设置背景为图片（主视图）
fn setWinBackground_forRoot_image(appMainWin: &mut window::DoubleWindow) -> Frame {
    let background_image = image::PngImage::from_data(include_bytes!("./assets/main_back.png"))
        .expect("set main icon error");
    // image::SvgImage::from_data(include_str!("./assets/main_back.svg"))
    // .expect("set main icon error");
    let mut frame = Frame::default().with_size(600, 0).center_of(appMainWin);
    frame.set_frame(FrameType::EngravedBox);
    frame.set_image(Some(background_image));
    return frame;
}

// 界面会回传为这个参数 用来控制鼠标手型
struct PointExistHasmap {
    // 关闭按钮
    quit: bool,
    // 按钮:: 打开dat所在路径
    shellOpenDatDir: bool,
    // 按钮:: 导出此文件夹
    shellOpenExportDir: bool,
    // 按钮:: 管理
    manageItme: bool,
    // 按钮:: 测试
    test: bool,
    // 按钮:: 创建
    create: bool,
    // 选项::自启动
    starting: bool,

    // 鼠标在按钮原件中
    existAllBtn: bool,
}

// 判断鼠标坐标是否在此元素内
fn getFormPointSpace(x: i32, y: i32) -> PointExistHasmap {
    let mut point_exist_hasmap = PointExistHasmap {
        quit: false,
        shellOpenDatDir: false,
        shellOpenExportDir: false,
        manageItme: false,
        test: false,
        starting: false,
        create: false,
        existAllBtn: false,
    };

    point_exist_hasmap.quit = x > 545 && x < 575 && y > 13 && y < 51;
    point_exist_hasmap.manageItme = x > 342 && x < 342 + 60 && y > 273 && y < 273 + 36;
    point_exist_hasmap.shellOpenDatDir = x > 511 && x < 511 + 36 && y > 147 && y < 147 + 39;
    point_exist_hasmap.shellOpenExportDir = x > 511 && x < 511 + 36 && y > 219 && y < 219 + 39;
    point_exist_hasmap.starting = x > 85 && x < 85 + 25 && y > 490 && y < 490 + 25;
    point_exist_hasmap.test = x > 413 && x < 413 + 60 && y > 273 && y < 273 + 36;
    point_exist_hasmap.create = x > 486 && x < 486 + 60 && y > 273 && y < 273 + 36;

    let mut win_coords_cursor_list = vec![
        point_exist_hasmap.quit,
        point_exist_hasmap.manageItme,
        point_exist_hasmap.shellOpenDatDir,
        point_exist_hasmap.shellOpenExportDir,
        point_exist_hasmap.starting,
        point_exist_hasmap.test,
        point_exist_hasmap.create,
    ];

    let mut has_cursor = false;

    for value in win_coords_cursor_list.iter() {
        // 关闭按钮
        if *(value) {
            has_cursor = true;
        }
    }

    point_exist_hasmap.existAllBtn = has_cursor;

    return point_exist_hasmap;
}


// 设置自启动按钮的状态
fn addBtnEnableStarting(appMainWin: &mut window::DoubleWindow) -> gui_imge::ImgPreview  {
    let w_h = 20;
    let mut preview = gui_imge::ImgPreview::new(90-3, 493, w_h, w_h, "gui::preview_main::index::user_select");

    if libWxIkunPlus::hasStartup() {
        preview.from_data(include_bytes!("./assets/enable.png").to_vec(), 0, 0,w_h, w_h);
    }else{
        preview.from_data(include_bytes!("./assets/un_enable.png").to_vec(), 0, 0,w_h, w_h);
    }
    
   

    return preview;
}

// dat的路径的输入框
fn addInput_shellOpenDatDir(appMainWin: &mut window::DoubleWindow) -> ConsoleItme {
    let mut mainTheme: MainTheme = getMainTheme();

    let mut buf = fltk::text::TextBuffer::default();
    let mut txt = fltk::text::TextEditor::default()
        .with_size(447, 25)
        .center_of_parent();
    // txt.set
    txt.set_buffer(buf.clone());
    txt.set_frame(FrameType::NoBox);
    txt.set_text_color(Color::from_rgb(120, 120, 120));
    txt.set_color(mainTheme.backgroundMain);
    txt.set_label_type(fltk::enums::LabelType::None);
    // txt.set_text_size(15);
    txt.set_pos(46, 153 + 3);
    txt.set_text_size(11);
    // txt.set_scrollbar_size(0);
    txt.set_scrollbar_size(3);
    txt.set_callback(move |usetup| {
        println!(
            "addInput_shellOpenExportDir => {} {}",
            usetup.buffer().unwrap().text(),
            usetup.buffer().unwrap().length()
        );
        if !wh_mod::convert::is_developer(){
        let mut buff = usetup.buffer().unwrap();
        buff.remove(0, buff.length());
        console_log!("[错误] 编辑被禁止".to_string());
        }
    });
  
    // buf.set(true);

    txt.show();

    return ConsoleItme {
        edit: txt,
        buff: buf,
    };
}

// 保存到的输入框
fn addInput_shellOpenExportDir(appMainWin: &mut window::DoubleWindow) -> ConsoleItme {
    let mut mainTheme: MainTheme = getMainTheme();

    let mut buf = fltk::text::TextBuffer::default();
    let mut txt = fltk::text::TextEditor::default()
        .with_size(447, 27)
        .center_of_parent();
    txt.set_buffer(buf.clone());
    txt.set_frame(FrameType::NoBox);
    txt.set_text_color(Color::from_rgb(120, 120, 120));
    txt.set_color(mainTheme.backgroundMain);
    txt.set_label_type(fltk::enums::LabelType::None);
    txt.set_text_size(13);
    txt.set_pos(46, 223 + 5);
    txt.set_scrollbar_size(2);
    // txt.set_scrollbar_align(Align:);
    txt.set_callback(move |usetup| {
        println!(
            "addInput_shellOpenDatDir => {} {}",
            usetup.buffer().unwrap().text(),
            usetup.buffer().unwrap().length()
        );
    });
    txt.show();

    return ConsoleItme {
        edit: txt,
        buff: buf,
    };
}

// 名称:
fn addInput_shellName(appMainWin: &mut window::DoubleWindow) -> ConsoleItme {
    let mut mainTheme: MainTheme = getMainTheme();

    let mut buf = fltk::text::TextBuffer::default();
    let mut txt = fltk::text::TextEditor::default()
        .with_size(180+8, 27)
        .center_of_parent();
    txt.set_buffer(buf.clone());
    txt.set_frame(FrameType::NoBox);
    txt.set_text_color(Color::from_rgb(120, 120, 120));
    txt.set_color(mainTheme.backgroundMain);
    txt.set_label_type(fltk::enums::LabelType::None);
    txt.set_text_size(15);
    txt.set_pos(98, 279);

    // txt.set_changed();
    txt.set_callback(move |usetup| {
        let mut stext = usetup.buffer().unwrap();
        if (stext.length() > 30) {
            stext.remove(30, stext.length());
        }
        println!("addInput_shellName => {} {}", stext.text(), stext.length());
    });
    txt.show();

    return ConsoleItme {
        edit: txt,
        buff: buf,
    };
}

// 打印台的控制
struct ConsoleItme {
    edit: TextEditor,
    buff: TextBuffer,
}

// 初始化打印台元素
fn addConsole(appMainWin: &mut window::DoubleWindow) -> ConsoleItme {
    let mut mainTheme: MainTheme = getMainTheme();

    let mut buf = fltk::text::TextBuffer::default();
    let mut txt = fltk::text::TextEditor::default()
        .with_size(528, 98)
        .center_of_parent();
    txt.set_buffer(buf.clone());
    txt.set_frame(FrameType::NoBox);
    txt.set_text_color(Color::from_rgb(120, 120, 120));
    txt.set_color(mainTheme.backgroundMain);
    txt.set_label_type(fltk::enums::LabelType::None);
    txt.set_text_size(12);
    txt.set_pos(34, 365);
    txt.set_scrollbar_size(6);
    txt.show();

    return ConsoleItme {
        edit: txt,
        buff: buf,
    };
}

// 处理文本添加时候风格的宏
macro_rules! setTheStyleToInterface {
    ($b:expr) => {{
        let MainTheme: MainTheme = getMainTheme();
        $b.show_cursor(false);
        $b.set_text_color(MainTheme.botBtnIconColor);
        $b.set_text_size(11);
        $b.set_label_type(LabelType::None);
        $b.set_color(MainTheme.backgroundMain);
        $b.clear_visible_focus();
        $b.set_frame(FrameType::FlatBox);
        $b.show_cursor(false);
        $b.deactivate();
        $b.set_text_color(MainTheme.cardText);
    }};
    ($b:expr,$x:expr,$y:expr,$w:expr,$h:expr) => {{
        let MainTheme: MainTheme = getMainTheme();
        $b.show_cursor(false);
        $b.set_text_color(MainTheme.botBtnIconColor);
        $b.set_text_size(11);
        $b.resize($x, $y, $w, $h);
        $b.set_label_type(LabelType::None);
        $b.set_color(MainTheme.backgroundMain);
        $b.clear_visible_focus();
        $b.set_frame(FrameType::FlatBox);
        $b.show_cursor(false);
        $b.deactivate();
        $b.set_text_color(MainTheme.cardText);
    }};

    ($b:expr,$x:expr,$y:expr,$w:expr,$h:expr,$fsize:expr) => {{
        let MainTheme: MainTheme = getMainTheme();
        $b.show_cursor(false);
        $b.set_text_color(MainTheme.botBtnIconColor);
        $b.set_text_size($fsize);
        $b.resize($x, $y, $w, $h);
        $b.set_label_type(LabelType::None);
        $b.set_color(MainTheme.backgroundMain);
        $b.clear_visible_focus();
        $b.set_frame(FrameType::NoBox);
        $b.show_cursor(false);
        $b.deactivate();
        $b.set_text_color(MainTheme.cardText);
    }};
}

struct MsgAttachExport {
    id: i32,
    time: String,
    name: String,
    ext: String,
    input: String,
    ouput: String,
    message: String,
    user_name: String,
}

pub struct MianWindowItme {
    appMainWin: DoubleWindow,
    appRootView: DoubleWindow,
}

macro_rules! set_theme{
    () => {
            // 设置主题
    let theme = ColorTheme::new(color_themes::BLACK_THEME);
    let widget_theme = WidgetTheme::new(ThemeType::HighContrast);
    widget_theme.apply();
    let widget_scheme = WidgetScheme::new(SchemeType::Aqua);
    widget_scheme.apply();
    theme.apply();
    }
}


fn get_window_hwnd(win:&window::Window) -> i128 {
    let mut xclass = win.xclass().unwrap_or_else(||String::new());
    let mut xtitle =String::new();// win.label();

    let hwnd = libWxIkunPlus::findWindow(xclass.as_str(), xtitle.as_str()) ;
    println!("xclass<{}> xtitle<{}> hwnd<{}>", xclass,xtitle, hwnd);
    hwnd
}

// 主界面
pub fn mianWindow(show: bool) -> Result<MianWindowItme> {

    set_theme!();


    // 状态栏用于显示任务图标的傀儡窗口
    // let mut dock_win = window::Window::default()
    //     .with_size(1, 1)
    //     .with_label("微信图片自动备份")
    //     .center_screen();
    // dock_win.size_range(0, 0, 0, 0);
    // dock_win.make_resizable(false);
    // dock_win.set_xclass("app_main_win_dock_win_wx_dat_viewer_auto_export_rust");
    // dock_win.show();
    // dock_win.end();
    
    let mut mainTheme: MainTheme = getMainTheme();

    let mut appMainWin = Window::new(0, 0, 600, 531, "Ikun导出");
    appMainWin.set_xclass("app_main_win_wx_dat_viewer_auto_export_rust");

    app::set_scrollbar_size(3);

    app::set_selection_color(24, 24, 24);
    let mut cwd = env::current_dir().expect("get current_dir error ");
    //  设置窗口图标
    //  let ICON1 = image::IcoImage::load(format!("{}/{}",cwd.display().to_string(),"app.ico").to_string().to_owned())
    //  .expect("set main icon error");
    // appMainWin.set_icon(Some(ICON1.clone()));

    // appMainWin.set_icon();
    appMainWin.set_border(false);
    // 主界面的窗口 2  悬浮在主窗口1上面
    let mut appRootView = window::Window::new(0, 0, 600, 531, "mian");
    setWinBackground_forRoot_image(&mut appRootView);
    // 界面
    let mut btnEnableStarting = addBtnEnableStarting(&mut appRootView);
    let mut input_shellOpenExportDir = addInput_shellOpenExportDir(&mut appRootView);
    let mut input_shellOpenDatDir = addInput_shellOpenDatDir(&mut appRootView);
    let mut input_Console = addConsole(&mut appRootView);
    let mut input_shellName = addInput_shellName(&mut appRootView);

    if !wh_mod::convert::is_developer(){
    input_Console.buff.set_text(("初始化成功！"));
    input_Console.buff.set_text("作者 @Ikun  ");
    input_Console.buff.append("\n");
    input_Console
        .buff
        .append("软件开源协议 GPL 3.0  (但是并不包含解码算法)  版本：1.0.1 ");
    input_Console.buff.append("\n\n");
    input_Console
        .buff
        .append("本软件 是免费软件 如果付费请维权退款\n");
    
    input_Console
    .buff
    .append("本软件只供备份自己的图片禁止用于其他用途\n");

        input_Console
        .buff
        .append("在此 @Ikun 向所有引用的开源项目表示感谢");

    }else {
        input_Console.buff.set_text(("初始化成功 [开发者模式]"));
    }

    // 界面
    appRootView.end();
    appMainWin.clone().center_screen(); // 将窗口居中

    appMainWin.hide();
    appRootView.hide();
    appMainWin.end();

    let mut input_buff_Console_move = input_Console.buff.clone();

    thread::spawn(move || loop {
        Sleep(150);
        let mut console_message = handle_dat::get_console_message().replace("\n\n", "\n");

        if console_message.starts_with('\n') {
            console_message = console_message.trim_start_matches('\n').to_string();
        }

        if (console_message.len() < 5) {
            continue;
        };

        let mut newline_count = 0;

        for line in input_buff_Console_move.text().lines() {
            newline_count += 1
        }

        if (newline_count > 5) {
            input_buff_Console_move.remove(0, input_buff_Console_move.length());
            input_buff_Console_move.set_text(&console_message);
        } else {
            input_buff_Console_move.append(&format!("\n{}", &console_message));
        }
    });

    let mut dat_buff_move= input_shellOpenDatDir.buff.clone();
    let mut copy_btnEnableStarting = btnEnableStarting.clone();
    thread::spawn(move || loop {
        let mut oid_app_start = false;

        if get_bool!(APP_STARTUP)!=oid_app_start{
            oid_app_start = true;
            copy_btnEnableStarting.from_data(include_bytes!("./assets/enable.png").to_vec(), 0, 0,20, 20);
        }else{
            oid_app_start = false;
            copy_btnEnableStarting.from_data(include_bytes!("./assets/un_enable.png").to_vec(), 0, 0,20, 20);
        }

        Sleep(550);

        let input_select_dir = global_var::get_string_default("user::config::input_select_dir");
        let user_select_path = global_var::get_string_default("user::config::user_select_path");
        let user_select_wxid = global_var::get_string_default("user::config::user_select_wxid");

        if !user_select_path.is_empty()&&!input_select_dir.is_empty()&&global_var::get_bool_default("gui::open::handle_dat") {


            let mut new_buff = format!("{}/{}/FileStorage/MsgAttach/{}",input_select_dir,user_select_wxid,user_select_path);

            // 判断路径有效性 无效则换文件夹  因为有些用户是可以多账户登录的
            if(!Path::new(new_buff.as_str()).exists()){
                let read_root_wxid_list  = wh_mod::wx_read_root_wxid(&Path::new(input_select_dir.as_str()).to_path_buf());
                for read_root_wxid in read_root_wxid_list {
                    if Path::new(read_root_wxid.attach.join(user_select_path.as_str() ) .as_os_str() ).exists(){
                        new_buff = format!("{}/{}",read_root_wxid.attach.to_str().unwrap(),user_select_path);
                        break;
                    }
                }
            }

            if(global_var::get_bool_default("user::config::check_button_the_month")){
                new_buff = new_buff+"*the_month";
            }
            if(global_var::get_bool_default("user::config::check_button_source")){
                new_buff = new_buff+"*source";
            }
            if(global_var::get_bool_default("user::config::check_button_thumbnail")){
                new_buff = new_buff+"*thumbnail";
            }

            if(!new_buff.as_bytes().eq(dat_buff_move.text().as_bytes() )){
                dat_buff_move.remove(0,dat_buff_move.length());
                dat_buff_move.append(new_buff.as_str());
            }

        }

    });

    let mut copy_AppRootView = appRootView.clone();
    let mut copy_appMainWin = appMainWin.clone();
    // let mut copy_dock_win = dock_win.clone();

    let mut g_appMainWinHwnd = 0;
    // let mut g_copy_dock_win_hwnd = 0;

    
 

    appMainWin.handle({
        let mut x = 0;
        let mut y = 0;
        let mut point_exist_hasmap = getFormPointSpace(x, y);
        let mut has_show = false;
        let mut copy_appMainWin = copy_appMainWin.clone();
        if(g_appMainWinHwnd.eq(&0)){
            g_appMainWinHwnd = get_window_hwnd(&copy_appMainWin);
            }
        move |win, ev| match ev {
            enums::Event::Focus=>{
                if(g_appMainWinHwnd.eq(&0)){
                    g_appMainWinHwnd = get_window_hwnd(&copy_appMainWin);
                    }
                true
            }
            enums::Event::Show => {
                copy_AppRootView.set_visible_focus();

                if(g_appMainWinHwnd.eq(&0)){
                    g_appMainWinHwnd = get_window_hwnd(&copy_appMainWin);
                    }

                env::set_var("ikunWinHwnd", format!("{}",g_appMainWinHwnd).to_string());
                // unsafe { setWinIcon(appMainWinHwnd.try_into().unwrap()) };
                libWxIkunPlus::setWinIcon(g_appMainWinHwnd);

                
                // libWxIkunPlus::setwinVisible(g_copy_dock_win_hwnd , true);

                println!("Show => window hwnd:{}",g_appMainWinHwnd);
                true
            }
            enums::Event::Close=>{

                println!("Close => window as {}",0);
                true
            }
            enums::Event::Focus=>{
            
            
                true
            }
            enums::Event::Push => {
                // 关闭按钮
                if (point_exist_hasmap.quit) {
                    libWxIkunPlus::setwinVisible(g_appMainWinHwnd , false);
                    // libWxIkunPlus::setwinVisible(g_copy_dock_win_hwnd , false);
                    // unsafe { setShowWindows((copyappMainWin.raw_handle() as i128).try_into().unwrap(), false) };
                }
                let mut has_inputPath = false;
                let mut has_ouputPath = false;
                let mut has_name = false;

                if (point_exist_hasmap.create) {
                    input_Console.buff.set_text("[用户] 创建新的配置文件");
                    println!("click => create");
                } else if (point_exist_hasmap.manageItme) {
                    // input_Console
                    //     .buff
                    //     .set_text("[用户] 很抱歉 当前还不支持配置管理");
                    println!("click => manageItme");
                    gui_manage_item::ManageItmeMain();
                } else if (point_exist_hasmap.shellOpenDatDir) {
                    input_Console
                        .buff
                        .set_text("[用户] 打开选取原始文件夹(dat 原目录)");
                  
                    // 有wx进程 而且有窗口
                    if(wh_mod::convert::is_developer()||(libWxIkunPlus::hasWeChat()&&libWxIkunPlus::hasWeChatWin())){
                       gui_select_user_base::mian_window();
                    }else{
                        // thread::spawn(||{
                        //     // libWxIkunPlus::stop("错误".to_owned(),"当前未发现wx进程或者未登录 拒绝提供选取方案".to_owned());
                        //     dialog::alert_default(format!("\n[错误] {}","当前未发现wx进程或者未登录 拒绝提供选取方案").as_str());

                        // });
                        libWxIkunPlus::stop("错误".to_owned(),"当前未发现wx进程或者未登录 拒绝提供选取方案".to_owned());
                        // dialog::alert_default(format!("\n[错误] {}","当前未发现wx进程或者未登录 拒绝提供选取方案").as_str());
                    }

                    println!("click => shellOpenDatDir");
                } else if (point_exist_hasmap.shellOpenExportDir) {
                    input_Console.buff.set_text("[用户] 打开选取导出到的文件夹");
                    
                    libWxIkunPlus::setwinVisible(g_appMainWinHwnd, false);
                    let mut open_path = libWxIkunPlus::openSelectFolder2();
                    libWxIkunPlus::setwinVisible(g_appMainWinHwnd, true);
                    
                    input_Console
                        .buff
                        .append(format!("\n[选取器] 用户输入的文件路径为: {}",open_path).as_str());
                   
                    if(open_path.len()>2){
                        input_shellOpenExportDir.buff.remove(0, input_shellOpenExportDir.buff.length());
                        input_shellOpenExportDir.buff.append(&open_path);
                        if(input_shellName.buff.length()<2){
                            input_shellName.buff.remove(0, input_shellName.buff.length());
                            let file_name = Path::new(&open_path).file_name().unwrap();
                            input_shellName.buff.append(&format!("{:#?}",file_name).replace("\"", ""));
                        }
                    }
                  

                    println!("click => shellOpenExportDir");
                } else if (point_exist_hasmap.starting) {
                    input_Console.buff.set_text("[用户] 配置自启动");
                    // input_Console
                    // .buff
                    // .append(format!("\n[错误] 暂时不支持此功能 使用其他软件添加").as_str());
                    
                    if(libWxIkunPlus::setStartup()){
                       input_Console
                   .buff
                 .append(format!("\n[状态] 添加自启动成功").as_str());  
                    }else{
                        input_Console
                        .buff
                      .append(format!("\n[状态] 自启动已被移除").as_str());  
                    }
                    
                    // if libWxIkunPlus::hasStartup() {
                    //     btnEnableStarting.from_data(include_bytes!("./assets/enable.png").to_vec(), 0, 0,20, 20);
                    // }else{
                    //     btnEnableStarting.from_data(include_bytes!("./assets/un_enable.png").to_vec(), 0, 0,20, 20);
                    // }

                    println!("click => starting");
                } else if (point_exist_hasmap.test) {
                    input_Console.buff.set_text("[用户] 测试新的配置文件");
                    println!("click => test");
                }

                if (point_exist_hasmap.test || point_exist_hasmap.create) {
                    if (input_shellOpenDatDir.buff.length() < 1) {
                        input_Console
                            .buff
                            .append(format!("\n[错误] 尚未输入dat目录文件夹").as_str());
                    } else {
                        let mut path_dir = parse_dat_path(input_shellOpenDatDir.buff.text());

                        has_inputPath = true;

                        match fs::metadata(path_dir.attach_dir.clone()) {
                            Ok(metadata) => {
                                if (!metadata.is_dir()) {
                                    input_Console.buff.append(
                                        format!("\n[错误] dat目录文件夹 不是文件夹").as_str(),
                                    );
                                } else if point_exist_hasmap.test {
                                    input_Console.buff.append(
                                        format!("\n[测试] 正在扫描当前文件夹存在的dat图片")
                                            .as_str(),
                                    );
                                    fn bool_to_str (b:bool) -> &'static str {
                                        if b {"是"} else { "否" }
                                    }

                                    input_Console.buff.append(
                                        format!("\n[测试] 处理范围: 仅本月:{}   缩略图:{}   原图:{}   全部:{}   ",bool_to_str(path_dir.is_the_month),bool_to_str(path_dir.is_thumbnail),bool_to_str(path_dir.is_source),bool_to_str(path_dir.is_all))
                                            .as_str(),
                                    );

                                    let pattern = format!(
                                        "{}",
                                        Path::new(&path_dir.attach_dir.clone())
                                            .join("**/*.dat")
                                            .display()
                                            .to_string()
                                    );
                                    let mut index = 0;

                                    input_Console.buff.append(
                                        format!("\n[测试] 开始扫描 “{}” 中的dat文件",pattern)
                                            .as_str(),
                                    );

                                    for entry in glob(&pattern).unwrap() {
                                        let path = entry.unwrap().display().to_string();
                                        let base = Path::new(&path).file_name().unwrap().to_str().unwrap();
                                        index = index + 1;
                                    }

                                    input_Console.buff.append(
                                        format!("\n[测试] 在 “{}” \n中发现了 [{}] 个dat文件",pattern,index)
                                            .as_str(),
                                    );
                                }
                            }
                            Err(err) => {
                                input_Console.buff.append(
                                    format!(
                                        "\n[错误] dat目录文件夹 无法被读取 因为-> {}",
                                        err.to_string()
                                    )
                                    .as_str(),
                                );
                            }
                        };
                    }

                    if (input_shellName.buff.length() < 1) {
                        
                        input_Console
                            .buff
                            .append(format!("\n[错误] 配置名称为空").as_str());
                    }else{
                        has_name = true;
                    }

                    if (input_shellOpenExportDir.buff.length() < 1) {
                        
                        

                        input_Console
                            .buff
                            .append(format!("\n[错误] 尚未输入存储转码文件的目录").as_str());
                    } else {
                        has_ouputPath = true;
                        match fs::metadata(input_shellOpenExportDir.buff.text()) {
                            Ok(metadata) => {
                                if (!metadata.is_dir()) {
                                    input_Console.buff.append(
                                        format!("\n[错误] 存储转码文件的目录 不是文件夹").as_str(),
                                    );
                                } }
                            Err(err) => {
                                // input_Console.buff.append(
                                //     format!(
                                //         "\n[提醒] 存储转码文件的目录 无法被读取 因为-> {}",
                                //         err.to_string()
                                //     )
                                //     .as_str(),
                                // );
                            }
                        };
                    }
                }
               
                // println!("{} , {} , {} , {}",has_name,has_inputPath,has_ouputPath,point_exist_hasmap.create);

                if (has_name&&has_inputPath&&has_ouputPath&&point_exist_hasmap.create){
                    if(wh_mod::convert::is_developer()||(libWxIkunPlus::hasWeChat()&&libWxIkunPlus::hasWeChatWin())){
                        let conn: Connection = Connection::open("ikun_user_data.db").unwrap();
                    
                        handle_dat::initialize_table(&conn);
                        match  conn.execute(
                            "INSERT INTO export_dir_path (name,time,path,ouput) values (?1, ?2, ?3, ?4)",
                            [input_shellName.buff.text(),Local::now().format("%Y-%m-%d").to_string(),input_shellOpenDatDir.buff.text(),input_shellOpenExportDir.buff.text()],
                        ) {
                          Ok(_)=>{
                            input_Console.buff.append(
                                format!("\n[存储] 添加成功").as_str(),
                            );
                          } 
                          Err(err)=>{
                            if(str_eq_ostr(err.to_string(),"UNIQUE constraint failed: export_dir_path.path")){
                                input_Console.buff.append(
                                    format!("\n[错误] 添加失败 因为-> {}","当前被导出的路径已经存在").as_str(),
                                );
                            }else
    
                            {
                                input_Console.buff.append(
                                    format!("\n[错误] 添加失败 因为-> {}",err.to_string()).as_str(),
                                );
                            }
                          } 
                        }
    
                        conn.close();
                        global_var_util::update_export_dir_itme_list();

                     }else{
                        //  libWxIkunPlus::stop("错误".to_owned(),"当前未发现wx进程 拒绝提供选取方案".to_owned())
                        // dialog::alert_default(format!("\n[错误] {}","当前未发现wx进程或者未登录 拒绝提供添加").as_str());
                        libWxIkunPlus::stop("错误".to_owned(),"当前未发现wx进程或者未登录 拒绝提供添加".to_owned());

                        // input_Console.buff.append(
                        //     format!("\n[错误] 添加失败 因为-> {}","当前未发现wx进程或者未登录 拒绝提供添加").as_str(),
                        // );
                       
                     }

                }
           

               
               
                true
            }

            enums::Event::Move => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;
                point_exist_hasmap = getFormPointSpace(x, y);
                // -处理鼠标图标的逻辑

                if (point_exist_hasmap.existAllBtn) {
                    win.clone().set_cursor(Cursor::Hand);
                } else {
                    win.clone().set_cursor(Cursor::Default);
                }

                true
            }

            // enums::Event:
            enums::Event::Drag => {
                if (y < 74) {
                    win.clone()
                        .set_pos(app::event_x_root() - x, app::event_y_root() - y);
                }

                true
            }
            _ => false,
        }
    });

    // dock_win.handle({
    //     let mut win = copy_appMainWin.clone();
    //     let mut dock_win = dock_win.clone();

    //     g_copy_dock_win_hwnd = get_window_hwnd(&dock_win);

    //     setTaskbarWin(g_copy_dock_win_hwnd);

    //     move |_wself, event| match event {
    //         enums::Event::Show=>{
    //             if(g_copy_dock_win_hwnd.eq(&0)){
    //             g_copy_dock_win_hwnd = get_window_hwnd(&dock_win);
    //             }
    //             true
    //         }
    //         enums::Event::Focus => {
               
    //             if(g_copy_dock_win_hwnd.eq(&0)){
    //                 g_copy_dock_win_hwnd = get_window_hwnd(&dock_win);
    //                 }

    //             setTaskbarWin(g_copy_dock_win_hwnd);
    //             libWxIkunPlus::setwinVisible(g_appMainWinHwnd , true);
    //             // win.show();
    //             true
    //         }
    //         enums::Event::Hide => {
    //             libWxIkunPlus::setwinVisible(g_appMainWinHwnd, false);
                
    //             true
    //         }
    //         enums::Event::Close => {
    //             process::exit(0);
    //             true
    //         }
    //         _ => false,
    //     }
    // });
    
    loop {
        Sleep(200);
        if (util::getVarBooleanValue("K9V7OKIIMR1E1_theInitializationWindowIsDisplayed".to_owned()))
        {
            appMainWin.show();
            appRootView.show();
            break;
        }
    }
    appRootView.set_visible_focus();
    // appMainWin.hide();
    // let path = gui_select_user_base::mian_window();

    Ok(MianWindowItme {
        appRootView,
        appMainWin,
    })
}

