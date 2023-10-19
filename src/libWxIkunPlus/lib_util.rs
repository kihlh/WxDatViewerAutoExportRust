use std::ffi::{c_int, c_uint, c_ulong, OsStr};
use std::iter::once;
use std::mem::transmute;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;
type DWORD = c_ulong;

pub fn encode_lpcstr(s: &str) -> Vec<i8> {
    let mut arr: Vec<i8> = s.bytes().map(|x| x as i8).collect();
    arr.push(0);
    arr
}

pub fn encode_wide_with_null(s: impl AsRef<str>) -> Vec<u16> {
    let wide: Vec<u16> = OsStr::new(s.as_ref())
        .encode_wide()
        .chain(once(0))
        .collect();
    wide
}

extern "system" {
    fn WideCharToMultiByte(
        page: c_uint,
        flags: c_ulong,
        wide_str: *const u16,
        wide_str_len: c_int,
        multi_str: *mut i8,
        multi_str_len: c_int,
        default_char: *const i8,
        used_default_char: *mut i32,
    ) -> c_int;
    fn MultiByteToWideChar(
        CodePage: c_uint,
        dwFlags: DWORD,
        lpMultiByteStr: *const u8,
        cbMultiByte: c_int,
        lpWideCharStr: *mut u16,
        cchWideChar: c_int,
    ) -> c_int;
}

/// Convert a rust string to a winapi-usable 0-terminated unicode u16 Vec
pub fn winapi_str<T: AsRef<OsStr>>(input: T) -> Vec<u16> {
    let mut buf = Vec::with_capacity(input.as_ref().len());
    buf.extend(input.as_ref().encode_wide());
    buf.push(0);
    buf
}

const CP_ACP: c_uint = 0;
const CP_OEMCP: c_uint = 1; // default to OEM  code page
const CP_MACCP: c_uint = 2; // default to MAC  code page
const CP_THREAD_ACP: c_uint = 3; // current thread's ANSI code page
const CP_SYMBOL: c_uint = 42; // SYMBOL translations

const CP_UTF7: c_uint = 65000; // UTF-7 translation
const CP_UTF8: c_uint = 65001;

// If the conversion was lossy, returns Err(lossy_result)
pub fn ansi_codepage_cstring<T: AsRef<OsStr>>(input: T) ->Result<Vec<i8>,Vec<i8>> {

    unsafe {
        let os_str = input.as_ref();
        let unicode = winapi_str(os_str);
        let length = WideCharToMultiByte(
            CP_ACP,
            0,
            unicode.as_ptr(),
            unicode.len() as i32,
            null_mut(),
            0,
            null_mut(),
            null_mut(),
        );
        let mut buffer = vec![0i8; length as usize];
        let mut used_default_char = 0;
        WideCharToMultiByte(
            CP_ACP,
            0,
            unicode.as_ptr(),
            unicode.len() as i32,
            buffer.as_mut_ptr() as *mut i8,
            length,
            null_mut(),
            &mut used_default_char,
        );
        
        if used_default_char != 0 {
            Err(buffer)
        } else {
            Ok(buffer)
        }
    }
    
}


pub fn utf16_to_utf8(utf16_string: &[u16]) -> String {
    let utf8_vec: Vec<u8> = utf16_string
        .iter()
        .flat_map(|&c| std::char::from_u32(c as u32))
        .flat_map(|c| c.to_string().as_bytes().to_vec())
        .collect();
    
    String::from_utf8(utf8_vec).unwrap_or_else(|_| String::new())
}