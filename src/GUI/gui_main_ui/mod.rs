#![allow(warnings, unused)]

use crate::{get_bool, global_var, gui_util, inject_fltk_theme, set_bool, set_item_id};
use fltk::enums::{Color, FrameType};
use fltk::window::DoubleWindow;
use fltk::{prelude::*, *};
use fltk_theme::{color_themes, ColorTheme, SchemeType, ThemeType, WidgetScheme, WidgetTheme};
use std::collections::{HashMap, HashSet};
use std::path;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicI64, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, MutexGuard, OnceLock, RwLock};
use std::thread::sleep;
use fltk::examples::tile;
use crate::libWxIkunPlus;
use crate::gui_util::{*};
use crate::{*};
mod lib;


pub(crate) const THE_WIN_CLASS_NAME: &str = "wx_auto_ex_im::gui_util::main::main<55216>";
pub(crate) const THE_WIN_UI_BORDER: &str = include_str!("./src/contour.svg");

macro_rules! gc_the_window {
    ($win:expr) => {
        fltk::window::Window::delete($win.clone());
    };
}

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

macro_rules! main_init_check {
    () => {
        // 禁止创建多个窗口
        if let hwnd = get_the_hwnd!() {
            if hwnd != 0 && libWxIkunPlus::isWindow(hwnd) {
                if let Some(mut win) =
                    app::widget_from_id(THE_WIN_CLASS_NAME) as Option<DoubleWindow>
                {
                    win.show();
                    win.set_visible_focus();
                }
                libWxIkunPlus::setWindowShake(hwnd);
                return Option::None;
            }
        }
    };
}

struct UiControl {
    btn_close: HotspotItmeControl,
    title: TextControl,
    text_control_list: Vec<TextControl>,
    check:FrameCheckButton,
    task_command: fltk::input::Input,
    name: fltk::input::Input,
    export: fltk::input::Input,
    console: FrameConsole,
}

struct FrameCheckButton{
    flex:  fltk::group::Flex,
    sync:  button::CheckButton,
    start_up:  button::CheckButton,
}

struct FrameConsole{
    buf: fltk::text::TextBuffer,
    txt: fltk::text::TextEditor,
}

