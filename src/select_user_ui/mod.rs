#![allow(warnings, unused)]

use crate::rename_ui::rename_tool_main;
use crate::{gui_util, libWxIkunPlus, global_var, wh_mod, get_arc_bind_variable, gui_detect_config, gui_drag_scan};
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
use std::path::{Path, PathBuf};

mod lib;

const WINDOW_CLASS_NAME : &'static str = "wx_auto_ex_im::gui_util::select_user_ui::main<win>";

macro_rules! set_item_id {
    ($win:expr,$id:expr) => {
        $win.set_xclass($id);
        $win.set_id($id);
    };
}

macro_rules! eq_wxid_dir{
    ($select_dir:expr)=>{
        {
            let mut is_wxid_dir = false;
            if !$select_dir.is_empty(){

                        if !$select_dir.contains("WeChat Files"){
                            // 没有 WeChat Files 则尝试为路径添加 WeChat Files
                            let mut to_path = std::path::Path::new($select_dir.as_str());
                            let mut join_path = to_path.join("WeChat Files");

                            if join_path.exists() && join_path.is_dir(){
                               $select_dir.push_str("\\WeChat Files");
                            }

                        }

                        // 判断路径下是否有 wxid_ 开头的文件夹
                        if let Ok(rd_dir) = std::fs::read_dir($select_dir.as_str()) {

                            for rd_dir in rd_dir {
                                if let Ok(dir) = rd_dir {
                                    is_wxid_dir= dir.file_name().to_string_lossy().contains("wxid_");
                                    if is_wxid_dir{
                                        break;
                                    }
                                }
                            }

                            if !is_wxid_dir{
                                dialog::alert_default("此路径可能不是有效的WX目录 因为未发现有效的用户数据");
                            }

                        }else{
                            dialog::alert_default("目录无法被打开 请注意路径有效性");
                        }

                    }
                   
            is_wxid_dir
        }
    }
}

