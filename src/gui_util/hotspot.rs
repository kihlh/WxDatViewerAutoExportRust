#![allow(warnings, unused)]

use fltk::enums::FrameType;
use fltk::prelude::{WidgetBase, WidgetExt};

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

}

pub fn create_hotspot(x: i32, y: i32, width: i32, height: i32) -> HotspotItmeControl {
    HotspotItmeControl::new(x, y, width, height)
}