fn add_ui_control() -> UiControl {
    let btn_close = gui_util::hotspot::create_hotspot(556,26,25,25);
    let title = gui_util::TextControl::new(60-30 , 24, 150, 20, 12, " WX 图片自动备份", [122, 120, 120]);
    let text_control_list =vec![
        TextControl::new(30-2, 80, 70, 15, 12, "创建新任务", [85,85,85]),
        TextControl::new(15, 125, 385, 15, 12, "dat 图片所在文件夹（点击右侧按钮在任务创建向导中选定）：", [85,85,85]),
        TextControl::new(-8, 200, 200, 15, 12, "导出到此文件夹：", [85,85,85]),
        TextControl::new(506, 160, 35, 15, 12, "向导", [85,85,85]),
        TextControl::new(10, 280+5, 115, 15, 13, "名称：", [85,85,85]),
        TextControl::new(356-1, 280+4, 35, 15, 13, "管理", [85,85,85]),
        TextControl::new(428-1, 280+4, 35, 15, 13, "测试", [85,85,85]),
        TextControl::new(500-1, 280+4, 35, 15, 13, "创建", [85,85,85]),
        TextControl::new(30-1, 333, 56, 15, 12, "执行日志", [85,85,85]),
    ];
    let mut flex = group::Flex::default()
        .with_size(200, 30)
        .row()
        .center_of_parent();

    flex.set_pos(30, 490-5);
    let mut check_button_sync = button::CheckButton::default().with_label("同步开关");
    let mut check_button_start_up = button::CheckButton::default().with_label("开机启动");
    flex.end();

    check_button_sync.set_checked(libWxIkunPlus::has_auto_sync());
    check_button_start_up.set_checked(libWxIkunPlus::hasStartup());

    // 同步
    check_button_sync.set_callback(|win|{
        libWxIkunPlus::set_auto_sync(win.is_checked());

        if libWxIkunPlus::has_auto_sync() {
            message::sub_message(get_the_hwnd!(),message::IconType::Success,"同步状态已被启用",3500u64);
        }else {
            message::sub_message(get_the_hwnd!(),message::IconType::Info,"同步状态已被禁用",3500u64);
        }
        // 用锁定线程来实现防抖
        app::sleep(0.300);
    });

    // 自启
    check_button_start_up.set_callback(|win|{
        if win.is_checked()!=libWxIkunPlus::hasStartup() {
            libWxIkunPlus::setStartup();
        }

        if libWxIkunPlus::hasStartup() {
            message::sub_message(get_the_hwnd!(),message::IconType::Success,"开机自启动已被启用",3500u64);
        }else {
            message::sub_message(get_the_hwnd!(),message::IconType::Info,"开机自启动已被禁用",3500u64);
        }
        // 用锁定线程来实现防抖
        app::sleep(0.300);
    });

    let mut task_command_input = input::Input::new(45, 152, 423, 30, "");
    let mut export_input = input::Input::new(45, 225, 450, 30, "");
    let mut name_input = input::Input::new(96, 276, 230, 30, "");
    task_command_input.set_readonly(!wh_mod::config::is_developer());

    let mut buf = fltk::text::TextBuffer::default();
    buf.append(lib::get_init_text().as_str());
    let mut txt = fltk::text::TextEditor::default()
        .with_size(530, 105)
        .center_of_parent();
    txt.set_buffer(buf.clone());
    txt.set_frame(FrameType::NoBox);
    txt.set_text_color(Color::from_rgb(120, 120, 120));
    txt.set_label_type(fltk::enums::LabelType::None);
    txt.set_text_size(12);
    txt.set_pos(33, 362);
    txt.set_scrollbar_size(6);
    txt.show();

    let mut move_buf = buf.clone();

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

        for line in move_buf.text().lines() {
            newline_count += 1
        }

        if (newline_count > 5) {
            move_buf.remove(0, move_buf.length());
            move_buf.set_text(&console_message);
        } else {
            move_buf.append(&format!("\n{}", &console_message));
        }
    });


    UiControl {
        btn_close,
        title,
        text_control_list,
        task_command:task_command_input,
        name:name_input,
        export:export_input,
        check:FrameCheckButton{
            flex,
            sync:check_button_sync,
            start_up:check_button_start_up
        },
        console:FrameConsole{
            buf,
            txt
        }
}
}


