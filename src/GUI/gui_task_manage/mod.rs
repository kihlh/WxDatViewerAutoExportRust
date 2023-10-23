#![allow(warnings, unused)]
use fltk::enums::Color;
use fltk::{prelude::*, window::Window, *};
use fltk::window::DoubleWindow;
use crate::gui_util::{*};
use crate::{gui_util, libWxIkunPlus};
use crate::inject_fltk_theme;
use crate::set_item_id;

pub(crate) const THE_WIN_CLASS_NAME: &str = "wx_auto_ex_im::gui_util::main::gui_task_manage<658683>";

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
                return ;
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
    result.next_btn.add_cursor_hand(&result.win);
    result.next_btn_text.add_cursor_hand(&result.win);
    result.win.set_label("没有更多数据了o_O");
    result.win.set_color(Color::from_rgb(23, 23, 23));
    result.win.end();
    result.win.hide();

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
}

impl CardItem{
    pub fn to_remove (&mut self){
       self.input_rename.readonly();
       self.input_select_dir.readonly();
       self.input_task_name.readonly();
       self.status_info.text.set_label("已被移除");
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
        self.check_item.check_button_sync.deactivate();
        self.check_item.check_button_source.deactivate();
        self.check_item.check_button_thumbnail.deactivate();
        self.check_item.check_button_the_month.deactivate();
        self.check_item.check_button_video.deactivate();
    }
    pub fn to_gc (&mut self){
        self.input_rename.set_readonly(false);
        self.input_select_dir.set_readonly(false);
        self.input_task_name.set_readonly(false);
        self.status_info.text.set_label("状态正常");
        self.status_info.text.set_color(Color::from_rgb(96, 139, 153));
        self.check_item.check_button_sync.activate();
        self.check_item.check_button_source.activate();
        self.user_thumbnail_preview.re_data(include_bytes!("./src/task_icon.png").to_vec());
        self.check_item.check_button_thumbnail.activate();
        self.check_item.check_button_the_month.activate();
        self.check_item.check_button_video.deactivate();
        self.input_rename.set_value("");
        self.input_select_dir.set_value("");
        self.input_task_name.set_value("");

    }

}

pub struct TaskManagePageItem {
    pub win: DoubleWindow,
    pub info_text: TextControl,
    pub next_btn: BorderPreview,
    pub back_btn: BorderPreview,
    pub next_btn_text: TextControl,
    pub back_btn_text: TextControl,
    pub card_list:Vec<CardItem>
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
    let status_info = gui_util::TextControl::new(-10,22,156,165,11,"状态正常","#608B99");
    gui_util::TextControl::new(293,36,117,15,12,"命名：","#525252");
    gui_util::TextControl::new(90,34,74,17,12,"名称：","#525252");
    gui_util::TextControl::new(69,81,117,15,12,"目录：","#525252");

    // 按钮
    let btn_rename = gui_util::hotspot::create_hotspot(528,29,27,26);
    let btn_select_dir = gui_util::hotspot::create_hotspot(474,70,38,36);
    let btn_remove = gui_util::hotspot::create_hotspot(518,70,38,36);
    let check_hotspot  = gui_util::create_hotspot(36,119,530,31);
    gui_util::ImgPreview::new_border(474,70,33,32,include_str!("src/open_icon.svg")).add_cursor_hand(&mut win);
    gui_util::ImgPreview::new_border(518,70,33,32,include_str!("src/remove_icon.svg")).add_cursor_hand(&mut win);
    gui_util::ImgPreview::new_border(532,33,17,16,include_str!("src/set_rename_icon.svg")).add_cursor_hand(&mut win);

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
    CardItem{
        win,
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
    }
}

// 任务管理的窗口
pub fn task_manage_page() -> TaskManagePageItem {
    let mut win = fltk::window::Window::new(0,0,600,490-15,None);
    let mut win_copy = win.clone();
    inject_fltk_theme!();
    win.set_color(Color::from_rgb(23, 23, 23));

    let mut result =  TaskManagePageItem{
        win,
        info_text: gui_util::TextControl::new(255,374,87,22,13,"01 / 50页",[122, 120, 120]),
        next_btn:gui_util::border::BorderPreview::new(325,413,125,38,4,(32, 32, 32, 1.0),(50, 50, 50, 1.0),2),
        back_btn:gui_util::border::BorderPreview::new(149,413,125,38,4,(24, 24, 24, 1.0),(50, 50, 50, 1.0),2),
        next_btn_text: gui_util::TextControl::new(203,425,15,15,13,"上一页",[122, 120, 120]),
        back_btn_text: gui_util::TextControl::new(378,425,15,15,13,"下一页",[122, 120, 120]),
        card_list: vec![create_card(20),create_card(195)],
    };

    // 添加按钮显示手型
    result.next_btn.add_cursor_hand(&win_copy);
    result.back_btn.add_cursor_hand(&win_copy);
    result.next_btn_text.add_cursor_hand(&win_copy);
    result.back_btn_text.add_cursor_hand(&win_copy);

    result.win.end();
    result.win.show();

    result
}



pub fn ManageItmeMain() {
    main_init_check!();
    
    let mut win = fltk::window::Window::default().with_size(600, 490 - 15).center_screen();
    win.set_color(Color::from_rgb(23, 23, 23));
    win.set_label("任务管理");
    set_item_id!(win, THE_WIN_CLASS_NAME);

    let mut win_empty_page = default_empty_page();

    let mut win_task_manage_page = task_manage_page();

    win.end();
    win.show();

    win.handle({
        let mut x = 0;
        let mut y = 0;
        move |win, ev| match ev {
            enums::Event::Show => {
                libWxIkunPlus::setWinIcon(get_the_hwnd!());

                true
            }
            enums::Event::Close => {
                false
            }
            enums::Event::Hide => {
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

}
