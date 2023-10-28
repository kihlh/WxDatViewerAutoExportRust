#![allow(warnings, unused)]
use fltk::enums::Color;
use fltk::{prelude::*, window::Window, *};
use fltk::frame::Frame;
use fltk::window::DoubleWindow;
use crate::gui_util::{*};
use crate::{gui_util, libWxIkunPlus, wh_mod, config, gui_rename_ui, global_var, APP_VERSION};
use crate::inject_fltk_theme;
use crate::set_item_id;
use crate::global_var_util;
use std::hint;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicUsize, Ordering,AtomicI64};
use std::sync::{Arc, Condvar, Mutex,RwLock};
use crate::atomic_util::{add_i32, add_usize, get_i32, get_usize, set_i32};
mod lib;
use crate::util;

pub(crate) const THE_WIN_CLASS_NAME: &str = "wx_auto_ex_im::gui_util::main::gui_task_manage<2103>";
pub(crate) const THE_NEXT_ID: &str = "wx_auto_ex_im::gui_task_manage::next_btn<2103>";
pub(crate) const THE_TASK_MANAGE_PAGE_CLASS_NAME: &str = "wx_auto_ex_im::gui_task_manage::task_manage_page<2103>";
pub(crate) const THE_EMPTY_PAGE_CLASS_NAME: &str = "wx_auto_ex_im::gui_task_manage::empty_page_item<2103>";
pub(crate) const THE_ATATUS_INFO_ID: &str = "wx_auto_ex_im::task_manage_page_result.info_text.text<2103>";

pub(crate) static EXPORT_LIST_PAGE_NEXT_ID:AtomicI32 = AtomicI32::new(1);
pub(crate) static FOCUS_ITEM_ID:AtomicI32 = AtomicI32::new(-1);

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


pub struct EmptyPageItem {
    pub win: DoubleWindow,
    pub icon: ImgPreview,
    pub next_btn: BorderPreview,
    pub empty_text: TextControl,
    pub next_btn_text: TextControl,
}

impl EmptyPageItem{
    pub fn to_empty (&mut self){
        self.empty_text.text.set_label("*_O 这里没有数据 ");
        self.next_btn_text.text.set_label("知道了");
        self.win.show();
    }
    pub fn to_next_empty(&mut self){
        self.empty_text.text.set_label("没有更多数据了o_O ");
        self.next_btn_text.text.set_label("上一页");
        self.win.show();
    }
}

// 没有任务
pub(crate) fn default_empty_page () ->EmptyPageItem {

    let mut result =  EmptyPageItem{
        win:fltk::window::Window::new(0,0,600,490-15,None),
        icon:gui_util::ImgPreview::new_border(167,67,261,200,include_str!("src/icon.svg")),
        next_btn:gui_util::border::BorderPreview::new(186,404,229,38,4,"#202020","#323232",2),
        empty_text:gui_util::TextControl::new(189,303,225,41,17,"没有更多数据了o_O ",[81, 81, 81]),
        next_btn_text:gui_util::TextControl::new(284,412,25,25,13,"上一页",[121, 121, 121])
    };

    result.next_btn.add_cursor_hand_callback(&result.win,{
        result.next_btn_text.text.set_id(THE_NEXT_ID);
        move|win,frame|{
            if let Some(next_text) = app::widget_from_id(THE_NEXT_ID) as Option<Frame>{
                if next_text.label().contains("知道了") {
                    libWxIkunPlus::setwinVisible(get_the_hwnd!(),false);
                }else{
                    if let Some(mut win) = app::widget_from_id(THE_TASK_MANAGE_PAGE_CLASS_NAME) as Option<DoubleWindow>{
                        win.show();
                    }
                    if let Some(mut win) = app::widget_from_id(THE_EMPTY_PAGE_CLASS_NAME) as Option<DoubleWindow>{
                        win.hide();
                    }
                        // println!("上一页");
                }
            }
        }

    });

    result.next_btn_text.add_cursor_hand(&result.win);
    result.win.set_label("没有更多数据了o_O");
    result.win.set_color(Color::from_rgb(23, 23, 23));
    result.win.end();
    result.win.hide();
    set_item_id!(result.win,THE_EMPTY_PAGE_CLASS_NAME);

    result
}

pub struct CardCheckItem{
    pub check_flex: fltk::group::Flex,
    pub check_button_video: fltk::button::CheckButton,
    pub check_button_thumbnail: fltk::button::CheckButton,
    pub check_button_source: fltk::button::CheckButton,
    pub check_button_sync: fltk::button::CheckButton,
    pub check_button_the_month: fltk::button::CheckButton,
}