pub fn main_init() ->Option<fltk::window::DoubleWindow> {
    main_init_check!();
    let hwnd:i128 = 0 ;
    let mut win: DoubleWindow = fltk::window::DoubleWindow::new(0, 0, 600,530, "WxAutoExIm").center_screen();
    inject_fltk_theme!();
    win.set_color(Color::from_rgb(24, 24, 24));
    win.set_border(false);
    set_item_id!(win,THE_WIN_CLASS_NAME);
    gui_util::img::ImgPreview::new_border(0,0,win.w(),win.h(),THE_WIN_UI_BORDER);
    let mut win_control = add_ui_control();
    let mut button_wizard = gui_util::hotspot::create_hotspot(475i32, 150i32 , 72i32, 32i32);
    let mut button_open_export_dir = gui_util::hotspot::create_hotspot(513i32, 225i32 , 33i32, 32i32);
    let mut button_show_manage = gui_util::hotspot::create_hotspot(342i32, 275i32 , 60i32, 32i32);
    let mut button_test = gui_util::hotspot::create_hotspot(415i32, 275i32 , 60i32, 32i32);
    let mut button_create = gui_util::hotspot::create_hotspot(486i32, 275i32 , 60i32, 32i32);
    let mut bottom_check_hotspot = gui_util::hotspot::create_hotspot(30, 490-5,200, 30);


    win.handle({
        let mut x = 0;
        let mut y = 0;
        move |win, ev| match ev {
            enums::Event::Show => {
                env::set_var("ikunWinHwnd", format!("{}",get_the_hwnd!()).to_string());
                libWxIkunPlus::setWinIconMain(get_the_hwnd!());

                true
            }
            enums::Event::Close => {
                // gc_the_window!(win);
                false
            }
            enums::Event::Hide => {
                // gc_the_window!(win);
                false
            }
            enums::Event::Push => {
                // 关闭
                if win_control.btn_close.existPoint(x,y){
                    // gc_the_window!(win);
                    libWxIkunPlus::setwinVisible(get_the_hwnd!(),false);
                }
                // 向导
                if button_wizard.existPoint(x,y) {
                    if(lib::eq_next()){
                        let mut token_id =  gui_select_user_ui::manage_tool_main();
                        println!("token_id-> {}",&token_id);
                        if(!token_id.is_empty()){

                            let mut task_command =  win_control.task_command.clone();
                            app::add_timeout3(0.3,move|handle|{
                                if !gui_select_user_ui::has_window(){

                                    let data = global_var::get_string_default(token_id.as_str());
                                    if !data.is_empty() {
                                        println!("{}",data);
                                        gui_util::sub_message(get_the_hwnd!(), gui_util::IconType::Success, "向导任务命令已赋值", 3500u64);
                                        task_command.set_value(wh_mod::get_show_mask_text(data.as_str()).as_str());
                                        global_var::set_string("user::config::task_command",data.clone());
                                    }else {
                                        gui_util::sub_message(get_the_hwnd!(), gui_util::IconType::Warning, "用户取消任务创建", 3500u64);
                                    }

                                    app::remove_timeout3(handle);
                                    return ;
                                }
                                app::repeat_timeout3(0.3, handle);
                            });

                        }else {
                            if gui_select_user_ui::has_window(){
                                gui_util::sub_message(get_the_hwnd!(), gui_util::IconType::Warning, "窗口重复创建", 3500u64);
                            }else {
                                gui_util::sub_message(get_the_hwnd!(), gui_util::IconType::Warning, "创建任务id失败", 3500u64);
                            }
                        }

                    }else{
                        libWxIkunPlus::stop("错误".to_owned(),"当前未发现wx进程或者未登录 拒绝提供选取方案".to_owned());
                    }
                }

                // 选择导出到
                if button_open_export_dir.existPoint(x,y) {
                    let mut open_path = libWxIkunPlus::openSelectFolder2();
                    // 非空
                    if !open_path.is_empty() {
                        win_control.export.set_value(open_path.as_str());
                    }
                    // 没有名称 则使用目录名
                    if win_control.name.value().is_empty() {
                        let path = path::Path::new(open_path.as_str());
                        if let Some(file_name) = path.file_name() {
                            if let Some(file_name) = file_name.to_str() {
                                win_control.name.set_value(file_name);
                            }
                        }
                    }
                }

                // 测试
                if button_test.existPoint(x,y) {
                    let mut task_command = global_var::get_string_default("user::config::task_command");
                    let [name,export]= [win_control.name.value(),win_control.export.value()];
                    lib::test_task(name.as_str(),export.as_str(),task_command.as_str());
                }
                // 管理
                if button_show_manage.existPoint(x,y) {
                    gui_manage_item::ManageItmeMain();
                }
                // 创建
                if button_create.existPoint(x,y) {
                    let mut task_command = global_var::get_string_default("user::config::task_command");
                    let [name,export]= [win_control.name.value(),win_control.export.value()];
                    lib::push_sql_export_dir_path(name.as_str(),export.as_str(),task_command.as_str());
                }
                true
            }
            enums::Event::Move => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;

                if win_control.btn_close.existPoint(x,y)
                    ||button_wizard.existPoint(x,y)
                    ||button_open_export_dir.existPoint(x,y)
                    ||button_show_manage.existPoint(x,y)
                    ||button_test.existPoint(x,y)
                    ||button_create.existPoint(x,y)
                    ||bottom_check_hotspot.existPoint(x,y)
                {
                    win.set_cursor(fltk::enums::Cursor::Hand);
                } else {
                    win.set_cursor(fltk::enums::Cursor::Default);
                }

                true
            }
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

    win.end();

    loop {
        Sleep(200);
        if (libWxIkunPlus::hasInitWindowIsDisplayed())
        {
            win.show();
            win.set_visible_focus();
            return  Some(win);
        }
    }

    Option::None
}
