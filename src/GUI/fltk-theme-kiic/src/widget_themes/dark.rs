use super::*;
use fltk::{app, enums::Color, misc::Tooltip};

fn dark_button_up_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top outer border
    set_draw_color(activated_color(Color::from_rgb(0x30, 0x30, 0x30)));
    draw_xyline(x + 2, y, x + w - 3);
    // side outer borders
    set_draw_color(activated_color(Color::from_rgb(0x2C, 0x2C, 0x2C)));
    draw_yxline(x, y + 2, y + h - 3);
    draw_yxline(x + w - 1, y + 2, y + h - 3);
    // bottom outer border
    set_draw_color(activated_color(Color::from_rgb(0x27, 0x27, 0x27)));
    draw_xyline(x + 2, y + h - 1, x + w - 3);
    // top inner border
    set_draw_color(activated_color(Color::from_rgb(0x91, 0x91, 0x91)));
    draw_xyline(x + 2, y + 1, x + w - 3);
    // corners
    set_draw_color(activated_color(Color::from_rgb(0x47, 0x47, 0x47)));
    draw_xyline2(x, y + 1, x + 1, y);
    draw_xyline2(x, y + h - 2, x + 1, y + h - 1);
    draw_yxline2(x + w - 2, y, y + 1, x + w - 1);
    draw_yxline2(x + w - 2, y + h - 1, y + h - 2, x + w - 1);
}

fn dark_button_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    if w >= h {
        vertical_gradient(
            x + 1,
            y + 2,
            x + w - 2,
            y + h - 1,
            activated_color(Color::from_rgb(0x75, 0x75, 0x75)),
            activated_color(Color::from_rgb(0x62, 0x62, 0x62)),
        );
    } else {
        horizontal_gradient(
            x + 1,
            y + 2,
            x + w - 2,
            y + h - 1,
            activated_color(Color::from_rgb(0x75, 0x75, 0x75)),
            activated_color(Color::from_rgb(0x62, 0x62, 0x62)),
        );
    }
    dark_button_up_frame(x, y, w, h, c);
}

fn dark_panel_thin_up_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top and left borders
    set_draw_color(activated_color(Color::from_rgb(0x6A, 0x6A, 0x6A)));
    draw_yxline2(x, y + h - 2, y, x + w - 2);
    // bottom and right borders
    set_draw_color(activated_color(Color::from_rgb(0x28, 0x28, 0x28)));
    draw_xyline2(x, y + h - 1, x + w - 1, y);
}

fn dark_panel_thin_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(c));
    draw_rectf(x + 1, y + 1, w - 2, h - 2);
    dark_panel_thin_up_frame(x, y, w, h, c);
}

fn dark_spacer_thin_down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top and left borders
    set_draw_color(activated_color(Color::from_rgb(0x38, 0x38, 0x38)));
    draw_yxline2(x, y + h - 2, y, x + w - 2);
    // bottom and right borders
    set_draw_color(activated_color(Color::from_rgb(0x74, 0x74, 0x74)));
    draw_xyline2(x, y + h - 1, x + w - 1, y);
}

fn dark_spacer_thin_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(c));
    draw_rectf(x + 1, y + 1, w - 2, h - 2);
    dark_spacer_thin_down_frame(x, y, w, h, c);
}

fn dark_radio_round_down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(Color::from_rgb(0x30, 0x30, 0x30)));
    draw_arc(x, y, w, h, 0.0, 360.0);
}

fn dark_radio_round_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top edges
    set_draw_color(activated_color(Color::from_rgb(0x75, 0x75, 0x75)));
    draw_arc(x + 1, y + 1, w - 2, h - 2, 0.0, 180.0);
    // bottom edges
    set_draw_color(activated_color(Color::from_rgb(0x62, 0x62, 0x62)));
    draw_arc(x + 1, y + 1, w - 2, h - 2, 180.0, 360.0);
    // gradient
    vertical_gradient(
        x + 2,
        y + 2,
        x + w - 3,
        y + h - 3,
        Color::from_rgb(0x74, 0x74, 0x74),
        Color::from_rgb(0x63, 0x63, 0x63),
    );
    dark_radio_round_down_frame(x, y, w, h, c);
}

