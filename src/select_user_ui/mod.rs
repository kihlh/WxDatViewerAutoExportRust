#![allow(warnings, unused)]

use crate::{gui_util, libWxIkunPlus};
use crate::gui_util::img::ImgPreview;
use crate::gui_util::text::TextControl;
use crate::gui_util::variable_tag_control::varTagControl;
use fltk::enums::{Color, FrameType};
use fltk::window::DoubleWindow;
use fltk::{prelude::*, *};
use fltk_theme::{color_themes, ColorTheme, SchemeType, ThemeType, WidgetScheme, WidgetTheme};
use crate::libWxIkunPlus::findWindow;
use std::ptr::null_mut;
use std::sync::{Arc, RwLock};
use once_cell::sync::Lazy;
use std::collections::HashSet;

macro_rules! set_item_id {
    ($win:expr,$id:expr) => {
        $win.set_xclass($id);
        $win.set_id($id);
    };
}

struct FrameText {
    选择: TextControl,
    文件管理: TextControl,
    选择用户: TextControl,
    通过拽入获取: TextControl,
    选择最近对象: TextControl,
    帮助: TextControl,
    备注名称: TextControl,
    用户目录: TextControl,
    命名规则: TextControl,
    编辑规则: TextControl,
    完成选定: TextControl,
    备注: TextControl,
}

// 添加无热点的文本
fn set_frame_text() -> FrameText {

    let mut preview = gui_util::img::ImgPreview::new(
        490-2, 167,
        18, 18,
        "gui_util::select_user_ui::imag<help>",
    );

    preview.from_svg(
        include_str!("./src/help.svg"),
        0,
        0,
        preview.preview.w(),
        preview.preview.h(),
    );

    FrameText {
        选择: TextControl::new(
            50 - 15,
            46,
            180,
            18,
            12,
            "请选择WX文件的默认保存位置*",
            [85, 85, 85],
        ),
        文件管理: TextControl::new(
            273,
            46,
            239,
            18,
            12,
            "此路径在：  设置  /  文件管理   /  文件管理(输入框)",
            [49,49,49],
        ),
        选择用户: TextControl::new(
            50 - 3 - 37,
            118,
            475,
            18,
            12,
            "选择用户*    [ 涉及用户数据安全   仅限已登录中的用户 并且能够收发的用户 ]",
            [85, 85, 85],
        ),
        选择最近对象: TextControl::new(
            59 ,
            207+5,
            465,
            18,
            12,
            "选择最近对象*  （如果不存在请发送一张随意的图片给对方 [不能是表情] ） ",
            [85, 85, 85],
        ),
        通过拽入获取: TextControl::new(366, 166, 85, 18, 12, "通过扫描获取", [85, 85, 85]),
        帮助: TextControl::new(510-2, 167, 30, 18, 12, "帮助", [85, 85, 85]),
        备注名称: TextControl::new(139, 398, 65, 18, 13, "备注名称:", [85, 85, 85]),
        用户目录: TextControl::new(139, 439 + 2, 65, 18, 13, "用户目录:", [85, 85, 85]),
        命名规则: TextControl::new(42, 525, 65, 18, 13, "命名规则:", [85, 85, 85]),
        编辑规则: TextControl::new(495, 523, 56, 18, 12, "编辑规则", [85, 85, 85]),
        完成选定: TextControl::new(495, 437, 58, 18, 12, "完成选定", [255, 255, 255]),
        备注: TextControl::new(513, 395, 30, 18, 13, "备注", [85, 85, 85]),
    }

}

struct FrameCheck {
    sync: button::CheckButton,
    video: button::CheckButton,
    thumbnail: button::CheckButton,
    source: button::CheckButton,
    the_month: button::CheckButton,
}

impl FrameCheck {
    pub fn existPoint(&self, x: i32, y: i32) -> bool {
        let self_x = 38;
        let self_y = 475;
        let self_w = 520;
        let self_h = 33;

        return x > self_x
            && x < self_x + self_w
            && y > self_y
            && y < self_y + self_h;
    }
}

