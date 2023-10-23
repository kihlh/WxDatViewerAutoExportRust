#![allow(warnings, unused)]

use fltk::enums::{Color, Cursor, Event, FrameType, Key};
use fltk::{prelude::*, *};
use fltk::app::event_key;
use fltk_theme::{color_themes, ColorTheme, SchemeType, ThemeType, WidgetScheme, WidgetTheme};
use crate::gui_util::lib;
use crate::gui_util::lib::LoadedAnyColor;
use crate::libWxIkunPlus;

pub struct BorderPreview {
    pub preview: frame::Frame,
    x:i32,
    y:i32,
    width: i32,
    height: i32,
    radius: usize,
    color:(u8,u8,u8,f32),
    stroke:(u8,u8,u8,f32),
    pub stroke_width: i32,
}

impl Clone for BorderPreview {
    fn clone(&self) -> Self {
        BorderPreview {
            preview: self.preview.clone(),
            x: self.x.clone(),
            y: self.y.clone(),
            width: self.width.clone(),
            height: self.height.clone(),
            radius: self.radius.clone(),
            color: (self.color.0.clone(),self.color.1.clone(),self.color.2.clone(),self.color.3.clone()),
            stroke:(self.stroke.0.clone(),self.stroke.1.clone(),self.stroke.2.clone(),self.stroke.3.clone()),
            stroke_width: self.stroke_width.clone(),
        }
    }
}

impl BorderPreview{
    pub fn new <Color: lib::LoadedAnyColor > (x: i32, y: i32, width: i32, height: i32, radius:usize,color:Color,stroke:Color,stroke_width:i32) -> Self {
        let mut preview = frame::Frame::new(x, y, width, height, "");
        preview.set_frame(enums::FrameType::NoBox);
        // <svg width="125" height="38" viewBox="0 0 125 38" fill="none" xmlns="http://www.w3.org/2000/svg">
        //     <rect width="125" height="38" fill="#181818" fill-opacity="0.3"/>
        //     <rect x="1.5" y="1.5" width="122" height="35" stroke="#323232" stroke-opacity="0.7" stroke-width="3"/>
        //     </svg>
        // <svg width="125" height="38" viewBox="0 0 125 38" fill="none" xmlns="http://www.w3.org/2000/svg">
        //     <rect width="125" height="38" rx="6" fill="#181818" fill-opacity="0.3"/>
        //     <rect x="1.5" y="1.5" width="122" height="35" rx="4.5" stroke="#323232" stroke-opacity="0.7" stroke-width="3"/>
        //     </svg>

        let mut svg_view =  format!(r#"
        <svg width="{width}" height="{height}" viewBox="0 0 {width} {height}" fill="none" >
            <rect width="{width}" height="{height}" rx="{radius}" fill="{fill}" fill-opacity="{fill_opacity}"/>
            <rect x="{stroke_width_rect_stroke2}" y="{stroke_width_rect_stroke2}" rx="{radius_sk2}" width="{stroke_width_2}" height="{stroke_height_2}" stroke="{stroke}" stroke-opacity="{stroke_opacity}" stroke-width="{stroke_width}"/>
        </svg>"#,

                                    width=width,
                                    height=height ,
                                    radius=radius,
                                    stroke=stroke.to_hex(),
                                    fill=color.to_hex(),
                                    stroke_opacity= stroke.to_rgba().to_rgba().3,
                                    fill_opacity= color.to_rgba().to_rgba().3,
                                    stroke_width = stroke_width,
                                    stroke_width_rect_stroke2 =stroke_width/2,
                                    stroke_width_2=width-stroke_width,
                                    stroke_height_2=height-stroke_width,
                                    radius_sk2= (radius as i32) - (stroke_width/2)
        );

        let mut result =
            Self { preview, x, y, width, height, radius, color: color.to_rgba(), stroke:stroke.to_rgba(),
                stroke_width
            };
        // println!("svg_view-> {}", &svg_view);


        result.from_svg(svg_view.as_str(),0,0,width, height);
        result
    }

    pub fn as_mut (&mut self) -> &mut BorderPreview {
        self
    }

    // 判断鼠标是否在当前元素
    pub fn existPoint(&self, x: i32, y: i32) -> bool {
        return x > self.x.clone()
            && x < self.x.clone() + self.width.clone()
            && y > self.y.clone()
            && y < self.y.clone() + self.height.clone();
    }

    fn default(&mut self) -> Self{
        let mut res = BorderPreview::new(22,30,50,50,5,(255,255,255,0.8),(255,255,255,0.8),2);
        res.resize_debug();
        res
    }

