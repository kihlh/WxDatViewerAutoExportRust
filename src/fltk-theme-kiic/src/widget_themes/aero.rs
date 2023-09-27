use super::*;
use fltk::{app, enums::Color, misc::Tooltip};

pub(crate) fn aero_button_up_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // outer border
    set_draw_color(activated_color(Color::from_rgb(0x70, 0x70, 0x70)));
    draw_xyline(x + 2, y, x + w - 3);
    draw_xyline(x + 2, y + h - 1, x + w - 3);
    draw_yxline(x, y + 2, y + h - 3);
    draw_yxline(x + w - 1, y + 2, y + h - 3);
    // top inner borders
    set_draw_color(activated_color(Color::from_rgb(0xFB, 0xFB, 0xFB)));
    draw_xyline(x + 2, y + 1, x + w - 3);
    draw_yxline(x + 1, y + 2, y + h / 2 - 1);
    draw_yxline(x + w - 2, y + 2, y + h / 2 - 1);
    draw_point(x + 2, y + 2);
    draw_point(x + w - 3, y + 2);
    // bottom inner borders
    set_draw_color(activated_color(Color::from_rgb(0xF2, 0xF2, 0xF2)));
    draw_yxline(x + 1, y + h / 2, y + h - 3);
    draw_yxline(x + w - 2, y + h / 2, y + h - 3);
    draw_xyline(x + 2, y + h - 2, x + w - 3);
    draw_point(x + 2, y + h - 3);
    draw_point(x + w - 3, y + h - 3);
    // top corners
    set_draw_color(activated_color(Color::from_rgb(0x90, 0x90, 0x90)));
    draw_xyline2(x, y + 1, x + 1, y);
    draw_yxline2(x + w - 2, y, y + 1, x + w - 1);
    // bottom corners
    set_draw_color(activated_color(Color::from_rgb(0x88, 0x88, 0x88)));
    draw_xyline2(x, y + h - 2, x + 1, y + h - 1);
    draw_yxline2(x + w - 2, y + h - 1, y + h - 2, x + w - 1);
}

pub(crate) fn aero_button_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    if w >= h {
        // top gradient
        vertical_gradient(
            x + 2,
            y + 2,
            x + w - 3,
            y + h / 2 - 1,
            activated_color(Color::from_rgb(0xF1, 0xF1, 0xF1)),
            activated_color(Color::from_rgb(0xEA, 0xEA, 0xEA)),
        );
        // bottom gradient
        vertical_gradient(
            x + 2,
            y + h / 2,
            x + w - 3,
            y + h - 3,
            activated_color(Color::from_rgb(0xDC, 0xDC, 0xDC)),
            activated_color(Color::from_rgb(0xCE, 0xCE, 0xCE)),
        );
    } else {
        // left gradient
        horizontal_gradient(
            x + 2,
            y + 2,
            x + w / 2 - 1,
            y + h - 3,
            activated_color(Color::from_rgb(0xF1, 0xF1, 0xF1)),
            activated_color(Color::from_rgb(0xEA, 0xEA, 0xEA)),
        );
        // right gradient
        horizontal_gradient(
            x + w / 2,
            y + 2,
            x + w - 3,
            y + h - 3,
            activated_color(Color::from_rgb(0xDC, 0xDC, 0xDC)),
            activated_color(Color::from_rgb(0xCE, 0xCE, 0xCE)),
        );
    }
    aero_button_up_frame(x, y, w, h, c);
}

pub(crate) fn aero_check_down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // outer border
    set_draw_color(activated_color(Color::from_rgb(0x8F, 0x8F, 0x8F)));
    draw_rect(x, y, w, h);
    // middle border
    set_draw_color(activated_color(Color::from_rgb(0xF4, 0xF4, 0xF4)));
    draw_rect(x + 1, y + 1, w - 2, h - 2);
    // top and left inner borders
    set_draw_color(activated_color(Color::from_rgb(0xAE, 0xB3, 0xB9)));
    draw_yxline2(x + 2, y + h - 3, y + 2, x + w - 3);
    // bottom and right inner borders
    set_draw_color(activated_color(Color::from_rgb(0xE9, 0xE9, 0xEA)));
    draw_xyline2(x + 3, y + h - 3, x + w - 3, y + 3);
}

