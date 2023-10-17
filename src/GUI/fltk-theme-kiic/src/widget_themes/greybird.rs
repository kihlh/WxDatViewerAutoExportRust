use super::*;
use fltk::{app, enums::Color, misc::Tooltip};

fn greybird_button_up_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top outer border
    set_draw_color(activated_color(Color::from_rgb(0xA6, 0xA6, 0xA6)));
    draw_xyline(x + 2, y, x + w - 3);
    // side outer borders
    set_draw_color(activated_color(Color::from_rgb(0x96, 0x96, 0x96)));
    draw_yxline(x, y + 2, y + h - 3);
    draw_yxline(x + w - 1, y + 2, y + h - 3);
    // bottom outer border
    set_draw_color(activated_color(Color::from_rgb(0x87, 0x87, 0x87)));
    draw_xyline(x + 2, y + h - 1, x + w - 3);
    // top inner border
    set_draw_color(activated_color(Color::from_rgb(0xEE, 0xEE, 0xEE)));
    draw_xyline(x + 2, y + 1, x + w - 3);
    // side inner borders
    set_draw_color(activated_color(Color::from_rgb(0xE4, 0xE4, 0xE4)));
    draw_yxline(x + 1, y + 2, y + h - 3);
    draw_yxline(x + w - 2, y + 2, y + h - 3);
    // top corners
    set_draw_color(activated_color(Color::from_rgb(0xB8, 0xB8, 0xB8)));
    draw_xyline2(x, y + 1, x + 1, y);
    draw_yxline2(x + w - 2, y, y + 1, x + w - 1);
    // bottom corners
    set_draw_color(activated_color(Color::from_rgb(0xA0, 0xA0, 0xA0)));
    draw_xyline2(x, y + h - 2, x + 1, y + h - 1);
    draw_yxline2(x + w - 2, y + h - 1, y + h - 2, x + w - 1);
}

fn greybird_button_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    if w >= h {
        vertical_gradient(
            x + 2,
            y + 2,
            x + w - 3,
            y + h - 2,
            activated_color(Color::from_rgb(0xDB, 0xDB, 0xDB)),
            activated_color(Color::from_rgb(0xCC, 0xCC, 0xCC)),
        );
    } else {
        horizontal_gradient(
            x + 2,
            y + 2,
            x + w - 3,
            y + h - 2,
            activated_color(Color::from_rgb(0xDB, 0xDB, 0xDB)),
            activated_color(Color::from_rgb(0xCC, 0xCC, 0xCC)),
        );
    }
    greybird_button_up_frame(x, y, w, h, c);
}

fn greybird_check_down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top border
    set_draw_color(activated_color(Color::from_rgb(0x80, 0x80, 0x80)));
    draw_xyline(x + 2, y, x + w - 3);
    // side borders
    set_draw_color(activated_color(Color::from_rgb(0x89, 0x89, 0x89)));
    draw_yxline(x, y + 2, y + h - 3);
    draw_yxline(x + w - 1, y + 2, y + h - 3);
    // bottom border
    set_draw_color(activated_color(Color::from_rgb(0x90, 0x90, 0x90)));
    draw_xyline(x + 2, y + h - 1, x + w - 3);
    // top corners
    set_draw_color(activated_color(Color::from_rgb(0xA6, 0xA6, 0xA6)));
    draw_xyline2(x, y + 1, x + 1, y);
    draw_yxline2(x + w - 2, y, y + 1, x + w - 1);
    // bottom corners
    set_draw_color(activated_color(Color::from_rgb(0xB0, 0xB0, 0xB0)));
    draw_xyline2(x, y + h - 2, x + 1, y + h - 1);
    draw_yxline2(x + w - 2, y + h - 1, y + h - 2, x + w - 1);
}

fn greybird_panel_thin_up_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top and left borders
    set_draw_color(activated_color(Color::from_rgb(0xDA, 0xDA, 0xDA)));
    draw_yxline2(x, y + h - 2, y, x + w - 2);
    // bottom and right borders
    set_draw_color(activated_color(Color::from_rgb(0xC1, 0xC1, 0xC1)));
    draw_xyline2(x, y + h - 1, x + w - 1, y);
}

fn greybird_check_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(c));
    draw_rectf(x + 1, y + 1, w - 2, h - 2);
    greybird_check_down_frame(x, y, w, h, c);
}

