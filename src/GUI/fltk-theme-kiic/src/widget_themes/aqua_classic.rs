use super::*;
use fltk::{app, enums::Color, misc::Tooltip};

fn aqua_classic_button_up_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top outer border
    set_draw_color(activated_color(Color::from_rgb(0x9A, 0x9A, 0x9A)));
    draw_xyline(x + 3, y, x + w - 4);
    // side outer borders
    set_draw_color(activated_color(Color::from_rgb(0x91, 0x91, 0x91)));
    draw_yxline(x, y + 3, y + h - 4);
    draw_yxline(x + w - 1, y + 3, y + h - 4);
    // bottom outer border
    set_draw_color(activated_color(Color::from_rgb(0x90, 0x90, 0x90)));
    draw_xyline(x + 3, y + h - 1, x + w - 4);
    // top inner border
    set_draw_color(activated_color(Color::from_rgb(0xFF, 0xFF, 0xFF)));
    draw_xyline(x + 3, y + 1, x + w - 4);
    // side top inner borders
    set_draw_color(activated_color(Color::from_rgb(0xFC, 0xFC, 0xFC)));
    draw_yxline(x + 1, y + 3, y + h / 2 - 1);
    draw_yxline(x + w - 2, y + 3, y + h / 2 - 1);
    // side bottom inner borders
    set_draw_color(activated_color(Color::from_rgb(0xF4, 0xF4, 0xF4)));
    draw_yxline(x + 1, y + h / 2 - 1, y + h - 4);
    draw_yxline(x + w - 2, y + 3, y + h - 4);
    // bottom inner border
    set_draw_color(activated_color(Color::from_rgb(0xF3, 0xF2, 0xF0)));
    draw_xyline(x + 3, y + h - 2, x + w - 4);
    // corners
    set_draw_color(activated_color(Color::from_rgb(0xAF, 0xAF, 0xAF)));
    draw_arc(x, y, 8, 8, 90.0, 180.0);
    draw_arc(x, y + h - 8, 8, 8, 180.0, 270.0);
    draw_arc(x + w - 8, y + h - 8, 8, 8, 270.0, 360.0);
    draw_arc(x + w - 8, y, 8, 8, 0.0, 90.0);
}

fn aqua_classic_button_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    if w >= h {
        // top gradient
        vertical_gradient(
            x + 2,
            y + 2,
            x + w - 3,
            y + h / 2 - 1,
            Color::from_rgb(0xFF, 0xFF, 0xFF),
            Color::from_rgb(0xF6, 0xF5, 0xF4),
        );
        // bottom fill
        set_draw_color(activated_color(Color::from_rgb(0xED, 0xEC, 0xEA)));
        draw_rectf(x + 2, y + h / 2, w - 4, h - h / 2 - 3);
        // bottom gradient
        set_draw_color(activated_color(Color::from_rgb(0xEF, 0xEE, 0xEC)));
        draw_xyline(x + 2, y + h - 3, x + w - 3);
    } else {
        // left gradient
        horizontal_gradient(
            x + 2,
            y + 2,
            x + w / 2 - 1,
            y + h - 3,
            Color::from_rgb(0xFF, 0xFF, 0xFF),
            Color::from_rgb(0xF6, 0xF5, 0xF4),
        );
        // right fill
        set_draw_color(activated_color(Color::from_rgb(0xED, 0xEC, 0xEA)));
        draw_rectf(x + w / 2, y + 2, w - w / 2 - 3, h - 4);
        // right gradient
        set_draw_color(activated_color(Color::from_rgb(0xEF, 0xEE, 0xEC)));
        draw_yxline(x + w - 3, y + 2, y + h - 3);
    }
    aqua_classic_button_up_frame(x, y, w, h, c);
}

fn aqua_classic_panel_thin_up_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(devalued(c, 0.06751)));
    draw_rect(x, y, w, h);
}

fn aqua_classic_panel_thin_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(c));
    draw_rectf(x + 1, y + 1, w - 2, h - 2);
    aqua_classic_panel_thin_up_frame(x, y, w, h, c);
}

