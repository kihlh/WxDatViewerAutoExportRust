use super::*;
use fltk::{app, enums::Color, misc::Tooltip};

fn classic_button_up_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    draw_frame2("AAWWMMRR", x, y, w, h);
}

fn classic_button_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(c));
    draw_rectf(x + 2, y + 2, w - 4, h - 4);
    classic_button_up_frame(x, y, w, h, c);
}

fn classic_check_down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    draw_frame2("WWMMPPAA", x, y, w, h);
}

fn classic_check_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(c));
    draw_rectf(x + 2, y + 2, w - 4, h - 4);
    classic_check_down_frame(x, y, w, h, c);
}

fn classic_panel_thin_up_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    draw_frame2("MMWW", x, y, w, h);
}

fn classic_panel_thin_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(c));
    draw_rectf(x + 1, y + 1, w - 2, h - 2);
    classic_panel_thin_up_frame(x, y, w, h, c);
}

fn classic_spacer_thin_down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    draw_frame2("WWMM", x, y, w, h);
}

fn classic_spacer_thin_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(c));
    draw_rectf(x + 1, y + 1, w - 2, h - 2);
    classic_spacer_thin_down_frame(x, y, w, h, c);
}

fn classic_default_button_up_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    draw_frame2("AAAAGGWWMMRR", x, y, w, h);
}

fn classic_default_button_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(c));
    draw_rectf(x + 3, y + 3, w - 6, h - 6);
    classic_default_button_up_frame(x, y, w, h, c);
}

fn classic_radio_round_down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top and left outer border
    set_draw_color(activated_color(Color::gray_ramp('M' as i32 - 'A' as i32)));
    draw_arc(x, y, w, h, 45.0, 225.0);
    // bottom and right outer border
    set_draw_color(activated_color(Color::gray_ramp('W' as i32 - 'A' as i32)));
    draw_arc(x, y, w, h, -135.0, 45.0);
    // top and left inner border
    set_draw_color(activated_color(Color::gray_ramp(0)));
    draw_arc(x + 1, y + 1, w - 2, h - 2, 45.0, 225.0);
    // bottom and right inner border
    set_draw_color(activated_color(Color::gray_ramp('T' as i32 - 'A' as i32)));
    draw_arc(x + 1, y + 1, w - 2, h - 2, -135.0, 45.0);
}

fn classic_radio_round_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(Color::gray_ramp('W' as i32 - 'A' as i32)));
    draw_pie(x + 2, y + 2, w - 4, h - 4, 0.0, 360.0);
    classic_radio_round_down_frame(x, y, w, h, c);
}

fn use_classic_scheme() {
    app::set_scheme(app::Scheme::Base);
    app::set_frame_type_cb(OS_BUTTON_UP_BOX, classic_button_up_box, 2, 2, 4, 4);
    app::set_frame_type_cb(OS_CHECK_DOWN_BOX, classic_check_down_box, 2, 2, 4, 4);
    app::set_frame_type_cb(OS_BUTTON_UP_FRAME, classic_button_up_frame, 2, 2, 4, 4);
    app::set_frame_type_cb(OS_CHECK_DOWN_FRAME, classic_check_down_frame, 2, 2, 4, 4);
    app::set_frame_type_cb(OS_PANEL_THIN_UP_BOX, classic_panel_thin_up_box, 1, 1, 2, 2);
    app::set_frame_type_cb(
        OS_SPACER_THIN_DOWN_BOX,
        classic_spacer_thin_down_box,
        1,
        1,
        2,
        2,
    );
    app::set_frame_type_cb(
        OS_PANEL_THIN_UP_FRAME,
        classic_panel_thin_up_frame,
        1,
        1,
        2,
        2,
    );
    app::set_frame_type_cb(
        OS_SPACER_THIN_DOWN_FRAME,
        classic_spacer_thin_down_frame,
        1,
        1,
        2,
        2,
    );
    app::set_frame_type_cb(
        OS_RADIO_ROUND_DOWN_BOX,
        classic_radio_round_down_box,
        2,
        2,
        4,
        4,
    );
    app::set_frame_type_cb(OS_HOVERED_UP_BOX, classic_button_up_box, 2, 2, 4, 4);
    app::set_frame_type_cb(OS_DEPRESSED_DOWN_BOX, classic_check_down_box, 2, 2, 4, 4);
    app::set_frame_type_cb(OS_HOVERED_UP_FRAME, classic_button_up_frame, 2, 2, 4, 4);
    app::set_frame_type_cb(
        OS_DEPRESSED_DOWN_FRAME,
        classic_check_down_frame,
        2,
        2,
        4,
        4,
    );
    app::set_frame_type_cb(OS_INPUT_THIN_DOWN_BOX, classic_check_down_box, 2, 3, 4, 6);
    app::set_frame_type_cb(
        OS_INPUT_THIN_DOWN_FRAME,
        classic_check_down_frame,
        2,
        3,
        4,
        6,
    );
    app::set_frame_type_cb(
        OS_DEFAULT_BUTTON_UP_BOX,
        classic_default_button_up_box,
        3,
        3,
        6,
        6,
    );
    app::set_frame_type_cb(
        OS_DEFAULT_HOVERED_UP_BOX,
        classic_default_button_up_box,
        3,
        3,
        6,
        6,
    );
    app::set_frame_type_cb(
        OS_DEFAULT_DEPRESSED_DOWN_BOX,
        classic_check_down_box,
        2,
        2,
        4,
        4,
    );
    app::set_frame_type2(OS_TOOLBAR_BUTTON_HOVER_BOX, FrameType::FlatBox);
    app::set_frame_type_cb(OS_TABS_BOX, classic_button_up_box, 2, 2, 4, 4);
    app::set_frame_type_cb(OS_SWATCH_BOX, classic_check_down_box, 2, 2, 4, 4);
    app::set_frame_type_cb(OS_MINI_BUTTON_UP_BOX, classic_button_up_box, 2, 2, 4, 4);
    app::set_frame_type_cb(
        OS_MINI_DEPRESSED_DOWN_BOX,
        classic_check_down_box,
        2,
        2,
        4,
        4,
    );
    app::set_frame_type_cb(OS_MINI_BUTTON_UP_FRAME, classic_button_up_frame, 2, 2, 4, 4);
    app::set_frame_type_cb(
        OS_MINI_DEPRESSED_DOWN_FRAME,
        classic_check_down_frame,
        2,
        2,
        4,
        4,
    );
    app::set_frame_type_cb(FrameType::UpBox, classic_button_up_box, 2, 2, 4, 4);
    app::set_frame_type_cb(FrameType::DownBox, classic_check_down_box, 2, 2, 4, 4);
    app::set_frame_type_cb(
        FrameType::RoundDownBox,
        classic_radio_round_down_box,
        2,
        2,
        4,
        4,
    );
    app::set_frame_type2(OS_BG_BOX, FrameType::FlatBox);
}

fn use_classic_colors() {
    app::background(0xD4, 0xD0, 0xC8);
    app::background2(0xFF, 0xFF, 0xFF);
    app::foreground(0x00, 0x00, 0x00);
    app::set_color(Color::Inactive, 0x5F, 0x5F, 0x5F);
    app::set_color(Color::Selection, 0x0A, 0x24, 0x6A);
    app::set_color(Color::Free, 0xD4, 0xD0, 0xC8);
    Tooltip::set_color(Color::from_rgb(0xFF, 0xFF, 0xE1));
    Tooltip::set_text_color(Color::Foreground);
}

pub(crate) fn use_classic_theme() {
    use_classic_scheme();
    use_classic_colors();
    use_native_settings();
}