fn greybird_panel_thin_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(c));
    draw_rectf(x + 1, y + 1, w - 2, h - 2);
    greybird_panel_thin_up_frame(x, y, w, h, c);
}

fn greybird_spacer_thin_down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top and left borders
    set_draw_color(activated_color(Color::from_rgb(0xBA, 0xBA, 0xBA)));
    draw_yxline2(x, y + h - 2, y, x + w - 2);
    // bottom and right borders
    set_draw_color(activated_color(Color::from_rgb(0xDA, 0xDA, 0xDA)));
    draw_xyline2(x, y + h - 1, x + w - 1, y);
}

fn greybird_spacer_thin_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(c));
    draw_rectf(x + 1, y + 1, w - 2, h - 2);
    greybird_spacer_thin_down_frame(x, y, w, h, c);
}

fn greybird_radio_round_down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(Color::from_rgb(0x80, 0x80, 0x80)));
    draw_arc(x, y, w, h, 0.0, 360.0);
}

fn greybird_radio_round_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(c);
    draw_pie(x + 1, y + 1, w - 2, h - 2, 0.0, 360.0);
    greybird_radio_round_down_frame(x, y, w, h, c);
}

fn greybird_hovered_up_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top outer border
    set_draw_color(activated_color(Color::from_rgb(0xAE, 0xAE, 0xAE)));
    draw_xyline(x + 2, y, x + w - 3);
    // side outer borders
    set_draw_color(activated_color(Color::from_rgb(0x9E, 0x9E, 0x9E)));
    draw_yxline(x, y + 2, y + h - 3);
    draw_yxline(x + w - 1, y + 2, y + h - 3);
    // bottom outer border
    set_draw_color(activated_color(Color::from_rgb(0x8E, 0x8E, 0x8E)));
    draw_xyline(x + 2, y + h - 1, x + w - 3);
    // top inner border
    set_draw_color(activated_color(Color::from_rgb(0xF3, 0xF3, 0xF3)));
    draw_xyline(x + 2, y + 1, x + w - 3);
    // side inner borders
    set_draw_color(activated_color(Color::from_rgb(0xED, 0xED, 0xED)));
    draw_yxline(x + 1, y + 2, y + h - 3);
    draw_yxline(x + w - 2, y + 2, y + h - 3);
    // top corners
    set_draw_color(activated_color(Color::from_rgb(0xC0, 0xC0, 0xC0)));
    draw_xyline2(x, y + 1, x + 1, y);
    draw_yxline2(x + w - 2, y, y + 1, x + w - 1);
    // bottom corners
    set_draw_color(activated_color(Color::from_rgb(0xA7, 0xA7, 0xA7)));
    draw_xyline2(x, y + h - 2, x + 1, y + h - 1);
    draw_yxline2(x + w - 2, y + h - 1, y + h - 2, x + w - 1);
}

fn greybird_hovered_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    vertical_gradient(
        x + 2,
        y + 2,
        x + w - 3,
        y + h - 2,
        activated_color(Color::from_rgb(0xE6, 0xE6, 0xE6)),
        activated_color(Color::from_rgb(0xD6, 0xD6, 0xD6)),
    );
    greybird_hovered_up_frame(x, y, w, h, c);
}

fn greybird_depressed_down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top outer border
    set_draw_color(activated_color(Color::from_rgb(0x8A, 0x8A, 0x8A)));
    draw_xyline(x + 2, y, x + w - 3);
    // side outer borders
    set_draw_color(activated_color(Color::from_rgb(0x7D, 0x7D, 0x7D)));
    draw_yxline(x, y + 2, y + h - 3);
    draw_yxline(x + w - 1, y + 2, y + h - 3);
    // bottom outer border
    set_draw_color(activated_color(Color::from_rgb(0x71, 0x71, 0x71)));
    draw_xyline(x + 2, y + h - 1, x + w - 3);
    // top corners
    set_draw_color(activated_color(Color::from_rgb(0x98, 0x98, 0x98)));
    draw_xyline2(x, y + 1, x + 1, y);
    draw_yxline2(x + w - 2, y, y + 1, x + w - 1);
    // bottom corners
    set_draw_color(activated_color(Color::from_rgb(0x88, 0x88, 0x88)));
    draw_xyline2(x, y + h - 2, x + 1, y + h - 1);
    draw_yxline2(x + w - 2, y + h - 1, y + h - 2, x + w - 1);
}