// 条件选定
fn add_check_button() -> FrameCheck {
    let mut flex = group::Flex::default()
        .with_size(530, 30)
        .row()
        .center_of_parent();

    flex.set_pos(43, 479);

    let mut check_button_sync = button::CheckButton::default().with_label("启用同步");
    let mut check_button_video = button::CheckButton::default().with_label("转存视频");
    let mut check_button_thumbnail = button::CheckButton::default().with_label("存缩略图");
    let mut check_button_source = button::CheckButton::default().with_label("保存原图");
    let mut check_button_the_month = button::CheckButton::default().with_label("只保存本月");

    check_button_source.set_checked(true);
    check_button_sync.set_checked(true);

    flex.end();

    FrameCheck {
        sync: check_button_sync,
        video: check_button_video,
        thumbnail: check_button_thumbnail,
        source: check_button_source,
        the_month: check_button_the_month,
    }
}


struct FrameThumbnailPreview{
    hotspot_list:Vec<gui_util::hotspot::HotspotItmeControl>,
    thumbnail_list:Vec<ImgPreview>
}


// 缩略图列表
fn add_frame_thumbnail_preview() ->FrameThumbnailPreview {
    let mut preview_main_point_list = [
        [71, 296, 75, 75],
        [167, 296, 75, 75],
        [263, 296, 75, 75],
        [359, 296, 75, 75],
        [455, 296, 75, 75],
    ];

    let mut preview_main = Vec::new();
    let mut hotspot_list =Vec::new();

    for index in 0..preview_main_point_list.len() {
        let point = preview_main_point_list[index];
        let [x, y, width, height] = point;

        let mut preview = ImgPreview::new(x, y - 52, width, height, "gui::preview_main::index::");

        preview.from_data(
            include_bytes!("./src/not.png").to_vec(),
            -1,
            -1,
            width - 2,
            height - 2,
        );

        preview_main.push(preview);
        hotspot_list.push(gui_util::hotspot::create_hotspot(x, y - 52, width, height));
    }

    FrameThumbnailPreview{
        hotspot_list:hotspot_list,
        thumbnail_list:preview_main,
    }
}

// 添加背景页
macro_rules! add_preview_contour {
    ($win:expr) => {{
        let mut preview = gui_util::img::ImgPreview::new(
            0,
            0,
            $win.w(),
            $win.h(),
            "gui_util::select_user_ui::main<win>",
        );
        preview.from_svg(
            include_str!("./src/contour.svg"),
            0,
            0,
            preview.preview.w(),
            preview.preview.h(),
        );
        preview
            .preview
            .set_id("gui_util::select_user_ui::main<contour>");
        preview
    }};
}

fn select_user_data_choice() -> menu::Choice {
    let mut choice = menu::Choice::default().with_size(277, 35).center_of_parent().with_label("");
    choice.set_pos(60,158);
    choice.add_choice("请在选择WX数据位置后 选择在线用户");
    choice.set_value(0);
    choice.set_color(Color::from_rgb(23, 23, 23));

    choice.set_callback(|c| {
        if let Some(choice) = c.choice() {
            println!("{}", choice.as_str());

        }
    });

    choice
}

struct AttachThumbnailPreview{
    thumbnail_preview: ImgPreview,
    btn_remark: gui_util::hotspot::HotspotItmeControl,
    btn_rename: gui_util::hotspot::HotspotItmeControl,
    btn_select: gui_util::hotspot::HotspotItmeControl,
    input_rename: fltk::input::Input,
    input_remark: fltk::input::Input,
    input_attach: fltk::input::Input,
}