impl CardCheckItem  {
   pub fn clone(&self) -> Self{
        CardCheckItem{
            check_flex: self.check_flex.clone(),
            check_button_video: self.check_button_video.clone(),
            check_button_thumbnail: self.check_button_thumbnail.clone(),
            check_button_source:self.check_button_source.clone(),
            check_button_sync: self.check_button_sync.clone(),
            check_button_the_month: self.check_button_the_month.clone(),
        }
    }
}

pub struct CardItem{
    pub win: DoubleWindow,
    pub check_item:CardCheckItem,
    pub input_rename: fltk::input::Input,
    pub input_select_dir: fltk::input::Input,
    pub input_task_name: fltk::input::Input,
    pub status_info: TextControl,
    pub user_thumbnail_preview: ImgPreview,
    pub btn_rename: HotspotItmeControl,
    pub btn_remove: HotspotItmeControl,
    pub btn_select_dir: HotspotItmeControl,
    pub check_hotspot: HotspotItmeControl,
    pub border: BorderPreview,
}

impl Clone for CardItem {
  fn clone(&self) -> Self{
        CardItem{
            win: self.win.clone(),
            check_item: self.check_item.clone(),
            input_rename: self.input_rename.clone(),
            input_select_dir: self.input_select_dir.clone(),
            input_task_name: self.input_task_name.clone(),
            status_info: self.status_info.clone(),
            user_thumbnail_preview: self.user_thumbnail_preview.clone(),
            btn_rename: self.btn_rename.clone(),
            btn_remove: self.btn_remove.clone(),
            btn_select_dir: self.btn_select_dir.clone(),
            check_hotspot: self.check_hotspot.clone(),
            border:self.border.clone(),
        }
    }
}

impl CardItem{
    pub fn is_remove(&mut self) -> bool{
        self.status_info.text.label().contains("已被移除")
    }
    pub fn to_remove (&mut self){
       self.input_rename.set_readonly(true);
       self.input_select_dir.set_readonly(true);
       self.input_task_name.set_readonly(true);
       self.status_info.text.set_label("已被移除");
       self.status_info.text.set_label_color(Color::from_rgb(215, 97, 97));
       self.status_info.text.set_color(Color::from_rgb(215, 97, 97));
       self.check_item.check_button_sync.deactivate();
       self.check_item.check_button_source.deactivate();
       self.check_item.check_button_thumbnail.deactivate();
       self.check_item.check_button_the_month.deactivate();
       self.check_item.check_button_video.deactivate();
    }
    pub fn to_unknown (&mut self){
        self.input_rename.readonly();
        self.input_select_dir.readonly();
        self.input_task_name.readonly();
        self.status_info.text.set_label("对象未知");
        self.status_info.text.set_color(Color::from_rgb(167, 137, 111));
        self.status_info.text.set_label_color(Color::from_rgb(215, 97, 97));

        self.check_item.check_button_sync.deactivate();
        self.check_item.check_button_source.deactivate();
        self.check_item.check_button_thumbnail.deactivate();
        self.check_item.check_button_the_month.deactivate();
        self.check_item.check_button_video.deactivate();
    }
    pub fn to_ok (&mut self){
        self.input_select_dir.set_readonly(false);
        self.input_task_name.set_readonly(false);
        self.status_info.text.set_label("撤销成功");
        self.status_info.text.set_color(Color::from_rgb(96, 139, 153));
        self.status_info.text.set_label_color(Color::from_rgb(96, 139, 153));
        self.check_item.check_button_sync.activate();
        self.check_item.check_button_source.activate();
        self.check_item.check_button_thumbnail.activate();
        self.check_item.check_button_the_month.activate();
        self.check_item.check_button_video.deactivate();
        self.input_rename.set_readonly(true);
        self.win.show();
    }
    pub fn to_gc (&mut self){
        self.input_rename.set_readonly(false);
        self.input_select_dir.set_readonly(false);
        self.input_task_name.set_readonly(false);
        self.status_info.text.set_label("状态正常");
        self.status_info.text.set_color(Color::from_rgb(96, 139, 153));
        self.status_info.text.set_label_color(Color::from_rgb(96, 139, 153));
        self.check_item.check_button_sync.activate();
        self.check_item.check_button_source.activate();
        self.user_thumbnail_preview.re_data(include_bytes!("./src/task_icon.png").to_vec());
        self.check_item.check_button_thumbnail.activate();
        self.check_item.check_button_the_month.activate();
        self.check_item.check_button_video.deactivate();
        self.input_rename.set_value("");
        self.input_select_dir.set_value("");
        self.input_task_name.set_value("");
        self.input_rename.set_readonly(true);
        self.win.set_label("");
        self.win.hide();
    }
    pub fn re_data(&mut self,item:&global_var_util::ExportTaskItem){
            self.to_gc();
            // self.user_thumbnail_preview
            self.input_task_name.set_value(item.name.as_str());
            // let
            let mut parse = wh_mod::parse_dat2var_path(item.path.as_str());

            // 赋值
            self.check_item.check_button_sync.set_checked(parse.is_sync);
            self.check_item.check_button_the_month.set_checked(parse.is_the_month);
            self.check_item.check_button_thumbnail.set_checked(parse.is_thumbnail);
            self.check_item.check_button_video.set_checked(parse.is_video);
            self.check_item.check_button_source.set_checked(parse.is_source);
            let mut input = parse.get_rename_output()
            .replace("<任务名>",item.name.as_str())
            .replace("<NNNN>","0001")
            .replace("<NNN>","001")
            .replace("<NN>","01")
            .replace("<N>","1")
            .replace("<哈希>","16a6a6a6a6a6a66a6a66a6a6a6a6a66a")
            .replace("<类型>","图片");

            self.input_rename.set_value(input.as_str());            
            
            std::thread::spawn({
                let mut id = item.id.clone();
                let mut user_thumbnail_preview_copy =self.user_thumbnail_preview.clone();
                move||{
                user_thumbnail_preview_copy.from_data(global_var_util::get_export_from_id_thumbnail(id),0,0,65,65);
            }});


            self.input_select_dir.set_value(item.ouput.as_str());
            self.win.set_label(&format!("[{},{}]",item.id,0));
            self.win.show();

    }

