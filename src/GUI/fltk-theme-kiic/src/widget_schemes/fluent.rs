use super::*;
use fltk::{draw, enums::FrameType};

fn rect(x: i32, y: i32, w: i32, h: i32, c: Color) {
    draw_rect_fill(x, y, w, h, c);
}

fn rectf(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(c));
    draw_rectf(x, y, w, h);
}

fn up_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    rect(x, y, w, h, Color::color_average(Color::White, c, 0.2));
}

fn up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    up_frame(x, y, w, h, c);
}

fn default_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    rectf(
        x,
        y,
        w,
        h,
        Color::color_average(Color::Black, Color::White, 0.3),
    );
}

fn down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    rect(
        x - 2,
        y - 2,
        w + 4,
        h + 4,
        Color::color_average(Color::Black, Color::White, 0.3),
    );
    rectf(x, y, w, h, c);
}

fn down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    down_frame(x, y, w, h, c);
}

fn border_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    rectf(x, y, w, h, c);
    rect(x, y, w, h, Color::color_average(Color::White, c, 0.2));
}

fn round_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    let col = Color::Background.to_rgb();
    let fg = Color::contrast(Color::Background, Color::Background);
    draw::draw_box(FrameType::OFlatBox, x, y, w, h, Color::Background);
    draw::draw_box(FrameType::OvalFrame, x, y, w, h, fg);
}

fn hover_up_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    rect(
        x - 2,
        y - 2,
        w + 4,
        h + 4,
        Color::color_average(Color::Black, Color::White, 0.2),
    );
}

fn hover_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    hover_up_frame(x - 2, y - 2, w + 4, h + 4, c);
    rect(
        x + 2,
        y + 2,
        w - 4,
        h - 4,
        Color::color_average(Color::Black, Color::White, 0.3),
    );
}

fn depressed_down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    rect(
        x - 2,
        y - 2,
        w + 4,
        h + 4,
        Color::color_average(Color::Black, Color::White, 0.3),
    );
    rectf(x, y, w, h, c);
}

fn depressed_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    depressed_down_frame(x, y, w, h, c);
    set_draw_color(Color::color_average(Color::Black, Color::White, 0.2));
    draw_rectf(x, y, w, h);
}

pub(crate) fn use_fluent_scheme() {
    app::set_visible_focus(false);
    app::set_scrollbar_size(15);
    use self::frames::*;
    use fltk::enums::FrameType::*;
    app::set_scheme(app::Scheme::Base);
    app::set_frame_type_cb(UpBox, up_box, 2, 2, 4, 4);
    app::set_frame_type_cb(DownBox, down_box, 2, 2, 4, 4);
    app::set_frame_type_cb(ThinUpBox, up_box, 2, 2, 4, 4);
    app::set_frame_type_cb(ThinDownBox, down_box, 2, 2, 4, 4);
    app::set_frame_type_cb(UpFrame, up_frame, 2, 2, 4, 4);
    app::set_frame_type_cb(DownFrame, down_frame, 2, 2, 4, 4);
    app::set_frame_type_cb(RoundUpBox, round_box, 2, 2, 4, 4);
    app::set_frame_type_cb(RoundDownBox, round_box, 1, 1, 2, 2);
    app::set_frame_type_cb(BorderBox, border_box, 2, 2, 4, 4);
    app::set_frame_type2(OS_BUTTON_UP_FRAME, UpFrame);
    app::set_frame_type_cb(OS_DEFAULT_BUTTON_UP_BOX, default_up_box, 2, 2, 4, 4);
    app::set_frame_type2(OS_BUTTON_UP_BOX, UpBox);
    app::set_frame_type2(OS_CHECK_DOWN_BOX, DownBox);
    app::set_frame_type2(OS_CHECK_DOWN_FRAME, DownFrame);
    app::set_frame_type_cb(OS_HOVERED_UP_FRAME, hover_up_frame, 2, 2, 4, 4);
    app::set_frame_type_cb(OS_HOVERED_UP_BOX, hover_up_box, 2, 2, 4, 4);
    app::set_frame_type2(OS_RADIO_ROUND_DOWN_BOX, RoundDownBox);
    app::set_frame_type_cb(OS_DEPRESSED_DOWN_FRAME, depressed_down_frame, 2, 2, 4, 4);
    app::set_frame_type_cb(OS_DEPRESSED_DOWN_BOX, depressed_down_box, 2, 2, 4, 4);
    app::set_frame_type2(OS_DEFAULT_DEPRESSED_DOWN_BOX, OS_DEPRESSED_DOWN_BOX);
    app::set_frame_type2(OS_DEFAULT_HOVERED_UP_BOX, OS_HOVERED_UP_BOX);
    app::set_frame_type2(OS_INPUT_THIN_DOWN_FRAME, DownFrame);
    app::set_frame_type2(OS_INPUT_THIN_DOWN_BOX, DownBox);
}

pub mod frames {
    use fltk::enums::FrameType::{self, *};

    pub const OS_BUTTON_UP_FRAME: FrameType = GtkUpFrame;
    pub const OS_DEFAULT_BUTTON_UP_BOX: FrameType = DiamondUpBox;
    pub const OS_BUTTON_UP_BOX: FrameType = GtkUpBox;
    pub const OS_CHECK_DOWN_BOX: FrameType = GtkDownBox;
    pub const OS_CHECK_DOWN_FRAME: FrameType = GtkDownFrame;
    pub const OS_HOVERED_UP_FRAME: FrameType = PlasticUpFrame;
    pub const OS_HOVERED_UP_BOX: FrameType = PlasticUpBox;
    pub const OS_RADIO_ROUND_DOWN_BOX: FrameType = FrameType::GtkRoundDownBox;
    pub const OS_DEPRESSED_DOWN_FRAME: FrameType = PlasticDownFrame;
    pub const OS_DEPRESSED_DOWN_BOX: FrameType = PlasticDownBox;
    pub const OS_DEFAULT_DEPRESSED_DOWN_BOX: FrameType = DiamondDownBox;
    pub const OS_DEFAULT_HOVERED_UP_BOX: FrameType = PlasticThinUpBox;
    pub const OS_INPUT_THIN_DOWN_FRAME: FrameType = PlasticRoundDownBox;
    pub const OS_INPUT_THIN_DOWN_BOX: FrameType = PlasticThinDownBox;
}

pub mod colors {
    pub const ACCENT_COLOR: (u8, u8, u8, u8) = (0x00, 0x78, 0xD4, 0xff);
    pub const SELECTION_COLOR: (u8, u8, u8, u8) = (0x33, 0x99, 0xFF, 0xFF);
}
