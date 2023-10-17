#![allow(warnings, unused)]

use crate::{global_var, gui_util, libWxIkunPlus, set_item_id};
use fltk::enums::{Color, FrameType};
use fltk::window::DoubleWindow;
use fltk::{prelude::*, *};
use fltk_theme::{color_themes, ColorTheme, SchemeType, ThemeType, WidgetScheme, WidgetTheme};
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Local};
pub(crate) const THE_WINDOW_CLASS_NAME: &'static str = "wx_auto_ex_im::gui_util::rename_tool::main<32626>";


struct time_info {
    // 年
    years:String,
    // 月
    month:String,
    // 日
    day:String,
    // 时
    hour:String,
    // 分
    minutes:String,
    // 今天
    time:String,
    // 创建月
    time_years:String
}

fn get_time_info () ->time_info {
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

macro_rules! the_token {
    ()=>{
       {
        let mut _the_token =format!("token<{}>@query",libWxIkunPlus::randomNum());
        loop{
            if global_var::has_string(_the_token.as_str()) {
                _the_token = format!("token<{}>@query",libWxIkunPlus::randomNum());
            }else{
                break;
            }
        }
            _the_token
        }
    }
}

pub fn rename_tool_main(input:&str) -> String {

    let mut the_token = the_token!();

    let mut win = window::Window::default().with_size(600, 550).center_screen();
    win.set_label("命名规则工具");
    set_item_id!(win, THE_WINDOW_CLASS_NAME);
    win.set_border(false);
    let mut rename_variable_input_oid_str = String::new();
    let time_info =get_time_info();
    
    let mut preview =
    gui_util::img::ImgPreview::new(0, 0, win.w(), win.h(), THE_WINDOW_CLASS_NAME);
    preview.from_svg(
        include_str!("./src/contour.svg"),
        0,
        0,
        preview.preview.w(),
        preview.preview.h(),
    );
    preview.preview.set_id("gui_util::rename_tool::main<contour>");

    let mut title =
    gui_util::text::TextControl::new(60 - 25, 24, 150, 20, 15, "命名规则工具", [122, 120, 120]);
    let mut title_box_01 = gui_util::text::TextControl::new(
        210 - 15, /* -15 Frame偏差*/
        76,
        206,
        18,
        12,
        "规则自变量 (自动)   单击可以输入",
        [122, 120, 120],
    );

    // 时间变量：（第一行）
    gui_util::text::TextControl::new(36, 113, 61, 18, 12, "时间变量:", [122, 120, 120]);
    //其他变量: （第二行）
    gui_util::text::TextControl::new(36, 156, 61, 18, 12, "其他变量:", [122, 120, 120]);
    //自增序列(必选): （第三行）
    gui_util::text::TextControl::new(36, 250, 95, 18, 12, "自增序列(必选):", [122, 120, 120]);

    //预览变量 (存储文件名)：
    gui_util::text::TextControl::new(
        36 - 6 + 32,
        318,
        150,
        18,
        12,
        "命名规则  变量公式 [ 在此可编辑 ]  ：",
        [122, 120, 120],
    );
    //预览变量 (存储文件名)：
    gui_util::text::TextControl::new(
        36,
        386 - 3, /* -3 Frame偏差*/
        150,
        18,
        12,
        "预览变量 (存储文件名)：",
        [122, 120, 120],
    );

    gui_util::text::TextControl::new(192, 505 - 7, 35, 18, 13, "取消", [121, 121, 121]);
    gui_util::text::TextControl::new(371 - 3, 505 - 7, 35, 18, 13, "确认", [122, 120, 120]);

    // 命名规则
    let mut rename_variable_input = input::Input::new(35 - 1, 346, 533, 30, "");
    rename_variable_input.set_id("gui_util::rename_variable_input");
    let rename_variable_input_hotspot = gui_util::hotspot::create_hotspot(35 - 1, 346, 533, 30);
    rename_variable_input.append(if input.is_empty() {"<创建月>/<任务名>/<类型>_<NN>"}else { input });

    // 预览
    let mut rename_preview_input = input::Input::new(35 - 1, 399 + 10, 533 /*490 - 12*/, 30, "");
    rename_preview_input.set_readonly(true);
    rename_preview_input.set_id("gui_util::rename_preview_input");

    // 后缀名
    // let rename_preview_ext =
    //     gui_util::text::TextControl::new(528 - 15, 399 + 16, 35, 20, 13, ".jpg", [122, 120, 120]);
    // rename_variable_input.set_id("gui_util::rename_preview_ext");

    // 退出窗口
    let exit_btn = gui_util::hotspot::create_hotspot(540, 15, 37, 37);

    // 确认/取消
    let mut cancel_btn = gui_util::hotspot::create_hotspot(147, 488 - 2, 125, 38);
    let mut confirm_btn = gui_util::hotspot::create_hotspot(139 + 8 + 170 + 5, 488 - 2, 125, 38);
    let mut variable_list = Vec::from([
        gui_util::variable_tag_control::varTagControl::new(
            106,
            108,
            128,
            31,
            "现在:",
            &time_info.time,
            &time_info.time,
        ),
        gui_util::variable_tag_control::varTagControl::new(245, 108, 72, 31, "年:", &time_info.years, &time_info.years),
        gui_util::variable_tag_control::varTagControl::new(328, 108, 72, 31, "月:", &time_info.month, &time_info.month),
        gui_util::variable_tag_control::varTagControl::new(388, 108, 72, 31, "日:", &time_info.day, &time_info.day),
        gui_util::variable_tag_control::varTagControl::new(450 - 2, 108, 72, 31, "时:", &time_info.hour, &time_info.hour),
        gui_util::variable_tag_control::varTagControl::new(508, 108, 72, 31, "分:", &time_info.minutes, &time_info.minutes),
        gui_util::variable_tag_control::varTagControl::new(
            106,
            150,
            178,
            31,
            "别名:",
            "软件内的用户备注名",
            "事妈老板",
        ),
        gui_util::variable_tag_control::varTagControl::new(
            293,
            150,
            182,
            31,
            "任务名:",
            "当前任务的任务名",
            "工作内容备份",
        ),
        gui_util::variable_tag_control::varTagControl::new(483, 150, 83, 31, "创建月:", "月", &time_info.time_years),
        gui_util::variable_tag_control::varTagControl::new(
            33,
            196,
            221,
            31,
            "类型:",
            "缩略图,视频,图片,手机截图",
            "图片",
        ),
        gui_util::variable_tag_control::varTagControl::new(
            265,
            196,
            100,
            31,
            "哈希:",
            "dat名称",
            "666a6b6666666abc999aa9b6bc99999a",
        ),
        gui_util::variable_tag_control::varTagControl::new(146, 231 + 13, 53, 31, "%N:", "1", "1"),
        gui_util::variable_tag_control::varTagControl::new(205, 231 + 13, 73, 31, "%NN:", "01", "01"),
        gui_util::variable_tag_control::varTagControl::new(289, 231 + 13, 92, 31, "%NNN:", "001", "001"),
        gui_util::variable_tag_control::varTagControl::new(
            392,
            231 + 13,
            115,
            31,
            "%NNNN:",
            "0001",
            "0001",
        ),
    ]);


    let mut rename_variable_input_copy = rename_variable_input.clone();
    let mut rename_preview_input_copy = rename_preview_input.clone();

    macro_rules! update_variable_input {
        () => {

            if !rename_variable_input_copy.value().as_bytes().eq(rename_variable_input_oid_str.as_bytes()){

            // 更新预览
            let mut label = rename_variable_input_copy.value();

            // 替换变量值
            for index in 1..variable_list.len() + 1 {
                if let Some(variable) = variable_list.get(variable_list.len() - index) {
                    label = label.replace(variable.get_var().as_str(), variable.data.as_str());
                }
            }
            rename_variable_input_oid_str = format!("{}{}",label.as_str(),".jpg");
            // 设置预览文本
            rename_preview_input_copy.set_value(rename_variable_input_oid_str.as_str());

            }
        };
    }

    update_variable_input!();

    win.handle({
        let mut the_token =the_token.clone();
        let mut x = 0;
        let mut y = 0;
        move |win, ev| match ev {
            enums::Event::Show => {
                win.set_visible_focus();

                true
            }
            enums::Event::KeyUp=>{
                update_variable_input!();
                true
            }

            enums::Event::Push => {
                if exit_btn.existPoint(x, y) {
                    fltk::window::Window::delete(win.clone());
                }

                for variable in variable_list.iter() {
                    if variable.existPoint(x, y) {
                        println!(
                            "click[variable]-> {}  var<  {}  >",
                            variable.id.clone(),
                            variable.get_var()
                        );
                        rename_variable_input_copy.append(variable.get_var().as_str());

                        break;
                    }
                }

                if cancel_btn.existPoint(x,y) {
                    global_var::set_string(the_token.as_str(),String::new());
                    fltk::window::Window::delete(win.clone());
                }

                if confirm_btn.existPoint(x,y){
                    global_var::set_string(the_token.as_str(),rename_variable_input_copy.value());
                    fltk::window::Window::delete(win.clone());
                }

                update_variable_input!();
                true
            }

            enums::Event::Move => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;

                if(rename_variable_input_hotspot.existPoint(x,y)){
                    update_variable_input!();
                }

                //  判断鼠标是否在变量标签上
                let mut is_variable_tag_control = false;
                for variable in variable_list.iter() {
                    is_variable_tag_control = variable.existPoint(x, y);
                    if (is_variable_tag_control) {
                        break;
                    }
                }

                // 判断是否显示手型鼠标
                if exit_btn.existPoint(x, y)
                    || confirm_btn.existPoint(x, y)
                    || cancel_btn.existPoint(x, y)
                    || is_variable_tag_control
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

    win.show();

    the_token.clone()
}
