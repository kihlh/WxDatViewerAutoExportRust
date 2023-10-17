use fltk::{
    enums::FrameType::{self, *},
    prelude::*,
    *,
};
use fltk_theme::{SchemeType, WidgetScheme};

const FRAMES: &[FrameType] = &[
    RoundedFrame,
    RoundedBox,
    RFlatBox,
    OvalBox,
    OvalFrame,
    OFlatFrame,
];

fn add_frames(frames: &[FrameType]) {
    for frame in frames {
        let mut f = frame::Frame::default().with_label(&format!("{:?}", frame));
        f.set_frame(*frame);
        f.set_label_size(14);
        f.set_color(enums::Color::from_hex(0x0078D4));
    }
}

fn main() {
    let a = app::App::default();
    app::background(0xfa, 0xfa, 0xfa);
    let scheme = WidgetScheme::new(SchemeType::SvgBased);
    scheme.apply();
    let mut win = window::Window::default().with_size(600, 400);
    let mut vgrid = group::VGrid::new(5, 5, 590, 390, None);
    vgrid.set_params(3, 2, 5);
    add_frames(FRAMES);
    vgrid.end();
    win.end();
    win.show();
    a.run().unwrap();
}
