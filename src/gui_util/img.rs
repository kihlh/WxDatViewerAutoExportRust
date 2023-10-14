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
    pub x:i32,
    pub y:i32,
    pub width: i32,
    pub height: i32,
    pub(crate) data: Vec<u8>,
    pub img_type:ImgPreviewDataType,
    pub data_x:i32,
    pub data_y:i32,
    pub data_width: i32,
    pub data_height: i32,
}
pub enum ImgPreviewDataType {
    NoneS,
    Svg,
    Jpeg,
    Png,
    Gif
}

impl Clone for ImgPreview {
    fn clone(&self) -> Self {
        ImgPreview {
            preview:self.preview.clone(),
            x: self.x.clone(),
            y: self.y.clone(),
            width: self.width.clone(),
            height: self.height.clone(),
            data:self.data.clone(),
            img_type: ImgPreviewDataType::Svg,
            data_x: self.data_x.clone(),
            data_y: self.data_y.clone(),
            data_width: self.data_width.clone(),
            data_height: self.data_height.clone(),
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
        Self { preview, x, y, width, height ,data:Vec::new(), img_type: ImgPreviewDataType::NoneS, data_x: 0, data_y: 0, data_width: 0, data_height: 0 }
    }

    pub fn new2(x: i32, y: i32, width: i32, height: i32, id: &'static str, data_x: i32, data_y: i32, data_width: i32, data_height: i32) -> Self {
        let mut preview = frame::Frame::new(x, y, width, height, "");
        preview.set_frame(enums::FrameType::FlatBox);
        preview.set_color(enums::Color::from_rgb(80, 80, 80));
        preview.set_id(id);
        Self { preview, x, y, width, height ,data:Vec::new(), img_type: ImgPreviewDataType::NoneS, data_x , data_y, data_width, data_height}
    }

    pub fn new_border(x: i32, y: i32, width: i32, height: i32,svg_data:&str) -> Self {
        let mut preview = frame::Frame::new(x, y, width, height, "");
        preview.set_frame(enums::FrameType::NoBox);
        preview.set_color(enums::Color::from_rgb(80, 80, 80));
        let mut self_data =  Self { preview, x, y, width, height ,data:svg_data.as_bytes().to_vec(), img_type: ImgPreviewDataType::Svg, data_x:0 , data_y:0, data_width:width, data_height:height};
        self_data.from_svg(svg_data,0,0,width,height);
        self_data
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
                self.data_height = height;
                self.data_width = width;
                self.data_x = x;
                self.data_y = y;
                self.preview.redraw();
                self.preview.redraw_label();
                res = true;
            };
        }

        if let Some(ext) = ImgPreview::detect_image_format(&data) {
            self.data = data.to_vec();

            if (ext.as_bytes().eq("png".as_bytes())) {
                if let Result::Ok(mut img) = image::PngImage::from_data(&*data) {
                    re_imag!(img);
                    self.img_type = ImgPreviewDataType::Png
                }
            } else if (ext.as_bytes().eq("jpg".as_bytes())) {
                if let Result::Ok(mut img) = image::JpegImage::from_data(&*data) {
                    self.img_type = ImgPreviewDataType::Jpeg;
                    re_imag!(img);
                }
            } else if (ext.as_bytes().eq("gif".as_bytes())) {
                if let Result::Ok(mut img) = image::GifImage::from_data(&*data) {
                    self.img_type = ImgPreviewDataType::Gif;
                    re_imag!(img);
                }
            }
        }

        res
    }

    pub fn re_data(&mut self, data: Vec<u8>){
        self.from_data(data,self.data_x,self.data_y,self.data_width,self.data_height);
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
            self.img_type = ImgPreviewDataType::Svg;
            self.data = data.as_bytes().to_vec();

            re_imag!(img);
        }
        res
    }

    pub fn get_data (&self) -> Vec<u8> {
        self.data.to_vec()
    }

    pub fn as_mut (&mut self) -> &mut ImgPreview {
         self
    }

}
