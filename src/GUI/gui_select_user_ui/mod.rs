#![allow(warnings, unused)]
use crate::config;

use crate::gui_rename_ui::rename_tool_main;
use crate::{gui_util, libWxIkunPlus, global_var, wh_mod, get_arc_bind_variable, atomic_util, inject_fltk_theme, gui_drag_scan2_ui, gui_detect_config_ui, util};
use crate::gui_util::img::ImgPreview;
use crate::gui_util::text::TextControl;
use crate::gui_util::variable_tag_control::varTagControl;
use fltk::enums::{Color, FrameType};
use fltk::window::DoubleWindow;
use fltk::{prelude::*, *};
use fltk_theme::{color_themes, ColorTheme, SchemeType, ThemeType, WidgetScheme, WidgetTheme};
use crate::libWxIkunPlus::findWindow;
use std::ptr::null_mut;
use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::{
    sync::atomic::Ordering,
    sync::Arc,
    sync::MutexGuard,
    sync::{atomic::AtomicUsize, OnceLock},
};
use std::sync::atomic::AtomicBool;
use crate::gui_drag_scan2_ui::{get_history_attach_name, get_wx_temp_imag_id};
use crate::util::Sleep;

mod lib;

pub(crate) const THE_WINDOW_CLASS_NAME : &'static str = "wx_auto_ex_im::gui_util::select_user_ui::main<win:56315:>";
pub(crate) const THE_SUB_WINDOW_CLASS_NAME_FRAME_THUMBNAIL_PREVIEW: &'static str = "wx_auto_ex_im::gui_util::select_user_ui::sub_main<6103>";
pub(crate) const THE_SUB_WINDOW_CLASS_NAME_SCAN: &'static str = "wx_auto_ex_im::gui_util::select_user_ui::sub_main<126126>";
const MAIN_CONTOUR: &str = include_str!("./src/contour.svg");

pub fn ASSETS_NOT_DATA() -> Vec<u8> {
    include_bytes!("./src/not_data.png").to_vec()
}

pub fn ASSETS_DEMO_DATA() -> Vec<u8> {
    include_bytes!("../../assets/icon/demo.png").to_vec()
}

pub fn ASSETS_DEMO_NOT_DATA() -> Vec<u8> {
    include_bytes!("../../assets/icon/demo_not.png").to_vec()
}
pub fn ASSETS_DEMO_NOT_SELECT() -> Vec<u8> {
    include_bytes!("../../assets/icon/demo_not_select.png").to_vec()
}

pub fn ASSETS_NOT_SELECT () -> Vec<u8> {
    include_bytes!("./src/not_select.png").to_vec()
}

macro_rules! set_item_id {
    ($win:expr,$id:expr) => {
        $win.set_xclass($id);
        $win.set_id($id);
    };
}


// macro_rules! eq_wxid_dir{
//     ($select_dir:expr)=>{
//         {
//             let mut is_wxid_dir = false;
//             if !$select_dir.is_empty(){

//                         if !$select_dir.contains("WeChat Files"){
//                             // 没有 WeChat Files 则尝试为路径添加 WeChat Files
//                             let mut to_path = std::path::Path::new($select_dir.as_str());
//                             let mut join_path = to_path.join("WeChat Files");

//                             if join_path.exists() && join_path.is_dir(){
//                                $select_dir.push_str("\\WeChat Files");
//                             }

//                         }

//                         // 判断路径下是否有 wxid_ 开头的文件夹
//                         if let Ok(rd_dir) = std::fs::read_dir($select_dir.as_str()) {

//                             for rd_dir in rd_dir {
//                                 if let Ok(dir) = rd_dir {
//                                     is_wxid_dir= wh_mod::get_wx_user_store(dir).is_some();
//                                     if is_wxid_dir{
//                                         break;
//                                     }
//                                 }
//                             }

//                             if !is_wxid_dir{
//                                 // dialog::alert_default("此路径可能不是有效的WX目录 因为未发现有效的用户数据");
//                               gui_util::message::sub_message(libWxIkunPlus::findWindow(THE_WINDOW_CLASS_NAME,""),gui_util::message::IconType::Warning,"此WX目录 未发现有效的用户数据目录",3500u64);
//                             }

//                         }else{
//                             gui_util::message::sub_message(libWxIkunPlus::findWindow(THE_WINDOW_CLASS_NAME,""),gui_util::message::IconType::Failure,"目录无法被打开 请注意路径有效性",3500u64);
//                             // dialog::alert_default("目录无法被打开 请注意路径有效性");
//                         }

//                     }
                   
//             is_wxid_dir
//         }
//     }
// }

macro_rules! get_the_hwnd {
    ($class_id:expr) => {
        {
        let mut _hwnd = 0 ;
        for _ in 0..8 {
          _hwnd = libWxIkunPlus::findWindow($class_id,"");
            if !libWxIkunPlus::isWindow(_hwnd) {
                 _hwnd=0;
            }else {
              break;
          }
            fltk::app::sleep(0.020);
        }
        _hwnd as i128}
    };
    ()=>{
        get_the_hwnd!(THE_WINDOW_CLASS_NAME)
    }
}

pub fn has_window() -> bool{
    let hwnd = get_the_hwnd!(THE_WINDOW_CLASS_NAME);
    libWxIkunPlus::isWindow(hwnd)
}

struct FrameText {
    选择: TextControl,
    文件管理: TextControl,
    选择用户: TextControl,
    通过拽入获取: TextControl,
    选择最近对象: TextControl,
    帮助: TextControl,
    别名备注: TextControl,
    用户目录: TextControl,
    命名规则: TextControl,
    编辑规则: TextControl,
    完成选定: TextControl,
    备注: TextControl,
    开始:TextControl
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
        别名备注: TextControl::new(139, 398, 65, 18, 13, "别名备注:", [85, 85, 85]),
        用户目录: TextControl::new(139, 439 + 2, 65, 18, 13, "用户目录:", [85, 85, 85]),
        命名规则: TextControl::new(42, 525, 65, 18, 13, "命名规则:", [85, 85, 85]),
        编辑规则: TextControl::new(495, 523, 56, 18, 12, "编辑规则", [85, 85, 85]),
        完成选定: TextControl::new(495, 437, 58, 18, 12, "完成选定", [255, 255, 255]),
        备注: TextControl::new(513, 395, 30, 18, 13, "备注", [85, 85, 85]),
        开始: TextControl::new(451+15, 73+7, 30, 18, 12, "开始", [85, 85, 85]),
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

    check_button_video.deactivate();

    check_button_sync.set_callback(|win|{
        global_var::set_bool("user::config::check_button_sync",win.is_checked());
    });

    check_button_video.set_callback(|win|{
        global_var::set_bool("user::config::check_button_video",win.is_checked());
    });
    