fn dark_depressed_down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top outer border
    set_draw_color(activated_color(Color::from_rgb(0x27, 0x27, 0x27)));
    draw_xyline(x + 2, y, x + w - 3);
    // side outer borders
    set_draw_color(activated_color(Color::from_rgb(0x2C, 0x2C, 0x2C)));
    draw_yxline(x, y + 2, y + h - 3);
    draw_yxline(x + w - 1, y + 2, y + h - 3);
    // bottom outer border
    set_draw_color(activated_color(Color::from_rgb(0x30, 0x30, 0x30)));
    draw_xyline(x + 2, y + h - 1, x + w - 3);
    // top inner border
    set_draw_color(activated_color(Color::from_rgb(0x33, 0x33, 0x33)));
    draw_xyline(x + 2, y + 1, x + w - 3);
    // corners
    set_draw_color(activated_color(Color::from_rgb(0x32, 0x32, 0x32)));
    draw_xyline2(x, y + 1, x + 1, y);
    draw_xyline2(x, y + h - 2, x + 1, y + h - 1);
    draw_yxline2(x + w - 2, y, y + 1, x + w - 1);
    draw_xyline2(x + w - 2, y + h - 1, y + h - 2, x + w - 1);
    set_draw_color(activated_color(Color::from_rgb(0x4B, 0x4B, 0x4B)));
    draw_point(x, y);
    draw_point(x + w - 1, y);
    draw_point(x, y + h - 1);
    draw_point(x + w - 1, y + h - 1);
}

fn dark_depressed_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    vertical_gradient(
        x + 1,
        y + 2,
        x + w - 2,
        y + h - 1,
        activated_color(Color::from_rgb(0x3F, 0x3F, 0x3F)),
        activated_color(Color::from_rgb(0x37, 0x37, 0x37)),
    );
    dark_depressed_down_frame(x, y, w, h, c);
}

fn dark_input_thin_down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top and side outer borders
    set_draw_color(activated_color(Color::from_rgb(0x30, 0x30, 0x30)));
    draw_xyline(x, y, x + w - 1);
    draw_yxline(x, y + 1, y + h - 2);
    draw_yxline(x + w - 1, y + 1, y + h - 2);
    // bottom outer border
    set_draw_color(activated_color(Color::from_rgb(0x29, 0x29, 0x29)));
    draw_xyline(x, y + h - 1, x + w - 1);
    // top inner border
    set_draw_color(activated_color(Color::from_rgb(0x37, 0x37, 0x37)));
    draw_xyline(x + 1, y + 1, x + w - 2);
    // top and side innermost borders
    set_draw_color(activated_color(Color::from_rgb(0x39, 0x39, 0x39)));
    draw_xyline(x + 1, y + 2, x + w - 2);
    draw_yxline(x + 1, y + 3, y + h - 2);
    draw_yxline(x + w - 2, y + 3, y + h - 2);
}

fn dark_input_thin_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(c));
    draw_rectf(x + 2, y + 3, w - 4, h - 4);
    dark_input_thin_down_frame(x, y, w, h, c);
}

fn dark_tabs_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // outer border
    set_draw_color(activated_color(Color::from_rgb(0x28, 0x28, 0x28)));
    draw_xyline(x + 1, y, x + w - 2);
    draw_xyline(x + 1, y + h - 1, x + w - 2);
    draw_yxline(x, y + 1, y + h - 2);
    draw_yxline(x + w - 1, y + 1, y + h - 2);
    // top inner border
    set_draw_color(activated_color(Color::from_rgb(0x6A, 0x6A, 0x6A)));
    draw_xyline(x + 2, y + 1, x + w - 3);
}

fn dark_tabs_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(c));
    draw_rectf(x + 1, y + 1, w - 2, h - 2);
    dark_tabs_frame(x, y, w, h, c);
}

fn dark_swatch_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // outer border
    set_draw_color(activated_color(Color::from_rgb(0x25, 0x25, 0x25)));
    draw_rect(x, y, w, h);
    // inner border
    set_draw_color(activated_color(Color::from_rgb(0xFF, 0xFF, 0xFF)));
    draw_rect(x + 1, y + 1, w - 2, h - 2);
}

fn dark_swatch_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(c));
    draw_rectf(x + 2, y + 2, w - 4, h - 4);
    dark_swatch_frame(x, y, w, h, c);
}

