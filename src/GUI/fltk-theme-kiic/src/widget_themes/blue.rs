use super::aero::*;
use super::*;
use fltk::{app, enums::Color, misc::Tooltip};

fn blue_button_up_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // outer border
    set_draw_color(activated_color(Color::from_rgb(0x87, 0x97, 0xAA)));
    draw_xyline(x + 2, y, x + w - 3);
    draw_xyline(x + 2, y + h - 1, x + w - 3);
    draw_yxline(x, y + 2, y + h - 3);
    draw_yxline(x + w - 1, y + 2, y + h - 3);
    // top inner borders
    set_draw_color(activated_color(Color::from_rgb(0xF6, 0xFA, 0xFE)));
    draw_xyline(x + 2, y + 1, x + w - 3);
    draw_yxline(x + 1, y + 2, y + h / 2 - 1);
    draw_yxline(x + w - 2, y + 2, y + h / 2 - 1);
    draw_point(x + 2, y + 2);
    draw_point(x + w - 3, y + 2);
    // bottom inner borders
    set_draw_color(activated_color(Color::from_rgb(0xFE, 0xFF, 0xFF)));
    draw_yxline(x + 1, y + h / 2, y + h - 3);
    draw_yxline(x + w - 2, y + h / 2, y + h - 3);
    draw_xyline(x + 2, y + h - 2, x + w - 3);
    draw_point(x + 2, y + h - 3);
    draw_point(x + w - 3, y + h - 3);
    // top corners
    set_draw_color(activated_color(Color::from_rgb(0x9B, 0xAA, 0xBB)));
    draw_xyline2(x, y + 1, x + 1, y);
    draw_yxline2(x + w - 2, y, y + 1, x + w - 1);
    // bottom corners
    set_draw_color(activated_color(Color::from_rgb(0xA1, 0xAE, 0xBD)));
    draw_xyline2(x, y + h - 2, x + 1, y + h - 1);
    draw_yxline2(x + w - 2, y + h - 1, y + h - 2, x + w - 1);
}

fn blue_button_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    if w >= h {
        // top gradient
        vertical_gradient(
            x + 2,
            y + 2,
            x + w - 3,
            y + h / 2 - 1,
            activated_color(Color::from_rgb(0xF0, 0xF6, 0xFB)),
            activated_color(Color::from_rgb(0xE2, 0xEA, 0xF3)),
        );
        // bottom gradient
        vertical_gradient(
            x + 2,
            y + h / 2,
            x + w - 3,
            y + h - 3,
            activated_color(Color::from_rgb(0xD5, 0xE0, 0xED)),
            activated_color(Color::from_rgb(0xD7, 0xE2, 0xEF)),
        );
    } else {
        // left gradient
        horizontal_gradient(
            x + 2,
            y + 2,
            x + w / 2 - 1,
            y + h - 3,
            activated_color(Color::from_rgb(0xF0, 0xF6, 0xFB)),
            activated_color(Color::from_rgb(0xE2, 0xEA, 0xF3)),
        );
        // right gradient
        horizontal_gradient(
            x + w / 2,
            y + 2,
            x + w - 3,
            y + h - 3,
            activated_color(Color::from_rgb(0xD5, 0xE0, 0xED)),
            activated_color(Color::from_rgb(0xD7, 0xE2, 0xEF)),
        );
    }
    blue_button_up_frame(x, y, w, h, c);
}

fn blue_panel_thin_up_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top and left borders
    set_draw_color(activated_color(Color::from_rgb(0xF0, 0xF6, 0xFB)));
    draw_yxline2(x, y + h - 2, y, x + w - 2);
    // bottom and right borders
    set_draw_color(activated_color(Color::from_rgb(0xBF, 0xCB, 0xDA)));
    draw_xyline2(x, y + h - 1, x + w - 1, y);
}

fn blue_panel_thin_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(c));
    draw_rectf(x + 1, y + 1, w - 2, h - 2);
    blue_panel_thin_up_frame(x, y, w, h, c);
}

fn blue_spacer_thin_down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top and left borders
    set_draw_color(activated_color(Color::from_rgb(0xBF, 0xCB, 0xDA)));
    draw_yxline2(x, y + h - 2, y, x + w - 2);
    // bottom and right borders
    set_draw_color(activated_color(Color::from_rgb(0xF0, 0xF6, 0xFB)));
    draw_xyline2(x, y + h - 1, x + w - 1, y);
}