    pub fn re_data_v2(&mut self,item:&global_var_util::ExportTaskItemThumbnail){
        self.to_gc();
        // self.user_thumbnail_preview
        self.input_task_name.set_value(item.name.as_str());
        // let
        let mut parse = wh_mod::parse_dat2var_path(item.path.as_str());

        // 赋值
        self.check_item.check_button_sync.set_checked(parse.is_sync);
        self.check_item.check_button_the_month.set_checked(parse.is_the_month);
        self.check_item.check_button_thumbnail.set_checked(parse.is_thumbnail);
        self.check_item.check_button_video.set_checked(parse.is_video);
        self.check_item.check_button_source.set_checked(parse.is_source);
        let mut input = parse.get_rename_output()
        .replace("<任务名>",item.name.as_str())
        .replace("<NNNN>","0001")
        .replace("<NNN>","001")
        .replace("<NN>","01")
        .replace("<N>","1")
        .replace("<哈希>","16a6a6a6a6a6a66a6a66a6a6a6a6a66a")
        .replace("<类型>","图片");

        self.input_rename.set_value(input.as_str());            
        self.user_thumbnail_preview.from_data(item.thumbnail.to_vec(),0,0,65,65);

        self.input_select_dir.set_value(item.ouput.as_str());
        self.win.set_label(&format!("[{},{}]",item.id,0));
        self.win.show();

}
}

pub struct TaskManagePageItem {
    pub win: DoubleWindow,
    pub info_text: TextControl,
    pub next_btn: BorderPreview,
    pub back_btn: BorderPreview,
    pub next_btn_text: TextControl,
    pub back_btn_text: TextControl,
    pub card_list:[CardItem;2]
}