fn aqua_classic_spacer_thin_down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top and left borders
    set_draw_color(activated_color(Color::from_rgb(0xD6, 0xD6, 0xD6)));
    draw_yxline2(x, y + h - 2, y, x + w - 2);
    // bottom and right borders
    set_draw_color(activated_color(Color::from_rgb(0xF3, 0xF3, 0xF3)));
    draw_xyline2(x, y + h - 1, x + w - 1, y);
}

fn aqua_classic_spacer_thin_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(c));
    draw_rectf(x + 1, y + 1, w - 2, h - 2);
    aqua_classic_spacer_thin_down_frame(x, y, w, h, c);
}

fn aqua_classic_radio_round_down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(devalued(c, 0.42194)));
    draw_arc(x, y, w, h, 0.0, 360.0);
}

fn aqua_classic_radio_round_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top edges
    set_draw_color(activated_color(Color::from_rgb(0xF6, 0xF6, 0xF6)));
    draw_arc(x + 1, y + 1, w - 2, h - 2, 0.0, 180.0);
    // bottom edges
    set_draw_color(activated_color(Color::from_rgb(0xEB, 0xEB, 0xEB)));
    draw_arc(x + 1, y + 1, w - 2, h - 2, 180.0, 360.0);
    // top gradient
    vertical_gradient(
        x + 2,
        y + 2,
        x + w - 3,
        y + h / 2 - 1,
        Color::from_rgb(0xFF, 0xFF, 0xFF),
        Color::from_rgb(0xF6, 0xF5, 0xF4),
    );
    // bottom fill
    set_draw_color(activated_color(Color::from_rgb(0xED, 0xEC, 0xEA)));
    draw_rectf(x + 2, y + h / 2, w - 4, h - h / 2 - 3);
    // bottom gradient
    set_draw_color(activated_color(Color::from_rgb(0xEF, 0xEE, 0xEC)));
    draw_xyline(x + 2, y + h - 3, x + w - 3);
    aqua_classic_radio_round_down_frame(x, y, w, h, c);
}

fn aqua_classic_depressed_down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top outer border
    set_draw_color(activated_color(Color::from_rgb(0x4C, 0x54, 0xAA)));
    draw_xyline(x + 3, y, x + w - 4);
    // side outer borders
    set_draw_color(activated_color(Color::from_rgb(0x49, 0x4C, 0x8F)));
    draw_yxline(x, y + 3, y + h - 4);
    draw_yxline(x + w - 1, y + 3, y + h - 4);
    // bottom outer border
    set_draw_color(activated_color(Color::from_rgb(0x43, 0x46, 0x72)));
    draw_xyline(x + 3, y + h - 1, x + w - 4);
    // top inner border
    set_draw_color(activated_color(Color::from_rgb(0xBC, 0xD6, 0xEF)));
    draw_xyline(x + 3, y + 1, x + w - 4);
    // side top inner borders
    set_draw_color(activated_color(Color::from_rgb(0x7C, 0xAB, 0xE9)));
    draw_yxline(x + 1, y + 3, y + h / 2 - 1);
    draw_yxline(x + w - 2, y + 3, y + h / 2 - 1);
    // side bottom inner borders
    set_draw_color(activated_color(Color::from_rgb(0x5F, 0xA1, 0xEA)));
    draw_yxline(x + 1, y + h / 2, y + h - 4);
    draw_yxline(x + w - 2, y + h / 2, y + h - 4);
    // top corners
    set_draw_color(activated_color(Color::from_rgb(0x79, 0x81, 0xBC)));
    draw_arc(x, y, 8, 8, 90.0, 180.0);
    draw_arc(x + w - 8, y, 8, 8, 0.0, 90.0);
    // bottom corners
    set_draw_color(activated_color(Color::from_rgb(0x72, 0x79, 0x96)));
    draw_arc(x, y + h - 8, 8, 8, 180.0, 270.0);
    draw_arc(x + w - 8, y + h - 8, 8, 8, 270.0, 360.0);
}