fn blue_spacer_thin_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(c));
    draw_rectf(x + 1, y + 1, w - 2, h - 2);
    blue_spacer_thin_down_frame(x, y, w, h, c);
}

fn blue_hovered_up_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // outer border
    set_draw_color(activated_color(Color::from_rgb(0xFF, 0xDB, 0x00)));
    draw_xyline(x + 2, y, x + w - 3);
    draw_xyline(x + 2, y + h - 1, x + w - 3);
    draw_yxline(x, y + 2, y + h - 3);
    draw_yxline(x + w - 1, y + 2, y + h - 3);
    // top inner borders
    set_draw_color(activated_color(Color::from_rgb(0xFF, 0xFC, 0xF8)));
    draw_xyline(x + 2, y + 1, x + w - 3);
    draw_yxline(x + 1, y + 2, y + h / 2 - 1);
    draw_yxline(x + w - 2, y + 2, y + h / 2 - 1);
    draw_point(x + 2, y + 2);
    draw_point(x + w - 3, y + 2);
    // bottom inner borders
    set_draw_color(activated_color(Color::from_rgb(0xFF, 0xFA, 0xE2)));
    draw_yxline(x + 1, y + h / 2, y + h - 3);
    draw_yxline(x + w - 2, y + h / 2, y + h - 3);
    draw_xyline(x + 2, y + h - 2, x + w - 3);
    draw_point(x + 2, y + h - 3);
    draw_point(x + w - 3, y + h - 3);
    // corners
    set_draw_color(activated_color(Color::from_rgb(0xF7, 0xD7, 0x3F)));
    draw_xyline2(x, y + 1, x + 1, y);
    draw_xyline2(x, y + h - 2, x + 1, y + h - 1);
    draw_yxline2(x + w - 2, y, y + 1, x + w - 1);
    draw_yxline2(x + w - 2, y + h - 1, y + h - 2, x + w - 1);
}

fn blue_hovered_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top gradient
    vertical_gradient(
        x + 2,
        y + 2,
        x + w - 3,
        y + h / 2 - 1,
        activated_color(Color::from_rgb(0xFF, 0xF0, 0xDF)),
        activated_color(Color::from_rgb(0xFF, 0xE2, 0xC2)),
    );
    // bottom gradient
    vertical_gradient(
        x + 2,
        y + h / 2,
        x + w - 3,
        y + h - 3,
        activated_color(Color::from_rgb(0xFF, 0xCF, 0x6A)),
        activated_color(Color::from_rgb(0xFF, 0xE9, 0x83)),
    );
    blue_hovered_up_frame(x, y, w, h, c);
}

fn blue_depressed_down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // outer border
    set_draw_color(activated_color(Color::from_rgb(0xC2, 0x9B, 0x29)));
    draw_xyline(x + 2, y, x + w - 3);
    draw_xyline(x + 2, y + h - 1, x + w - 3);
    draw_yxline(x, y + 2, y + h - 3);
    draw_yxline(x + w - 1, y + 2, y + h - 3);
    // top inner border
    set_draw_color(activated_color(Color::from_rgb(0xE3, 0xC1, 0x85)));
    draw_xyline(x + 2, y + 1, x + w - 3);
    // corners
    set_draw_color(activated_color(Color::from_rgb(0xCB, 0xAB, 0x53)));
    draw_xyline2(x, y + 1, x + 1, y);
    draw_xyline2(x, y + h - 2, x + 1, y + h - 1);
    draw_yxline2(x + w - 2, y, y + 1, x + w - 1);
    draw_yxline2(x + w - 2, y + h - 1, y + h - 2, x + w - 1);
}

fn blue_depressed_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top gradient
    vertical_gradient(
        x + 1,
        y + 2,
        x + w - 2,
        y + h / 2 - 1,
        activated_color(Color::from_rgb(0xEE, 0xCB, 0x8E)),
        activated_color(Color::from_rgb(0xF5, 0xC7, 0x79)),
    );
    // bottom gradient
    vertical_gradient(
        x + 1,
        y + h / 2,
        x + w - 2,
        y + h - 1,
        activated_color(Color::from_rgb(0xF5, 0xBB, 0x57)),
        activated_color(Color::from_rgb(0xF3, 0xE1, 0x77)),
    );
    blue_depressed_down_frame(x, y, w, h, c);
}

