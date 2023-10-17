use fltk::enums::Color;
use fltk::frame;
use fltk::prelude::{WidgetBase, WidgetExt};

pub struct TextControl {
    pub text: frame::Frame,
}

impl TextControl {
    // 创建
    pub fn new(x:i32, y:i32, width: i32, height: i32, size:i32, input:String, rgb: [u8; 3]) -> Self {
        let mut text = frame::Frame::new(x, y, (if width==0 {input.len()as i32 * size}else {width}), (if height==0 {size+2}else {height}), "");
        text.set_label(input.as_str());
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