fn aqua_classic_depressed_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top gradient
    vertical_gradient(
        x + 2,
        y + 2,
        x + w - 3,
        y + h / 2 - 1,
        Color::from_rgb(0xA3, 0xC1, 0xEF),
        Color::from_rgb(0x67, 0xA1, 0xE9),
    );
    // bottom gradient
    vertical_gradient(
        x + 2,
        y + h / 2,
        x + w - 3,
        y + h - 2,
        Color::from_rgb(0x46, 0x93, 0xE9),
        Color::from_rgb(0xAA, 0xD4, 0xF0),
    );
    aqua_classic_depressed_down_frame(x, y, w, h, c);
}

fn aqua_classic_input_thin_down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top outer border
    set_draw_color(activated_color(Color::from_rgb(0x9B, 0x9B, 0x9B)));
    draw_xyline(x, y, x + w - 1);
    // side and bottom outer borders
    set_draw_color(activated_color(Color::from_rgb(0xBA, 0xBA, 0xBA)));
    draw_yxline3(x, y + 1, y + h - 1, x + w - 1, y + 1);
    // top shadow
    set_draw_color(activated_color(Color::from_rgb(0xE3, 0xE3, 0xE3)));
    draw_xyline(x + 1, y + 1, x + w - 2);
    // inner border
    set_draw_color(activated_color(Color::from_rgb(0xF5, 0xF5, 0xF5)));
    draw_yxline3(x + 1, y + h - 2, y + 2, x + w - 2, y + h - 2);
}

fn aqua_classic_input_thin_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(Color::from_rgb(0xFF, 0xFF, 0xFF)));
    draw_rectf(x + 2, y + 3, w - 4, h - 4);
    aqua_classic_input_thin_down_frame(x, y, w, h, c);
}

fn aqua_classic_default_button_up_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top outer border
    set_draw_color(activated_color(Color::from_rgb(0x4E, 0x59, 0xA6)));
    draw_xyline(x + 3, y, x + w - 4);
    // side outer borders
    set_draw_color(activated_color(Color::from_rgb(0x4C, 0x52, 0x89)));
    draw_yxline(x, y + 3, y + h - 4);
    draw_yxline(x + w - 1, y + 3, y + h - 4);
    // bottom outer border
    set_draw_color(activated_color(Color::from_rgb(0x48, 0x4F, 0x69)));
    draw_xyline(x + 3, y + h - 1, x + w - 4);
    // top inner border
    set_draw_color(activated_color(Color::from_rgb(0xD0, 0xEA, 0xF6)));
    draw_xyline(x + 3, y + 1, x + w - 4);
    // side top inner borders
    set_draw_color(activated_color(Color::from_rgb(0x7A, 0xBF, 0xEF)));
    draw_yxline(x + 1, y + 3, y + h / 2 - 1);
    draw_yxline(x + w - 2, y + 3, y + h / 2 - 1);
    // side bottom inner borders
    set_draw_color(activated_color(Color::from_rgb(0x53, 0xAF, 0xEF)));
    draw_yxline(x + 1, y + h / 2, y + h - 4);
    draw_yxline(x + w - 2, y + h / 2, y + h - 4);
    // top corners
    set_draw_color(activated_color(Color::from_rgb(0x76, 0x80, 0xB5)));
    draw_arc(x, y, 8, 8, 90.0, 180.0);
    draw_arc(x + w - 8, y, 8, 8, 0.0, 90.0);
    // bottom corners
    set_draw_color(activated_color(Color::from_rgb(0x6F, 0x75, 0x89)));
    draw_arc(x, y + h - 8, 8, 8, 180.0, 270.0);
    draw_arc(x + w - 8, y + h - 8, 8, 8, 270.0, 360.0);
}

fn aqua_classic_default_button_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top gradient
    vertical_gradient(
        x + 2,
        y + 2,
        x + w - 3,
        y + h / 2 - 1,
        Color::from_rgb(0xBF, 0xDC, 0xF7),
        Color::from_rgb(0x84, 0xC4, 0xF1),
    );
    // bottom gradient
    vertical_gradient(
        x + 2,
        y + h / 2,
        x + w - 3,
        y + h - 2,
        Color::from_rgb(0x59, 0xB5, 0xF1),
        Color::from_rgb(0xBA, 0xE9, 0xF7),
    );
    aqua_classic_default_button_up_frame(x, y, w, h, c);
}