fn use_dark_scheme() {
    app::set_scheme(app::Scheme::Gtk);
    app::set_frame_type_cb(OS_BUTTON_UP_BOX, dark_button_up_box, 1, 1, 2, 2);
    app::set_frame_type2(OS_CHECK_DOWN_BOX, OS_BUTTON_UP_BOX);
    app::set_frame_type_cb(OS_BUTTON_UP_FRAME, dark_button_up_frame, 1, 1, 2, 2);
    app::set_frame_type2(OS_CHECK_DOWN_FRAME, OS_BUTTON_UP_FRAME);
    app::set_frame_type_cb(OS_PANEL_THIN_UP_BOX, dark_panel_thin_up_box, 1, 1, 2, 2);
    app::set_frame_type_cb(
        OS_SPACER_THIN_DOWN_BOX,
        dark_spacer_thin_down_box,
        1,
        1,
        2,
        2,
    );
    app::set_frame_type_cb(OS_PANEL_THIN_UP_FRAME, dark_panel_thin_up_frame, 1, 1, 2, 2);
    app::set_frame_type_cb(
        OS_SPACER_THIN_DOWN_FRAME,
        dark_spacer_thin_down_frame,
        1,
        1,
        2,
        2,
    );
    app::set_frame_type_cb(
        OS_RADIO_ROUND_DOWN_BOX,
        dark_radio_round_down_box,
        1,
        1,
        2,
        2,
    );
    app::set_frame_type2(OS_HOVERED_UP_BOX, OS_BUTTON_UP_BOX);
    app::set_frame_type_cb(OS_DEPRESSED_DOWN_BOX, dark_depressed_down_box, 1, 1, 2, 2);
    app::set_frame_type2(OS_HOVERED_UP_FRAME, OS_BUTTON_UP_FRAME);
    app::set_frame_type_cb(
        OS_DEPRESSED_DOWN_FRAME,
        dark_depressed_down_frame,
        1,
        1,
        2,
        2,
    );
    app::set_frame_type_cb(OS_INPUT_THIN_DOWN_BOX, dark_input_thin_down_box, 1, 2, 2, 4);
    app::set_frame_type_cb(
        OS_INPUT_THIN_DOWN_FRAME,
        dark_input_thin_down_frame,
        1,
        2,
        2,
        4,
    );
    app::set_frame_type2(OS_DEFAULT_BUTTON_UP_BOX, OS_BUTTON_UP_BOX);
    app::set_frame_type2(OS_DEFAULT_HOVERED_UP_BOX, OS_HOVERED_UP_BOX);
    app::set_frame_type2(OS_DEFAULT_DEPRESSED_DOWN_BOX, OS_DEPRESSED_DOWN_BOX);
    app::set_frame_type2(OS_TOOLBAR_BUTTON_HOVER_BOX, OS_BUTTON_UP_BOX);
    app::set_frame_type_cb(OS_TABS_BOX, dark_tabs_box, 2, 2, 4, 4);
    app::set_frame_type_cb(OS_SWATCH_BOX, dark_swatch_box, 2, 2, 4, 4);
    app::set_frame_type2(OS_MINI_BUTTON_UP_BOX, OS_BUTTON_UP_BOX);
    app::set_frame_type2(OS_MINI_DEPRESSED_DOWN_BOX, OS_DEPRESSED_DOWN_BOX);
    app::set_frame_type2(OS_MINI_BUTTON_UP_FRAME, OS_BUTTON_UP_FRAME);
    app::set_frame_type2(OS_MINI_DEPRESSED_DOWN_FRAME, OS_DEPRESSED_DOWN_FRAME);
    app::set_frame_type2(FrameType::UpBox, OS_BUTTON_UP_BOX);
    app::set_frame_type2(FrameType::DownBox, OS_BUTTON_UP_BOX);
    app::set_frame_type2(FrameType::RoundDownBox, OS_RADIO_ROUND_DOWN_BOX);
    app::set_frame_type2(OS_BG_BOX, FrameType::FlatBox);
    // app::set_frame_type2(OS_BG_DOWN_BOX, OS_BG_BOX);
}

fn use_dark_colors() {
    app::background(0x53, 0x53, 0x53);
    app::background2(0x3A, 0x3A, 0x3A);
    app::foreground(0xFF, 0xFF, 0xFF);
    app::set_color(Color::Inactive, 0x26, 0x26, 0x26);
    app::set_color(Color::Selection, 0xD6, 0xD6, 0xD6);
    app::set_color(Color::Free, 0x53, 0x53, 0x53);
    Tooltip::set_color(Color::from_rgb(0xFF, 0xFF, 0xCC));
    Tooltip::set_text_color(Color::from_rgb(0x00, 0x00, 0x00));
}

pub(crate) fn use_dark_theme() {
    use_dark_scheme();
    use_dark_colors();
    use_native_settings();
}
