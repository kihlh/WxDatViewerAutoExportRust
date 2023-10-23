#![allow(warnings, unused)]

use fltk::enums::{Color, Cursor, Event, FrameType, Key};
use fltk::{prelude::*, *};
use fltk::app::event_key;
use fltk_theme::{color_themes, ColorTheme, SchemeType, ThemeType, WidgetScheme, WidgetTheme};

use crate::libWxIkunPlus;
use crate::util::OverloadedAnyStr;

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

    pub fn resize_debug (&mut self){

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
            let mut _tap_hold_shift = false;
    
            move |win, ev| match ev {
                enums::Event::Show=>{
                    if !log{
                        println!("TextPreview[debug] -> {}   单击激活/关闭  \n ShiftL/ShiftR 加减速\n Control 调整位置\n 调整大小 调整宽高\n CapsLock 调整文本 win 启用/关闭框",win.label());
                        log=true;
                    }
                    true
                }
                enums::Event::Push=>{
                    _debug_activate=!_debug_activate;
                    
                    println!("<{}>启用元素debug -> {}",win.label(),_debug_activate);
                    if _debug_activate{
                        _re_pos=!_re_pos;
                        _re_size = false;
                        _re_label_size = false;
                        _re_fast_add = false;
                    }
                    
                    true
                }
                
                enums::Event::KeyUp=>{
                    
                    if(!_debug_activate){
                        return  false;
                    }
    
                    if (_tap_hold_shift&&fltk::app::event_key()==fltk::enums::Key::ShiftL){
                        _tap_hold_shift= false;
                        return false;
                    }
    
                    if fltk::app::event_key()==fltk::enums::Key::Menu{
                        _debug_activate =false;
                    }
    
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
    
                    let (mut x,mut y,mut w,mut h,mut label_size) = (0,0,0,0,0);
                    
                    // println!("app::event_key()->{:?}",app::event_key());
                    
                    if app::event_key()==fltk::enums::Key::MetaL{
                        win.hide();
                        _frame = !_frame;
                        if _frame {
                            win.set_frame(fltk::enums::FrameType::EngravedBox);
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
    
                    if app::event_key()==fltk::enums::Key::AltL || app::event_key()==fltk::enums::Key::AltR  {
                        _re_label_size=!_re_label_size;
                        _re_pos = false;
                        _re_size = false;
                        _re_fast_add = false;
    
                        if _re_label_size{
                            println!("(√)开始  文本大小",);
                        }else{
                            println!("( )结束  文本大小");
                        }
                        return  true;
                    }
    
                    if app::event_key()==fltk::enums::Key::CapsLock {
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
                        if app::event_key()==fltk::enums::Key::Up {
                            if _re_fast>20{
                                _re_fast=20;
                            }
                            _re_fast+=1;
                            println!("设置加速 增加->{}",_re_fast);
                        }
                        else if app::event_key()==fltk::enums::Key::Down {
                            if _re_fast<0{
                                _re_fast=0;
                            }
                            _re_fast-=1;
                            println!("设置加速 减少->{}",_re_fast);
                        }
                        return false;
                      }
    
                    else if _re_pos {
                        _tap_hold_shift= libWxIkunPlus::getBasicKeys().shift;
                        let fast_temp = (if _tap_hold_shift {5} else {0}) ;
    
                        if app::event_key()==fltk::enums::Key::Down {
                            y+=(1+_re_fast+fast_temp);
                        }
                        else if app::event_key()==fltk::enums::Key::Up {
                            y-=(1+_re_fast+fast_temp);
                        }
                        else if app::event_key()==fltk::enums::Key::Left {
                            x-=(1+_re_fast+fast_temp);
                        }
                        else if app::event_key()==fltk::enums::Key::Right {
                            x+=(1+_re_fast+fast_temp);
                        }
        
                    }
                  
                    else if _re_size {
                        _tap_hold_shift= libWxIkunPlus::getBasicKeys().shift;
                        let fast_temp = (if _tap_hold_shift {5} else {0}) ;
    
                        if app::event_key()==fltk::enums::Key::Down {
                            h-=(1+_re_fast+fast_temp);
                        }
                        else if app::event_key()==fltk::enums::Key::Up {
                            h+=(1+_re_fast+fast_temp);
                        }else if app::event_key()==fltk::enums::Key::Left {
                            w-=(1+_re_fast+fast_temp);
                        }
                        else if app::event_key()==fltk::enums::Key::Right {
                            w+=(1+_re_fast+fast_temp);
                        }
                    }
                  
                    else if _re_label_size {
                        _tap_hold_shift= libWxIkunPlus::getBasicKeys().shift;
                        let fast_temp = (if _tap_hold_shift {2} else {0}) ;
    
                    if app::event_key()==fltk::enums::Key::Down {
                        label_size+=1+fast_temp;
                    }
                    else if app::event_key()==fltk::enums::Key::Up {
                        label_size-=1+fast_temp;
                    }
                  }
    
             
                    if _re_pos||_re_size||_re_label_size{
                    win.hide();
                    // win.set_pos(win.x()+x,win.y()+y);
                    win.resize(win.x()+x,win.y()+y,win.w()+w,win.h()+h);
                    win.set_text_size(win.text_size()+label_size);
    
                    // win.redraw();
                    win.show();
    
                    println!(
                        "win [pos]-> gui_util::TextPreview::new({},{},{},{},{},\"{}\",[122, 120, 120]); "
                        ,win.x(),win.y(),win.w(),win.h(),
                        win.text_size(),win.text_size()
                    );
    
                  }
    
    
                    true
                }
                _ => false,
            }
        });
       }

       
}