fn greybird_depressed_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top gradient
    vertical_gradient(
        x + 1,
        y + 1,
        x + w - 2,
        y + 4,
        activated_color(Color::from_rgb(0xAF, 0xAF, 0xAF)),
        activated_color(Color::from_rgb(0xB4, 0xB4, 0xB4)),
    );
    vertical_gradient(
        x + 1,
        y + 5,
        x + w - 2,
        y + h - 1,
        activated_color(Color::from_rgb(0xB4, 0xB4, 0xB4)),
        activated_color(Color::from_rgb(0xAA, 0xAA, 0xAA)),
    );
    greybird_depressed_down_frame(x, y, w, h, c);
}

fn greybird_input_thin_down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top outer border
    set_draw_color(activated_color(Color::from_rgb(0x84, 0x84, 0x84)));
    draw_xyline(x + 2, y, x + w - 3);
    // side outer borders
    set_draw_color(activated_color(Color::from_rgb(0x97, 0x97, 0x97)));
    draw_yxline(x, y + 2, y + h - 3);
    draw_yxline(x + w - 1, y + 2, y + h - 3);
    // bottom outer border
    set_draw_color(activated_color(Color::from_rgb(0xAA, 0xAA, 0xAA)));
    draw_xyline(x + 2, y + h - 1, x + w - 3);
    // inner border
    set_draw_color(activated_color(Color::from_rgb(0xEC, 0xEC, 0xEC)));
    draw_xyline(x + 2, y + 1, x + w - 3);
    draw_yxline(x + 1, y + 2, y + h - 3);
    draw_yxline(x + w - 2, y + 2, y + h - 3);
    // top corners
    set_draw_color(activated_color(Color::from_rgb(0xA4, 0xA4, 0xA4)));
    draw_xyline2(x, y + 1, x + 1, y);
    draw_yxline2(x + w - 2, y, y + 1, x + w - 1);
    // bottom corners
    set_draw_color(activated_color(Color::from_rgb(0xBE, 0xBE, 0xBE)));
    draw_xyline2(x, y + h - 2, x + 1, y + h - 1);
    draw_yxline2(x + w - 2, y + h - 1, y + h - 2, x + w - 1);
}

fn greybird_input_thin_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(c));
    draw_rectf(x + 2, y + 2, w - 4, h - 3);
    greybird_input_thin_down_frame(x, y, w, h, c);
}

fn greybird_default_button_up_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top outer border
    set_draw_color(activated_color(Color::from_rgb(0x69, 0x82, 0x9D)));
    draw_xyline(x + 2, y, x + w - 3);
    // side outer borders
    set_draw_color(activated_color(Color::from_rgb(0x61, 0x77, 0x8E)));
    draw_yxline(x, y + 2, y + h - 3);
    draw_yxline(x + w - 1, y + 2, y + h - 3);
    // bottom outer border
    set_draw_color(activated_color(Color::from_rgb(0x59, 0x6B, 0x7D)));
    draw_xyline(x + 2, y + h - 1, x + w - 3);
    // top inner border
    set_draw_color(activated_color(Color::from_rgb(0x88, 0xB7, 0xE9)));
    draw_xyline(x + 2, y + 1, x + w - 3);
    // side inner borders
    set_draw_color(activated_color(Color::from_rgb(0x79, 0xAC, 0xE1)));
    draw_yxline(x + 1, y + 2, y + h - 3);
    draw_yxline(x + w - 2, y + 2, y + h - 3);
    // top corners
    set_draw_color(activated_color(Color::from_rgb(0x76, 0x99, 0xBE)));
    draw_xyline2(x, y + 1, x + 1, y);
    draw_yxline2(x + w - 2, y, y + 1, x + w - 1);
    // bottom corners
    set_draw_color(activated_color(Color::from_rgb(0x5D, 0x81, 0xA6)));
    draw_xyline2(x, y + h - 2, x + 1, y + h - 1);
    draw_yxline2(x + w - 2, y + h - 1, y + h - 2, x + w - 1);
}

fn greybird_default_button_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    vertical_gradient(
        x + 2,
        y + 2,
        x + w - 3,
        y + h - 2,
        activated_color(Color::from_rgb(0x72, 0xA7, 0xDF)),
        activated_color(Color::from_rgb(0x63, 0x9C, 0xD7)),
    );
    greybird_default_button_up_frame(x, y, w, h, c);
}