// 用户选定预览的卡片(底部)
fn add_select_attach_card() -> AttachThumbnailPreview {
    let mut preview = gui_util::img::ImgPreview::new(
        41+3, 385,
        82, 82,
        "gui_util::select_user_ui::imag<add_select_attach_card>",
    );

    preview.from_data(
        include_bytes!("./src/not_select.png").to_vec(),
        -1,
        -1,
        preview.preview.w()-2,
        preview.preview.h()-2,
    );

    let mut button_remark = gui_util::hotspot::create_hotspot(495, 387 , 66, 32);
    let mut button_select = gui_util::hotspot::create_hotspot(484, 430 , 77, 32);
    let mut button_rename = gui_util::hotspot::create_hotspot(484, 516 , 77, 32);

    // 命名规则
    let mut rename_input = input::Input::new(115, 518, 357, 30, "");

    // 备注输入框
    let mut remark_input = input::Input::new(211, 389, 275, 30, "");

    // 用户目录
    let mut user_data_preview = input::Input::new(213, 432, 263, 30, "");
    user_data_preview.set_readonly(true);

    AttachThumbnailPreview{
        thumbnail_preview:preview,
        btn_remark:button_remark,
        btn_rename:button_rename,
        btn_select:button_select,
        input_rename:rename_input,
        input_remark:remark_input,
        input_attach:user_data_preview
    }

}