fn blue_default_button_up_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // outer border
    set_draw_color(activated_color(Color::from_rgb(0x87, 0x97, 0xAA)));
    draw_xyline(x + 2, y, x + w - 3);
    draw_xyline(x + 2, y + h - 1, x + w - 3);
    draw_yxline(x, y + 2, y + h - 3);
    draw_yxline(x + w - 1, y + 2, y + h - 3);
    // top inner borders
    set_draw_color(activated_color(Color::from_rgb(0xF7, 0xFB, 0xFF)));
    draw_xyline(x + 2, y + 1, x + w - 3);
    draw_yxline(x + 1, y + 2, y + h / 2 - 1);
    draw_yxline(x + w - 2, y + 2, y + h / 2 - 1);
    draw_point(x + 2, y + 2);
    draw_point(x + w - 3, y + 2);
    // bottom inner borders
    set_draw_color(activated_color(Color::from_rgb(0xFB, 0xFE, 0xFF)));
    draw_yxline(x + 1, y + h / 2, y + h - 3);
    draw_yxline(x + w - 2, y + h / 2, y + h - 3);
    draw_xyline(x + 2, y + h - 2, x + w - 3);
    draw_point(x + 2, y + h - 3);
    draw_point(x + w - 3, y + h - 3);
    // top corners
    set_draw_color(activated_color(Color::from_rgb(0x9B, 0xAA, 0xBB)));
    draw_xyline2(x, y + 1, x + 1, y);
    draw_yxline2(x + w - 2, y, y + 1, x + w - 1);
    // bottom corners
    set_draw_color(activated_color(Color::from_rgb(0xA1, 0xAE, 0xBD)));
    draw_xyline2(x, y + h - 2, x + 1, y + h - 1);
    draw_yxline2(x + w - 2, y + h - 1, y + h - 2, x + w - 1);
}

fn blue_default_button_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top gradient
    vertical_gradient(
        x + 2,
        y + 2,
        x + w - 3,
        y + h / 2 - 1,
        activated_color(Color::from_rgb(0xF7, 0xFB, 0xFF)),
        activated_color(Color::from_rgb(0xED, 0xF3, 0xF8)),
    );
    // bottom gradient
    vertical_gradient(
        x + 2,
        y + h / 2,
        x + w - 3,
        y + h - 3,
        activated_color(Color::from_rgb(0xE7, 0xED, 0xF5)),
        activated_color(Color::from_rgb(0xEA, 0xF1, 0xF8)),
    );
    blue_default_button_up_frame(x, y, w, h, c);
}

fn blue_tabs_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // border
    set_draw_color(activated_color(Color::from_rgb(0x87, 0x97, 0xAA)));
    draw_xyline(x + 2, y, x + w - 3);
    draw_xyline(x + 2, y + h - 1, x + w - 3);
    draw_yxline(x, y + 2, y + h - 3);
    draw_yxline(x + w - 1, y + 2, y + h - 3);
    // top corners
    set_draw_color(activated_color(Color::from_rgb(0x9B, 0xAA, 0xBB)));
    draw_xyline2(x, y + 1, x + 1, y);
    draw_yxline2(x + w - 2, y, y + 1, x + w - 1);
    // bottom corners
    set_draw_color(activated_color(Color::from_rgb(0xA1, 0xAE, 0xBD)));
    draw_xyline2(x, y + h - 2, x + 1, y + h - 1);
    draw_yxline2(x + w - 2, y + h - 1, y + h - 2, x + w - 1);
}

fn blue_tabs_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(c));
    draw_rectf(x + 1, y + 1, w - 2, h - 2);
    blue_tabs_frame(x, y, w, h, c);
}