pub struct TextControl {
    pub text: frame::Frame,
}

impl TextControl {
    // 创建
    pub fn new<T: super::lib::OverloadedAnyStr,Color: super::lib::LoadedAnyColor  >(x:i32, y:i32, width: i32, height: i32, size:i32, input:T, color: Color) -> Self {

    let input = input.to_string_default();
    let rgb = color.to_rgb();

    let mut text = frame::Frame::new(x, y, (if width==0 {input.len()as i32 * size}else {width}), (if height==0 {size+2}else {height}), "");
        text.set_label(&input);
        text.set_label_size(size);
        text.set_label_color(fltk::enums::Color::from_rgb(rgb.0,rgb.1,rgb.2));

        Self { text  }
    }
    // 获取文本内容
    pub fn get_label(&self)->String {
        return self.text.label();
    }
    // 设置文本内容
    pub fn set_label <T: super::lib::OverloadedAnyStr > (&mut self,input:T){
        let input = input.to_string_default();
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
    pub fn default <T: super::lib::OverloadedAnyStr > (input: T) -> Self  {
        let input = input.to_string_default();
        let [x,y,width,height,size] = [25,25,0,0,13];
        let rgb = [255u8,255u8,255u8];
        let mut text = frame::Frame::new(x, y, (if width==0 {input.len()as i32 * size}else {width}), (if height==0 {size+2}else {height}), "");
        text.set_label(&input);
        text.set_label_size(size);
        text.set_label_color(Color::from_rgb(rgb[0],rgb[1],rgb[2]));

        let mut res = Self { text  };
        res.resize_debug();
        res
    }
    pub fn add_cursor_hand(&mut self,win:& window::DoubleWindow){
        self.text.handle({
            let mut win = win.clone();
            move |this_win, ev| match ev {
                enums::Event::Move => {
                    win.set_cursor(fltk::enums::Cursor::Hand);
                    true
                }
                enums::Event::Leave=>{
                    win.set_cursor(fltk::enums::Cursor::Default);
                    true
                }

                _=>false
            } });
    }

    pub fn resize_debug (&mut self) -> TextControl {

    self.text.handle({
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
        let mut _tap_hold_shift = false;

        move |win, ev| match ev {
            enums::Event::Show=>{
                if !log{
                    println!("TextControl[debug] -> {}   单击激活/关闭  \n ShiftL/ShiftR 加减速\n Control 调整位置\n 调整大小 调整宽高\n CapsLock 调整文本 win 启用/关闭框",win.label());
                    log=true;
                }
                true
            }
            enums::Event::Push=>{
                _debug_activate=!_debug_activate;
                
                println!("<{}>启用元素debug -> {}",win.label(),_debug_activate);
                if _debug_activate{
                    _re_pos=!_re_pos;
                    _re_size = false;
                    _re_label_size = false;
                    _re_fast_add = false;
                }
                
                true
            }
            
            enums::Event::KeyUp=>{
                
                if(!_debug_activate){
                    return  false;
                }

                if (_tap_hold_shift&&fltk::app::event_key()==fltk::enums::Key::ShiftL){
                    _tap_hold_shift= false;
                    return false;
                }

                if fltk::app::event_key()==fltk::enums::Key::Menu{
                    _debug_activate =false;
                }

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

                let (mut x,mut y,mut w,mut h,mut label_size) = (0,0,0,0,0);
                
                // println!("app::event_key()->{:?}",app::event_key());
                
                if app::event_key()==fltk::enums::Key::MetaL{
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

                if app::event_key()==fltk::enums::Key::AltL || app::event_key()==fltk::enums::Key::AltR  {
                    _re_label_size=!_re_label_size;
                    _re_pos = false;
                    _re_size = false;
                    _re_fast_add = false;

                    if _re_label_size{
                        println!("(√)开始  文本大小",);
                    }else{
                        println!("( )结束  文本大小");
                    }
                    return  true;
                }

                if app::event_key()==fltk::enums::Key::CapsLock {
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
                    if app::event_key()==fltk::enums::Key::Up {
                        if _re_fast>20{
                            _re_fast=20;
                        }
                        _re_fast+=1;
                        println!("设置加速 增加->{}",_re_fast);
                    }
                    else if app::event_key()==fltk::enums::Key::Down {
                        if _re_fast<0{
                            _re_fast=0;
                        }
                        _re_fast-=1;
                        println!("设置加速 减少->{}",_re_fast);
                    }
                    return false;
                  }

                else if _re_pos {
                    _tap_hold_shift= libWxIkunPlus::getBasicKeys().shift;
                    let fast_temp = (if _tap_hold_shift {5} else {0}) ;

                    if app::event_key()==fltk::enums::Key::Down {
                        y+=(1+_re_fast+fast_temp);
                    }
                    else if app::event_key()==fltk::enums::Key::Up {
                        y-=(1+_re_fast+fast_temp);
                    }
                    else if app::event_key()==fltk::enums::Key::Left {
                        x-=(1+_re_fast+fast_temp);
                    }
                    else if app::event_key()==fltk::enums::Key::Right {
                        x+=(1+_re_fast+fast_temp);
                    }
    
                }
              
                else if _re_size {
                    _tap_hold_shift= libWxIkunPlus::getBasicKeys().shift;
                    let fast_temp = (if _tap_hold_shift {5} else {0}) ;

                    if app::event_key()==fltk::enums::Key::Down {
                        h-=(1+_re_fast+fast_temp);
                    }
                    else if app::event_key()==fltk::enums::Key::Up {
                        h+=(1+_re_fast+fast_temp);
                    }else if app::event_key()==fltk::enums::Key::Left {
                        w-=(1+_re_fast+fast_temp);
                    }
                    else if app::event_key()==fltk::enums::Key::Right {
                        w+=(1+_re_fast+fast_temp);
                    }
                }
              
                else if _re_label_size {
                    _tap_hold_shift= libWxIkunPlus::getBasicKeys().shift;
                    let fast_temp = (if _tap_hold_shift {2} else {0}) ;

                if app::event_key()==fltk::enums::Key::Down {
                    label_size+=1+fast_temp;
                }
                else if app::event_key()==fltk::enums::Key::Up {
                    label_size-=1+fast_temp;
                }
              }

         
                if _re_pos||_re_size||_re_label_size{
                win.hide();
                // win.set_pos(win.x()+x,win.y()+y);
                win.resize(win.x()+x,win.y()+y,win.w()+w,win.h()+h);
                win.set_label_size(win.label_size()+label_size);

                // win.redraw();
                win.show();

                println!(
                    "win [pos]-> gui_util::TextControl::new({},{},{},{},{},\"{}\",[122, 120, 120]); "
                    ,win.x(),win.y(),win.w(),win.h(),
                    win.label_size(),win.label()
                );

              }


                true
            }
            _ => false,
        }
    });

        self.clone()
   }
}
impl Clone for TextControl {
    fn clone(&self) -> Self {
        TextControl {
            text:self.text.clone()
        }
    }
}