pub(crate) fn aero_check_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(Color::from_rgb(0xF5, 0xF5, 0xF5)));
    draw_rectf(x + 3, y + 3, w - 6, h - 6);
    aero_check_down_frame(x, y, w, h, c);
}

pub(crate) fn aero_panel_thin_up_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top and left borders
    set_draw_color(activated_color(Color::from_rgb(0xFA, 0xFA, 0xFA)));
    draw_yxline2(x, y + h - 2, y, x + w - 2);
    // bottom and right borders
    set_draw_color(activated_color(Color::from_rgb(0xC9, 0xC9, 0xC9)));
    draw_xyline2(x, y + h - 1, x + w - 1, y);
}

pub(crate) fn aero_panel_thin_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(c));
    draw_rectf(x + 1, y + 1, w - 2, h - 2);
    aero_panel_thin_up_frame(x, y, w, h, c);
}

pub(crate) fn aero_spacer_thin_down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top and left borders
    set_draw_color(activated_color(Color::from_rgb(0xC9, 0xC9, 0xC9)));
    draw_yxline2(x, y + h - 2, y, x + w - 2);
    // bottom and right borders
    set_draw_color(activated_color(Color::from_rgb(0xFA, 0xFA, 0xFA)));
    draw_xyline2(x, y + h - 1, x + w - 1, y);
}

pub(crate) fn aero_spacer_thin_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(c));
    draw_rectf(x + 1, y + 1, w - 2, h - 2);
    aero_spacer_thin_down_frame(x, y, w, h, c);
}

pub(crate) fn aero_radio_round_down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // middle border
    set_draw_color(activated_color(Color::from_rgb(0xF4, 0xF4, 0xF4)));
    draw_arc(x + 1, y + 1, w - 2, h - 2, 0.0, 360.0);
    // outer border
    set_draw_color(activated_color(Color::from_rgb(0x8F, 0x8F, 0x8F)));
    draw_arc(x, y, w, h, 0.0, 360.0);
    // top and left inner border
    set_draw_color(activated_color(Color::from_rgb(0xAE, 0xAE, 0xAE)));
    draw_arc(x + 2, y + 2, w - 4, h - 4, 45.0, 225.0);
    // bottom and right inner border
    set_draw_color(activated_color(Color::from_rgb(0xE4, 0xE4, 0xE4)));
    draw_arc(x + 2, y + 2, w - 4, h - 4, -135.0, 45.0);
}

pub(crate) fn aero_radio_round_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(Color::from_rgb(0xF5, 0xF5, 0xF5)));
    draw_pie(x + 2, y + 2, w - 4, h - 4, 0.0, 360.0);
    aero_radio_round_down_frame(x, y, w, h, c);
}

pub(crate) fn aero_hovered_up_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // outer border
    set_draw_color(activated_color(Color::from_rgb(0x3C, 0x7F, 0xB0)));
    draw_xyline(x + 2, y, x + w - 3);
    draw_xyline(x + 2, y + h - 1, x + w - 3);
    draw_yxline(x, y + 2, y + h - 3);
    draw_yxline(x + w - 1, y + 2, y + h - 3);
    // top inner borders
    set_draw_color(activated_color(Color::from_rgb(0xF9, 0xFC, 0xFD)));
    draw_xyline(x + 2, y + 1, x + w - 3);
    draw_yxline(x + 1, y + 2, y + h / 2 - 1);
    draw_yxline(x + w - 2, y + 2, y + h / 2 - 1);
    draw_point(x + 2, y + 2);
    draw_point(x + w - 3, y + 2);
    // bottom inner borders
    set_draw_color(activated_color(Color::from_rgb(0xE7, 0xF4, 0xFB)));
    draw_yxline(x + 1, y + h / 2, y + h - 3);
    draw_yxline(x + w - 2, y + h / 2, y + h - 3);
    draw_xyline(x + 2, y + h - 2, x + w - 3);
    draw_point(x + 2, y + h - 3);
    draw_point(x + w - 3, y + h - 3);
    // top corners
    set_draw_color(activated_color(Color::from_rgb(0x69, 0x9C, 0xC2)));
    draw_xyline2(x, y + 1, x + 1, y);
    draw_yxline2(x + w - 2, y, y + 1, x + w - 1);
    // bottom corners
    set_draw_color(activated_color(Color::from_rgb(0x64, 0x9A, 0xC2)));
    draw_xyline2(x, y + h - 2, x + 1, y + h - 1);
    draw_yxline2(x + w - 2, y + h - 1, y + h - 2, x + w - 1);
}

