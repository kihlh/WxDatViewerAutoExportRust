#![allow(warnings, unused)]

use fltk::{enums::{Color, FrameType, self}, app, prelude::{WidgetBase, WidgetExt, InputExt}, input::Input};

use crate::libWxIkunPlus;



pub fn inject_input (input:& mut Input){
    
    input.handle({
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