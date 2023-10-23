use super::*;
use fltk::{app, enums::Color, misc::Tooltip};

fn high_contrast_button_up_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(Color::from_rgb(0x00, 0xFF, 0xFF)));
    draw_rect(x, y, w, h);
    draw_rect(x + 1, y + 1, w - 2, h - 2);
}

fn high_contrast_button_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(Color::from_rgb(0x20, 0x20, 0x20)));
    draw_rectf(x + 2, y + 2, w - 4, h - 4);
    high_contrast_button_up_frame(x, y, w, h, c);
}

fn high_contrast_check_down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(Color::from_rgb(0x00, 0x80, 0x80)));
    draw_rect(x, y, w, h);
    draw_rect(x + 1, y + 1, w - 2, h - 2);
}

fn high_contrast_check_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(c));
    draw_rectf(x + 2, y + 2, w - 4, h - 4);
    high_contrast_check_down_frame(x, y, w, h, c);
}

fn high_contrast_panel_thin_up_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(Color::from_rgb(0x00, 0xFF, 0xFF)));
    draw_yxline2(x, y + h - 1, y, x + w - 1);
    set_draw_color(activated_color(Color::from_rgb(0x00, 0x80, 0x80)));
    draw_xyline2(x, y + h - 1, x + w - 1, y);
}

fn high_contrast_panel_thin_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(c));
    draw_rectf(x + 1, y + 1, w - 2, h - 2);
    high_contrast_panel_thin_up_frame(x, y, w, h, c);
}

fn high_contrast_spacer_thin_down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(Color::from_rgb(0x00, 0x80, 0x80)));
    draw_yxline2(x, y + h - 1, y, x + w - 1);
    set_draw_color(activated_color(Color::from_rgb(0x00, 0xFF, 0xFF)));
    draw_xyline2(x, y + h - 1, x + w - 1, y);
}

fn high_contrast_spacer_thin_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(c));
    draw_rectf(x + 1, y + 1, w - 2, h - 2);
    high_contrast_spacer_thin_down_frame(x, y, w, h, c);
}

fn high_contrast_default_button_up_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(Color::from_rgb(0xFF, 0xFF, 0xFF)));
    draw_rect(x, y, w, h);
    draw_rect(x + 1, y + 1, w - 2, h - 2);
}

fn high_contrast_default_button_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(Color::from_rgb(0x20, 0x20, 0x20)));
    draw_rectf(x + 2, y + 2, w - 4, h - 4);
    high_contrast_default_button_up_frame(x, y, w, h, c);
}

fn high_contrast_radio_round_down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
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

fn high_contrast_radio_round_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(Color::gray_ramp('W' as i32 - 'A' as i32)));
    draw_pie(x + 2, y + 2, w - 4, h - 4, 0.0, 360.0);
    high_contrast_radio_round_down_frame(x, y, w, h, c);
}

fn high_contrast_hovered_up_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(Color::from_rgb(0xFF, 0xFF, 0xFF)));
    draw_rect(x, y, w, h);
    draw_rect(x + 1, y + 1, w - 2, h - 2);
}

fn high_contrast_hovered_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(Color::from_rgb(0x00, 0x80, 0x80)));
    draw_rectf(x + 2, y + 2, w - 4, h - 4);
    high_contrast_hovered_up_frame(x, y, w, h, c);
}