pub(crate) fn aero_hovered_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top gradient
    vertical_gradient(
        x + 2,
        y + 2,
        x + w - 3,
        y + h / 2 - 1,
        activated_color(Color::from_rgb(0xE9, 0xF5, 0xFC)),
        activated_color(Color::from_rgb(0xD8, 0xEF, 0xFB)),
    );
    // bottom gradient
    vertical_gradient(
        x + 2,
        y + h / 2,
        x + w - 3,
        y + h - 3,
        activated_color(Color::from_rgb(0xBD, 0xE5, 0xFC)),
        activated_color(Color::from_rgb(0xA7, 0xD8, 0xF4)),
    );
    aero_hovered_up_frame(x, y, w, h, c);
}

pub(crate) fn aero_depressed_down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // outer border
    set_draw_color(activated_color(Color::from_rgb(0x2C, 0x62, 0x8B)));
    draw_xyline(x + 2, y, x + w - 3);
    draw_xyline(x + 2, y + h - 1, x + w - 3);
    draw_yxline(x, y + 2, y + h - 3);
    draw_yxline(x + w - 1, y + 2, y + h - 3);
    // top inner borders
    set_draw_color(activated_color(Color::from_rgb(0x9D, 0xAF, 0xB9)));
    draw_xyline(x + 2, y + 1, x + w - 3);
    draw_yxline(x + 1, y + 2, y + h / 2 - 1);
    draw_yxline(x + w - 2, y + 2, y + h / 2 - 1);
    draw_point(x + 2, y + 2);
    draw_point(x + w - 3, y + 2);
    // bottom inner borders
    set_draw_color(activated_color(Color::from_rgb(0x55, 0x92, 0xB5)));
    draw_yxline(x + 1, y + h / 2, y + h - 3);
    draw_yxline(x + w - 2, y + h / 2, y + h - 3);
    // corners
    set_draw_color(activated_color(Color::from_rgb(0x33, 0x56, 0x71)));
    draw_xyline2(x, y + 1, x + 1, y);
    draw_xyline2(x, y + h - 2, x + 1, y + h - 1);
    draw_yxline2(x + w - 2, y, y + 1, x + w - 1);
    draw_yxline2(x + w - 2, y + h - 1, y + h - 2, x + w - 1);
}

pub(crate) fn aero_depressed_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top gradient
    vertical_gradient(
        x + 2,
        y + 2,
        x + w - 3,
        y + h / 2 - 1,
        activated_color(Color::from_rgb(0xE4, 0xF3, 0xFB)),
        activated_color(Color::from_rgb(0xC4, 0xE5, 0xF6)),
    );
    // bottom gradient
    vertical_gradient(
        x + 2,
        y + h / 2,
        x + w - 3,
        y + h - 2,
        activated_color(Color::from_rgb(0x98, 0xD1, 0xEF)),
        activated_color(Color::from_rgb(0x68, 0xB3, 0xDB)),
    );
    aero_depressed_down_frame(x, y, w, h, c);
}

