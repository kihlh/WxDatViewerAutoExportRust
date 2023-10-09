#![allow(warnings, unused)]

use fltk::enums::{Color, Cursor, Event, FrameType, Key};
use fltk::{prelude::*, *};
use fltk::app::event_key;
use fltk_theme::{color_themes, ColorTheme, SchemeType, ThemeType, WidgetScheme, WidgetTheme};


pub struct varTagControl {
    pub title: frame::Frame,
    pub name: frame::Frame,
    x:i32, y:i32, width: i32, height: i32,
    pub id:String,
    pub data:String
}

impl varTagControl {
    // 创建
    pub fn new(x:i32, y:i32, width: i32, height: i32,title:&str , name: &str, data:&str) -> Self {
        // 按照总宽度分配可用控件
        // let all_input_len = format!("{}{}",&title,&name).len() as i32;
        // let width_per_char = width / all_input_len;
        // let title_len = title.len() as i32;


        // let name_width = width_per_char * (all_input_len - title_len)+5;
        // let title_width = width - name_width;

        // 计算字符宽度 如果包含了宽字符 前面要加几像素
        let title_len =  title.chars().count() as i32;
        let mut title_width = title_len*11 ;
        let has_title_wide = title_len!=title.len() as i32;
        let mut title_x =(if has_title_wide {5} else {0} )+ x.clone();

        // 添加文本显示控件
        let mut title_frame = frame::Frame::new(title_x, y,title_width ,height , "");
        title_frame.set_label(title);
        title_frame.set_label_size(12);
        title_frame.set_label_color(Color::from_rgb(77, 77, 77));


        let name_len =  title.chars().count() as i32;
        let mut name_width = title_len*12 ;
        let has_name_wide = title_len!=title.len() as i32;
        let mut name_x =(if has_title_wide {5} else {-20} )+ x.clone()+name_len*11;
        let name_text_size = name.len();
        if  name_text_size==1 {
            name_x += 6;
        }

        if  name.contains("月") {
            name_x -= 10;
        }

        if name_text_size>9 {
            name_x += 15;
        }

        if name_text_size>11 {
            name_x += 25;
        }

        if name_text_size>28 {
            name_x += 25;
        }

        let mut name_frame = frame::Frame::new(name_x, y, name_width ,height, "");
        name_frame.set_label(name);
        name_frame.set_label_size(12);
        name_frame.set_label_color(Color::from_rgb(40, 40, 40));

        Self { title:title_frame , name:name_frame, x, y, width, height , data:data.to_string(),id:title.to_string().replace(" ","").replace(":","")}
    }


    // 判断鼠标是否在当前元素
    pub fn existPoint(&self, x: i32, y: i32) -> bool {
        return x > self.x
            && x < self.x + self.width
            && y > self.y
            && y < self.y + self.height;
    }
    pub(crate) fn get_var(&self) -> String{
        return format!("<{}>",self.id.clone().replace("%",""));
    }
}
impl Clone for varTagControl {
    fn clone(&self) -> Self {
        varTagControl {
            title:self.title.clone(),
            name:self.name.clone(),
            x: self.x.clone(),
            y: self.y.clone(),
            width: self.width.clone(),
            height: self.height.clone(),
            id:self.id.clone(),
            data:self.data.clone()
        }
    }
}