    check_button_thumbnail.set_callback(|win|{
        global_var::set_bool("user::config::check_button_thumbnail",win.is_checked());
    });
    
    check_button_source.set_callback(|win|{
        global_var::set_bool("user::config::check_button_source",win.is_checked());
    });
    
    check_button_the_month.set_callback(|win|{
        global_var::set_bool("user::config::check_button_the_month",win.is_checked());
    });

    check_button_source.set_checked(true);
    check_button_sync.set_checked(true);
    
    global_var::set_bool("user::config::check_button_source",true);
    global_var::set_bool("user::config::check_button_sync",true);

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
    // let mut preview_main2 = Vec::new();

    for index in 0..preview_main_point_list.len() {
        let point = preview_main_point_list[index];
        let [x, y, width, height] = point;

        let mut preview = ImgPreview::new(x, y - 52, width, height, "gui::preview_main::index::");
        let pre: Vec<u8> = if config::is_show_dome() { ASSETS_DEMO_NOT_DATA() } else{ ASSETS_NOT_DATA() };

        preview.from_data(
            pre,
            -1,
            -1,
            width - 2,
            height - 2,
        );

        preview_main.push(preview);
        // preview_main2.push(preview.clone());
        hotspot_list.push(gui_util::hotspot::create_hotspot(x, y - 52, width, height));
    }