fn greybird_default_depressed_down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top outer border
    set_draw_color(activated_color(Color::from_rgb(0x58, 0x71, 0x8C)));
    draw_xyline(x + 2, y, x + w - 3);
    // side outer borders
    set_draw_color(activated_color(Color::from_rgb(0x50, 0x66, 0x7D)));
    draw_yxline(x, y + 2, y + h - 3);
    draw_yxline(x + w - 1, y + 2, y + h - 3);
    // bottom outer border
    set_draw_color(activated_color(Color::from_rgb(0x48, 0x5A, 0x6C)));
    draw_xyline(x + 2, y + h - 1, x + w - 3);
    // top corners
    set_draw_color(activated_color(Color::from_rgb(0x65, 0x88, 0xAD)));
    draw_xyline2(x, y + 1, x + 1, y);
    draw_yxline2(x + w - 2, y, y + 1, x + w - 1);
    // bottom corners
    set_draw_color(activated_color(Color::from_rgb(0x4C, 0x70, 0x95)));
    draw_xyline2(x, y + h - 2, x + 1, y + h - 1);
    draw_yxline2(x + w - 2, y + h - 1, y + h - 2, x + w - 1);
}

fn greybird_default_depressed_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top gradient
    vertical_gradient(
        x + 1,
        y + 1,
        x + w - 2,
        y + 4,
        activated_color(Color::from_rgb(0x53, 0x83, 0xB2)),
        activated_color(Color::from_rgb(0x5C, 0x92, 0xC7)),
    );
    vertical_gradient(
        x + 1,
        y + 5,
        x + w - 2,
        y + h - 1,
        activated_color(Color::from_rgb(0x5C, 0x92, 0xC7)),
        activated_color(Color::from_rgb(0x4D, 0x7B, 0xA5)),
    );
    greybird_default_depressed_down_frame(x, y, w, h, c);
}

fn greybird_tabs_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top outer border
    set_draw_color(activated_color(Color::from_rgb(0xA6, 0xA6, 0xA6)));
    draw_xyline(x + 2, y, x + w - 3);
    // side outer borders
    set_draw_color(activated_color(Color::from_rgb(0x96, 0x96, 0x96)));
    draw_yxline(x, y + 2, y + h - 3);
    draw_yxline(x + w - 1, y + 2, y + h - 3);
    // bottom outer border
    set_draw_color(activated_color(Color::from_rgb(0x87, 0x87, 0x87)));
    draw_xyline(x + 2, y + h - 1, x + w - 3);
    // top inner border
    set_draw_color(activated_color(Color::from_rgb(0xEE, 0xEE, 0xEE)));
    draw_xyline(x + 2, y + 1, x + w - 3);
    // side inner borders
    set_draw_color(activated_color(Color::from_rgb(0xE4, 0xE4, 0xE4)));
    draw_yxline(x + 1, y + 2, y + h - 3);
    draw_yxline(x + w - 2, y + 2, y + h - 3);
    // top corners
    set_draw_color(activated_color(Color::from_rgb(0xB8, 0xB8, 0xB8)));
    draw_xyline2(x, y + 1, x + 1, y);
    draw_yxline2(x + w - 2, y, y + 1, x + w - 1);
    // bottom corners
    set_draw_color(activated_color(Color::from_rgb(0xA0, 0xA0, 0xA0)));
    draw_xyline2(x, y + h - 2, x + 1, y + h - 1);
    draw_yxline2(x + w - 2, y + h - 1, y + h - 2, x + w - 1);
}

fn greybird_tabs_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(Color::from_rgb(0xD9, 0xD9, 0xD9)));
    draw_rectf(x + 2, y + 2, w - 3, h - 2);
    greybird_tabs_frame(x, y, w, h, c);
}

