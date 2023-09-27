use fltk::{
    app, draw,
    enums::{Color, FrameType},
};

fn up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    let col1 = c.to_rgb();
    let col = Color::color_average(c, Color::Background, 0.8).to_rgb();
    draw::draw_rbox(x, y, w, h, 5, true, Color::from_rgb(col.0, col.1, col.2));
}

fn default_button_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    let col1 = c.to_rgb();
    let col = Color::color_average(c, Color::Background, 0.8).to_rgb();
    draw::draw_rbox(x, y, w, h, 5, true, Color::from_rgb(col.0, col.1, col.2));
}

fn down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    let col1 = c.to_rgb();
    let col = Color::color_average(c, Color::Background, 0.8).to_rgb();
    draw::draw_rbox(x, y, w, h, 5, true, Color::from_rgb(col.0, col.1, col.2));
}

fn radio_round_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    let col = c.to_rgb();
    draw::draw_box(FrameType::OFlatBox, x, y, w, h, c);
}

fn border_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    draw::draw_rbox(
        x,
        y,
        w,
        h,
        5,
        true,
        *crate::colors::aqua::dark::systemBlueColor,
    );
}

fn use_scheme() {
    app::set_scheme(app::Scheme::Gtk);
    // app::set_menu_linespacing(5);
    app::set_frame_type_cb(FrameType::UpBox, up_box, 2, 2, 4, 4);
    app::set_frame_type_cb(FrameType::DiamondUpBox, default_button_up_box, 2, 2, 4, 4);
    app::set_frame_type_cb(FrameType::DownBox, down_box, 2, 2, 4, 4);
    app::set_frame_type_cb(FrameType::DiamondDownBox, down_box, 2, 2, 4, 4);
    app::set_frame_type_cb(FrameType::RoundDownBox, radio_round_down_box, 2, 2, 4, 4);
    app::set_frame_type_cb(FrameType::BorderBox, border_box, 2, 2, 4, 4);
}

pub(crate) fn use_aqua_scheme() {
    use_scheme();
    app::set_visible_focus(false);
    app::set_scrollbar_size(15);
}

pub mod frames {
    use fltk::enums::FrameType::{self, *};
    pub const OS_DEFAULT_BUTTON_UP_BOX: FrameType = DiamondUpBox;
    pub const OS_DEFAULT_DEPRESSED_DOWN_BOX: FrameType = FrameType::DiamondDownBox;
}
