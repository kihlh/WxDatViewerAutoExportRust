#![allow(warnings, unused)]

use fltk::enums::{Color, Cursor, Event, FrameType, Key};
use fltk::{prelude::*, *};
use fltk::app::event_key;
use fltk_theme::{color_themes, ColorTheme, SchemeType, ThemeType, WidgetScheme, WidgetTheme};

pub struct TextPreview {
    pub buf:text::TextBuffer,
    pub preview:text::TextDisplay,
    x:i32,
    y:i32,
    height:i32,
    width:i32,
    size:i32
}
impl Clone for TextPreview {
    fn clone(&self) -> Self {
        TextPreview{
            buf: self.buf.clone(),
            preview:self.preview.clone(),
            x: self.x.clone(),
            y: self.y.clone(),
            width: self.width.clone(),
            height: self.height.clone(),
            size:self.size.clone()
        }
    }
}
impl TextPreview {
    // 创建
    pub fn new(x:i32, y:i32, width: i32, height: i32, size:i32, input:&str, rgb: [u8; 3]) -> Self {
        let mut buf = text::TextBuffer::default();
        buf.set_text(input);

        let mut txt = text::TextDisplay::default()
            .with_size(width, height)
            .center_of_parent();
        txt.set_buffer(buf.clone());
        txt.set_pos(x,y);
        txt.set_frame(fltk::enums::FrameType::NoBox);
        txt.set_scrollbar_size(-1);
        txt.set_text_size(size);
        txt.set_text_color(Color::from_rgb(rgb[0],rgb[1],rgb[2]));
        txt.scroll(0, 0);
        txt.set_label_type(fltk::enums::LabelType::None);

        // txt.handle(move |txt, event| match event {
        //     Event::Move=>{
        //
        //         true
        //     }
        //     Event::Leave=>{
        //
        //         true
        //     }
        //     _ => false,
        // });

        TextPreview{
            buf:buf,
            preview:txt,
            x,
            y,
            height,
            width,
            size
        }
    }

    // 获取文本内容
    pub fn get_label(&self)->String {
        return self.preview.label();
    }

    // 设置文本内容
    pub fn set_label(&mut self,input:String){
        self.preview.set_label(input.as_str());
        self.preview.redraw_label();
        self.preview.redraw();
    }

    // 设置颜色
    pub fn set_color(&mut self, r: u8, g: u8, b: u8) -> &mut TextPreview {
        self.preview.set_text_color(Color::from_rgb(r,g,b));
        self
    }

    // 判断鼠标是否在当前元素
    pub fn existPoint(&self, x: i32, y: i32) -> bool {
        return x > self.preview.x()
            && x < self.preview.x() + self.preview.width()
            && y > self.preview.y()
            && y < self.preview.y() + self.preview.height();
    }
    pub fn set_back_color(&mut self, r: u8, g: u8, b: u8) -> &mut TextPreview {
        self.preview.set_color(Color::from_rgb(r,g,b));
        self
    }

}




pub struct TextControl {
    pub text: frame::Frame,
}

impl TextControl {
    // 创建
    pub fn new(x:i32, y:i32, width: i32, height: i32, size:i32, input: &str, rgb: [u8; 3]) -> Self {
        let mut text = frame::Frame::new(x, y, (if width==0 {input.len()as i32 * size}else {width}), (if height==0 {size+2}else {height}), "");
        text.set_label(input);
        text.set_label_size(size);
        text.set_label_color(Color::from_rgb(rgb[0],rgb[1],rgb[2]));

        Self { text  }
    }
    // 获取文本内容
    pub fn get_label(&self)->String {
        return self.text.label();
    }
    // 设置文本内容
    pub fn set_label(&mut self,input:String){
        self.text.set_label(input.as_str());
        self.text.redraw_label();
        self.text.redraw();
    }
    // 设置颜色
    pub fn set_color(&mut self, r: u8, g: u8, b: u8)  {
        self.text.set_label_color(Color::from_rgb(r,g,b));

    }
    // 判断鼠标是否在当前元素
    pub fn existPoint(&self, x: i32, y: i32) -> bool {
        return x > self.text.x()
            && x < self.text.x() + self.text.width()
            && y > self.text.y()
            && y < self.text.y() + self.text.height();
    }

}
impl Clone for TextControl {
    fn clone(&self) -> Self {
        TextControl {
            text:self.text.clone()
        }
    }
}