fn use_high_contrast_scheme() {
    app::set_scheme(app::Scheme::Base);
    app::set_frame_type_cb(OS_BUTTON_UP_BOX, high_contrast_button_up_box, 2, 2, 4, 4);
    app::set_frame_type_cb(OS_CHECK_DOWN_BOX, high_contrast_check_down_box, 2, 2, 4, 4);
    app::set_frame_type_cb(
        OS_BUTTON_UP_FRAME,
        high_contrast_button_up_frame,
        2,
        2,
        4,
        4,
    );
    app::set_frame_type_cb(
        OS_CHECK_DOWN_FRAME,
        high_contrast_check_down_frame,
        2,
        2,
        4,
        4,
    );
    app::set_frame_type_cb(
        OS_PANEL_THIN_UP_BOX,
        high_contrast_panel_thin_up_box,
        1,
        1,
        2,
        2,
    );
    app::set_frame_type_cb(
        OS_SPACER_THIN_DOWN_BOX,
        high_contrast_spacer_thin_down_box,
        1,
        1,
        2,
        2,
    );
    app::set_frame_type_cb(
        OS_PANEL_THIN_UP_FRAME,
        high_contrast_panel_thin_up_frame,
        1,
        1,
        2,
        2,
    );
    app::set_frame_type_cb(
        OS_SPACER_THIN_DOWN_FRAME,
        high_contrast_spacer_thin_down_frame,
        1,
        1,
        2,
        2,
    );
    app::set_frame_type_cb(
        OS_RADIO_ROUND_DOWN_BOX,
        high_contrast_radio_round_down_box,
        2,
        2,
        4,
        4,
    );
    app::set_frame_type_cb(OS_HOVERED_UP_BOX, high_contrast_hovered_up_box, 2, 2, 4, 4);
    app::set_frame_type_cb(
        OS_DEPRESSED_DOWN_BOX,
        high_contrast_check_down_box,
        2,
        2,
        4,
        4,
    );
    app::set_frame_type_cb(
        OS_HOVERED_UP_FRAME,
        high_contrast_hovered_up_frame,
        2,
        2,
        4,
        4,
    );
    app::set_frame_type_cb(
        OS_DEPRESSED_DOWN_FRAME,
        high_contrast_check_down_frame,
        2,
        2,
        4,
        4,
    );
    app::set_frame_type_cb(
        OS_INPUT_THIN_DOWN_BOX,
        high_contrast_check_down_box,
        2,
        3,
        4,
        6,
    );
    app::set_frame_type_cb(
        OS_INPUT_THIN_DOWN_FRAME,
        high_contrast_check_down_frame,
        2,
        3,
        4,
        6,
    );
    app::set_frame_type_cb(
        OS_DEFAULT_BUTTON_UP_BOX,
        high_contrast_default_button_up_box,
        2,
        2,
        4,
        4,
    );
    app::set_frame_type2(OS_DEFAULT_HOVERED_UP_BOX, OS_HOVERED_UP_BOX);
    app::set_frame_type2(OS_DEFAULT_DEPRESSED_DOWN_BOX, OS_DEPRESSED_DOWN_BOX);
    app::set_frame_type2(OS_TOOLBAR_BUTTON_HOVER_BOX, OS_HOVERED_UP_BOX);
    app::set_frame_type_cb(OS_TABS_BOX, high_contrast_button_up_box, 2, 2, 4, 4);
    app::set_frame_type_cb(OS_SWATCH_BOX, high_contrast_check_down_box, 2, 2, 4, 4);
    app::set_frame_type_cb(
        OS_MINI_BUTTON_UP_BOX,
        high_contrast_button_up_box,
        2,
        2,
        4,
        4,
    );
    app::set_frame_type_cb(
        OS_MINI_DEPRESSED_DOWN_BOX,
        high_contrast_check_down_box,
        2,
        2,
        4,
        4,
    );
    app::set_frame_type_cb(
        OS_MINI_BUTTON_UP_FRAME,
        high_contrast_button_up_frame,
        2,
        2,
        4,
        4,
    );
    app::set_frame_type_cb(
        OS_MINI_DEPRESSED_DOWN_FRAME,
        high_contrast_check_down_frame,
        2,
        2,
        4,
        4,
    );
    app::set_frame_type_cb(FrameType::UpBox, high_contrast_button_up_box, 2, 2, 4, 4);
    app::set_frame_type_cb(FrameType::DownBox, high_contrast_check_down_box, 2, 2, 4, 4);
    app::set_frame_type_cb(
        FrameType::RoundDownBox,
        high_contrast_radio_round_down_box,
        2,
        2,
        4,
        4,
    );
    app::set_frame_type2(OS_BG_BOX, FrameType::FlatBox);
    // app::set_frame_type_cb(OS_BG_DOWN_BOX, OS_BG_BOX);
}

fn use_high_contrast_colors() {
    app::background(0x00, 0x00, 0x00);
    app::background2(0x00, 0x20, 0x20);
    app::foreground(0xFF, 0xFF, 0xFF);
    app::set_color(Color::Inactive, 0x00, 0x20, 0x20);
    app::set_color(Color::Selection, 0x00, 0xFF, 0xFF);
    app::set_color(Color::Free, 0x20, 0x20, 0x20);
    Tooltip::set_color(Color::from_rgb(0x00, 0x00, 0x00));
    Tooltip::set_text_color(Color::Foreground);
}

pub(crate) fn use_high_contrast_theme() {
    use_high_contrast_scheme();
    use_high_contrast_colors();
    use_native_settings();
}