fn aqua_classic_tabs_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // top outer border
    set_draw_color(activated_color(Color::from_rgb(0xAE, 0xAE, 0xAE)));
    draw_xyline(x + 3, y, x + w - 4);
    // side outer borders
    set_draw_color(activated_color(Color::from_rgb(0x9E, 0x9E, 0x9E)));
    draw_yxline(x, y + 3, y + h - 4);
    draw_yxline(x + w - 1, y + 3, y + h - 4);
    // bottom outer border
    set_draw_color(activated_color(Color::from_rgb(0x8E, 0x8E, 0x8E)));
    draw_xyline(x + 3, y + h - 1, x + w - 4);
    // top inner border
    set_draw_color(activated_color(Color::from_rgb(0xFA, 0xFA, 0xFA)));
    draw_xyline(x + 3, y + 1, x + w - 4);
    // side inner borders
    set_draw_color(activated_color(Color::from_rgb(0xF6, 0xF6, 0xF6)));
    draw_yxline(x + 1, y + 3, y + h - 4);
    draw_yxline(x + w - 2, y + 3, y + h - 4);
    // bottom inner border
    set_draw_color(activated_color(Color::from_rgb(0xF2, 0xF2, 0xF2)));
    draw_xyline(x + 3, y + h - 2, x + w - 4);
    // top corners
    set_draw_color(activated_color(Color::from_rgb(0xA4, 0xA4, 0xA4)));
    draw_arc(x, y, 8, 8, 90.0, 180.0);
    draw_arc(x + w - 8, y, 8, 8, 0.0, 90.0);
    // bottom corners
    set_draw_color(activated_color(Color::from_rgb(0x94, 0x94, 0x94)));
    draw_arc(x, y + h - 8, 8, 8, 180.0, 270.0);
    draw_arc(x + w - 8, y + h - 8, 8, 8, 270.0, 360.0);
}

fn aqua_classic_tabs_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(c));
    draw_rectf(x + 2, y + 2, w - 4, h - 4);
    aqua_classic_tabs_frame(x, y, w, h, c);
}

fn aqua_classic_swatch_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    // outer border
    set_draw_color(activated_color(Color::from_rgb(0xA3, 0xA3, 0xA3)));
    draw_rect(x, y, w, h);
    // inner border
    set_draw_color(activated_color(Color::from_rgb(0xFF, 0xFF, 0xFF)));
    draw_rect(x + 1, y + 1, w - 2, h - 2);
}

fn aqua_classic_swatch_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(c));
    draw_rectf(x + 2, y + 2, w - 4, h - 4);
    aqua_classic_swatch_frame(x, y, w, h, c);
}

