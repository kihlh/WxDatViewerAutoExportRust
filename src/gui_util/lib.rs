use fltk::enums::Color;

pub fn hex_to_rgba(hex: &str) -> Option<(u8, u8, u8, f32)> {
    let hex = hex.trim_start_matches('#');

    match hex.len() {
        3 => {
            let r = u8::from_str_radix(&hex[0..1], 16).ok()?;
            let g = u8::from_str_radix(&hex[1..2], 16).ok()?;
            let b = u8::from_str_radix(&hex[2..3], 16).ok()?;
            Some((r * 17, g * 17, b * 17, 1.0))
        }
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some((r, g, b, 1.0))
        }
        _ => None,
    }
}

pub fn rgba_to_hex(rgba: (u8, u8, u8, f32)) -> String {
    let (r, g, b, _) = rgba;
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

pub fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    let hex = hex.trim_start_matches('#');

    match hex.len() {
        3 => {
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
            Some((r, g, b))
        }
        5 => {
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
            Some((r, g, b))
        }
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some((r, g, b))
        }
        _ => None,
    }
}

pub fn rgb_to_hex(rgb: (u8, u8, u8)) -> String {
    let (r, g, b) = rgb;
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

pub fn parse_rgb(rgb: &str) -> Option<(u8, u8, u8)> {
    if rgb.starts_with("rgb(") && rgb.ends_with(")") {
        let color = rgb
            .trim_start_matches("rgb(")
            .trim_end_matches(")")
            .split(',')
            .map(|component| component.trim().parse::<u8>())
            .collect::<Result<Vec<_>, _>>()
            .ok()?;

        if color.len() == 3 {
            return Some((color[0], color[1], color[2]));
        }
    }

    None
}

pub fn parse_rgba(rgba: &str) -> Option<(u8, u8, u8, f32)> {
    if rgba.starts_with("rgba(") && rgba.ends_with(")") {
        let color = rgba
            .trim_start_matches("rgba(")
            .trim_end_matches(")")
            .split(',')
            .map(|component| component.trim().parse::<f32>())
            .collect::<Result<Vec<_>, _>>()
            .ok()?;

        if color.len() == 4 {
            let r = (color[0].clamp(0.0, 1.0) * 255.0) as u8;
            let g = (color[1].clamp(0.0, 1.0) * 255.0) as u8;
            let b = (color[2].clamp(0.0, 1.0) * 255.0) as u8;
            let alpha = color[3].clamp(0.0, 1.0);

            return Some((r, g, b, alpha));
        }
    }

    None
}

pub trait LoadedAnyColor {
    // 转换为文本
    fn to_hex(&self) -> String;
    fn to_rgb(&self) -> (u8, u8, u8);
    fn to_rgba(&self) -> (u8, u8, u8 , f32);
    fn to_color(&self)->fltk::enums::Color;

}

impl LoadedAnyColor for &str {
    fn to_hex(&self) -> String {

        // rgba
        if self.contains("rgba") {
            if let Some((r,g,b,a)) = parse_rgba(self) {
                return rgba_to_hex((r,g,b,a));
            }
        }

        else if self.contains("rgba") {
            if let Some((r,g,b)) = parse_rgb(self) {
                return rgb_to_hex((r,g,b));
            }
        }

        else if self.contains("#"){
            return format!("{}",self);
        }

        String::from("#FFFFFF")
    }

    fn to_rgb(&self) -> (u8, u8, u8) {

        if self.contains("rgba") {
            if let Some((r,g,b,a)) = parse_rgba(self) {
                return (r,g,b);
            }
        }

        else if self.contains("rgba") {
            if let Some((r,g,b)) = parse_rgb(self) {
                return (r,g,b);
            }
        }

        else if self.contains("#"){
            if let Some((r,g,b)) = hex_to_rgb(self) {
                return (r,g,b);
            }
        }

        (255u8,255u8,255u8)
    }

    fn to_rgba(&self) -> (u8, u8, u8, f32) {
        if self.contains("rgba") {
            if let Some((r,g,b,a)) = parse_rgba(self) {
                return (r,g,b,a);
            }
        }

        else if self.contains("rgba") {
            if let Some((r,g,b)) = parse_rgb(self) {
                return (r,g,b,1.0);
            }
        }

        else if self.contains("#"){
            if let Some((r,g,b)) = hex_to_rgb(self) {
                return (r,g,b,1.0);
            }
        }
        (255u8,255u8,255u8,1.0)
    }

    fn to_color(&self) -> Color {
        if self.contains("rgba") {
            if let Some((r,g,b,a)) = parse_rgba(self) {
                return fltk::enums::Color::from_rgb(r,g,b);
            }
        }

        else if self.contains("rgba") {
            if let Some((r,g,b)) = parse_rgb(self) {
                return fltk::enums::Color::from_rgb(r,g,b);
            }
        }

        else if self.contains("#"){
            if let Some((r,g,b)) = hex_to_rgb(self) {
                return fltk::enums::Color::from_rgb(r,g,b);
            }
        }

        fltk::enums::Color::from_rgb(255,255,255)
    }
}

impl LoadedAnyColor for [i32;3] {
    fn to_hex(&self) -> String {
        rgb_to_hex(((self[0]) as u8 , self[1]  as u8 , self[2]  as u8))
    }

    fn to_rgb(&self) -> (u8, u8, u8) {
        (self[0]  as u8 , self[1]  as u8 , self[2]  as u8)
    }

    fn to_rgba(&self) -> (u8, u8, u8, f32) {
        (self[0]  as u8 , self[1]  as u8 , self[2]  as u8,1.0)
    }

    fn to_color(&self) -> Color {
        fltk::enums::Color::from_rgb(self[0]  as u8 , self[1]  as u8 , self[2]  as u8)
    }

}

impl LoadedAnyColor for (u8,u8,u8,f32) {
    fn to_hex(&self) -> String {
        rgb_to_hex(((self.0)  as u8 , self.1  as u8 , self.2  as u8))
    }

    fn to_rgb(&self) -> (u8, u8, u8) {
        ((self.0)  as u8 , self.1  as u8 , self.2  as u8)
    }

    fn to_rgba(&self) -> (u8, u8, u8, f32) {
        ((self.0)  as u8 , self.1  as u8 , self.2  as u8, self.3  as f32)
    }

    fn to_color(&self) -> Color {
        fltk::enums::Color::from_rgb((self.0)  as u8 , self.1  as u8 , self.2  as u8)
    }
}

impl LoadedAnyColor for (u8,u8,u8) {
    fn to_hex(&self) -> String {
        rgb_to_hex(((self.0)  as u8 , self.1  as u8 , self.2  as u8))
    }

    fn to_rgb(&self) -> (u8, u8, u8) {
        ((self.0)  as u8 , self.1  as u8 , self.2  as u8)
    }

    fn to_rgba(&self) -> (u8, u8, u8, f32) {
        ((self.0)  as u8 , self.1  as u8 , self.2  as u8, 1.0)
    }

    fn to_color(&self) -> Color {
        fltk::enums::Color::from_rgb((self.0)  as u8 , self.1  as u8 , self.2  as u8)
    }
}

impl LoadedAnyColor for u32 {
    fn to_hex(&self) -> String {
        fltk::enums::Color::from_u32(self.clone()).to_hex_str()
    }

    fn to_rgb(&self) -> (u8, u8, u8) {
        fltk::enums::Color::from_u32(self.clone()).to_rgb()
    }

    fn to_rgba(&self) -> (u8, u8, u8, f32) {
        if self.clone()== 0u32 {
            return  (0,0,0,0.0);
        }

        (255,255,255,1.0)
    }

    fn to_color(&self) -> Color {
        fltk::enums::Color::from_u32(self.clone())
    }
}



pub trait OverloadedAnyStr {
    // 转换为文本
    fn to_string(&self) -> Option<String>;
    // 如果转换失败返回空文本
    fn to_string_default(&self) -> String;
    // 如果转化失败则返回此文本
    fn to_string_or(&self,or_str:String) ->String;

}

macro_rules! over_loaded_any_option_convert_fun {
    ($funRes:expr,$or:expr)=>{
        if let Some(data) = $funRes {
            data
        }else {
            $or
        }
    }
}

impl OverloadedAnyStr for std::path::Path {
    fn to_string(&self) -> Option<String> {
        Some(self.to_string_lossy().to_string())
    }
    fn to_string_default(&self) -> String {
        self.to_string_lossy().to_string()
    }
    fn to_string_or(&self,or_str:String) -> String {
        self.to_string_lossy().to_string()
    }

}

impl OverloadedAnyStr for &std::path::Path {
    fn to_string(&self) -> Option<String> {
        Some(self.to_string_lossy().to_string())
    }
    fn to_string_default(&self) -> String {
        self.to_string_lossy().to_string()
    }
    fn to_string_or(&self,or_str:String) -> String {
        self.to_string_lossy().to_string()
    }

}

impl OverloadedAnyStr for &std::path::PathBuf {
    fn to_string(&self) -> Option<String> {
        Some(self.to_string_lossy().to_string())
    }
    fn to_string_default(&self) -> String {
        self.to_string_lossy().to_string()
    }
    fn to_string_or(&self,or_str:String) -> String {
        self.to_string_lossy().to_string()
    }

}

impl OverloadedAnyStr for std::path::PathBuf {
    fn to_string(&self) -> Option<String> {
        Some(self.to_string_lossy().to_string())
        // None
    }
    fn to_string_default(&self) -> String {
        self.to_string_lossy().to_string()
    }
    fn to_string_or(&self,or_str:String) -> String {
        self.to_string_lossy().to_string()
    }

}

impl OverloadedAnyStr for &str {
    fn to_string(&self) -> Option<String> {
        Some(format!("{}",self))
    }
    fn to_string_default(&self) -> String {
        format!("{}",self)
    }
    fn to_string_or(&self,or_str:String) -> String {
        format!("{}",self)
    }

}

impl OverloadedAnyStr for String {
    fn to_string(&self) -> Option<String> {
        Some(format!("{}",self))
    }
    fn to_string_default(&self) -> String {
        format!("{}",self)
    }
    fn to_string_or(&self,or_str:String) -> String {
        format!("{}",self)
    }
}

impl OverloadedAnyStr for &String {
    fn to_string(&self) -> Option<String> {
        Some(format!("{}",self))
    }
    fn to_string_default(&self) -> String {
        format!("{}",self)
    }
    fn to_string_or(&self,or_str:String) -> String {
        format!("{}",self)
    }

}

impl OverloadedAnyStr for &std::ffi::OsStr {
    fn to_string(&self) -> Option<String> {
        if let Some(str) = self.to_str() {
            return  Some(format!("{}",str));
        }
        // Some(format!("{}",self))
        None
    }
    fn to_string_default(&self) -> String {
        if let Some(str) = self.to_str() {
            return format!("{}",str);
        }
        String::new()
    }
    fn to_string_or(&self,or_str:String) -> String {
        if let Some(str) = self.to_str() {
            return format!("{}",str);
        }
        or_str
    }
}

impl OverloadedAnyStr for &&std::ffi::OsStr {
    fn to_string(&self) -> Option<String> {
        if let Some(str) = self.to_str() {
            return  Some(format!("{}",str));
        }
        // Some(format!("{}",self))
        None
    }
    fn to_string_default(&self) -> String {
        if let Some(str) = self.to_str() {
            return format!("{}",str);
        }
        String::new()
    }
    fn to_string_or(&self,or_str:String) -> String {
        if let Some(str) = self.to_str() {
            return format!("{}",str);
        }
        or_str
    }
}

impl OverloadedAnyStr for Option<String> {
    fn to_string(&self) -> Option<String> {
        self.clone()
    }
    fn to_string_default(&self) -> String {
        if let Some(str) = self {
            return format!("{}",str);
        }
        String::new()
    }
    fn to_string_or(&self,or_str:String) -> String {
        if let Some(str) = self {
            return format!("{}",str);
        }
        or_str
    }
}

impl OverloadedAnyStr for Option<&str> {
    fn to_string(&self) -> Option<String> {
        if let Some(str) = self {
            return Some(format!("{}",str));
        }
        None
    }
    fn to_string_default(&self) -> String {
        if let Some(str) = self {
            return format!("{}",str);
        }
        String::new()
    }
    fn to_string_or(&self,or_str:String) -> String {
        if let Some(str) = self {
            return format!("{}",str);
        }
        or_str
    }
}

impl OverloadedAnyStr for Option<&std::ffi::OsStr> {
    fn to_string(&self) -> Option<String> {
        if let Some(str) = self {
            return  Some( str.to_string_lossy().to_string())
        }
        None
    }
    fn to_string_default(&self) -> String {
        if let Some(str) = self {
            return format!("{}",str.to_string_lossy().to_string());
        }
        String::new()
    }
    fn to_string_or(&self,or_str:String) -> String {
        if let Some(str) = self {
            return format!("{}",str.to_string_lossy().to_string());
        }
        or_str
    }
}


impl OverloadedAnyStr for std::fs::DirEntry {
    fn to_string(&self) -> Option<String> {
        self.to_string()
    }
    fn to_string_default(&self) -> String {
        if let Some(str) =  self.to_string() {
            return str;
        }
        String::new()
    }
    fn to_string_or(&self,or_str:String) -> String {
        if let Some(str) =  self.to_string() {
            return str;
        }
        or_str
    }
}

impl OverloadedAnyStr for &std::fs::DirEntry {
    fn to_string(&self) -> Option<String> {
        self.to_string()
    }
    fn to_string_default(&self) -> String {
        if let Some(str) =  self.to_string() {
            return str;
        }
        String::new()
    }
    fn to_string_or(&self,or_str:String) -> String {
        if let Some(str) =  self.to_string() {
            return str;
        }
        or_str
    }
}

impl OverloadedAnyStr for Option<std::fs::DirEntry> {
    fn to_string(&self) -> Option<String> {
        if let Some(item) = self {
            return  item.to_string()
        }
        None
    }
    fn to_string_default(&self) -> String {
        if let Some(item) = self {

            if let Some(item) = item.to_string() {
                return item ;
            }
        }
        String::new()
    }
    fn to_string_or(&self,or_str:String) -> String {
        if let Some(item) = self {

            if let Some(item) = item.to_string() {
                return item ;
            }
        }
        or_str
    }
}

use std::os::raw::c_char;
use std::ffi::{CStr, CString};
use std::fmt::format;

impl OverloadedAnyStr for *const c_char {
    fn to_string(&self) -> Option<String> {
        let c_str = unsafe { CStr::from_ptr(self.clone()) };
        let bytes = c_str.to_bytes();
        Some(String::from_utf8_lossy(bytes).into_owned())
    }
    fn to_string_default(&self) -> String {
        let c_str = unsafe { CStr::from_ptr(self.clone()) };
        let bytes = c_str.to_bytes();
        String::from_utf8_lossy(bytes).into_owned()
    }
    fn to_string_or(&self,or_str:String) -> String {
        let c_str = unsafe { CStr::from_ptr(self.clone()) };
        let bytes = c_str.to_bytes();
        String::from_utf8_lossy(bytes).into_owned()

    }
}