fn use_greybird_scheme() {
    app::set_scheme(app::Scheme::Gtk);
    app::set_frame_type_cb(OS_BUTTON_UP_BOX, greybird_button_up_box, 2, 2, 4, 4);
    app::set_frame_type_cb(OS_CHECK_DOWN_BOX, greybird_check_down_box, 1, 1, 2, 2);
    app::set_frame_type_cb(OS_BUTTON_UP_FRAME, greybird_button_up_frame, 2, 2, 4, 4);
    app::set_frame_type_cb(OS_CHECK_DOWN_FRAME, greybird_check_down_frame, 1, 1, 2, 2);
    app::set_frame_type_cb(OS_PANEL_THIN_UP_BOX, greybird_panel_thin_up_box, 1, 1, 2, 2);
    app::set_frame_type_cb(
        OS_SPACER_THIN_DOWN_BOX,
        greybird_spacer_thin_down_box,
        1,
        1,
        2,
        2,
    );
    app::set_frame_type_cb(
        OS_PANEL_THIN_UP_FRAME,
        greybird_panel_thin_up_frame,
        1,
        1,
        2,
        2,
    );
    app::set_frame_type_cb(
        OS_SPACER_THIN_DOWN_FRAME,
        greybird_spacer_thin_down_frame,
        1,
        1,
        2,
        2,
    );
    app::set_frame_type_cb(
        OS_RADIO_ROUND_DOWN_BOX,
        greybird_radio_round_down_box,
        3,
        3,
        6,
        6,
    );
    app::set_frame_type_cb(OS_HOVERED_UP_BOX, greybird_hovered_up_box, 2, 2, 4, 4);
    app::set_frame_type_cb(
        OS_DEPRESSED_DOWN_BOX,
        greybird_depressed_down_box,
        2,
        2,
        4,
        4,
    );
    app::set_frame_type_cb(OS_HOVERED_UP_FRAME, greybird_hovered_up_frame, 2, 2, 4, 4);
    app::set_frame_type_cb(
        OS_DEPRESSED_DOWN_FRAME,
        greybird_depressed_down_frame,
        2,
        2,
        4,
        4,
    );
    app::set_frame_type_cb(
        OS_INPUT_THIN_DOWN_BOX,
        greybird_input_thin_down_box,
        2,
        3,
        4,
        6,
    );
    app::set_frame_type_cb(
        OS_INPUT_THIN_DOWN_FRAME,
        greybird_input_thin_down_frame,
        2,
        3,
        4,
        6,
    );
    app::set_frame_type_cb(
        OS_DEFAULT_BUTTON_UP_BOX,
        greybird_default_button_up_box,
        2,
        2,
        4,
        4,
    );
    app::set_frame_type2(OS_DEFAULT_HOVERED_UP_BOX, OS_HOVERED_UP_BOX);
    app::set_frame_type_cb(
        OS_DEFAULT_DEPRESSED_DOWN_BOX,
        greybird_default_depressed_down_box,
        2,
        2,
        4,
        4,
    );
    app::set_frame_type2(OS_TOOLBAR_BUTTON_HOVER_BOX, OS_BUTTON_UP_BOX);
    app::set_frame_type_cb(OS_TABS_BOX, greybird_tabs_box, 2, 2, 4, 4);
    app::set_frame_type2(OS_SWATCH_BOX, OS_SPACER_THIN_DOWN_BOX);
    app::set_frame_type2(OS_MINI_BUTTON_UP_BOX, OS_BUTTON_UP_BOX);
    app::set_frame_type2(OS_MINI_DEPRESSED_DOWN_BOX, OS_DEPRESSED_DOWN_BOX);
    app::set_frame_type2(OS_MINI_BUTTON_UP_FRAME, OS_BUTTON_UP_FRAME);
    app::set_frame_type2(OS_MINI_DEPRESSED_DOWN_FRAME, OS_DEPRESSED_DOWN_FRAME);
    app::set_frame_type2(FrameType::UpBox, OS_BUTTON_UP_BOX);
    app::set_frame_type2(FrameType::DownBox, OS_CHECK_DOWN_BOX);
    app::set_frame_type2(FrameType::RoundDownBox, OS_RADIO_ROUND_DOWN_BOX);
    app::set_frame_type2(OS_BG_BOX, FrameType::FlatBox);
}

fn use_greybird_colors() {
    app::background(0xCE, 0xCE, 0xCE);
    app::background2(0xFC, 0xFC, 0xFC);
    app::foreground(0x3C, 0x3C, 0x3C);
    app::set_color(Color::Inactive, 0x55, 0x55, 0x55);
    app::set_color(Color::Selection, 0x50, 0xA0, 0xF4);
    app::set_color(Color::Free, 0xD9, 0xD9, 0xD9);
    Tooltip::set_color(Color::from_rgb(0x0A, 0x0A, 0x0A));
    Tooltip::set_text_color(Color::from_rgb(0xFF, 0xFF, 0xFF));
}

pub(crate) fn use_greybird_theme() {
    use_greybird_scheme();
    use_greybird_colors();
    use_native_settings();
}