    pub fn add_cursor_hand(&mut self,win:& window::DoubleWindow){

        self.preview.handle({
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

    pub fn from_svg <T: super::lib::OverloadedAnyStr > (&mut self, data: T, x: i32, y: i32, width: i32, height: i32) -> bool {
        let data = data.to_string_default();

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
        if let Result::Ok(mut img) = image::SvgImage::from_data(&data) {
            re_imag!(img);
        }
        res
    }

    pub fn re_svg_view(&mut self,width:i32,height:i32,radius:usize,stroke_width:i32){
        // let width =self.preview.w();
        // let height =self.preview.h();
        // let radius = self.radius.clone();
        let stroke = self.stroke;
        let color = self.color;
        // let stroke_width = self.stroke_width.clone();

        // <svg width="125" height="38" viewBox="0 0 125 38" fill="none" xmlns="http://www.w3.org/2000/svg">
        //     <rect width="125" height="38" fill="#181818" fill-opacity="0.3"/>
        //     <rect x="1.5" y="1.5" width="122" height="35" stroke="#323232" stroke-opacity="0.7" stroke-width="3"/>
        //     </svg>
        // <svg width="125" height="38" viewBox="0 0 125 38" fill="none" xmlns="http://www.w3.org/2000/svg">
        //     <rect width="125" height="38" rx="6" fill="#181818" fill-opacity="0.3"/>
        //     <rect x="1.5" y="1.5" width="122" height="35" rx="4.5" stroke="#323232" stroke-opacity="0.7" stroke-width="3"/>
        //     </svg>

        let mut svg_view =  format!(r#"
        <svg width="{width}" height="{height}" viewBox="0 0 {width} {height}" fill="none" >
            <rect width="{width}" height="{height}" rx="{radius}" fill="{fill}" fill-opacity="{fill_opacity}"/>
            <rect x="{stroke_width_rect_stroke2}" y="{stroke_width_rect_stroke2}" rx="{radius_sk2}" width="{stroke_width_2}" height="{stroke_height_2}" stroke="{stroke}" stroke-opacity="{stroke_opacity}" stroke-width="{stroke_width}"/>
        </svg>"#,

                                    width=width,
                                    height=height ,
                                    radius=radius,
                                    stroke=stroke.to_hex(),
                                    fill=color.to_hex(),
                                    stroke_opacity= stroke.to_rgba().to_rgba().3,
                                    fill_opacity= color.to_rgba().to_rgba().3,
                                    stroke_width = stroke_width,
                                    stroke_width_rect_stroke2 =stroke_width/2,
                                    stroke_width_2=width-stroke_width,
                                    stroke_height_2=height-stroke_width,
                                    radius_sk2= if radius!=0 {(radius as i32) - (stroke_width/2)}else{0}
        );

        // println!(" self.from_svg-> {}", &svg_view);

        self.from_svg(svg_view.as_str(),0,0,width, height);
    }


    pub fn resize_debug (&mut self) -> BorderPreview{

        self.preview.handle({
            let mut x = 0;
            let mut y = 0;
            let mut _debug_activate = false;
            let mut _re_pos = false;
            let mut _re_size = false;
            let mut _re_radius_size = false;
            let mut _re_fast = 0;
            let mut _re_fast_add = false;
            let mut log = false;
            let mut _frame  = false;
            let mut _tap_hold_shift = false;
            let mut _re_stroke_width_c = false;

            let mut _self = self.clone();

            let mut radius = _self.radius.clone() as i32;
            let mut stroke_width = _self.stroke_width.clone() as i32;

            move |win, ev| match ev {
                enums::Event::Show=>{
                    if !log{
                        println!("BorderPreview[debug] -> {}   单击激活/关闭  \n ShiftL/ShiftR 加减速\n Control 调整位置\n 调整大小 调整宽高\n CapsLock 调整圆角 win 启用/关闭框 Tab 调整描边",win.label());
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
                        _re_radius_size = false;
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
                    }
                    if fltk::app::event_key()==fltk::enums::Key::from_char('1') {
                        _re_fast=1;
                        println!("设置加速 值->{}",_re_fast);
                    }
                    if fltk::app::event_key()==fltk::enums::Key::from_char('2') {
                        _re_fast=2;
                        println!("设置加速 值->{}",_re_fast);
                    }
                    if fltk::app::event_key()==fltk::enums::Key::from_char('3') {
                        _re_fast=3;
                        println!("设置加速 值->{}",_re_fast);
                    }
                    if fltk::app::event_key()==fltk::enums::Key::from_char('4') {
                        _re_fast=4;
                        println!("设置加速 值->{}",_re_fast);
                    }
                    if fltk::app::event_key()==fltk::enums::Key::from_char('5') {
                        _re_fast=5;
                        println!("设置加速 值->{}",_re_fast);
                    }
                    if fltk::app::event_key()==fltk::enums::Key::from_char('6') {
                        _re_fast=6;
                        println!("设置加速 值->{}",_re_fast);
                    }
                    if fltk::app::event_key()==fltk::enums::Key::from_char('7') {
                        _re_fast=7;
                        println!("设置加速 值->{}",_re_fast);
                    }
                    if fltk::app::event_key()==fltk::enums::Key::from_char('8') {
                        _re_fast=8;
                        println!("设置加速 值->{}",_re_fast);
                    }
                    if fltk::app::event_key()==fltk::enums::Key::from_char('9') {
                        _re_fast=9;
                        println!("设置加速 值->{}",_re_fast);
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
                        _re_radius_size = false;
                        _re_fast_add = false;
                        _re_stroke_width_c = false;

                        if _re_pos{
                            println!("(√)开始  调整坐标 (x,y)",);
                        }else{
                            println!("( )结束  调整坐标 (x,y)");
                        }
                        return  true;
                    }

                    if fltk::app::event_key()==fltk::enums::Key::Tab{
                        _re_stroke_width_c=!_re_stroke_width_c;
                        _re_pos= false;
                        _re_size = false;
                        _re_radius_size = false;
                        _re_fast_add = false;

                        if _re_pos{
                            println!("(√)开始  调整描边",);
                        }else{
                            println!("( )结束  调整描边");
                        }
                        return  true;
                    }

                    if fltk::app::event_key()==fltk::enums::Key::ShiftL ||fltk::app::event_key()==fltk::enums::Key::ShiftR {
                        _re_size=!_re_size;
                        _re_pos = false;
                        _re_radius_size = false;
                        _re_fast_add = false;
                        _re_stroke_width_c = false;

                        if _re_size{
                            println!("(√)开始  调整宽高 (w,h)",);
                        }else{
                            println!("( )结束  调整宽高 (w,h)");
                        }
                        return  true;
                    }

                    if app::event_key()==fltk::enums::Key::AltL || app::event_key()==fltk::enums::Key::AltR  {
                        _re_radius_size=!_re_radius_size;
                        _re_pos = false;
                        _re_size = false;
                        _re_fast_add = false;
                        _re_stroke_width_c = false;

                        if _re_radius_size{
                            println!("(√)开始  圆角大小",);
                        }else{
                            println!("( )结束  圆角大小");
                        }
                        return  true;
                    }

                    if app::event_key()==fltk::enums::Key::CapsLock {
                        _re_fast_add=!_re_fast_add;
                        _re_pos = false;
                        _re_size = false;
                        _re_radius_size = false;
                        _re_stroke_width_c = false;

                        if _re_fast_add{
                            println!("(√)开始  调整描边");
                        }else{
                            println!("( )结束  圆角大小");
                        }
                        return  true;
                    }

                    //
                    // if _re_fast_add {
                    //     if app::event_key()==fltk::enums::Key::Up {
                    //         if _re_fast>20{
                    //             _re_fast=20;
                    //         }
                    //         _re_fast+=1;
                    //         println!("设置加速 增加->{}",_re_fast);
                    //     }
                    //     else if app::event_key()==fltk::enums::Key::Down {
                    //         if _re_fast<0{
                    //             _re_fast=0;
                    //         }
                    //         _re_fast-=1;
                    //         println!("设置加速 减少->{}",_re_fast);
                    //     }
                    //     return false;
                    // }

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
                            h+=(1+_re_fast+fast_temp);
                        }
                        else if app::event_key()==fltk::enums::Key::Up {
                            h-=(1+_re_fast+fast_temp);
                        }else if app::event_key()==fltk::enums::Key::Left {
                            w-=(1+_re_fast+fast_temp);
                        }
                        else if app::event_key()==fltk::enums::Key::Right {
                            w+=(1+_re_fast+fast_temp);
                        }
                    }

                    else if _re_radius_size {
                        _tap_hold_shift= libWxIkunPlus::getBasicKeys().shift;
                        let fast_temp = (if _tap_hold_shift {2} else {0}) ;

                        if app::event_key()==fltk::enums::Key::Down {
                            radius+=1+fast_temp;
                        }
                        else if app::event_key()==fltk::enums::Key::Up {
                            radius-=1+fast_temp;
                        }
                    }
                    else if _re_fast_add {
                        _tap_hold_shift= libWxIkunPlus::getBasicKeys().shift;
                        let fast_temp = (if _tap_hold_shift {2} else {0}) ;

                        if app::event_key()==fltk::enums::Key::Down {
                            stroke_width-=1+fast_temp;
                        }
                        else if app::event_key()==fltk::enums::Key::Up {
                            stroke_width+=1+fast_temp;
                        }
                    }


                    if radius > 90{
                        radius = 90
                    }
                    if radius<1 {
                        radius=0;
                    }

                    if stroke_width<1 {
                        stroke_width=0;
                    }

                    if _re_pos||_re_size||_re_radius_size||_re_fast_add{
                        win.hide();
                        // win.set_pos(win.x()+x,win.y()+y);
                        win.resize(win.x()+x,win.y()+y,win.w()+w,win.h()+h);
                        // win.set_text_size(win.text_size()+label_size);

                        let width =_self.preview.w();
                        let height =_self.preview.h();

                        _self.re_svg_view(width,height,radius as usize ,stroke_width);

                        // win.redraw();
                        win.show();

                        println!(
                            "gui_util::border::BorderPreview::new({},{},{},{},{radius},{color},{stroke},{stroke_size});"
                            ,win.x(),win.y(),win.w(),win.h(),radius=radius,color=format!("{:?}",_self.color.to_rgba()),stroke=format!("{:?}",_self.stroke.to_rgba()),stroke_size=stroke_width
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
