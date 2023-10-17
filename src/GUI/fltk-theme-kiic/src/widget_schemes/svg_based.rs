use super::*;
use fltk::{enums::FrameType, image, prelude::ImageExt};

fn rounded_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    let (r, g, b) = c.to_rgb();
    let svg = format!("<svg viewBox='0 0 {} {}'>
  <rect x='1%' y='1%' stroke-width='2' rx='10%' width='96%' height='96%' stroke='rgb({}, {}, {})' fill='none' />
</svg>",w, h, r, g, b);
    let mut image = image::SvgImage::from_data(&svg).unwrap();
    image.draw(x, y, w, h);
}

fn rounded_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    let (r, g, b) = c.to_rgb();
    let svg = format!(
        "<svg viewBox='0 0 {} {}'>
    <rect x='1%' y='1%' rx='10%' width='96%' height='96%' fill='rgb({}, {}, {})' />
  </svg>",
        w, h, r, g, b
    );
    let mut image = image::SvgImage::from_data(&svg).unwrap();
    image.draw(x, y, w, h);
}

fn rflat_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    let (r, g, b) = c.to_rgb();
    let svg = format!(
        "<svg viewBox='0 0 {} {}'>
    <rect x='1%' y='1%' rx='10%' width='96%' height='96%' fill='rgb({}, {}, {})' />
  </svg>",
        w, h, r, g, b
    );
    let mut image = image::SvgImage::from_data(&svg).unwrap();
    image.draw(x, y, w, h);
}

fn oval_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    let (r, g, b) = c.to_rgb();
    let svg = format!(
        "<svg viewBox='0 0 {} {}'>
    <rect x='1%' y='1%' rx='100%' width='96%' height='96%' fill='rgb({}, {}, {})' />
  </svg>",
        w, h, r, g, b
    );
    let mut image = image::SvgImage::from_data(&svg).unwrap();
    image.draw(x, y, w, h);
}

fn oval_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    let (r, g, b) = c.to_rgb();
    let svg = format!("<svg viewBox='0 0 {} {}'>
    <rect x='1%' y='1%' stroke-width='2' rx='100%' width='96%' height='96%' stroke='rgb({}, {}, {})' fill='none' />
  </svg>",w, h, r, g, b);
    let mut image = image::SvgImage::from_data(&svg).unwrap();
    image.draw(x, y, w, h);
}

fn oflat_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    let (r, g, b) = c.to_rgb();
    let svg = format!(
        "<svg viewBox='0 0 {} {}'>
    <rect x='1%' y='1%' rx='100%' width='96%' height='96%' fill='rgb({}, {}, {})' />
  </svg>",
        w, h, r, g, b
    );
    let mut image = image::SvgImage::from_data(&svg).unwrap();
    image.draw(x, y, w, h);
}

pub(crate) fn use_svg_based_scheme() {
    app::reload_scheme().ok();
    app::set_scheme(app::Scheme::Base);
    app::set_frame_type_cb(FrameType::RoundedFrame, rounded_frame, 2, 2, 4, 4);
    app::set_frame_type_cb(FrameType::RoundedBox, rounded_box, 2, 2, 4, 4);
    app::set_frame_type_cb(FrameType::RFlatBox, rflat_box, 2, 2, 4, 4);
    app::set_frame_type_cb(FrameType::OvalBox, oval_box, 2, 2, 4, 4);
    app::set_frame_type_cb(FrameType::OvalFrame, oval_frame, 2, 2, 4, 4);
    app::set_frame_type_cb(FrameType::OFlatBox, oflat_box, 2, 2, 4, 4);
}