pub(crate) fn aero_input_thin_down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top border
    set_draw_color(activated_color(Color::from_rgb(0xAA, 0xAC, 0xB2)));
    draw_xyline(x + 1, y, x + w - 2);
    // side borders
    set_draw_color(activated_color(Color::from_rgb(0xDA, 0xDE, 0xE5)));
    draw_yxline(x, y + 1, y + h - 2);
    draw_yxline(x + w - 1, y + 1, y + h - 2);
    // bottom border
    set_draw_color(activated_color(Color::from_rgb(0xE2, 0xE8, 0xEE)));
    draw_xyline(x + 1, y + h - 1, x + w - 2);
    // inner corners
    set_draw_color(activated_color(Color::from_rgb(0xE8, 0xEB, 0xEF)));
    draw_point(x + 1, y + 1);
    draw_point(x + w - 2, y + 1);
    draw_point(x + 1, y + h - 2);
    draw_point(x + w - 2, y + h - 2);
}

pub(crate) fn aero_input_thin_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(c));
    draw_rectf(x + 1, y + 1, w - 2, h - 2);
    aero_input_thin_down_frame(x, y, w, h, c);
}

pub(crate) fn aero_default_button_up_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // outer border
    set_draw_color(activated_color(Color::from_rgb(0x3C, 0x7F, 0xB1)));
    draw_xyline(x + 2, y, x + w - 3);
    draw_xyline(x + 2, y + h - 1, x + w - 3);
    draw_yxline(x, y + 2, y + h - 3);
    draw_yxline(x + w - 1, y + 2, y + h - 3);
    // top inner borders
    set_draw_color(activated_color(Color::from_rgb(0x40, 0xD7, 0xFC)));
    draw_xyline(x + 2, y + 1, x + w - 3);
    draw_yxline(x + 1, y + 2, y + h / 2 - 1);
    draw_yxline(x + w - 2, y + 2, y + h / 2 - 1);
    draw_point(x + 2, y + 2);
    draw_point(x + w - 3, y + 2);
    // bottom inner borders
    set_draw_color(activated_color(Color::from_rgb(0x35, 0xCE, 0xF4)));
    draw_yxline(x + 1, y + h / 2, y + h - 3);
    draw_yxline(x + w - 2, y + h / 2, y + h - 3);
    draw_xyline(x + 2, y + h - 2, x + w - 3);
    draw_point(x + 2, y + h - 3);
    draw_point(x + w - 3, y + h - 3);
    // top innermost borders
    set_draw_color(activated_color(Color::from_rgb(0xD2, 0xEE, 0xF6)));
    draw_xyline(x + 3, y + 2, x + w - 4);
    draw_yxline(x + 2, y + 3, y + h / 2 - 1);
    draw_yxline(x + w - 3, y + 3, y + h / 2 - 1);
    // bottom innermost borders
    set_draw_color(activated_color(Color::from_rgb(0xB0, 0xD1, 0xDC)));
    draw_yxline(x + 2, y + h / 2, y + h - 4);
    draw_yxline(x + w - 3, y + h / 2, y + h - 4);
    draw_xyline(x + 3, y + h - 3, x + w - 4);
    // top corners
    set_draw_color(activated_color(Color::from_rgb(0x3A, 0x93, 0xC2)));
    draw_xyline2(x, y + 1, x + 1, y);
    draw_yxline2(x + w - 2, y, y + 1, x + w - 1);
    // bottom corners
    set_draw_color(activated_color(Color::from_rgb(0x38, 0x91, 0xC1)));
    draw_xyline2(x, y + h - 2, x + 1, y + h - 1);
    draw_yxline2(x + w - 2, y + h - 1, y + h - 2, x + w - 1);
}

pub(crate) fn aero_default_button_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top gradient
    vertical_gradient(
        x + 3,
        y + 3,
        x + w - 4,
        y + h / 2 - 1,
        activated_color(Color::from_rgb(0xF0, 0xF3, 0xF5)),
        activated_color(Color::from_rgb(0xE9, 0xEE, 0xF1)),
    );
    // bottom gradient
    vertical_gradient(
        x + 3,
        y + h / 2,
        x + w - 4,
        y + h - 4,
        activated_color(Color::from_rgb(0xD7, 0xE1, 0xE7)),
        activated_color(Color::from_rgb(0xC8, 0xD5, 0xDD)),
    );
    aero_default_button_up_frame(x, y, w, h, c);
}

