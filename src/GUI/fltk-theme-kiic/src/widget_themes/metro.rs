use super::aero::*;
use super::*;
use fltk::{app, enums::Color, misc::Tooltip};

fn metro_button_up_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(Color::from_rgb(0xAC, 0xAC, 0xAC)));
    draw_rect(x, y, w, h);
}

fn metro_button_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    if w >= h {
        vertical_gradient(
            x + 1,
            y + 1,
            x + w - 2,
            y + h - 2,
            activated_color(Color::from_rgb(0xF0, 0xF0, 0xF0)),
            activated_color(Color::from_rgb(0xE5, 0xE5, 0xE5)),
        );
    } else {
        horizontal_gradient(
            x + 1,
            y + 1,
            x + w - 2,
            y + h - 2,
            activated_color(Color::from_rgb(0xF0, 0xF0, 0xF0)),
            activated_color(Color::from_rgb(0xE5, 0xE5, 0xE5)),
        );
    }
    metro_button_up_frame(x, y, w, h, c);
}

fn metro_check_down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(Color::from_rgb(0x70, 0x70, 0x70)));
    draw_rect(x, y, w, h);
}

fn metro_check_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(Color::from_rgb(0xFF, 0xFF, 0xFF)));
    draw_rectf(x + 1, y + 1, w - 2, h - 2);
    metro_check_down_frame(x, y, w, h, c);
}

fn metro_radio_round_down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(Color::from_rgb(0x73, 0x73, 0x73)));
    draw_arc(x, y, w, h, 0.0, 360.0);
}

fn metro_radio_round_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(Color::from_rgb(0xFF, 0xFF, 0xFF)));
    draw_pie(x, y, w, h, 0.0, 360.0);
    metro_radio_round_down_frame(x, y, w, h, c);
}

fn metro_hovered_up_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(Color::from_rgb(0x7E, 0xB4, 0xEA)));
    draw_rect(x, y, w, h);
}

fn metro_hovered_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    vertical_gradient(
        x + 1,
        y + 1,
        x + w - 2,
        y + h - 2,
        activated_color(Color::from_rgb(0xEC, 0xF4, 0xFC)),
        activated_color(Color::from_rgb(0xDC, 0xEC, 0xFC)),
    );
    metro_hovered_up_frame(x, y, w, h, c);
}

fn metro_depressed_down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(Color::from_rgb(0x56, 0x9D, 0xE5)));
    draw_rect(x, y, w, h);
}

fn metro_depressed_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    vertical_gradient(
        x + 1,
        y + 1,
        x + w - 2,
        y + h - 2,
        activated_color(Color::from_rgb(0xDA, 0xEC, 0xFC)),
        activated_color(Color::from_rgb(0xC4, 0xE0, 0xFC)),
    );
    metro_depressed_down_frame(x, y, w, h, c);
}

fn metro_input_thin_down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(Color::from_rgb(0xAB, 0xAD, 0xB3)));
    draw_rect(x, y, w, h);
}

fn metro_input_thin_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(Color::from_rgb(0xFF, 0xFF, 0xFF)));
    draw_rectf(x + 1, y + 1, w - 2, h - 2);
    metro_input_thin_down_frame(x, y, w, h, c);
}

fn metro_default_button_up_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(Color::from_rgb(0x33, 0x99, 0xFF)));
    draw_rect(x, y, w, h);
}

fn metro_default_button_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    vertical_gradient(
        x + 1,
        y + 1,
        x + w - 2,
        y + h - 2,
        activated_color(Color::from_rgb(0xF0, 0xF0, 0xF0)),
        activated_color(Color::from_rgb(0xE5, 0xE5, 0xE5)),
    );
    metro_default_button_up_frame(x, y, w, h, c);
}

fn metro_default_hovered_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    vertical_gradient(
        x + 1,
        y + 1,
        x + w - 2,
        y + h - 2,
        activated_color(Color::from_rgb(0xEC, 0xF4, 0xFC)),
        activated_color(Color::from_rgb(0xDC, 0xEC, 0xFC)),
    );
    metro_default_button_up_frame(x, y, w, h, c);
}

