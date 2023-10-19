use super::*;

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
    rectf(x, y, w, h, c);
    rect(x, y, w, h, Color::color_average(Color::White, c, 0.2));
}

fn down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    rect(x, y, w, h, Color::color_average(Color::Black, c, 0.2));
}

fn down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    rectf(x, y, w, h, c);
    rect(x, y, w, h, Color::color_average(Color::White, c, 0.2));
}

fn border_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    rectf(x, y, w, h, c);
    rect(x, y, w, h, Color::color_average(Color::White, c, 0.2));
}

pub(crate) fn use_clean_scheme() {
    use fltk::enums::FrameType::*;
    app::reload_scheme().ok();
    app::set_scheme(app::Scheme::Base);
    app::set_frame_type_cb(UpBox, up_box, 1, 1, 2, 2);
    app::set_frame_type_cb(DownBox, down_box, 1, 1, 2, 2);
    app::set_frame_type_cb(ThinUpBox, up_box, 1, 1, 2, 2);
    app::set_frame_type_cb(ThinDownBox, down_box, 1, 1, 2, 2);
    app::set_frame_type_cb(UpFrame, up_frame, 1, 1, 2, 2);
    app::set_frame_type_cb(DownFrame, down_frame, 1, 1, 2, 2);
    app::set_frame_type_cb(RoundUpBox, up_box, 1, 1, 2, 2);
    app::set_frame_type_cb(RoundDownBox, down_box, 1, 1, 2, 2);
    app::set_frame_type_cb(BorderBox, border_box, 1, 1, 2, 2);
}