// 任务卡片
fn create_card (y:i32) -> CardItem{
    let mut win = fltk::window::Window::new(0,y,600,170,None);
    inject_fltk_theme!();
    win.set_color(Color::from_rgb(23, 23, 23));
    let border = gui_util::BorderPreview::new(16,9,566,153,23,(21, 21, 21, 1.0),(29, 29, 29, 1.0),3);

    // 三个输入框的描边
    gui_util::BorderPreview::new(152,26,169,31,4,(21, 21, 21, 0.0),(29, 29, 29, 1.0),2);
    gui_util::BorderPreview::new(375,26,149,31,4,(21, 21, 21, 0.0),(29, 29, 29, 1.0),2);
    gui_util::BorderPreview::new(151,71,305,31,4,(21, 21, 21, 0.0),(29, 29, 29, 1.0),2);

    let mut user_thumbnail_preview = gui_util::ImgPreview::new2(33,27,65,65,"",0,0,65,65);
    user_thumbnail_preview.preview.set_color(Color::from_rgb(255,255,255));
    user_thumbnail_preview.re_data(include_bytes!("./src/task_icon.png").to_vec());
    let status_info = gui_util::TextControl::new(-13,22,156,165,11,"状态正常","#608B99");
    gui_util::TextControl::new(293,36,117,15,12,"命名：","#525252");
    gui_util::TextControl::new(90,34,74,17,12,"名称：","#525252");
    gui_util::TextControl::new(69,81,117,15,12,"目录：","#525252");

    // 按钮
    let btn_rename = gui_util::hotspot::create_hotspot(528,29,27,26);
    let btn_select_dir = gui_util::hotspot::create_hotspot(474,70,38,36);
    let btn_remove = gui_util::hotspot::create_hotspot(518,70,38,36);
    let check_hotspot  = gui_util::create_hotspot(36,119,530,31);
    let mut icon_open = gui_util::ImgPreview::new_border(474,70,33,32,include_str!("src/open_icon.svg")).add_cursor_hand(&mut win);
    let mut icon_set_rename = gui_util::ImgPreview::new_border(532,33,17,16,include_str!("src/set_rename_icon.svg")).add_cursor_hand(&mut win);

    let mut icon_remove = gui_util::ImgPreview::new_border(518,70,33,32,include_str!("src/remove_icon.svg")).add_cursor_hand(&mut win);
    let mut icon_save = gui_util::ImgPreview::new_border(518,70,33,32,include_str!("src/save_icon.svg")).add_cursor_hand(&mut win);
    let mut icon_backup = gui_util::ImgPreview::new_border(518,70,33,32,include_str!("src/backup_icon.svg")).add_cursor_hand(&mut win);
    icon_save.preview.hide();
    // icon_remove.preview.hide();
    icon_backup.preview.hide();

        

    // 重命名预览
    let mut input_task_name = fltk::input::Input::new(153,28,167,28,None);
    input_task_name.set_value("");

    // 导出到
    let mut input_select_dir = fltk::input::Input::new(152,73,303,28,None);
    input_select_dir.set_value("");
    // 任务名称
    let mut input_rename = fltk::input::Input::new(377,28,146,28,None);
    input_rename.set_value("");
    input_rename.set_readonly(true);

    let mut flex = group::Flex::default()
        .with_size(530,31)
        .row()
        .center_of_parent();

    flex.set_pos(36+5,119);
    let mut check_button_sync = fltk::button::CheckButton::default().with_label("启用同步");
    let mut check_button_video = fltk::button::CheckButton::default().with_label("转存视频");
    check_button_video.deactivate();
    let mut check_button_thumbnail = fltk::button::CheckButton::default().with_label("存缩略图");
    let mut check_button_source = fltk::button::CheckButton::default().with_label("保存原图");
    let mut check_button_the_month = fltk::button::CheckButton::default().with_label("只保存本月");
    flex.end();

    win.end();

    let mut card_item = CardItem{
        win:win.clone(),
        check_item: CardCheckItem {
            check_flex:flex,
            check_button_sync,
            check_button_video,
            check_button_thumbnail,
            check_button_source,
            check_button_the_month,
        },
        input_rename,
        input_select_dir,
        input_task_name,
        status_info,
        user_thumbnail_preview,
        btn_rename,
        btn_remove,
        btn_select_dir,
        check_hotspot,
        border
    };
    
    win.handle({
        let mut x = 0;
        let mut y = 0;
        let mut btn_remove = card_item.btn_remove.clone();
        let mut btn_select_dir = card_item.btn_select_dir.clone();
        let mut back_up = None;
        let mut card_item = card_item.clone();
        let mut is_save = false;
        let mut OID_FOCUS_ITEM_ID = 0;
        let mut select_file_ing = false; 
        move |win, ev| match ev {
            enums::Event::Show => {
                if !select_file_ing&&get_i32(&FOCUS_ITEM_ID)!=OID_FOCUS_ITEM_ID {
                    is_save = false;
                    icon_save.preview.hide();
                    // icon_remove.preview.hide();
                    icon_backup.preview.hide();
                    icon_remove.preview.show();
                    OID_FOCUS_ITEM_ID = get_i32(&FOCUS_ITEM_ID);
                }
                true
            }
            enums::Event::Close => {
                
                false
            }
            enums::Event::Hide => {
                if !select_file_ing&&get_i32(&FOCUS_ITEM_ID)!=OID_FOCUS_ITEM_ID {
                    is_save = false;
                    icon_save.preview.hide();
                    // icon_remove.preview.hide();
                    icon_backup.preview.hide();
                    icon_remove.preview.show();
                    OID_FOCUS_ITEM_ID = get_i32(&FOCUS_ITEM_ID);
                }
                false
            }
            
            enums::Event::Push => {

                if win.visible() {
                    let mut label = win.label();
                    // title 会是一个json [id]
                    if let Ok(label2json) = serde_json::from_str(label.as_str()) as serde_json::Result<serde_json::Value>{
                        if let Some(id) = label2json[0].as_i64() {
                            set_i32(&FOCUS_ITEM_ID,id as i32);
                            OID_FOCUS_ITEM_ID = id as i32;
                        }
                    }
                }
                
                // 删除热区按钮有三个用途/ 保存/删除/撤回
                if btn_remove.existPoint(x, y)&&!is_save{
    
                        if !card_item.is_remove() {
                            
                            if let Ok(item) = global_var_util::get_thumbnail_from_id(get_i32(&FOCUS_ITEM_ID)) {
                                if let Some(item) = item.get(0){
                                    back_up.replace(item.clone());
                                    // back_up = Some(item.clone())
                                }
                            }

                            // v2接口错误使用v1代替v2
                            if back_up.is_none() {
                                if let Some(item) = global_var_util::get_export_dir_itme_from_id(get_i32(&FOCUS_ITEM_ID)) {
                                    let mut data = global_var_util::ExportTaskItemThumbnail{
                                        id: item.id,
                                        time: item.time,
                                        name: item.name,
                                        path: item.path,
                                        ouput: item.ouput,
                                        thumbnail: include_bytes!("./src/task_icon.png").to_vec(),
                                        version:APP_VERSION
                                    };
                                    back_up.replace(data);
                                }

                            }
                            
                            if lib::remove_export_id(get_i32(&FOCUS_ITEM_ID)) {
                                gui_util::sub_message(get_the_hwnd!(),gui_util::IconType::Success, "已成功移除", 2500u64);
                                card_item.clone().to_remove();
                               
                                if back_up.is_some() {
                                    icon_remove.preview.hide();
                                    icon_save.preview.hide();
                                    icon_backup.preview.show();
                                    println!("删除");
                                }

                            }
                            
                            else{
                                gui_util::sub_message(get_the_hwnd!(),gui_util::IconType::Failure, "删除失败", 2500u64);
                            }
                        
                    }
                    
                    else if card_item.is_remove(){
                        // 撤回
                        if back_up.is_none(){
                            gui_util::sub_message(get_the_hwnd!(),gui_util::IconType::Failure,"撤回缓冲已被清空", 3500u64);
                        }else{
                            let mut data = back_up.as_mut().unwrap();
                            match global_var_util::insert_export_task_from_id_thumbnail1(data.clone()) {
                                Ok(_) =>{
                                    gui_util::sub_message(get_the_hwnd!(),gui_util::IconType::Success,"撤回成功", 3500u64);
                                    card_item.to_ok();
                                    back_up=None;
                                },
                                Err(err) => {
                                    gui_util::sub_message(get_the_hwnd!(),gui_util::IconType::Failure,"撤回失败", 3500u64);
                                    eprintln!("{}", err);
                                },
                            }
                            is_save = false;
                            icon_remove.preview.show();
                            icon_save.preview.hide();
                            icon_backup.preview.hide();
                        }
                       
                        
                    }

                }

                // 保存
                if btn_remove.existPoint(x, y)&&is_save{
                    let mut result_data = String::new();

                     // 识标
                     result_data.push_str("*wizards");
                    // 可选项
                    if card_item.check_item.check_button_thumbnail.is_checked() {
                        result_data.push_str("*thumbnail");
                    }
                    if card_item.check_item.check_button_source.is_checked() {
                        result_data.push_str("*source");
                    }
                    if card_item.check_item.check_button_video.is_checked() {
                        result_data.push_str("*video");
                    }
                    if card_item.check_item.check_button_sync.is_checked() {
                        result_data.push_str("*Sync");
                    }
                    if card_item.check_item.check_button_the_month.is_checked() {
                        result_data.push_str("*the_month");
                    }
                    // 添加名称格式化自变量
                    if !card_item.input_rename.readonly()&&!card_item.input_rename.value().is_empty() {
                        result_data.push_str(&format!("*rename_rule={}*", &card_item.input_rename.value()));
                    }
                    else if let Some(item) = global_var_util::get_export_dir_itme_from_id(get_i32(&FOCUS_ITEM_ID)) {
                        let mut path = wh_mod::parse_dat2var_path(item.path);
                        if !path.rename_rule.is_empty() {
                            result_data.push_str(&format!("*rename_rule={}*",path.rename_rule));
                        }
                    }

                    if let Some(item) = global_var_util::get_export_dir_itme_from_id(get_i32(&FOCUS_ITEM_ID)) {
                        let mut path = wh_mod::parse_dat2var_path(&item.path);
                        let mut new_path = format!("{}",path.attach_dir);
                        new_path.push_str(&result_data);
                        let input_task_name =card_item.input_task_name.value();
                        let input_select_dir =card_item.input_select_dir.value();

                        if input_task_name.is_empty()||input_select_dir.is_empty() {
                            gui_util::sub_message(get_the_hwnd!(),gui_util::IconType::Failure, "名称或路径不能为空", 3500u64);

                                return false;
                        }

                        if let Err(item) = global_var_util::update_export_task_from_id_thumbnail(item.id,&input_task_name,&new_path,&input_select_dir,None) {
                            
                            eprintln!("item->{:?}",item);
                            gui_util::sub_message(get_the_hwnd!(),gui_util::IconType::Failure, "更新失败", 3500u64);

                        }else{
                            gui_util::sub_message(get_the_hwnd!(),gui_util::IconType::Success, "更新成功", 3500u64);
                            icon_remove.preview.show();
                            icon_save.preview.hide();
                            icon_backup.preview.hide();
                            is_save = false;
                        }
                        
                        println!("result_data->>{}",&new_path);
                    }

                }

                // 检测更新动作
                if card_item.btn_rename.existPoint(x, y)||card_item.check_hotspot.existPoint(x, y) {
                    is_save =true;
                    icon_remove.preview.hide();
                    icon_backup.preview.hide();
                    icon_save.preview.show();
                }

                // 
                if btn_select_dir.existPoint(x, y){
                    select_file_ing= true;

                    let select_dir = libWxIkunPlus::openSelectFolder2();
                    select_file_ing= false;

                    if select_dir.is_empty() {
                        gui_util::sub_message(get_the_hwnd!(),gui_util::IconType::Info,"用户取消选择", 3500u64);
                    }else{
                        card_item.input_select_dir.set_value(&select_dir);
                        is_save =true;
                        icon_remove.preview.hide();
                        icon_backup.preview.hide();
                        icon_save.preview.show();
                    }
                }

                if card_item.btn_rename.existPoint(x, y){
                    
                    println!("[click] existPoint {}", "卡片按钮 > 编辑命名规则");
                    let mut point_rename_value = String::new();

                    for  value in global_var_util::get_export_task_item_list() {
                            if value.id == get_i32(&FOCUS_ITEM_ID) {
                                point_rename_value=  wh_mod::parse_dat2var_path(value.path).rename_rule;       
                            }
                        }

                     let mut rename_token = gui_rename_ui::rename_tool_main(point_rename_value.as_str());

                     let mut input_rename = card_item.input_rename.clone();
 
                     app::add_timeout3(0.3,{

                        move|handle|{
                         if global_var::has_string(rename_token.as_str()) {
                             let data = global_var::get_string_default(rename_token.as_str());
                             if data.is_empty() {
                                 println!("{} 用户取消 data-> [{}]",&rename_token,&data);
                                 gui_util::message::sub_message(get_the_hwnd!(),gui_util::message::IconType::Warning,"用户取消处理",3500u64);
                             }else{
                                 if !input_rename.value().as_bytes().eq(data.as_bytes()) {
                                     input_rename.set_value(data.as_str());
                                     input_rename.set_readonly(false);
                                     println!("{} 名称更新 data-> [{}]",&rename_token,&data);
                                     gui_util::message::sub_message(get_the_hwnd!(),gui_util::message::IconType::Success,"名称输入框已更新",3500u64);
                                 
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
                     }});

                }

                if card_item.user_thumbnail_preview.existPoint(x, y){
                    select_file_ing= true;
                    let path = libWxIkunPlus::selectFile();
                    select_file_ing= false;

                   if(!path.is_empty()) {
                    std::thread::spawn({
                        let mut user_thumbnail_preview= card_item.user_thumbnail_preview.clone();
                        let mut path = path.clone();
                        move||{
                            if let Ok(data) = util::load_thumbnai_data(&path) {
                                global_var_util::set_export_from_id_thumbnail(get_i32(&FOCUS_ITEM_ID), Some(data.to_vec()));
                                user_thumbnail_preview.re_data(data)
                            }else{
                                gui_util::sub_message(get_the_hwnd!(),gui_util::IconType::Failure, "图片打开失败", 2500u64);
        
                            }
                        }});
                    
                    }
                   
                }
                true
            }
            enums::Event::KeyUp=>{
                is_save =true;
                icon_remove.preview.hide();
                icon_backup.preview.hide();
                icon_save.preview.show();
                true
            }
            enums::Event::Move => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;

                if get_i32(&FOCUS_ITEM_ID)!=OID_FOCUS_ITEM_ID {
                    is_save = false;
                    icon_save.preview.hide();
                    // icon_remove.preview.hide();
                    icon_backup.preview.hide();
                    icon_remove.preview.show();
                    OID_FOCUS_ITEM_ID = get_i32(&FOCUS_ITEM_ID);
                }
                
                true
            }
            enums::Event::Drag => {


                true
            }
            _ => false,
        }
    });

    card_item}


// 任务管理的窗口
pub fn task_manage_page() -> TaskManagePageItem {

    let mut win = fltk::window::Window::new(0,0,600,490-15,None);
    set_item_id!(win,THE_TASK_MANAGE_PAGE_CLASS_NAME);
    let mut win_copy = win.clone();
    inject_fltk_theme!();
    win.set_color(Color::from_rgb(23, 23, 23));

    let mut result =  TaskManagePageItem{
        win,
        info_text: gui_util::TextControl::new(255,374,87,22,13,"NN / NN页",[122, 120, 120]),
        next_btn:gui_util::border::BorderPreview::new(325,413,125,38,4,(32, 32, 32, 1.0),(50, 50, 50, 1.0),2),
        back_btn:gui_util::border::BorderPreview::new(149,413,125,38,4,(24, 24, 24, 1.0),(50, 50, 50, 1.0),2),
        next_btn_text: gui_util::TextControl::new(203,425,15,15,13,"上一页",[122, 120, 120]),
        back_btn_text: gui_util::TextControl::new(378,425,15,15,13,"下一页",[122, 120, 120]),
        card_list: [create_card(20),create_card(195)],
    };

    set_i32(&FOCUS_ITEM_ID,0);
    set_i32(&EXPORT_LIST_PAGE_NEXT_ID,1);

    result.next_btn.add_cursor_hand(&win_copy);
   

    result.back_btn.add_cursor_hand(&win_copy);
    result.next_btn_text.add_cursor_hand(&win_copy);
    result.back_btn_text.add_cursor_hand(&win_copy);
    
    //强制更新到最新
    global_var_util::update_export_task_item_list();

    let mut export_dir_path_list = global_var_util::get_group_export_task_value_list(2);

    let mut header = format!("{:02} / {:02} 页",get_i32(&EXPORT_LIST_PAGE_NEXT_ID),export_dir_path_list.len());
    
    // 页码
    let mut info_text = result.info_text.text.clone();
    info_text.set_label(&header);

    result.info_text.text.set_id(THE_ATATUS_INFO_ID);
    result.win.end();
    result.win.show();
    
    result.win.handle({
        let mut x = 0;
        let mut y = 0;
        let mut card_list = result.card_list.to_vec();
        let mut next_btn = result.next_btn.clone();
        let mut back_btn  = result.back_btn.clone();
        let mut init_win = false;

        move |win, ev| match ev {
            enums::Event::Show => {
                if(!init_win){
                    // 数据初始化
                    if let Some(export_dir_path_list) = export_dir_path_list.get(0) {
                        for index in 0..card_list.len() {
                            if let Some(item) =export_dir_path_list.get(index)  {
                                card_list[index].re_data_v2(&global_var_util::export_task_item_to_v2(&item));
                            }else{
                                card_list[index].to_gc();
                            }
                        }
                    }

                    init_win= true;
                }


                true
            }
            enums::Event::Close => {
                set_i32(&FOCUS_ITEM_ID,-1);
                set_i32(&EXPORT_LIST_PAGE_NEXT_ID,1);
                false
            }
            enums::Event::Hide => {

                false
            }
            enums::Event::Push => {

                // 当点击了以后就会将 id 赋值到 FOCUS_ITEM_ID 以此判断内容指定位置
                for index in 0..card_list.len() {
                    if let Some(card) = card_list.get(index) {
                        if card.win.visible() {
                            let [x2,y2,w,h] = [card.win.x(),card.win.y(),card.win.w(),card.win.h()];
                            if x <x2 + w && y > y2 && y < y2 + h {
                                let mut label = card.win.label();
                                // title 会是一个json [id]
                                if let Ok(label2json) = serde_json::from_str(label.as_str()) as serde_json::Result<serde_json::Value>{
                                    if let Some(id) = label2json[0].as_i64() {
                                        set_i32(&FOCUS_ITEM_ID,id as i32);
                                        println!("[enums::Event::Push] FOCUS_ITEM_ID->{:?}",&FOCUS_ITEM_ID);

                                    }
                                }
                            }
                        }

                    }
                }

                // 下一页
                if next_btn.existPoint(x,y) {
                  
                    // 更新数据
                    export_dir_path_list.clear();
                    for value in global_var_util::get_group_export_task_value_list(2) {
                        export_dir_path_list.push(value)
                    }

                    // 计算页码
                    if export_dir_path_list.len() < (get_i32(&EXPORT_LIST_PAGE_NEXT_ID)+1) as usize{
                        if !config::is_developer(){
                            gui_util::sub_message(get_the_hwnd!(),gui_util::IconType::Info,"没有更多数据了",2500u64);
                        }
                    }else {
                        add_i32(&EXPORT_LIST_PAGE_NEXT_ID,1);
            
                        if let Some(mut info_text) = app::widget_from_id(THE_ATATUS_INFO_ID) as Option<Frame> {
                            let mut header = format!("{:02} / {:02} 页",get_i32(&EXPORT_LIST_PAGE_NEXT_ID),export_dir_path_list.len());
                            info_text.set_label(&header);
                        }
                    }
                    set_i32(&FOCUS_ITEM_ID,-1);

                    // 更新到列表
                    
                    let mut export_index = get_i32(&EXPORT_LIST_PAGE_NEXT_ID) ;

                    if let Some(export_dir_path_list) = export_dir_path_list.get((export_index-1) as usize) {
                        for index in 0..card_list.len() {
                            if let Some(item) =export_dir_path_list.get(index)  {
                                card_list[index].re_data_v2(&global_var_util::export_task_item_to_v2(&item));
                            }else {
                                card_list[index].to_gc();
                            }
                        }
                    }
                }

                // 上一页
                if back_btn.existPoint(x,y) {
        

                    export_dir_path_list.clear();
                    for value in global_var_util::get_group_export_task_value_list(2) {
                        export_dir_path_list.push(value)
                    }

                    if get_i32(&EXPORT_LIST_PAGE_NEXT_ID)-1 < 1 {
                        if !config::is_developer(){
                            gui_util::sub_message(get_the_hwnd!(),gui_util::IconType::Info,"我也是有底线的",2500u64);
                        }
                    }else {
                        let mut data = get_i32(&EXPORT_LIST_PAGE_NEXT_ID);
                        if data>1 {
                            set_i32(&EXPORT_LIST_PAGE_NEXT_ID,data-1);
        
                            if let Some(mut info_text) = app::widget_from_id(THE_ATATUS_INFO_ID) as Option<Frame> {
                                let mut header = format!("{:02} / {:02} 页",get_i32(&EXPORT_LIST_PAGE_NEXT_ID),export_dir_path_list.len());
                                info_text.set_label(&header);
                            }
                        }
        
                    }

                    let mut export_index = get_i32(&EXPORT_LIST_PAGE_NEXT_ID) ;

                    if let Some(export_dir_path_list) = export_dir_path_list.get((export_index-1) as usize) {
                        for index in 0..card_list.len() {
                            if let Some(item) =export_dir_path_list.get(index)  {
                                card_list[index].re_data_v2(&global_var_util::export_task_item_to_v2(&item));
                            }else {
                                card_list[index].to_gc();
                            }
                        }
                    }

                }

                true
            }
            enums::Event::Move => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;

                true
            }
            enums::Event::Drag => {


                true
            }
            _ => false,
        }
    });
    result
}


pub struct ManageItemMain{
    pub win:DoubleWindow,
    pub empty_page:EmptyPageItem,
    pub task_manage_page:TaskManagePageItem
}

impl ManageItemMain {
    pub fn to_empty_page(&mut self,is_empty:bool){
        self.task_manage_page.win.hide();
        self.empty_page.win.show();
        if is_empty {
            self.empty_page.to_empty();
        }else {
            self.empty_page.to_next_empty();
        }
    }
    pub fn to_task_manage_page(&mut self){
        self.task_manage_page.win.show();
        self.empty_page.win.hide();
    }
    // 判断当前是否是管理页中
    pub fn is_task_visible(&mut self) -> bool {
        let mut empty_page_visible = self.empty_page.win.visible();
        let mut task_manage_visible = self.task_manage_page.win.visible();

        // 同时显示就不对
        if empty_page_visible&&task_manage_visible{
            self.to_task_manage_page();
            task_manage_visible = true;
            empty_page_visible = false;
        }

        // 同时不显示也不对
        if !empty_page_visible&&!task_manage_visible {
            self.to_empty_page(false);
            task_manage_visible = false;
            empty_page_visible = true;
        }

       return !empty_page_visible&&task_manage_visible ;
    }
}

pub fn ManageItemMain() ->Option<ManageItemMain> {
    main_init_check!();

    let mut win = fltk::window::Window::default().with_size(600, 490 - 15).center_screen();
    win.set_color(Color::from_rgb(23, 23, 23));
    win.set_label("任务管理");
    set_item_id!(win, THE_WIN_CLASS_NAME);

    let mut win_empty_page = default_empty_page();
    let mut win_task_manage_page = task_manage_page();

    let mut win_copy = win.clone();

    win.end();
    win.show();



    win.handle({
        let mut x = 0;
        let mut y = 0;
        let mut win_empty = win_empty_page.win.clone();
        // let mut next_btn = win_empty_page.next_btn.preview.clone();
        let mut next_btn_text = win_empty_page.next_btn_text.clone();

        move |win, ev| match ev {
            enums::Event::Show => {
                libWxIkunPlus::setWinIcon(get_the_hwnd!());

                true
            }
            enums::Event::Close => {
                fltk::window::Window::delete(win.clone());

                false
            }
            enums::Event::Hide => {
                if win_empty.visible() &&next_btn_text.get_label().contains("知道了"){
                    fltk::window::Window::delete(win.clone());
                }

                false
            }
            enums::Event::Push => {
                
                true
            }
            enums::Event::Move => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;

                true
            }
            enums::Event::Drag => {
                // if y < 69 {
                //     win.clone()
                //         .set_pos(app::event_x_root() - x, app::event_y_root() - y);
                // }

                true
            }
            _ => false,
        }
    });

    libWxIkunPlus::setWinIcon(get_the_hwnd!());



    let mut result = ManageItemMain{
        win:win.clone(),
        empty_page:win_empty_page,
        task_manage_page: win_task_manage_page,
    };

    if global_var_util::get_export_task_item_len()==0 {
        result.to_empty_page(true);
    }

    gui_util::redraw_win(&result.win);

    
    Some(result)
}