pub(crate) fn aero_default_hovered_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top gradient
    vertical_gradient(
        x + 2,
        y + 2,
        x + w - 3,
        y + h / 2 - 1,
        activated_color(Color::from_rgb(0xE9, 0xF5, 0xFC)),
        activated_color(Color::from_rgb(0xD8, 0xEF, 0xFB)),
    );
    // bottom gradient
    vertical_gradient(
        x + 2,
        y + h / 2,
        x + w - 3,
        y + h - 3,
        activated_color(Color::from_rgb(0xBD, 0xE5, 0xFC)),
        activated_color(Color::from_rgb(0xA7, 0xD8, 0xF4)),
    );
    aero_default_button_up_frame(x, y, w, h, c);
}

pub(crate) fn aero_tabs_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(Color::from_rgb(0x88, 0x8B, 0x94)));
    draw_rect(x, y, w, h);
}

pub(crate) fn aero_tabs_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(c));
    draw_rectf(x + 1, y + 1, w - 2, h - 2);
    aero_tabs_frame(x, y, w, h, c);
}

pub(crate) fn aero_swatch_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // outer border
    set_draw_color(activated_color(Color::from_rgb(0xA0, 0xA0, 0xA0)));
    draw_rect(x, y, w, h);
    // inner border
    set_draw_color(activated_color(Color::from_rgb(0xFF, 0xFF, 0xFF)));
    draw_rect(x + 1, y + 1, w - 2, h - 2);
}

pub(crate) fn aero_swatch_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(c));
    draw_rectf(x + 2, y + 2, w - 4, h - 4);
    aero_swatch_frame(x, y, w, h, c);
}

pub(crate) fn use_aero_scheme() {
    app::set_scheme(app::Scheme::Gtk);
    app::set_frame_type_cb(OS_BUTTON_UP_BOX, aero_button_up_box, 2, 2, 4, 4);
    app::set_frame_type_cb(OS_CHECK_DOWN_BOX, aero_check_down_box, 2, 2, 4, 4);
    app::set_frame_type_cb(OS_BUTTON_UP_FRAME, aero_button_up_frame, 2, 2, 4, 4);
    app::set_frame_type_cb(OS_CHECK_DOWN_FRAME, aero_check_down_frame, 2, 2, 4, 4);
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
        aero_radio_round_down_box,
        3,
        3,
        6,
        6,
    );
    app::set_frame_type_cb(OS_HOVERED_UP_BOX, aero_hovered_up_box, 2, 2, 4, 4);
    app::set_frame_type_cb(OS_DEPRESSED_DOWN_BOX, aero_depressed_down_box, 2, 2, 4, 4);
    app::set_frame_type_cb(OS_HOVERED_UP_FRAME, aero_hovered_up_frame, 2, 2, 4, 4);
    app::set_frame_type_cb(
        OS_DEPRESSED_DOWN_FRAME,
        aero_depressed_down_frame,
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
        aero_default_button_up_box,
        2,
        2,
        4,
        4,
    );
    app::set_frame_type_cb(
        OS_DEFAULT_HOVERED_UP_BOX,
        aero_default_hovered_up_box,
        2,
        2,
        4,
        4,
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
    // app::set_frame_type_cb(OS_BG_DOWN_BOX, OS_BG_BOX);
}

pub(crate) fn use_aero_colors() {
    app::background(0xF0, 0xF0, 0xF0);
    app::background2(0xFF, 0xFF, 0xFF);
    app::foreground(0x00, 0x00, 0x00);
    app::set_color(Color::Inactive, 0x6F, 0x6F, 0x6F);
    app::set_color(Color::Selection, 0x33, 0x99, 0xFF);
    app::set_color(Color::Free, 0xFF, 0xFF, 0xFF);
    Tooltip::set_color(Color::from_rgb(0xFF, 0xFF, 0xF0));
    Tooltip::set_text_color(Color::Foreground);
}

pub(crate) fn use_aero_theme() {
    use_aero_scheme();
    use_aero_colors();
    use_native_settings();
}
