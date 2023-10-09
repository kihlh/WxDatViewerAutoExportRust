use fltk::{
    app::{self, handle},
    button::{self, Button},
    draw::font,
    enums::{self, Color, Cursor, Event, Font, FrameType, LabelType},
    frame::{self, Frame},
    group::{self, Flex, Group},
    image::{self, Image, PngImage},
    input::{InputType, IntInput},
    prelude::*,
    text::TextDisplay,
    tree,
    window::{self, DoubleWindow, Window},
};
use std::{
    fs,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    process,
    process::{Command, Stdio},
    thread,
};

pub struct ImgPreview {
    pub preview: frame::Frame,
    x:i32,
    y:i32,
    width: i32,
    height: i32
}
impl Clone for ImgPreview {
    fn clone(&self) -> Self {
        ImgPreview {
            preview:self.preview.clone(),
            x: self.x.clone(),
            y: self.y.clone(),
            width: self.width.clone(),
            height: self.height.clone(),
        }
    }
}

impl ImgPreview {
    pub fn detect_image_format(data: &[u8]) -> Option<&'static str> {
        if data.len() < 4 {
            return None; // 数据不够长，无法识别
        }

        match &data[0..4] {
            [0x89, 0x50, 0x4E, 0x47] => Some("png"),
            [0xFF, 0xD8, _, _] => Some("jpg"),
            [0x47, 0x49, 0x46, 0x38] => Some("gif"),
            [0x52, 0x49, 0x46, 0x46]
            if data.len() >= 12 && &data[8..12] == [0x57, 0x45, 0x42, 0x50] =>
                {
                    Some("webp")
                }
            _ => None, // 未知格式
        }
    }

    pub fn new(x: i32, y: i32, width: i32, height: i32, id: &'static str) -> Self {
        let mut preview = frame::Frame::new(x, y, width, height, "");
        preview.set_frame(enums::FrameType::FlatBox);
        preview.set_color(enums::Color::from_rgb(80, 80, 80));
        preview.set_id(id);
        Self { preview, x, y, width, height }
    }

    // 判断鼠标是否在当前元素
    pub fn existPoint(&self, x: i32, y: i32) -> bool {
        return x > self.x.clone()
            && x < self.x.clone() + self.width.clone()
            && y > self.y.clone()
            && y < self.y.clone() + self.height.clone();
    }

    pub fn load(&mut self, path: String, x: i32, y: i32, width: i32, height: i32) -> bool {
        let mut res = false;
        if let Result::Ok(data) = fs::read(path) {
            res = self.from_data(data, x, y, width, height);
        }
        res
    }

    pub fn from_data(&mut self, data: Vec<u8>, x: i32, y: i32, width: i32, height: i32) -> bool {
        let mut res = false;
        macro_rules! re_imag {
            ($imag:expr) => {
                $imag.scale(width, height, false, true);
                self.preview.draw(move |cb| {
                    let cbx: i32 = cb.x();
                    let cby: i32 = cb.y();
                    let cbh: i32 = cb.h();
                    let cbw: i32 = cb.w();
                    let cx: i32 = x;
                    let cy: i32 = y;
                    $imag.draw_ext(cbx, cby, cbw, cbh, cx, cy);
                });
                self.preview.redraw();
                self.preview.redraw_label();
                res = true;
            };
        }

        if let Some(ext) = ImgPreview::detect_image_format(&data) {
            if (ext.as_bytes().eq("png".as_bytes())) {
                if let Result::Ok(mut img) = image::PngImage::from_data(&*data) {
                    re_imag!(img);
                }
            } else if (ext.as_bytes().eq("jpg".as_bytes())) {
                if let Result::Ok(mut img) = image::JpegImage::from_data(&*data) {
                    re_imag!(img);
                }
            } else if (ext.as_bytes().eq("gif".as_bytes())) {
                if let Result::Ok(mut img) = image::GifImage::from_data(&*data) {
                    re_imag!(img);
                }
            }
        }

        res
    }

    pub fn from_svg(&mut self, data: &str, x: i32, y: i32, width: i32, height: i32) -> bool {
        let mut res = false;
        macro_rules! re_imag {
            ($imag:expr) => {
                $imag.scale(width, height, false, true);
                self.preview.draw(move |cb| {
                    let cbx: i32 = cb.x();
                    let cby: i32 = cb.y();
                    let cbh: i32 = cb.h();
                    let cbw: i32 = cb.w();
                    let cx: i32 = x;
                    let cy: i32 = y;
                    $imag.draw_ext(cbx, cby, cbw, cbh, cx, cy);
                });
                self.preview.redraw();
                self.preview.redraw_label();
                res = true;
            };
        }
        if let Result::Ok(mut img) = image::SvgImage::from_data(data) {
            re_imag!(img);
        }
        res
    }
}