pub fn manage_tool_main() {
    let mut win = window::Window::default()
        .with_size(600, 595)
        .center_screen();
    win.set_label("任务创建向导");
    set_item_id!(win, "gui_util::select_user_ui::main<win>");
    win.set_pos(2106,150);

    // 退出窗口
    // let exit_btn = gui_util::hotspot::create_hotspot(540, 15, 37, 37);

    let mut preview = add_preview_contour!(win);

    // 固定文本
    let mut frame_text = set_frame_text();
    // 条件选定
    let mut frame_check = add_check_button();
    // 五张缩略图
    let mut frame_thumbnail_preview = add_frame_thumbnail_preview();
    // 用户选择数据库
    let mut select_user_data_choice = select_user_data_choice();
    // 预览卡片
    let mut select_attach_card  = add_select_attach_card();

    // 文件的默认保存位置
    let mut user_select_database_dir_input = input::Input::new(45+3, 74, 451, 30, "");

    let mut button_open_dir = gui_util::hotspot::create_hotspot(516, 73 , 33, 32);
    let mut button_show_drag = gui_util::hotspot::create_hotspot(346, 156 , 123, 38);
    let mut button_show_help = gui_util::hotspot::create_hotspot(479, 156 , 66, 38);
    select_attach_card.input_rename.set_value("<创建月>/<任务名>/<类型>_<NN>");

    win.handle({
        let mut x = 0;
        let mut y = 0;
        // 是否显示手型
        let mut show_cursor = false;
        let mut hwnd :i128 = 0;

        move |win, ev| match ev {
            enums::Event::Show => {
                win.set_visible_focus();
                hwnd = win.raw_handle() as i128;
                libWxIkunPlus::setWinIcon(hwnd);
                libWxIkunPlus::setWinTop(hwnd ,true);

                true
            }

            enums::Event::KeyUp => true,

            enums::Event::Push => {
                // if exit_btn.existPoint(x, y) {
                //     fltk::window::Window::delete(win.clone());
                // }

                // 选择最近5个对象
                {
                    let mut index = 0;
                    for hotspot in &frame_thumbnail_preview.hotspot_list {
                        index+=1;
                        if hotspot.existPoint(x,y) {
                            println!("[click] frame_thumbnail_preview -> {}",index);
                            break;
                        }
                    }
                }

                // 打开文件夹选择器
                if button_open_dir.existPoint(x, y){

                    let mut select_dir = libWxIkunPlus::openSelectFolder2();
                    if !select_dir.is_empty(){

                        if !select_dir.contains("WeChat Files"){
                            select_dir.push_str("\\WeChat Files");
                        }

                        user_select_database_dir_input.set_value(select_dir.as_str());
                    }

                    println!("[click] existPoint {}  select_dir-> {} ","打开文件夹选择器",select_dir);
                }

                // 显示扫描获取面板
                if button_show_drag.existPoint(x, y){
                    println!("[click] existPoint {}","显示扫描获取面板");
                }

                // 显示帮助面板
                if button_show_help.existPoint(x, y) {
                    println!("[click] existPoint {}","");
                }

                // 卡片按钮 > 完成选定
                if select_attach_card.btn_select.existPoint(x, y) {
                    println!("[click] existPoint {}","卡片按钮 > 完成选定");

                    let mut data = String::new();
                    let mut is_effective = true;

                    if frame_check.thumbnail.is_checked(){
                        data.push_str("*thumbnail");
                    }
                    if frame_check.source.is_checked(){
                        data.push_str("*source");
                    }
                    if frame_check.video.is_checked(){
                        data.push_str("*video");
                    }
                    if frame_check.sync.is_checked(){
                        data.push_str("*Sync");
                    }
                    if frame_check.the_month.is_checked(){
                        data.push_str("*the_month");
                    }

                    let mut rename_rule = select_attach_card.input_rename.value();
                    let select_attach_id = select_attach_card.input_attach.value();

                    // 没有选定的路径
                    if user_select_database_dir_input.value().is_empty(){
                        fltk::dialog::alert_default("没有选定Wx路径");
                        is_effective = false;
                    }
                    //  判断是否有Att id
                    else if select_attach_id.is_empty()||select_attach_id.len()<25{
                        fltk::dialog::alert_default("attach id 无效 （尚未选定有效聊天对象）");
                        is_effective = false;
                    }

                    // 有命名规则 要求规则最少有一个%N.. 自变量
                    if !rename_rule.is_empty() && (!rename_rule.contains("<N")||!rename_rule.contains("N>")){
                        rename_rule.push_str("<NN>");
                    }

                    // 添加名称格式化自变量
                    if !select_attach_card.input_rename.value().is_empty(){
                        data.push_str(format!("*rename_rule={}*",&rename_rule).as_str());
                    }


                    println!("select_attach_card -> {}",&data);

                    if is_effective{
                        fltk::window::Window::delete(win.clone());
                    }

                }

                // 卡片按钮 > 备注名称 完成按钮
                if select_attach_card.btn_remark.existPoint(x, y) {
                    println!("[click] existPoint {}","卡片按钮 > 备注名称 完成按钮");
                }

                // 卡片按钮 > 编辑命名规则
                if select_attach_card.btn_rename.existPoint(x, y) {
                    println!("[click] existPoint {}","卡片按钮 > 编辑命名规则");
                }

                true
            }

            enums::Event::Move => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;
                let mut new_show_cursor = false;

                // 关闭窗口按钮
                // if !show_cursor { show_cursor = exit_btn.existPoint(x, y) }

                // 可选项
                if !new_show_cursor { new_show_cursor = frame_check.existPoint(x, y) }

                // 卡片按钮
                if !new_show_cursor {
                    new_show_cursor = {
                            select_attach_card.btn_select.existPoint(x, y)||
                            select_attach_card.btn_remark.existPoint(x, y)||
                            select_attach_card.btn_rename.existPoint(x, y)
                    }
                }

                // 主界面按钮 打开 / 显示拖拽 / 显示帮助
                if !new_show_cursor {
                    new_show_cursor = {
                        button_open_dir.existPoint(x, y)||
                            button_show_drag.existPoint(x, y)||
                            button_show_help.existPoint(x, y)
                    }
                }

                // 缩略图（5张）
                if !new_show_cursor {
                    let mut index = 0;
                    for hotspot in &frame_thumbnail_preview.hotspot_list {
                        index+=1;
                        if hotspot.existPoint(x,y) {
                            new_show_cursor=true;
                            break;
                        }
                    }
                }


                if new_show_cursor!=show_cursor{
                    // 判断是否显示手型鼠标
                    if new_show_cursor {
                        win.set_cursor(fltk::enums::Cursor::Hand);
                    } else {
                        win.set_cursor(fltk::enums::Cursor::Default);
                    }
                    show_cursor=new_show_cursor ;
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

}
