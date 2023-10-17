pub mod dark;
pub mod light;
#[cfg(all(target_os = "macos", feature = "cocoa-colors"))]
pub mod sys;

// use fltk::enums::Color;

// const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
//     let val = ((r as u32 & 0xff) << 24) + ((g as u32 & 0xff) << 16) + ((b as u32 & 0xff) << 8) + (a as u32 & 0xff);
//     Color::from_rgbi(val)
// }