macro_rules! add_theme {
    () => {
        let theme = ColorTheme::new(color_themes::BLACK_THEME);
        let widget_theme = WidgetTheme::new(ThemeType::HighContrast);
        widget_theme.apply();
        let widget_scheme = WidgetScheme::new(SchemeType::Aqua);
        widget_scheme.apply();
        theme.apply();
    };
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


macro_rules! set_select_user_base_input_default{
    ($input_select_dir:expr)=>{
        if let Ok(history) = lib::get_wx_user_history_path() {
            
            let paths = history.path;
            $input_select_dir.set_value(paths.as_str());
            
        }
        if ($input_select_dir.value().is_empty()) {
            if let Some(paths) = wh_mod::convert::get_user_data_path() {
                $input_select_dir.set_value(paths.as_str());
            }
        }

        global_var::set_string("user::config::user_select_path", $input_select_dir.value());
    }
}

macro_rules! initialize_window_hwnd{
    ($hwnd:expr)=>{
        if $hwnd==0{
            $hwnd =  libWxIkunPlus::findWindow(WINDOW_CLASS_NAME, "");
            libWxIkunPlus::setWinIcon($hwnd);
            println!("[initialize-window] {} -> {}",WINDOW_CLASS_NAME, $hwnd);
        }
    }
}

pub fn manage_tool_main() {
    
    // 禁止创建多个窗口
    if let hwnd = libWxIkunPlus::findWindow(WINDOW_CLASS_NAME, "") {
        if hwnd!=0 && libWxIkunPlus::isWindow(hwnd) {
            if let Some(mut win) =app::widget_from_id("gui::DoubleWindow::handle_dat::main") as Option<DoubleWindow>
             {
                 win.show();
                 win.set_visible_focus();
             }
             libWxIkunPlus::setWindowShake(hwnd);
            return;
        }
    }

    let mut hwnd :i128 = 0;

    let mut win = window::DoubleWindow::new(0, 0,600, 595,None).center_screen();
    
    add_theme!();
    
    win.set_label("任务创建向导");
    set_item_id!(win, WINDOW_CLASS_NAME);
    
    let mut g_the_select_wxid = String::new();
    let mut g_the_select_attach_id = String::new();

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

    select_user_data_choice.set_callback(move |c| {
        if let Some(choice_value) = c.choice() {
            
            if let Some(item) = lib::get_active_user_list().get(c.value() as usize).clone() {
            
            lib::store_wx_user_path_history(item.user_root.clone(),item.user_wxid.clone());
            
            println!("[{}] {}  , {}  ,  {} ",c.value(), choice_value.as_str() ,&item.user_root,&item.user_wxid);
            
            global_var::set_string("user::config::user_select_path", item.user_root.clone());
            global_var::set_string("user::config::user_select_wxid", item.user_wxid.clone());

            c.deactivate();

            }
            
            
        }
    });


    select_attach_card.input_attach.set_value("valafdsgdfhfjhutyiyuogyo");
    if let Some(item) = lib::get_store_user_remark(wh_mod::convert::get_user_id1(),"valafdsgdfhfjhutyiyuogyo".to_string()) {
        println!("attach_id->{}",item);
        select_attach_card.input_remark.set_value(item.as_str());
    }
    
    win.handle({
        let mut x = 0;
        let mut y = 0;
        // 是否显示手型
        let mut show_cursor = false;

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
                            global_var::set_i32("user::config::select_user_thumbnail_obj",index);
                            break;
                        }
                    }
                }

                // 打开文件夹选择器
                if button_open_dir.existPoint(x, y){

                    let mut select_dir = libWxIkunPlus::openSelectFolder2();
                    let eq_wxid_dir = eq_wxid_dir!(select_dir);
                    user_select_database_dir_input.set_value(select_dir.as_str());
                    
                    select_user_data_choice.clear();
                    select_user_data_choice.add_choice("请点击 [开始] 获取在线用户列表");
                    select_user_data_choice.set_value(0);
                    lib::set_active_user_list(Vec::new());
                    
                    global_var::set_string("user::config::user_select_path", select_dir.clone());

                    println!("[click] existPoint {}  select_dir-> {} eq-> {}","打开文件夹选择器",select_dir,eq_wxid_dir);

                }

                // 显示扫描获取面板
                if button_show_drag.existPoint(x, y){
                    println!("[click] existPoint {}","显示扫描获取面板");

                    if(!libWxIkunPlus::hasWeChat()||!libWxIkunPlus::hasWeChatWin()){
                        fltk::dialog::alert_default("尚未找到已经登录的WX进程 为避免滥用 面板开启请求被拒绝 ");
                        return false;
                    }

                    gui_drag_scan::main_window();

                }

                // 显示帮助面板
                if button_show_help.existPoint(x, y) {
                    println!("[click] existPoint {}","");
                    gui_detect_config::main_window();
                }

                // 开始
                if button_start.existPoint(x,y){
                    if !user_select_database_dir_input.value().is_empty(){
                        
                      


                        if(!libWxIkunPlus::hasWeChat()||!libWxIkunPlus::hasWeChatWin()){
                            fltk::dialog::alert_default("尚未找到已经登录的WX进程 为避免滥用 扫描被拒绝 ");
                            return false;
                        }

                        // 释放资源
                        select_user_data_choice.activate();
                        lib::set_active_user_list(Vec::new());


                        select_user_data_choice.clear();
                        select_user_data_choice.add_choice("【状态】  当前正在扫描用户列表... ");
                        select_user_data_choice.set_value(0);

                        let user_select_database = user_select_database_dir_input.value();
                        
                        if !user_select_database.is_empty(){
                            let active_user_list = wh_mod::convert::get_active_user(user_select_database.as_str());
                            select_user_data_choice.clear();
                            
                            if active_user_list.is_empty() {
                                select_user_data_choice.add_choice("【状态】  未找到用户列表 请检查目录是否正确");
                                select_user_data_choice.set_value(0);
                                return false;
                            }

                            // 添加到列表
                            for active_user in active_user_list {

                                if let Some(accinfo) = active_user.accinfo.clone() {
                                    select_user_data_choice.add_choice(format!("{} <{}>",accinfo.wx_id , accinfo.name).as_str());
                                    
                                }else{

                                    select_user_data_choice.add_choice(active_user.user_data.as_str());
                                }

                                lib::push_active_user_list(active_user.clone());
                                
                            }
                          
                            if let Some(item) = lib::get_active_user_list().get(0) {
                                select_user_data_choice.set_value(0);
                                lib::store_wx_user_path_history(item.user_root.clone(),item.user_wxid.clone());
                                global_var::set_string("user::config::user_select_path",item.user_root.clone());
                                global_var::set_string("user::config::user_select_wxid", item.user_wxid.clone());
                            }
                          

                        }


                    }
                    else{
                        dialog::alert_default("没有选择WX文件默认保存位置");
                    }

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

                    // 没有选定的路径
                    if user_select_database_dir_input.value().is_empty(){
                        fltk::dialog::alert_default("没有选定Wx路径");
                        is_effective = false;
                        return false;
                    }

                    //  判断是否有Att id
                    else if g_the_select_attach_id.is_empty()||g_the_select_attach_id.len()<25{
                        fltk::dialog::alert_default("attach id 无效 （尚未选定有效聊天对象）");
                        is_effective = false;
                        return false;
                    }

                    // 有命名规则 要求规则最少有一个%N.. 自变量
                    if !rename_rule.is_empty() && (!rename_rule.contains("<N")||!rename_rule.contains("N>")){
                        rename_rule.push_str("<NN>");
                    }

                    // 添加名称格式化自变量
                    if !select_attach_card.input_rename.value().is_empty(){
                        data.push_str(format!("*rename_rule={}*",&rename_rule).as_str());
                    }

                    let mut select_dir = user_select_database_dir_input.value();
                    let eq_wxid_dir = eq_wxid_dir!(select_dir);

                    // 拼合路径并判断有效性 有且为文件夹
                    let mut attach_path  = PathBuf::from(select_dir).join(g_the_select_wxid.clone()).join("FileStorage\\MsgAttach").join(g_the_select_attach_id.as_str());

                    println!("attach_path=> {:?}",&attach_path);

                    if !attach_path.exists()&&!attach_path.exists() {

                        dialog::alert_default("attach 目录无效");
                        is_effective = false;
                        return false;
                    }

                    if is_effective && eq_wxid_dir{
                        wh_mod::watch_path::un_next_exits();
                        global_var::set_string("user::config::user_select_path", String::new());
                        global_var::set_string("user::config::user_select_wxid", String::new());

                        fltk::window::Window::delete(win.clone());
                    }

                }

                // 卡片按钮 > 备注名称 完成按钮
                if select_attach_card.btn_remark.existPoint(x, y) {
                    println!("[click] existPoint {}","卡片按钮 > 备注名称 完成按钮");
                    
                    let  wxid = global_var::get_string_default("user::config::user_select_wxid");
                    let  attach_id = select_attach_card.input_attach.value();
                    let  remark_name = select_attach_card.input_remark.value();

                    if wxid.is_empty() {
                        dialog::alert_default("没有选择用户 (WXID)");
                        return false;
                    }

                    if attach_id.is_empty() {
                        dialog::alert_default("没有选择聊天对象 (attach ID)");
                        return false;
                    }

                    if remark_name.is_empty() {
                        dialog::alert_default("没有备注内容 (备注将用于命名与显示对象名称)");
                        return false;
                    }

                    lib::set_store_user_remark(wxid,attach_id,remark_name);

                }

                // 卡片按钮 > 编辑命名规则
                if select_attach_card.btn_rename.existPoint(x, y) {
                    println!("[click] existPoint {}","卡片按钮 > 编辑命名规则");
                    rename_tool_main(select_attach_card.input_rename.value().as_str());
                }

                true
            }

            enums::Event::Move => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;
                let mut new_show_cursor = false;

                initialize_window_hwnd!(hwnd);

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

    win.end();
    win.show();
    initialize_window_hwnd!(hwnd);

}