fn use_blue_scheme() {
    app::set_scheme(app::Scheme::Gtk);
    app::set_frame_type_cb(OS_BUTTON_UP_BOX, blue_button_up_box, 2, 2, 4, 4);
    app::set_frame_type_cb(OS_CHECK_DOWN_BOX, aero_check_down_box, 2, 2, 4, 4);
    app::set_frame_type_cb(OS_BUTTON_UP_FRAME, blue_button_up_frame, 2, 2, 4, 4);
    app::set_frame_type_cb(OS_CHECK_DOWN_FRAME, aero_check_down_frame, 2, 2, 4, 4);
    app::set_frame_type_cb(OS_PANEL_THIN_UP_BOX, blue_panel_thin_up_box, 1, 1, 2, 2);
    app::set_frame_type_cb(
        OS_SPACER_THIN_DOWN_BOX,
        blue_spacer_thin_down_box,
        1,
        1,
        2,
        2,
    );
    app::set_frame_type_cb(OS_PANEL_THIN_UP_FRAME, blue_panel_thin_up_frame, 1, 1, 2, 2);
    app::set_frame_type_cb(
        OS_SPACER_THIN_DOWN_FRAME,
        blue_spacer_thin_down_frame,
        1,
        1,
        2,
        2,
    );
    app::set_frame_type_cb(
        OS_RADIO_ROUND_DOWN_BOX,
        aero_radio_round_down_box,
        3,
        3,
        6,
        6,
    );
    app::set_frame_type_cb(OS_HOVERED_UP_BOX, blue_hovered_up_box, 2, 2, 4, 4);
    app::set_frame_type_cb(OS_DEPRESSED_DOWN_BOX, blue_depressed_down_box, 2, 2, 4, 4);
    app::set_frame_type_cb(OS_HOVERED_UP_FRAME, blue_hovered_up_frame, 2, 2, 4, 4);
    app::set_frame_type_cb(
        OS_DEPRESSED_DOWN_FRAME,
        blue_depressed_down_frame,
        2,
        2,
        4,
        4,
    );
    app::set_frame_type_cb(OS_INPUT_THIN_DOWN_BOX, aero_input_thin_down_box, 1, 2, 2, 4);
    app::set_frame_type_cb(
        OS_INPUT_THIN_DOWN_FRAME,
        aero_input_thin_down_frame,
        1,
        2,
        2,
        4,
    );
    app::set_frame_type_cb(
        OS_DEFAULT_BUTTON_UP_BOX,
        blue_default_button_up_box,
        2,
        2,
        4,
        4,
    );
    app::set_frame_type2(OS_DEFAULT_HOVERED_UP_BOX, OS_HOVERED_UP_BOX);
    app::set_frame_type2(OS_DEFAULT_DEPRESSED_DOWN_BOX, OS_DEPRESSED_DOWN_BOX);
    app::set_frame_type2(OS_TOOLBAR_BUTTON_HOVER_BOX, FrameType::FlatBox);
    app::set_frame_type_cb(OS_TABS_BOX, blue_tabs_box, 2, 1, 4, 2);
    app::set_frame_type_cb(OS_SWATCH_BOX, aero_swatch_box, 2, 2, 4, 4);
    app::set_frame_type2(OS_MINI_BUTTON_UP_BOX, OS_BUTTON_UP_BOX);
    app::set_frame_type2(OS_MINI_DEPRESSED_DOWN_BOX, OS_DEPRESSED_DOWN_BOX);
    app::set_frame_type2(OS_MINI_BUTTON_UP_FRAME, OS_BUTTON_UP_FRAME);
    app::set_frame_type2(OS_MINI_DEPRESSED_DOWN_FRAME, OS_DEPRESSED_DOWN_FRAME);
    app::set_frame_type2(FrameType::UpBox, OS_BUTTON_UP_BOX);
    app::set_frame_type2(FrameType::DownBox, OS_CHECK_DOWN_BOX);
    app::set_frame_type2(FrameType::RoundDownBox, OS_RADIO_ROUND_DOWN_BOX);
    app::set_frame_type2(OS_BG_BOX, FrameType::FlatBox);
}

fn use_blue_colors() {
    app::background(0xD9, 0xE4, 0xF1);
    app::background2(0xFF, 0xFF, 0xFF);
    app::foreground(0x1E, 0x39, 0x5B);
    app::set_color(Color::Inactive, 0x35, 0x49, 0x6A);
    app::set_color(Color::Selection, 0x33, 0x33, 0x33);
    app::set_color(Color::Free, 0xEA, 0xF1, 0xFA);
    Tooltip::set_color(Color::from_rgb(0xFF, 0xFF, 0xFF));
    Tooltip::set_text_color(Color::Foreground);
}

pub(crate) fn use_blue_theme() {
    use_blue_scheme();
    use_blue_colors();
    use_native_settings();
}
