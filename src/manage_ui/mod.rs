#![allow(warnings, unused)]

use crate::gui;
use fltk::enums::{Color, FrameType};
use fltk::window::DoubleWindow;
use fltk::{prelude::*, *};
use fltk_theme::{color_themes, ColorTheme, SchemeType, ThemeType, WidgetScheme, WidgetTheme};

macro_rules! set_item_id {
    ($win:expr,$id:expr) => {
        $win.set_xclass($id);
        $win.set_id($id);
    };
}

pub fn manage_tool_main() {
    let mut win = window::Window::default().with_size(600, 450).center_screen();
    win.set_label("用户任务管理");
    set_item_id!(win, "gui::manage_tool::main<win>");
    // win.set_border(false);
    // 退出窗口
    let exit_btn = gui::hotspot::create_hotspot(540, 15, 37, 37);

    let mut preview =
        gui::img::ImgPreview::new(0, 0, win.w(), win.h(), "gui::rename_tool::main<win>");
    preview.from_svg(
        include_str!("./src/contour.svg"),
        0,
        0,
        preview.preview.w(),
        preview.preview.h(),
    );
    preview.preview.set_id("gui::rename_tool::main<contour>");

    gui::text::TextControl::new(60 - 25, 24, 150, 20, 15, "用户任务管理", [122, 120, 120]);





    win.handle({
        let mut x = 0;
        let mut y = 0;

        move |win, ev| match ev {
            enums::Event::Show => {
                win.set_visible_focus();

                true
            }
            enums::Event::KeyUp => true,

            enums::Event::Push => {
                if exit_btn.existPoint(x, y) {
                    fltk::window::Window::delete(win.clone());
                }

                true
            }

            enums::Event::Move => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;

                // 判断是否显示手型鼠标
                if exit_btn.existPoint(x, y) {
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
}