fn use_aqua_classic_scheme() {
    app::set_scheme(app::Scheme::Gtk);
    app::set_frame_type_cb(OS_BUTTON_UP_BOX, aqua_classic_button_up_box, 1, 1, 2, 2);
    app::set_frame_type2(OS_CHECK_DOWN_BOX, OS_BUTTON_UP_BOX);
    app::set_frame_type_cb(OS_BUTTON_UP_FRAME, aqua_classic_button_up_frame, 1, 1, 2, 2);
    app::set_frame_type2(OS_CHECK_DOWN_FRAME, OS_BUTTON_UP_FRAME);
    app::set_frame_type_cb(
        OS_PANEL_THIN_UP_BOX,
        aqua_classic_panel_thin_up_box,
        1,
        1,
        2,
        2,
    );
    app::set_frame_type_cb(
        OS_SPACER_THIN_DOWN_BOX,
        aqua_classic_spacer_thin_down_box,
        1,
        1,
        2,
        2,
    );
    app::set_frame_type_cb(
        OS_PANEL_THIN_UP_FRAME,
        aqua_classic_panel_thin_up_frame,
        1,
        1,
        2,
        2,
    );
    app::set_frame_type_cb(
        OS_SPACER_THIN_DOWN_FRAME,
        aqua_classic_spacer_thin_down_frame,
        1,
        1,
        2,
        2,
    );
    app::set_frame_type_cb(
        OS_RADIO_ROUND_DOWN_BOX,
        aqua_classic_radio_round_down_box,
        2,
        2,
        4,
        4,
    );
    app::set_frame_type2(OS_HOVERED_UP_BOX, OS_BUTTON_UP_BOX);
    app::set_frame_type_cb(
        OS_DEPRESSED_DOWN_BOX,
        aqua_classic_depressed_down_box,
        1,
        1,
        2,
        2,
    );
    app::set_frame_type2(OS_HOVERED_UP_FRAME, OS_BUTTON_UP_FRAME);
    app::set_frame_type_cb(
        OS_DEPRESSED_DOWN_FRAME,
        aqua_classic_depressed_down_frame,
        1,
        1,
        2,
        2,
    );
    app::set_frame_type_cb(
        OS_INPUT_THIN_DOWN_BOX,
        aqua_classic_input_thin_down_box,
        2,
        3,
        4,
        6,
    );
    app::set_frame_type_cb(
        OS_INPUT_THIN_DOWN_FRAME,
        aqua_classic_input_thin_down_frame,
        2,
        3,
        4,
        6,
    );
    app::set_frame_type_cb(
        OS_DEFAULT_BUTTON_UP_BOX,
        aqua_classic_default_button_up_box,
        1,
        1,
        2,
        2,
    );
    app::set_frame_type2(OS_DEFAULT_HOVERED_UP_BOX, OS_HOVERED_UP_BOX);
    app::set_frame_type2(OS_DEFAULT_DEPRESSED_DOWN_BOX, OS_DEPRESSED_DOWN_BOX);
    app::set_frame_type2(OS_TOOLBAR_BUTTON_HOVER_BOX, FrameType::FlatBox);
    app::set_frame_type_cb(OS_TABS_BOX, aqua_classic_tabs_box, 2, 1, 4, 2);
    app::set_frame_type_cb(OS_SWATCH_BOX, aqua_classic_swatch_box, 2, 2, 4, 4);
    app::set_frame_type2(OS_MINI_BUTTON_UP_BOX, OS_BUTTON_UP_BOX);
    app::set_frame_type2(OS_MINI_DEPRESSED_DOWN_BOX, OS_DEPRESSED_DOWN_BOX);
    app::set_frame_type2(OS_MINI_BUTTON_UP_FRAME, OS_BUTTON_UP_FRAME);
    app::set_frame_type2(OS_MINI_DEPRESSED_DOWN_FRAME, OS_DEPRESSED_DOWN_FRAME);
    app::set_frame_type2(FrameType::UpBox, OS_BUTTON_UP_BOX);
    app::set_frame_type2(FrameType::DownBox, OS_BUTTON_UP_BOX);
    app::set_frame_type2(FrameType::RoundDownBox, OS_RADIO_ROUND_DOWN_BOX);
    app::set_frame_type2(OS_BG_BOX, FrameType::FlatBox);
    // app::set_frame_type_cb(OS_BG_DOWN_BOX, OS_BG_BOX);
}

fn use_aqua_classic_colors() {
    app::background(0xED, 0xED, 0xED);
    app::background2(0xFF, 0xFF, 0xFF);
    app::foreground(0x00, 0x00, 0x00);
    app::set_color(Color::Inactive, 0x4D, 0x4D, 0x69);
    app::set_color(Color::Selection, 0x30, 0x60, 0xF6);
    app::set_color(Color::Free, 0xFB, 0xFB, 0xFB);
    Tooltip::set_color(Color::from_rgb(0xFF, 0xFF, 0xC7));
    Tooltip::set_text_color(Color::Foreground);
}

pub(crate) fn use_aqua_classic_theme() {
    use_aqua_classic_scheme();
    use_aqua_classic_colors();
    use_native_settings();
}
