#![allow(warnings, unused)]

use fltk::{enums, window};
use fltk::enums::FrameType;
use fltk::prelude::{WidgetBase, WidgetExt, WindowExt};

use crate::libWxIkunPlus;

use super::img;

pub struct HotspotItmeControl {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}
impl HotspotItmeControl {
    pub(crate) fn clone(&self) -> Self {
        HotspotItmeControl{
            x: self.x.clone(),
            y: self.y.clone(),
            width: self.width.clone(),
            height: self.height.clone(),
        }
    }
}
impl HotspotItmeControl {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        let mut check_item_control = HotspotItmeControl {
            x,
            y,
            width,
            height,
        };

        check_item_control
    }

    // 判断鼠标是否在当前元素
    pub fn existPoint(&self, x: i32, y: i32) -> bool {
        return x > self.x.clone()
            && x < self.x.clone() + self.width.clone()
            && y > self.y.clone()
            && y < self.y.clone() + self.height.clone();
    }

    pub fn set_callback (&mut self, cb: fn()){
        let mut main = fltk::frame::Frame::new(self.x,self.y,self.width,self.height, "");
        main.set_frame(FrameType::NoBox);
        main.handle(move |txt, event| match event {
            fltk::enums::Event::Push=>{
                cb();
                true
            }
           fltk::enums::Event::Move=>{
                // cb();
                true
            }
            fltk::enums::Event::Leave=>{
                // cb();
                true
            }
            _ => false,
        });
    }

    pub fn add_cursor_hand(&mut self,win:& window::DoubleWindow){
        let mut main = fltk::frame::Frame::new(self.x,self.y,self.width,self.height, "");
        main.set_frame(FrameType::NoBox);
        main.handle({
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
    pub fn add_cursor_hand_callback(&mut self,win:& window::DoubleWindow,back:fn(win:window::DoubleWindow,frame:fltk::frame::Frame)){
        let mut main = fltk::frame::Frame::new(self.x,self.y,self.width,self.height, "");
        main.set_frame(FrameType::NoBox);
        main.handle({
            let mut win = win.clone();

            move |this_win, ev| match ev {
                enums::Event::Push => {
                    back(win.clone(),this_win.clone());
                    true
                }
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
    fn default(&mut self) -> Self{
        let [ x,
        y,
        width,
        height] = [15,15,50,50];

        let mut check_item_control = HotspotItmeControl {
            x,
            y,
            width,
            height,
        };

        check_item_control.resize_debug();

        check_item_control
    }

    pub fn resize_debug(&self){
        let mut svg_view =  format!(r#"<svg width="{}" height="{}" viewBox="0 0 {} {}" fill="none" xmlns="http://www.w3.org/2000/svg">
        <rect x="0.5" y="0.5" width="{}" height="{}" fill='#397B84' fill-opacity="0.25" stroke='#C09898'/>
        </svg>
        "#,self.width, self.height,self.width, self.height,self.width-1, self.height-1);

       let mut view = img::ImgPreview::new_border(self.x, self.y, self.width, self.height,svg_view.as_str());

       view.preview.handle({
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
        let mut view = view.clone();
        let mut _tap_hold_shift = false;

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
                if _debug_activate{
                    _re_pos=!_re_pos;
                    _re_size = false;
                    _re_label_size = false;
                    _re_fast_add = false;
                }
                
                true
            }
            
            fltk::enums::Event::KeyUp=>{
                
                if(!_debug_activate){
                    return  false;
                }
                
                if (_tap_hold_shift&&fltk::app::event_key()==fltk::enums::Key::ShiftL){
                    _tap_hold_shift= false;
                    return false;
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

                    _tap_hold_shift= libWxIkunPlus::getBasicKeys().shift;
                    let fast_temp = (if _tap_hold_shift {5} else {0}) ;


                    if fltk::app::event_key()==fltk::enums::Key::Down {
                        y+=(1+_re_fast+fast_temp);
                    }
                    else if fltk::app::event_key()==fltk::enums::Key::Up {
                        y-=(1+_re_fast+fast_temp);
                    }
                    else if fltk::app::event_key()==fltk::enums::Key::Left {
                        x-=(1+_re_fast+fast_temp);
                    }
                    else if fltk::app::event_key()==fltk::enums::Key::Right {
                        x+=(1+_re_fast+fast_temp);
                    }
    
                }
              
                else if _re_size {
                    _tap_hold_shift= libWxIkunPlus::getBasicKeys().shift;
                    let fast_temp = (if _tap_hold_shift {5} else {0}) ;


                    if fltk::app::event_key()==fltk::enums::Key::Down {
                        h-=(1+_re_fast+fast_temp);
                    }
                    else if fltk::app::event_key()==fltk::enums::Key::Up {
                        h+=(1+_re_fast+fast_temp);
                    }else if fltk::app::event_key()==fltk::enums::Key::Left {
                        w-=(1+_re_fast+fast_temp);
                    }
                    else if fltk::app::event_key()==fltk::enums::Key::Right {
                        w+=(1+_re_fast+fast_temp);
                    }
                }
              
                else if _re_label_size {
                    _tap_hold_shift= libWxIkunPlus::getBasicKeys().shift;
                    let fast_temp = (if _tap_hold_shift {2} else {0}) ;

                if fltk::app::event_key()==fltk::enums::Key::Down {
                    label_size+=1+fast_temp;
                }
                else if fltk::app::event_key()==fltk::enums::Key::Up {
                    label_size-=1+fast_temp;
                }
              }

         
                if _re_pos||_re_size||_re_label_size{
                win.hide();
                // win.set_pos(win.x()+x,win.y()+y);
                win.resize(win.x()+x,win.y()+y,win.w()+w,win.h()+h);
                win.set_label_size(win.label_size()+label_size);

                let mut svg_view =  format!(r#"<svg width="{}" height="{}" viewBox="0 0 {} {}" fill="none" xmlns="http://www.w3.org/2000/svg">
                 <rect x="0.5" y="0.5" width="{}" height="{}" fill='#397B84' fill-opacity="0.25" stroke='#C09898'/>
                 </svg>
                 "#,win.w(), win.h(),win.w(), win.h(),win.w()-1, win.h()-1);

                view.from_svg(svg_view.as_str(), 0, 0, win.w(), win.h());

                // win.redraw();
                win.show();
                println!("hotspot [pos]-> gui_util::create_hotspot({},{},{},{});" ,win.x(),win.y(),win.w(),win.h());
              }


                true
            }
            _ => false,
        }
    });
    }
}

pub fn create_hotspot(x: i32, y: i32, width: i32, height: i32) -> HotspotItmeControl {
    HotspotItmeControl::new(x, y, width, height)
}