fn use_metro_scheme() {
    app::set_scheme(app::Scheme::Gtk);
    app::set_frame_type_cb(OS_BUTTON_UP_BOX, metro_button_up_box, 1, 1, 2, 2);
    app::set_frame_type_cb(OS_CHECK_DOWN_BOX, metro_check_down_box, 1, 1, 2, 2);
    app::set_frame_type_cb(OS_BUTTON_UP_FRAME, metro_button_up_frame, 1, 1, 2, 2);
    app::set_frame_type_cb(OS_CHECK_DOWN_FRAME, metro_check_down_frame, 1, 1, 2, 2);
    app::set_frame_type_cb(OS_PANEL_THIN_UP_BOX, aero_panel_thin_up_box, 1, 1, 2, 2);
    app::set_frame_type_cb(
        OS_SPACER_THIN_DOWN_BOX,
        aero_spacer_thin_down_box,
        1,
        1,
        2,
        2,
    );
    app::set_frame_type_cb(OS_PANEL_THIN_UP_FRAME, aero_panel_thin_up_frame, 1, 1, 2, 2);
    app::set_frame_type_cb(
        OS_SPACER_THIN_DOWN_FRAME,
        aero_spacer_thin_down_frame,
        1,
        1,
        2,
        2,
    );
    app::set_frame_type_cb(
        OS_RADIO_ROUND_DOWN_BOX,
        metro_radio_round_down_box,
        3,
        3,
        6,
        6,
    );
    app::set_frame_type_cb(OS_HOVERED_UP_BOX, metro_hovered_up_box, 1, 1, 2, 2);
    app::set_frame_type_cb(OS_DEPRESSED_DOWN_BOX, metro_depressed_down_box, 1, 1, 2, 2);
    app::set_frame_type_cb(OS_HOVERED_UP_FRAME, metro_hovered_up_frame, 1, 1, 2, 2);
    app::set_frame_type_cb(
        OS_DEPRESSED_DOWN_FRAME,
        metro_depressed_down_frame,
        1,
        1,
        2,
        2,
    );
    app::set_frame_type_cb(
        OS_INPUT_THIN_DOWN_BOX,
        metro_input_thin_down_box,
        1,
        2,
        2,
        4,
    );
    app::set_frame_type_cb(
        OS_INPUT_THIN_DOWN_FRAME,
        metro_input_thin_down_frame,
        1,
        2,
        2,
        4,
    );
    app::set_frame_type_cb(
        OS_DEFAULT_BUTTON_UP_BOX,
        metro_default_button_up_box,
        1,
        1,
        2,
        2,
    );
    app::set_frame_type_cb(
        OS_DEFAULT_HOVERED_UP_BOX,
        metro_default_hovered_up_box,
        1,
        1,
        2,
        2,
    );
    app::set_frame_type2(OS_DEFAULT_DEPRESSED_DOWN_BOX, OS_DEPRESSED_DOWN_BOX);
    app::set_frame_type2(OS_TOOLBAR_BUTTON_HOVER_BOX, FrameType::FlatBox);
    app::set_frame_type_cb(OS_TABS_BOX, aero_tabs_box, 2, 1, 4, 2);
    app::set_frame_type_cb(OS_SWATCH_BOX, aero_swatch_box, 2, 2, 4, 4);
    app::set_frame_type2(OS_MINI_BUTTON_UP_BOX, OS_BUTTON_UP_BOX);
    app::set_frame_type2(OS_MINI_DEPRESSED_DOWN_BOX, OS_DEPRESSED_DOWN_BOX);
    app::set_frame_type2(OS_MINI_BUTTON_UP_FRAME, OS_BUTTON_UP_FRAME);
    app::set_frame_type2(OS_MINI_DEPRESSED_DOWN_FRAME, OS_DEPRESSED_DOWN_FRAME);
    app::set_frame_type2(FrameType::UpBox, OS_BUTTON_UP_BOX);
    app::set_frame_type2(FrameType::DownBox, OS_CHECK_DOWN_BOX);
    app::set_frame_type2(FrameType::RoundDownBox, OS_RADIO_ROUND_DOWN_BOX);
    app::set_frame_type2(OS_BG_BOX, FrameType::FlatBox);
    // app::set_frame_type2(OS_BG_DOWN_BOX, OS_BG_BOX);
}

fn use_metro_colors() {
    app::reload_scheme().ok();
    app::background(0xF0, 0xF0, 0xF0);
    app::background2(0xFF, 0xFF, 0xFF);
    app::foreground(0x00, 0x00, 0x00);
    app::set_color(Color::Inactive, 0x6F, 0x6F, 0x6F);
    app::set_color(Color::Selection, 0x33, 0x99, 0xFF);
    app::set_color(Color::Free, 0xFF, 0xFF, 0xFF);
    Tooltip::set_color(Color::from_rgb(0xFF, 0xFF, 0xFF));
    Tooltip::set_text_color(Color::Foreground);
}

pub(crate) fn use_metro_theme() {
    use_metro_scheme();
    use_metro_colors();
    use_native_settings();
}