    lib::initialize_img_preview_list(&preview_main);

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
            MAIN_CONTOUR,
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

// 用户选择
fn select_user_data_choice() -> menu::Choice {
    let mut choice = menu::Choice::default().with_size(277, 35).center_of_parent().with_label("");
    choice.set_pos(60,158);
    choice.add_choice("请点击 [开始] 获取在线用户列表");
    choice.set_value(0);
    choice.set_color(Color::from_rgb(23, 23, 23));

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

impl AttachThumbnailPreview {
    pub fn gc(&mut self) {
        let pre: Vec<u8> = if config::is_show_dome() { ASSETS_DEMO_NOT_SELECT() } else{ ASSETS_NOT_SELECT() };

        self.thumbnail_preview.from_data(
            pre,
            -1,
            -1,
            self.thumbnail_preview.preview.w()-2,
            self.thumbnail_preview.preview.h()-2,
        );

        self.input_remark.set_value("");
        self.input_attach.set_value("");
        global_var::set_string("user::config::user_select_attach",String::new());
        // self.input_rename.set_value("");

    }
    fn clone(&self) -> Self {
        AttachThumbnailPreview{
            thumbnail_preview: self.thumbnail_preview.clone(),
            btn_remark: self.btn_remark.clone(),
            btn_rename: self.btn_rename.clone(),
            btn_select: self.btn_select.clone(),
            input_rename: self.input_rename.clone(),
            input_remark: self.input_remark.clone(),
            input_attach: self.input_attach.clone(),
        }
    }
    fn redata(&mut self,thumbnail:wh_mod::AttachThumbnail){
        self.input_remark.set_value("");
        self.input_attach.set_value("");
        global_var::set_string("user::config::user_select_attach",String::new());

        let pre: Vec<u8> = if config::is_show_dome() { ASSETS_DEMO_DATA() } else{ thumbnail.thumbnail.to_vec() };

        // self.input_rename.set_value("");
        // 设置预览图
        self.thumbnail_preview.from_data(pre,-1,
                                                       -1,
                                         self.thumbnail_preview.preview.width()-2,
                                         self.thumbnail_preview.preview.height()-2,);
        // 绑定内容
        self.input_attach.set_value(wh_mod::get_show_mask_text( &thumbnail.attach_id).as_str());
        global_var::set_string("user::config::user_select_attach",thumbnail.attach_id.clone());

        let retrieval_struct = wh_mod::wx_parse_path(thumbnail.thumbnail_path.to_string());

        // 获取备注
        if let Some(user_remark) = lib::get_store_user_remark(retrieval_struct.wxid,thumbnail.attach_id.clone()) {
            self.input_remark.set_value(user_remark.as_str())
        }

    }
}

// 用户选定预览的卡片(底部)
fn add_select_attach_card() -> AttachThumbnailPreview {
    let mut preview = gui_util::img::ImgPreview::new(
        41+3, 385,
        82, 82,
        "gui_util::select_user_ui::imag<add_select_attach_card>",
    );
    let pre: Vec<u8> = if config::is_show_dome() { ASSETS_DEMO_NOT_SELECT() } else{ ASSETS_NOT_SELECT() };

    preview.from_data(pre,
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

macro_rules! set_select_user_base_input_default{
    ($input_select_dir:expr)=>{{

        let mut _path = String::new();
        if let Ok(history) = lib::get_wx_user_history_path() {

            let paths = history.path;
             _path = format!("{}",paths.as_str());
            $input_select_dir.set_value(wh_mod::get_show_mask_text(paths.as_str()).as_str());

        }
        if ($input_select_dir.value().is_empty()) {
            if let Some(paths) = wh_mod::convert::get_user_data_path() {
                _path = format!("{}",paths.as_str());
                $input_select_dir.set_value(wh_mod::get_show_mask_text(paths.as_str()).as_str());
            }
        }

        global_var::set_string("user::config::user_select_path",_path);

    }}
}

macro_rules! initialize_window_hwnd{
    ($hwnd:expr)=>{
        if $hwnd==0{
            $hwnd =  libWxIkunPlus::findWindow(THE_WINDOW_CLASS_NAME, "");
            libWxIkunPlus::setWinIcon($hwnd);
            lib::initialize_gc_select_user_ui($hwnd);
            println!("[initialize-window] {} -> {}",THE_WINDOW_CLASS_NAME, $hwnd);
        }
    }
}

struct PreviewData{
    preview_list:Vec<ImgPreview>,
    preview_main: ImgPreview,
}

impl PreviewData {

    pub fn gc_data(&mut self) {
        let pre: Vec<u8> = if config::is_show_dome() { ASSETS_DEMO_NOT_DATA() } else{ ASSETS_NOT_DATA() };

        let mut index = 0;
        for mut preview in self.preview_list.clone() {
            index+=1;
            let mut w = 90;
            if index==1 {
                w+=1;
            }
            if index==4 {
                w+=1;
            }
            if index==6 {
                w+=1;
            }
            preview.from_data(pre.clone(),-1,-1,w ,90 - 2,);
        }
        self.preview_main.from_data(pre , -1, -1, 230-2 , 230 - 2, );
    }

    fn clone(&self) -> Self {
        PreviewData{
            preview_list: self.preview_list.clone(),
            preview_main: self.preview_main.clone(),
        }
    }

    fn redata(&mut self,thumbnail_list:Vec<wh_mod::AttachThumbnail>){
        // self.gc_data();

        if let Some(main_thumbnail) = thumbnail_list.get(0) {
            let pre: Vec<u8> = if config::is_show_dome() { ASSETS_DEMO_DATA() } else{ main_thumbnail.thumbnail.to_vec() };
            self.preview_main.re_data(pre/*, -1, -1, 230-2 , 230 - 2, */);

        }

        let mut index = 0;
        for mut preview in self.preview_list.clone() {
            index+=1;
            // let mut w = 90;
            // // 第一列有1px的误差
            // if index==1 ||index==4||index==7{
            //     w+=1;
            // }

            if let Some(thumbnail) = thumbnail_list.get(index) {
                let pre: Vec<u8> = if config::is_show_dome() { ASSETS_DEMO_DATA() } else{ thumbnail.thumbnail.to_vec() };

                preview.re_data(pre/*,-1,-1,w ,90 - 2,*/);

            }else {
                let pre: Vec<u8> = if config::is_show_dome() { ASSETS_DEMO_NOT_DATA() } else{ ASSETS_NOT_DATA() };
                preview.re_data(pre/*,-1,-1,w ,90 - 2,*/);
            }

        }

    }

    pub fn existPoint(&self, x: i32, y: i32) -> bool {
        let preview_pint = [
            [35,280,85,85],[35,381,85,85],[35,482,85,85],
            [137,280,85,85],[137,381,85,85],[137,482,85,85],
            [239,280,85,85],[239,381,85,85],[239,482,85,85]
        ];

        for preview in preview_pint {
            if
            x > preview[1]
                && x < preview[1] + 85
                && y > preview[0]
                && y < preview[0] + 85
            {
                return  true
            }
        }
        false
    }

    pub fn existPointIndex(&self, x: i32, y: i32) -> usize {
        let mut index = 0 ;
        let preview_pint = [
            [35,280,85,85],[35,381,85,85],[35,482,85,85],
            [137,280,85,85],[137,381,85,85],[137,482,85,85],
            [239,280,85,85],[239,381,85,85],[239,482,85,85]
        ];

        for preview in preview_pint {
            index+=1;

            if
            x > preview[1]
                && x < preview[1] + 85
                && y > preview[0]
                && y < preview[0] + 85
            {
                return  index
            }

        }
        0
    }
}

// 九宫格
fn preview_main_list() ->PreviewData{

    let mut preview_main = gui_util::img::ImgPreview::new2(35,35,230,230,"",-1, -1, 230-2 , 230 - 2);

    // 九宫格位置预设
    let preview_pint = [
        [[35,280,85,85],[35,381,85,85],[35,482,85,85]],
        [[137,280,85,85],[137,381,85,85],[137,482,85,85]],
        [[239,280,85,85],[239,381,85,85],[239,482,85,85]]
    ];

    let mut flex = group::Flex::default()
        .with_size(287, 90)
        .row()
        .center_of_parent();

    flex.set_pos(280, 35);

    let mut preview_main_1 = gui_util::img::ImgPreview::new2(0,0,85,85,"",-1,-1,91 ,90 - 2);
    let mut preview_main_2 = gui_util::img::ImgPreview::new2(0,0,85,85,"",-1,-1,90 ,90 - 2);
    let mut preview_main_3 = gui_util::img::ImgPreview::new2(0,0,85,85,"",-1,-1,90 ,90 - 2);
    flex.end();

    let mut flex = group::Flex::default()
        .with_size(287, 90)
        .row()
        .center_of_parent();

    flex.set_pos(280, 137);

    let mut preview_main_4 = gui_util::img::ImgPreview::new2(0,0,85,85,"",-1,-1,91 ,90 - 2);
    let mut preview_main_5 = gui_util::img::ImgPreview::new2(0,0,85,85,"",-1,-1,90 ,90 - 2);
    let mut preview_main_6 = gui_util::img::ImgPreview::new2(0,0,85,85,"",-1,-1,90 ,90 - 2);
    flex.end();

    let mut flex = group::Flex::default()
        .with_size(287, 90)
        .row()
        .center_of_parent();

    flex.set_pos(280, 239);

    let mut preview_main_7 = gui_util::img::ImgPreview::new2(0,0,85,85,"",-1,-1,91 ,90 - 2);
    let mut preview_main_8 = gui_util::img::ImgPreview::new2(0,0,85,85,"",-1,-1,90 ,90 - 2);
    let mut preview_main_9 = gui_util::img::ImgPreview::new2(0,0,85,85,"",-1,-1,90 ,90 - 2);

    flex.end();

   let result = vec![preview_main_1,preview_main_2,preview_main_3,preview_main_4,preview_main_5,preview_main_6,preview_main_7,preview_main_8,preview_main_9];

   let mut  preview_data =  PreviewData{
        preview_list: result,
        preview_main
    };

    preview_data.gc_data();

    preview_data
}

struct ThumbnailPreviewMain{
    main:DoubleWindow,
    btn_close: gui_util::hotspot::HotspotItmeControl,
    preview_list: PreviewData,
}

// 图片预览窗口 九宫格
fn add_frame_thumbnail_preview_main (mut win: &DoubleWindow) -> ThumbnailPreviewMain {
    // 图片预览窗口
    let mut preview_win = fltk::window::Window::new(0,0,win.w(),359,"");
    preview_win.set_color(Color::from_rgb(23, 23, 23));
    set_item_id!(preview_win,THE_SUB_WINDOW_CLASS_NAME_FRAME_THUMBNAIL_PREVIEW);

    let mut preview_win_border = gui_util::img::ImgPreview::new(0,0,preview_win.w(),preview_win.h(),"");
    preview_win_border.from_svg(include_str!("./src/preview_win.svg"),0,0,preview_win.w(),preview_win.h());

    let mut preview_main_close_btn = gui_util::hotspot::create_hotspot(82,282,130,32);
    let mut preview_main_list = preview_main_list();
    gui_util::text::TextControl::new(82,283,130,32,13,"关闭预览",[121, 121, 121]);

    preview_win.end();
    preview_win.hide();

    ThumbnailPreviewMain{
        main:preview_win,
        btn_close:preview_main_close_btn,
        preview_list:preview_main_list,

    }
}

// 通过扫描获取的界面
struct ScanPreviewMain{
    main:fltk::window::DoubleWindow,
    btn_all_obj: gui_util::hotspot::HotspotItmeControl,
    btn_clip:  gui_util::hotspot::HotspotItmeControl,
    btn_close: gui_util::hotspot::HotspotItmeControl,
    text_bottom:Vec<gui_util::text::TextControl>,
    progress_bar: ImgPreview,
    all_text_list: Vec<TextControl>,
    btn_scan_drag: gui_util::hotspot::HotspotItmeControl,
}

impl ScanPreviewMain {
    pub fn existPoint(&self, x: i32, y: i32) -> bool {
        let self_x = self.main.x();
        let self_y = self.main.y();
        let self_w = self.main.w();
        let self_h = self.main.h();

        return x > self_x
            && x < self_x + self_w
            && y > self_y
            && y < self_y + self_h;
    }

    pub fn show_progress(& mut self){
        for (mut index) in 0..3 {
            if let Some(text_bottom) = self.text_bottom.get_mut(index) {
                text_bottom.text.hide();
            }
        }
        self.progress_bar.preview.show();
    }

    pub fn show_bottom_text(& mut self){
        for (mut index) in 0..3 {
            if let Some(text_bottom) = self.text_bottom.get_mut(index) {
                text_bottom.text.show();
            }
        }
        self.progress_bar.preview.hide();
    }
}

// 進度條
fn show_progress_bar_border(x: i32, y: i32) -> gui_util::img::ImgPreview {
    let mut progress_bar_border = gui_util::img::ImgPreview::new_border(x,y,520,15,"<svg width=\"520\" height=\"15\" viewBox=\"0 0 520 15\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\"> <rect x=\"0.5\" y=\"0.5\" width=\"520\" height=\"14\" rx=\"7\" fill=\"#181818\" stroke=\"#2C2C2C\"/> </svg> ");
    let width = progress_bar_border.width;
    let mut progress_bar_border_slider = gui_util::img::ImgPreview::new_border(x, y, 41, 15, "<svg width=\"40\" height=\"15\" viewBox=\"0 0 40 15\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\">\n<rect x=\"0.5\" y=\"0.5\" width=\"39\" height=\"14\" rx=\"7\" fill=\"#333333\" stroke=\"#2C2C2C\"/>\n</svg>");
    progress_bar_border_slider.preview.hide();
    progress_bar_border.preview.hide();

    progress_bar_border.preview.handle({
        let mut progress_bar_border_slider = progress_bar_border_slider.clone();
        let mut preview_main = progress_bar_border.preview.clone();

        move |win, ev| match ev {
            enums::Event::Show => {
                progress_bar_border_slider.preview.show();

                let mut preview = progress_bar_border_slider.preview.clone();
                let mut preview_main = preview_main.clone();

                app::add_timeout3(0.0086, move |handle| {
                    if !preview.visible()||!preview_main.visible() {
                        preview_main.hide();
                        preview.hide();
                        app::remove_timeout3(handle);
                        return;
                    }

                    let mut the_x = preview.x() + 2;
                    if the_x > width {
                        the_x = x + preview.width();
                        the_x -= preview.width();
                    }
                    preview.set_pos(the_x, preview.y());
                    preview.parent().unwrap().redraw();
                    app::repeat_timeout3(0.0086, handle);
                });
                true
            }
            enums::Event::Hide => {
                progress_bar_border_slider.preview.hide();
                true
            }
            _ => false,
        }
    });

    // progress_bar_border.preview.show();
    progress_bar_border
}

// 通过扫描获取的界面
fn add_scan_preview_window() -> ScanPreviewMain {
    // 图片预览窗口
    let mut preview_win = fltk::window::Window::new(0,0,600,359,"");
    preview_win.set_color(Color::from_rgb(23, 23, 23));
    set_item_id!(preview_win,THE_SUB_WINDOW_CLASS_NAME_SCAN);

    let mut preview_win_border = gui_util::img::ImgPreview::new(0,0,preview_win.w(),preview_win.h(),"");
    preview_win_border.from_svg(include_str!("./src/scan_cursor.svg"),0,0,preview_win.w(),preview_win.h());

    let mut show_all_user_obj = gui_util::hotspot::create_hotspot(83,231,87,40);
    let mut get_clip_data = gui_util::hotspot::create_hotspot(432,231,87,40);
    let mut scan_drag_data = gui_util::hotspot::create_hotspot(256,231,87,40);
    let mut btn_close = gui_util::hotspot::create_hotspot(537,33,25,25);

    let mut text_list = Vec::new();

    // 標題
    text_list.push(gui_util::text::TextControl::new(150-25,33,345,15,12,"请选择一种您喜欢的方式扫描聊天对象",[149, 149, 149]));
    text_list.push(gui_util::text::TextControl::new(130,58,345,15,12,"如果您有很多好友需要动态管理可以设置别名 在所有好友中可显示别名备注",[68, 68, 68]));

    //  卡片中上
    text_list.push(gui_util::text::TextControl::new(57,170+3 ,135,15,12,"所有存在图片的聊天对象",[149, 149, 149]));
    text_list.push(gui_util::text::TextControl::new(257,170+3 ,85,15,12,"拖拽聊天的图片",[149, 149, 149]));
    text_list.push(gui_util::text::TextControl::new(442,170+3 ,64,15,12,"剪贴板获取",[149, 149, 149]));

    // 卡片中下
    text_list.push(gui_util::text::TextControl::new(87,196+5,78,15,13,"查看所有人",[121, 121, 121]));
    text_list.push(gui_util::text::TextControl::new(255,196+5,91,15,13,"打开扫描窗口",[121, 121, 121]));
    text_list.push(gui_util::text::TextControl::new(410,198+5,135,15,12,"复制一张图片后点击开始",[121, 121, 121]));

    // 卡片下面
    text_list.push(gui_util::text::TextControl::new(111,239+5,31,15,13,"开始",[255, 255, 255]));
    text_list.push(gui_util::text::TextControl::new(256+(460-432),239+5,31,15,13,"开始",[255, 255, 255]));
    text_list.push(gui_util::text::TextControl::new(460,239+5,31,15,13,"开始",[255, 255, 255]));

    // 底部三言 / 進度條
    let mut text_01 = gui_util::text::TextControl::new(70,308,115,15,13,"聊天对象选择面板",[195, 195, 195]);
    let mut text_02 = gui_util::text::TextControl::new(260,308,85,15,13,"通过拖拽查找",[195, 195, 195]);
    let mut text_03 = gui_util::text::TextControl::new(435,308,85,15,13,"粘贴文件查找",[195, 195, 195]);

    let mut progress_bar_border = show_progress_bar_border(40,312);

    preview_win.end();
    preview_win.hide();

    preview_win.handle({
        let mut preview = progress_bar_border.preview.clone();
        move |win, ev| match ev {
            enums::Event::Show=>{
                true
            }
            enums::Event::Hide=>{
                preview.hide();
                true
            }
            _ => false
        }

    });

    ScanPreviewMain{
        main:preview_win,
        btn_clip:get_clip_data,
        btn_all_obj:show_all_user_obj,
        btn_scan_drag:scan_drag_data,
        btn_close,
        all_text_list:text_list,
        text_bottom: vec![text_01,text_02,text_03],
        progress_bar:progress_bar_border
    }
}

fn initialize_watch_walk_drag_path (mut preview1: AttachThumbnailPreview) {
    let mut oid_walk_drag_path = String::new();
    std::thread::spawn( move || loop{
        if !has_window(){
            lib::gc_select_user_ui();
            return;
        }

        let walk_drag_path = global_var::get_string_default("user::config::walk_drag_path");
        if(!oid_walk_drag_path.as_bytes().eq(walk_drag_path.as_bytes())){
            oid_walk_drag_path.clear();
            oid_walk_drag_path.push_str(walk_drag_path.as_str());
            let wx_parse = wh_mod::wx_parse_path(walk_drag_path.clone());
            // println!("wx_parse-> {:?}",&wx_parse);
            preview1.input_attach.set_value(wh_mod::get_show_mask_text(&wx_parse.attach_id).as_str());
            global_var::set_string("user::config::user_select_attach",format!("{}",wx_parse.attach_id.as_str()));

            if let Some(remark) = lib::get_store_user_remark(wx_parse.wxid,wx_parse.attach_id.clone()) {
                preview1.input_remark.set_value(remark.as_str());
            }else {
                preview1.input_remark.set_value("");
            }

            if let Ok(buff_thumbnail_data) = wh_mod::convert::convert_dat_images_buff(std::path::PathBuf::from(walk_drag_path.as_str())) {
                let pre: Vec<u8> = if config::is_show_dome() { ASSETS_DEMO_DATA() } else{ buff_thumbnail_data };

                preview1.thumbnail_preview.from_data(pre,-1, -1, 80, 80);
            }

            // println!("walk_drag_path->{}",&walk_drag_path);
        }

        if !gui_drag_scan2_ui::has_window() {
            return;
        }

        Sleep(500);
    });
}

macro_rules! the_token {
    ()=>{
       {
        let mut _the_token =format!("[select_user_ui]token<{}>@query",libWxIkunPlus::randomNum());
        loop{
            if global_var::has_string(_the_token.as_str()) {
                _the_token = format!("[select_user_ui]token<{}>@query",libWxIkunPlus::randomNum());
            }else{
                break;
            }
        }
            _the_token
        }
    }
}

pub fn manage_tool_main() -> String{
    let the_token = the_token!();
    // 禁止创建多个窗口
    if let hwnd = libWxIkunPlus::findWindow(THE_WINDOW_CLASS_NAME, "") {
        if hwnd!=0 && libWxIkunPlus::isWindow(hwnd) {
            if let Some(mut win) =app::widget_from_id(THE_WINDOW_CLASS_NAME) as Option<DoubleWindow>
             {
                 win.show();
                 win.set_visible_focus();
             }
             libWxIkunPlus::setWindowShake(hwnd);
            global_var::set_string(the_token.as_str(),String::new());
            return the_token;
        }
    }

    let mut hwnd :i128 = 0;
    let mut win = window::DoubleWindow::new(0, 0,600, 595,None).center_screen();
    win.set_color(Color::from_rgb(23, 23, 23));

    inject_fltk_theme!();
    win.set_label("任务创建向导");
    set_item_id!(win, THE_WINDOW_CLASS_NAME);
    
    let mut g_the_select_wxid = String::new();
    let mut g_the_select_attach_id = String::new();
    let mut preview_win =add_frame_thumbnail_preview_main(&win);
    let mut scan_preview_window = add_scan_preview_window();

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

    // 文件的默认保存位置(D:\...\...\WeChat Files)
    let mut user_select_database_dir_input = input::Input::new(45+3, 74, 395, 30, "");
    set_select_user_base_input_default!(user_select_database_dir_input);

    // 按钮 > 打开文件选择
    let mut button_open_dir = gui_util::hotspot::create_hotspot(516, 73 , 33, 32);
    // 按钮 > 扫描获取
    let mut button_show_drag = gui_util::hotspot::create_hotspot(346, 156 , 123, 38);
   
    // 按钮 > 帮助
    let mut button_show_help = gui_util::hotspot::create_hotspot(479, 156 , 66, 38);
    // 按钮 > 开始
    let mut button_start = gui_util::hotspot::create_hotspot(451, 73 , 57, 32);

    select_attach_card.input_rename.set_value("<创建月>/<任务名>/<类型>_<NN>");

    let mut move_select_attach_card = select_attach_card.clone();

    select_user_data_choice.set_callback(move |c| {
        if let Some(choice_value) = c.choice() {
            
            if let Some(item) = lib::get_active_user_list().get(c.value() as usize).clone() {
            move_select_attach_card.gc();
            c.deactivate();

            lib::store_wx_user_path_history(item.user_root.clone(),item.user_wxid.clone());
            
            println!("[{}] {}  , {}  ,  {} ",c.value(), choice_value.as_str() ,&item.user_root,&item.user_wxid);
            
            global_var::set_string("user::config::user_select_path", item.user_root.clone());
            global_var::set_string("user::config::user_select_wxid", item.user_wxid.clone());

            lib::initialize_thumbnail_preview(item.user_root.as_str(),item.user_wxid.as_str());
            lib::initialize_watch_path_puppet(format!("{}\\{}", &item.user_root, &item.user_wxid));

            c.activate();
            }
            
            
        }
    });

    let mut drag_path = std::path::PathBuf::new();



    win.end();
    win.show();
    // preview_win.show();

    win.handle({
        let mut x = 0;
        let mut y = 0;
        // 是否显示手型
        let mut show_cursor = false;
        let mut preview_win_show = false;
        let mut scan_win_show = false;

        let mut released = true;
        let mut dnd = true;
        let the_token =the_token.clone();
        let move_select_attach_card2 = select_attach_card.clone();

        move |win, ev| match ev {

            enums::Event::Focus=>{
                initialize_window_hwnd!(hwnd);
                true
            }

            enums::Event::Show => {
                initialize_window_hwnd!(hwnd);
                true
            }

            enums::Event::KeyUp => true,

            enums::Event::Close=>{
                lib::gc_select_user_ui();
                true
            }

            enums::Event::Push => {

                macro_rules! add_preview_win_show {
                    ()=>{
                        preview_win_show = true;
                        user_select_database_dir_input.hide();
                        select_user_data_choice.hide();
                        let path = PathBuf::from(global_var::get_string_default("user::config::user_select_path"))
                            .join(global_var::get_string_default("user::config::user_select_wxid"))
                            .join("FileStorage\\MsgAttach")
                            .join(g_the_select_attach_id.as_str())
                            ;
                        let dat  = wh_mod::read_attach_buff_thumbnail_data(&path,10);
                        println!("dat<{}> path:<{:?}>",dat.len(),&path);
                        preview_win.preview_list.redata(dat);
                        preview_win.main.show();
                    }
                }
                if preview_win_show {
                    if preview_win.btn_close.existPoint(x,y) {
                        preview_win_show =false;
                        preview_win.main.hide();
                        user_select_database_dir_input.show();
                        select_user_data_choice.show();
                        preview_win.preview_list.gc_data();
                    }

                }
                else if scan_win_show {

                    // 关闭扫描窗口
                    if scan_preview_window.btn_close.existPoint(x,y){
                        scan_preview_window.main.hide();
                        // user_select_database_dir_input.activate();
                        // select_attach_card.input_rename.activate();
                        // select_attach_card.input_remark.activate();
                        scan_preview_window.show_bottom_text();
                        scan_win_show = false;
                    }

                    if !scan_preview_window.progress_bar.preview.visible(){
                        if scan_preview_window.btn_all_obj.existPoint(x,y) {
                            gui_util::message::sub_message(hwnd,gui_util::message::IconType::Warning,"作者正在玩命的开发中。。。",3500u64);
                        }
                        if scan_preview_window.btn_scan_drag.existPoint(x,y) {
                            if global_var::get_string_default("user::config::user_select_wxid").is_empty() {
                                gui_util::message::sub_message(hwnd,gui_util::message::IconType::Warning,"尚未选择用户",3500u64);
                                return false;
                            }
                            if global_var::get_string_default("user::config::user_select_path").is_empty() {
                                gui_util::message::sub_message(hwnd,gui_util::message::IconType::Warning,"没有选择WX根目录",3500u64);
                                return false;
                            }
                            // gui_drag_scan::main_window();
                            gui_drag_scan2_ui::main_window("");
                            initialize_watch_walk_drag_path (move_select_attach_card2.clone());
                        }

                        if  scan_preview_window.btn_clip.existPoint(x,y) {
                            let clip_file_path_single = libWxIkunPlus::getClipFilePathSingle();
                            if clip_file_path_single.is_empty() {
                                gui_util::message::sub_message(hwnd,gui_util::message::IconType::Warning,"当前剪贴板无内容或者不是可识别格式",3500u64);
                            }else{

                                if global_var::get_string_default("user::config::user_select_wxid").is_empty() {
                                    gui_util::message::sub_message(hwnd,gui_util::message::IconType::Warning,"尚未选择用户",3500u64);
                                    return false;
                                }
                                if global_var::get_string_default("user::config::user_select_path").is_empty() {
                                    gui_util::message::sub_message(hwnd,gui_util::message::IconType::Warning,"没有选择WX根目录",3500u64);
                                    return false;
                                }

                                // scan_preview_window.show_progress();
                                drag_path.clear();
                                drag_path.push(clip_file_path_single.clone());
                                println!("getClipFilePathSingle-> {:?}",&drag_path);

                                // 获取id 并开始处理
                                let temp_imag_id = get_wx_temp_imag_id(clip_file_path_single.as_str());

                                if !temp_imag_id.is_empty() {
                                    gui_drag_scan2_ui::main_window(clip_file_path_single.as_str());
                                    initialize_watch_walk_drag_path (move_select_attach_card2.clone());
                                }

                            }

                        }

                    }else{
                        gui_util::message::sub_message(hwnd,gui_util::message::IconType::Failure,"当前正在扫描中 不能使用此功能",3500u64);
                    }
                }
                // 正常窗口
                else {
                    // 选择最近5个对象
                    {
                        let mut index = 0;
                        for hotspot in &frame_thumbnail_preview.hotspot_list {
                            index += 1;
                            let thumbnail_list = lib::get_thumbnail_list();
                            if hotspot.existPoint(x, y) {
                                select_attach_card.gc();

                                if let Some(thumbnail) = thumbnail_list.get(index - 1usize) {
                                    println!("[click] frame_thumbnail_preview -> {}", index);
                                    global_var::set_i32("user::config::select_user_thumbnail_obj", index as i32);
                                    select_attach_card.redata(thumbnail.clone());
                                    g_the_select_attach_id.clear();
                                    g_the_select_attach_id.push_str(thumbnail.attach_id.as_str());

                                    if config::is_click_open_preview() {
                                        add_preview_win_show!();
                                    }
                                    break;
                                } else {
                                    gui_util::message::sub_message(hwnd, gui_util::message::IconType::Warning, "没有选中聊天对象", 3500u64);
                                }
                            }
                        }
                    }

                    // 打开文件夹选择器
                    if button_open_dir.existPoint(x, y) {
                        let mut select_dir = libWxIkunPlus::openSelectFolder2();
                        
                        if !select_dir.is_empty() {
                            let wx_search_store_root =  wh_mod::wx_search_store_root(&select_dir);                        
                            if wx_search_store_root.is_empty() {
                                gui_util::message::sub_message(libWxIkunPlus::findWindow(THE_WINDOW_CLASS_NAME, ""), gui_util::message::IconType::Failure, "此路径下未找到有效wxid文件夹", 3500u64);
                                return false;
                            }
                            user_select_database_dir_input.set_value(wh_mod::get_show_mask_text(select_dir.as_str()).as_str());

                            select_user_data_choice.clear();
                            select_user_data_choice.add_choice("请点击 [开始] 获取在线用户列表");
                            select_user_data_choice.set_value(0);
                            lib::set_active_user_list(Vec::new());
                            global_var::set_string("user::config::user_select_path", select_dir.clone());

                        } else {
                            gui_util::message::sub_message(libWxIkunPlus::findWindow(THE_WINDOW_CLASS_NAME, ""), gui_util::message::IconType::Info, "用户取消", 3500u64);
                        }
                    }

                    // 显示帮助面板
                    if button_show_help.existPoint(x, y) {
                        println!("[click] existPoint {}", "");
                        gui_detect_config_ui::main_window();
                    }

                    // 图片预览大图
                    if select_attach_card.thumbnail_preview.existPoint(x, y) {

                        let attach_id = global_var::get_string_default("user::config::user_select_attach");//select_attach_card.input_attach.value();
                        if attach_id.is_empty() {
                            // dialog::alert_default("没有选择聊天对象 (attach ID)");
                            gui_util::message::sub_message(hwnd, gui_util::message::IconType::Warning, "没有选择聊天对象 (attach ID)", 3500u64);

                            return false;
                        }

                        add_preview_win_show!();

                    }

                    // 开始
                    if button_start.existPoint(x, y)||(button_show_drag.existPoint(x, y)&&
                        (
                            global_var::get_string_default("user::config::user_select_wxid").is_empty()||
                            user_select_database_dir_input.value().is_empty()
                        )
                    ) {
                        select_attach_card.gc();

                        if !user_select_database_dir_input.value().is_empty() {
                            if (!libWxIkunPlus::hasWeChat() || !libWxIkunPlus::hasWeChatWin()) {
                                // fltk::dialog::alert_default("尚未找到已经登录的WX进程 为避免滥用 扫描被拒绝 ");
                                gui_util::message::sub_message(hwnd, gui_util::message::IconType::Failure, "WX未登录 为避免滥用 面板开启请求被拒绝", 3500u64);
                                return false;
                            }

                            // 释放资源
                            select_user_data_choice.activate();
                            lib::set_active_user_list(Vec::new());

                            select_user_data_choice.clear();
                            select_user_data_choice.add_choice("【状态】  当前正在扫描用户列表... ");
                            select_user_data_choice.set_value(0);

                            let mut user_select_database = global_var::get_string_default("user::config::user_select_path");
                            let mut user_select_database_input = user_select_database_dir_input.value();

                            // 没有* 则引用input内容 并重新设置input消敏
                            if !user_select_database_input.contains("*") {
                                let mut new_path = PathBuf::from(user_select_database_input.as_str());

                                if !user_select_database_input.contains("WeChat Files") {
                                    let new_data = new_path.clone().join("WeChat Files");
                                    if new_data.exists() {
                                        new_path = new_data;
                                    }
                                }

                                if(new_path.exists()){
                                    user_select_database = util::to_string_default(&new_path);
                                    global_var::set_string("user::config::user_select_path",user_select_database.to_string());

                                    user_select_database_dir_input.set_value(wh_mod::get_show_mask_text(&user_select_database).as_str());

                                }
                            }

                            if !user_select_database.is_empty() {
                                let active_user_list = wh_mod::convert::get_active_user(user_select_database.as_str());
                                select_user_data_choice.clear();

                                if active_user_list.is_empty() {
                                    select_user_data_choice.add_choice("【状态】  未找到用户列表");
                                    select_user_data_choice.set_value(0);
                                    gui_util::message::sub_message(libWxIkunPlus::findWindow(THE_WINDOW_CLASS_NAME, ""), gui_util::message::IconType::Failure, "未找到用户列表 请注意路径有效性", 3500u64);
                                    return false;
                                }


                                // 添加到列表
                                for active_user in active_user_list {
                                    if let Some(accinfo) = active_user.accinfo.clone() {
                                        select_user_data_choice.add_choice(format!("{} <{}>", wh_mod::get_show_mask_text(&active_user.user_wxid), wh_mod::get_show_mask_text(&accinfo.name)).as_str());
                                    } else {
                                        select_user_data_choice.add_choice( wh_mod::get_show_mask_text(&active_user.user_data).as_str());
                                    }

                                    lib::push_active_user_list(active_user.clone());
                                }

                                // 开始扫描
                                if let Some(item) = lib::get_active_user_list().get(0) {
                                    select_user_data_choice.set_value(0);
                                    lib::store_wx_user_path_history(item.user_root.clone(), item.user_wxid.clone());
                                    global_var::set_string("user::config::user_select_path", item.user_root.clone());
                                    global_var::set_string("user::config::user_select_wxid", item.user_wxid.clone());
                                    lib::initialize_thumbnail_preview(item.user_root.as_str(), item.user_wxid.as_str());
                                    lib::initialize_watch_path_puppet(format!("{}\\{}", &item.user_root, &item.user_wxid));
                                }
                            }
                        } else {
                            // dialog::alert_default("没有选择WX文件默认保存位置");
                            gui_util::message::sub_message(hwnd, gui_util::message::IconType::Warning, "没有选择WX文件默认保存位置", 3500u64);
                        }
                    }

                    // 显示扫描获取面板
                    if button_show_drag.existPoint(x, y) {
                        println!("[click] existPoint {}", "显示扫描获取面板");

                        if (!libWxIkunPlus::hasWeChat() || !libWxIkunPlus::hasWeChatWin()) {
                            gui_util::message::sub_message(hwnd, gui_util::message::IconType::Failure, "WX未登录 为避免滥用 面板开启请求被拒绝", 3500u64);
                            // fltk::dialog::alert_default("尚未找到已经登录的WX进程 为避免滥用 面板开启请求被拒绝 ");
                            return false;
                        }

                        if global_var::get_string_default("user::config::user_select_wxid").is_empty() {
                            gui_util::message::sub_message(hwnd,gui_util::message::IconType::Warning,"尚未选择用户",3500u64);
                            return false;
                        }
                        if global_var::get_string_default("user::config::user_select_path").is_empty() {
                            gui_util::message::sub_message(hwnd,gui_util::message::IconType::Warning,"没有选择WX根目录",3500u64);
                            return false;
                        }

                        // gui_drag_scan::main_window();


                        scan_preview_window.main.show();
                        // libWxIkunPlus::setWindowEnabled(libWxIkunPlus::findWindow(THE_SUB_WINDOW_CLASS_NAME_SCAN,""),true);
                        // user_select_database_dir_input.deactivate();
                        // select_attach_card.input_rename.deactivate();
                        // select_attach_card.input_remark.deactivate();
                        // let mut scan_drag_window = show_scan_drag_window();
                        // scan_drag_window.hide();
                        scan_win_show = true;
                    }

                }

                if g_the_select_attach_id.is_empty() {
                    g_the_select_attach_id.push_str(global_var::get_string_default("user::config::user_select_attach").as_str());
                }

                // 卡片按钮 > 完成选定
                if select_attach_card.btn_select.existPoint(x, y) {
                    println!("[click] existPoint {}", "卡片按钮 > 完成选定");

                    let mut result_data = String::new();
                    let mut is_effective = true;

                    let mut rename_rule = select_attach_card.input_rename.value();

                    // 没有选定的路径
                    if user_select_database_dir_input.value().is_empty() {
                        fltk::dialog::alert_default("没有选定Wx路径");
                        is_effective = false;
                        return false;
                    }
                    // 没有选定WX用户
                    if global_var::get_string_default("user::config::user_select_wxid").is_empty() {
                        fltk::dialog::alert_default("没有选定WX用户");
                        is_effective = false;
                        return false;
                    }

                    //  判断是否有Att id
                    else if g_the_select_attach_id.is_empty()  {
                        // fltk::dialog::alert_default("attach id 无效 （尚未选定有效聊天对象）");
                        // gui_util::message::message(x+100,y+80,gui_util::message::IconType::Warning,"attach id 无效 （尚未选定有效聊天对象）",3500u64);
                        gui_util::message::sub_message(hwnd, gui_util::message::IconType::Warning, "attach id 无效 （尚未选定有效聊天对象）", 3500u64);
                        is_effective = false;
                        return false;
                    }
                    // 有命名规则 要求规则最少有一个%N.. 自变量
                    if !rename_rule.is_empty() && (!rename_rule.contains("<N") || !rename_rule.contains("N>")) {
                        rename_rule.push_str("<NN>");
                    }
                    let mut select_dir = global_var::get_string_default("user::config::user_select_path");
                    let mut user_select_wxid = global_var::get_string_default("user::config::user_select_wxid");
                    let eq_wxid_dir = wh_mod::get_wx_user_store(format!("{}/{}",&select_dir,&user_select_wxid)).is_some();

                    // 拼合路径并判断有效性 有且为文件夹
                    let mut attach_path = PathBuf::from(select_dir).join(user_select_wxid.as_str()).join("FileStorage\\MsgAttach").join(g_the_select_attach_id.as_str());

                    println!("attach_path=> {:?}", &attach_path);

                    if let Some(attach_path_str) = attach_path.to_str() {
                        result_data.push_str(attach_path_str);
                        // 识标
                        if frame_check.thumbnail.is_checked() {
                            result_data.push_str("*wizards");
                        }
                        // 可选项
                        if frame_check.thumbnail.is_checked() {
                            result_data.push_str("*thumbnail");
                        }
                        if frame_check.source.is_checked() {
                            result_data.push_str("*source");
                        }
                        if frame_check.video.is_checked() {
                            result_data.push_str("*video");
                        }
                        if frame_check.sync.is_checked() {
                            result_data.push_str("*Sync");
                        }
                        if frame_check.the_month.is_checked() {
                            result_data.push_str("*the_month");
                        }
                        // 添加名称格式化自变量
                        if !select_attach_card.input_rename.value().is_empty() {
                            result_data.push_str(format!("*rename_rule={}*", &rename_rule).as_str());
                        }

                    }else{
                        gui_util::message::sub_message(hwnd, gui_util::message::IconType::Warning, "路径转义失败 错误代码[3061]", 3500u64);
                        return false;
                    }


                    if !attach_path.exists() && !attach_path.exists() {

                        // dialog::alert_default("attach 目录无效");
                        gui_util::message::sub_message(hwnd, gui_util::message::IconType::Warning, "attach目录无效 <聊天对象目录无法打开>", 3500u64);
                        is_effective = false;
                        return false;
                    }

                    if is_effective && eq_wxid_dir {
                        println!("result_data->{}",&result_data);
                        global_var::set_string(the_token.as_str(),result_data);
                        lib::gc_select_user_ui();
                        fltk::window::Window::delete(win.clone());
                    }
                }

                // 卡片按钮 > 备注名称 完成按钮
                if select_attach_card.btn_remark.existPoint(x, y) {
                    println!("[click] existPoint {}", "卡片按钮 > 备注名称 完成按钮");

                    let wxid = global_var::get_string_default("user::config::user_select_wxid");
                    let attach_id = global_var::get_string_default("user::config::user_select_attach");//select_attach_card.input_attach.value();
                    let remark_name = select_attach_card.input_remark.value();

                    if wxid.is_empty() {
                        // dialog::alert_default("没有选择用户 (WXID)");
                        gui_util::message::sub_message(hwnd, gui_util::message::IconType::Warning, "没有选择用户 (WXID)", 3500u64);
                        return false;
                    }

                    if attach_id.is_empty() {
                        // dialog::alert_default("没有选择聊天对象 (attach ID)");
                        gui_util::message::sub_message(hwnd, gui_util::message::IconType::Warning, "没有选择聊天对象 (attach ID)", 3500u64);

                        return false;
                    }

                    if remark_name.is_empty() {
                        // dialog::alert_default("没有备注内容 (备注将用于命名与显示对象名称)");
                        gui_util::message::sub_message(hwnd, gui_util::message::IconType::Warning, "没有备注内容(用于命名与显示对象)", 3500u64);

                        return false;
                    }

                    lib::set_store_user_remark(wxid, attach_id, remark_name);
                }


                // 卡片按钮 > 编辑命名规则
                if select_attach_card.btn_rename.existPoint(x, y) {
                    println!("[click] existPoint {}", "卡片按钮 > 编辑命名规则");
                   let mut rename_token = rename_tool_main(select_attach_card.input_rename.value().as_str());
                    let mut input_rename = select_attach_card.input_rename.clone();

                    app::add_timeout3(0.3,move|handle|{
                        if global_var::has_string(rename_token.as_str()) {
                            let data = global_var::get_string_default(rename_token.as_str());
                            if data.is_empty() {
                                println!("{} 用户取消 data-> [{}]",&rename_token,&data);
                                gui_util::message::sub_message(get_the_hwnd!(),gui_util::message::IconType::Warning,"用户取消处理",3500u64);
                            }else{
                                if !input_rename.value().as_bytes().eq(data.as_bytes()) {
                                    input_rename.set_value(data.as_str());
                                    println!("{} 名称更新 data-> [{}]",&rename_token,&data);
                                    gui_util::message::sub_message(get_the_hwnd!(),gui_util::message::IconType::Success,"名称已成功更新",3500u64);
                                }else {
                                    println!("{} 没有需要更新的名称内容 data-> [{}]",&rename_token,&data);
                                    gui_util::message::sub_message(get_the_hwnd!(),gui_util::message::IconType::Info,"名称内容没有变化",3500u64);
                                }
                            }
                            global_var::set_string(rename_token.as_str(),String::new());
                            app::remove_timeout3(handle);
                        }else {
                            app::repeat_timeout3(0.3, handle);
                        }
                    });

                }

                true
            }

            enums::Event::Move => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;
                let mut new_show_cursor = false;

                initialize_window_hwnd!(hwnd);

                // 启用了预览图片模式
                if preview_win_show {

                    let mut new_show_cursor = false;

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

                    if preview_win.btn_close.existPoint(x,y)||new_show_cursor{
                        win.set_cursor(fltk::enums::Cursor::Hand);
                    } else {
                        win.set_cursor(fltk::enums::Cursor::Default);
                    }

                }
                else if scan_win_show {

                    if scan_preview_window.btn_clip.existPoint(x,y)
                     ||scan_preview_window.btn_scan_drag.existPoint(x,y)
                     ||scan_preview_window.btn_all_obj.existPoint(x,y)
                     ||scan_preview_window.btn_close.existPoint(x,y)
                    {
                        win.set_cursor(fltk::enums::Cursor::Hand);
                    } else {
                        win.set_cursor(fltk::enums::Cursor::Default);
                    }

                }
                // 正常窗口
                else  {
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

                // 主界面按钮 打开 / 显示拖拽 / 显示帮助 / 开始
                if !new_show_cursor {
                    new_show_cursor = {
                        button_open_dir.existPoint(x, y)||
                            button_show_drag.existPoint(x, y)||
                            button_show_help.existPoint(x, y)||
                            button_start.existPoint(x,y)
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

                // 图片预览大图
                if select_attach_card.thumbnail_preview.existPoint(x,y){
                    new_show_cursor=true;
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

                }
                true
            }

            _ => {
                false
            },
        }

    });



    initialize_window_hwnd!(hwnd);
    the_token
}
