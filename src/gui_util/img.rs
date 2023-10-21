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

    pub fn resize_debug(&mut self){

       self.preview.handle({
        let mut x = 0;
        let mut y = 0;
        let mut _debug_activate = false;
        let mut _re_pos = false;
        let mut _re_size = false;
        let mut _re_label_size = false;
        let mut _re_fast = 0;
        let mut _re_fast_add = false;
        let mut log = false;
        let mut _frame  = false;
        let mut _re_interior = false;
        let mut _self = self.clone();
        let mut _self_data_x = _self.data_x;
        let mut _self_data_y = _self.data_y;
        let mut _self_data_width = self.width;
        let mut _self_data_height = self.height;

        move |win, ev| match ev {
            fltk::enums::Event::Show=>{
                if !log{
                    println!("TextControl[debug] -> {}   单击激活/关闭  \n ShiftL/ShiftR 加减速\n Control 调整位置\n 调整大小 调整宽高\n CapsLock 调整文本 win 启用/关闭框",win.label());
                    log=true;
                }
                true
            }
            fltk::enums::Event::Push=>{
                _debug_activate=!_debug_activate;
                
                println!("<{}>启用元素debug -> {}",win.label(),_debug_activate);

                true
            }
            
            fltk::enums::Event::KeyUp=>{
                
                if(!_debug_activate){
                    return  false;
                }

                let (mut x,mut y,mut w,mut h,mut label_size) = (0,0,0,0,0);
                
                // println!("fltk::app::event_key()->{:?}",fltk::app::event_key());

                if fltk::app::event_key()==fltk::enums::Key::from_char('0') {
                    _re_fast=0;
                    println!("设置加速 值->{}",_re_fast);
                    _re_fast_add =false;
                }
                if fltk::app::event_key()==fltk::enums::Key::from_char('1') {
                    _re_fast=1;
                    println!("设置加速 值->{}",_re_fast);
                    _re_fast_add =false;
                }
                if fltk::app::event_key()==fltk::enums::Key::from_char('2') {
                    _re_fast=2;
                    println!("设置加速 值->{}",_re_fast);
                    _re_fast_add =false;
                }
                if fltk::app::event_key()==fltk::enums::Key::from_char('3') {
                    _re_fast=3;
                    println!("设置加速 值->{}",_re_fast);
                    _re_fast_add =false;
                }
                if fltk::app::event_key()==fltk::enums::Key::from_char('4') {
                    _re_fast=4;
                    println!("设置加速 值->{}",_re_fast);
                    _re_fast_add =false;
                }
                if fltk::app::event_key()==fltk::enums::Key::from_char('5') {
                    _re_fast=5;
                    println!("设置加速 值->{}",_re_fast);
                    _re_fast_add =false;
                }
                if fltk::app::event_key()==fltk::enums::Key::from_char('6') {
                    _re_fast=6;
                    println!("设置加速 值->{}",_re_fast);
                    _re_fast_add =false;
                }
                if fltk::app::event_key()==fltk::enums::Key::from_char('7') {
                    _re_fast=7;
                    println!("设置加速 值->{}",_re_fast);
                    _re_fast_add =false;
                }
                if fltk::app::event_key()==fltk::enums::Key::from_char('8') {
                    _re_fast=8;
                    println!("设置加速 值->{}",_re_fast);
                    _re_fast_add =false;
                }
                if fltk::app::event_key()==fltk::enums::Key::from_char('9') {
                    _re_fast=9;
                    println!("设置加速 值->{}",_re_fast);
                    _re_fast_add =false;
                }

                if fltk::app::event_key()==fltk::enums::Key::Menu{
                    _debug_activate =false;
                }

                if fltk::app::event_key()==fltk::enums::Key::MetaL{
                    win.hide();
                    _frame = !_frame;
                    if _frame {
                        win.set_frame(fltk::enums::FrameType::FlatBox);
                        println!("框框启用");
                    }else{
                        win.set_frame(fltk::enums::FrameType::NoBox);
                        println!("框框关闭");
                    }
                    win.show();
                }

                if fltk::app::event_key()==fltk::enums::Key::ControlL||fltk::app::event_key()==fltk::enums::Key::ControlR {
                    _re_pos=!_re_pos;
                    _re_size = false;
                    _re_label_size = false;
                    _re_fast_add = false;

                    if _re_pos{
                        println!("(√)开始  调整坐标 (x,y)",);
                    }else{
                        println!("( )结束  调整坐标 (x,y)");
                    }
                    return  true;
                }
                

                if fltk::app::event_key()==fltk::enums::Key::ShiftL ||fltk::app::event_key()==fltk::enums::Key::ShiftR {
                    _re_size=!_re_size;
                    _re_pos = false;
                    _re_label_size = false;
                    _re_fast_add = false;

                    if _re_size{
                        println!("(√)开始  调整宽高 (w,h)",);
                    }else{
                        println!("( )结束  调整宽高 (w,h)");
                    }
                    return  true;
                }

                if fltk::app::event_key()==fltk::enums::Key::AltL || fltk::app::event_key()==fltk::enums::Key::AltR  {
                    _re_interior=!_re_interior;

                    if _re_label_size{
                        println!("(√)开始  内径大小",);
                    }else{
                        println!("( )结束  内径大小");
                    }
                    return  true;
                }

                if fltk::app::event_key()==fltk::enums::Key::CapsLock {
                    _re_fast_add=!_re_fast_add;
                    _re_pos = false;
                    _re_size = false;
                    _re_label_size = false;

                    if _re_fast_add{
                        println!("开始设置加速");
                    }else{
                        println!("关闭设置加速");
                    }
                    return  true;
                }


                if _re_fast_add {
                    if fltk::app::event_key()==fltk::enums::Key::Up {
                        if _re_fast>20{
                            _re_fast=20;
                        }
                        _re_fast+=1;
                        println!("设置加速 增加->{}",_re_fast);
                    }
                    else if fltk::app::event_key()==fltk::enums::Key::Down {
                        if _re_fast<0{
                            _re_fast=0;
                        }
                        _re_fast-=1;
                        println!("设置加速 减少->{}",_re_fast);
                    }
                    return false;
                  }

                else if _re_pos {
                    if fltk::app::event_key()==fltk::enums::Key::Down {
                        y+=(1+_re_fast);
                    }
                    else if fltk::app::event_key()==fltk::enums::Key::Up {
                        y-=(1+_re_fast);
                    }
                    else if fltk::app::event_key()==fltk::enums::Key::Left {
                        x-=(1+_re_fast);
                    }
                    else if fltk::app::event_key()==fltk::enums::Key::Right {
                        x+=(1+_re_fast);
                    }
    
                }
              
                else if _re_size {
                    if fltk::app::event_key()==fltk::enums::Key::Down {
                        h-=(1+_re_fast);
                    }
                    else if fltk::app::event_key()==fltk::enums::Key::Up {
                        h+=(1+_re_fast);
                    }else if fltk::app::event_key()==fltk::enums::Key::Left {
                        w-=(1+_re_fast);
                    }
                    else if fltk::app::event_key()==fltk::enums::Key::Right {
                        w+=(1+_re_fast);
                    }
                }
              
                else if _re_label_size {
                if fltk::app::event_key()==fltk::enums::Key::Down {
                    label_size+=1;
                }
                else if fltk::app::event_key()==fltk::enums::Key::Up {
                    label_size-=1;
                }
              }

               if(_re_interior&&(_re_pos||_re_size)){
                win.hide();
                // win.set_pos(win.x()+x,win.y()+y);
                // win.resize(win.x()+x,win.y()+y,win.w()+w,win.h()+h);
                // win.set_label_size(win.label_size()+label_size);
             
                _self_data_y-=y;
                _self_data_x-=x;
                _self_data_width+=w;
                _self_data_height-=h;
                
                _self.from_data(_self.data.to_vec(),_self_data_x,_self_data_y,_self_data_width,_self_data_height);
              
                win.redraw();
                win.redraw_label();

                win.show();
                println!("hotspot [pos]-> gui_util::ImgPreview::new({},{},{},{},'').load(String::from('img'), {},{},{},{});" ,win.x(),win.y(),win.w(),win.h() , _self_data_x,_self_data_y,_self_data_width,_self_data_height);
               }
               else if _re_pos||_re_size||_re_label_size{
                win.hide();
                // win.set_pos(win.x()+x,win.y()+y);
                win.resize(win.x()+x,win.y()+y,win.w()+w,win.h()+h);
                win.set_label_size(win.label_size()+label_size);
              
                _self.from_data(_self.data.to_vec(), 0, 0, win.w(), win.h());
                _self_data_height = win.h();
                _self_data_width = win.w();

                win.redraw();
                win.redraw_label();

                win.show();
                println!("hotspot [pos]-> gui_util::ImgPreview::new({},{},{},{},'');" ,win.x(),win.y(),win.w(),win.h());
              }


                true
            }
            _ => false,
        }
    });
    }


}
