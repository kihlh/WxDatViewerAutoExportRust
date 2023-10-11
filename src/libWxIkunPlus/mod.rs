#![allow(warnings, unused)]

use std::{env, thread};
use std::ffi::{c_int, c_long, c_void, OsStr,c_uint,};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
pub type PCSTR =*const c_char;

type wchar_t = u16;
type WCHAR = wchar_t;

type LPCWCHAR = *const WCHAR;

use self::util::{encode_lpcstr, ansi_codepage_cstring};
pub mod util;

// #![crate_type = "staticlib"]
//  请注意 所有传入的文本都必须是utf8
#[link(name = "libWxIkunPlus", kind = "static")]
extern "C" {
    fn _setWinIcon(_hWnd: c_long) -> c_void;
    fn _setShowWindows(_hWnd: c_long, visible: bool) -> bool;
    fn _set_tray() -> c_void;
    fn _createMutex(mutex:PCSTR) -> bool;
    fn _removeMutex(mutex:PCSTR) -> bool;
    fn _hasMutex(mutex:PCSTR) -> bool;
    fn _setStartup() -> bool;
    fn _hasStartup() -> bool;
    fn _openSelectFolder() -> c_void;
    fn _setWindowsTop(_hWnd: c_long, visible: bool) -> bool;
    fn _setCloseWindow(_hWnd: c_long, closeRoot: bool) -> bool;
    fn _openSelectFolder2() ->PCSTR;
    fn _Error(title:PCSTR, info:PCSTR) -> c_void;
    fn _Stop(mutex:PCSTR, info:PCSTR) -> c_void;
    fn _Confirm(title:PCSTR, info:PCSTR) -> bool;
    fn _Alert(mutex:PCSTR, info:PCSTR) -> bool;
    fn _getRegistrValue(hKey: c_long, _subKey:PCSTR, _key:PCSTR)->PCSTR;
    fn _hasWeChat() -> bool;
    fn _setTaskbarWin(_hWnd: c_long) -> c_void;
    fn _setMinWindows(_hWnd: c_long) -> bool;
    fn _findWindow(className:PCSTR, title:PCSTR) -> c_long;
    // fn _findWindowW(className:LPCWCHAR, title:LPCWCHAR) -> c_long;
    // fn _findWindowU8(className:PCSTR, title:PCSTR) -> c_long;
    fn _has_auto_sync() -> bool;
    fn _set_auto_sync(value:bool);
    fn _has_sync_token()-> bool;
    fn _hasStartupGlobalVar()-> bool;
    fn _getFocusTopWindow()->c_long;
    fn _getFocusWindow()->c_long;

}

// 设置窗口图标 从当前二进制获取
pub fn setWinIcon(hWnd: i128) {
    unsafe {
        match hWnd.try_into() {
            Ok(hwnds) => {
                _setWinIcon(hwnds);
            }
            Err(_) => {}
        }
    };
}

// 关闭窗口
pub fn closeWindow(hWnd: i128, destroy: bool) {
    unsafe {
        match hWnd.try_into() {
            Ok(hwnds) => {
                _setCloseWindow(hwnds, destroy);
            }
            Err(_) => {}
        }
    };
}

// 设置窗口可见 如果可见会激活窗口
pub fn setwinVisible(hWnd: i128, visible: bool) -> bool {
    let mut res = false;

    unsafe {
        match hWnd.try_into() {
            Ok(hwnds) => {
                _setShowWindows(hwnds, visible);
            }
            Err(_) => {
                res = false;
            }
        }
    };
    return res;
}

// 设置窗口顶置
pub fn setWinTop(hWnd: i128, visible: bool) -> bool {
    let mut res = false;

    unsafe {
        match hWnd.try_into() {
            Ok(hwnds) => {
                _setWindowsTop(hwnds, visible);
            }
            Err(_) => {
                res = false;
            }
        }
    };
    return res;
}

// 启用托盘
pub fn set_tray() {
    unsafe {
        _set_tray();
    };
}

// 创建互斥体
pub fn createMutex(mutex: String) -> bool {
    let mut result = false;
    unsafe {
        match CString::new(mutex) {
            Ok(mutexs) => {
                result = _createMutex(mutexs.as_ptr());
            }
            Err(_) => {}
        }
    };

    result
}

// 删除互斥体
pub fn removeMutex(mutex: String) -> bool {
    let mut result = false;
    unsafe {
        match CString::new(mutex) {
            Ok(mutexs) => {
                result = _removeMutex(mutexs.as_ptr());
            }
            Err(_) => {}
        }
    };

    result
}

// 判断是否有互斥体
pub fn hasMutex(mutex: String) -> bool {
    let mut result = false;
    unsafe {
        match CString::new(mutex) {
            Ok(mutexs) => {
                result = _hasMutex(mutexs.as_ptr());
            }
            Err(_) => {}
        }
    };

    result
}

// 设置自启
pub fn setStartup() -> bool {
    unsafe {
        return _setStartup();
    };
}

pub fn hasStartup() -> bool {
    unsafe {
        return _hasStartup();
    };
}

// 文件夹选取器
pub fn openSelectFolder() -> String {
    unsafe {
        _openSelectFolder();
        let mut open_path = env::var("IKUN@SelectedFolderPath").unwrap_or_else(|_| "".to_owned());
        return open_path;
    };
}

// 将C字符串转换为Rust字符串
fn c_string_to_rust_string(ptr:PCSTR) -> String {
    unsafe {
        let c_str = CStr::from_ptr(ptr);
        let bytes = c_str.to_bytes();
        String::from_utf8_lossy(bytes).into_owned()
    }
}

// 启用托盘
pub fn openSelectFolder2() -> String {
    let mut result = String::new();
    
    let the_win = getFocusWindow();

    setwinVisible(the_win.clone(), false);
    
    unsafe { result = c_string_to_rust_string(_openSelectFolder2()) };
    
    setwinVisible(the_win.clone(), true);
    
    return result;
}

// 将Rust字符串转换为C字符串
fn rust_string_to_c_string(s: String) -> CString {
    if let Result::Ok(mut buff) = CString::new(s.as_str()) {
        return buff;
    };
    let c_ptr = CString::new("").unwrap();
    return c_ptr;
}

fn rust_string_to_ansi_str(s: String)->Vec<i8>{
    if let Result::Ok(item) = ansi_codepage_cstring(s) {
        return item;
    }
    let c_ptr = CString::new("").unwrap();
    let as_bytes = c_ptr.as_bytes().to_vec();
    let mut result = Vec::new();
    for value in as_bytes {
        result.push(value as i8);
    }

    return result;
}

fn option_vec_u8_to_cstring(option_vec: Option<Vec<u8>>) -> Result<CString, &'static str> {
    match option_vec {
        Some(vec) => {
            match CString::new(vec) {
                Ok(cstring) => Ok(cstring),
                Err(_) => Err("Failed to create CString"),
            }
        }
        None => Err("Option<Vec<u8>> is None"),
    }
}


// // 将Rust UTF-8字符串转换为Windows API中的A字符
// fn utf8_to_ansi(s: &str) -> Vec<c_char> {
//     let wide: Vec<u16> = OsStr::new(s).encode_wide().collect();
//     let wide_len = wide.len() + 1;

//     let mut ansi: Vec<c_char> = Vec::with_capacity(wide_len);
//     let ansi_len = wide.len();

//     unsafe {
//         WideCharToMultiByte(
//             CP_UTF8,
//             0,
//             wide.as_ptr(),
//             wide_len as i32,
//             ansi.as_mut_ptr(),
//             ansi_len as i32,
//             ptr::null(),
//             ptr::null_mut(),
//         );
//         // 确保在末尾添加一个空字符
//         ansi.push(0);
//         ansi.set_len(ansi_len);
//     }

//     ansi
// }

// MessageBox -> alert
pub fn alert(title: String, message: String) -> bool {
    unsafe {
        return _Alert(
            rust_string_to_ansi_str(title).as_ptr(),
            rust_string_to_ansi_str(message).as_ptr(),
        );
    }
    return false;
}

// MessageBox -> confirm
pub fn confirm(title: String, message: String) -> bool {
    unsafe {
        return _Confirm(
            rust_string_to_ansi_str(title).as_ptr(),
            rust_string_to_ansi_str(message).as_ptr(),
        );
    }
    return false;
}

// MessageBox -> stop
pub fn stop(title: String, message: String) {
    unsafe {
        _Stop(
            rust_string_to_ansi_str(title).as_ptr(),
            rust_string_to_ansi_str(message).as_ptr(),
        );
    }
}

// MessageBox -> error
pub fn error(title: String, message: String) {
    unsafe {
        _Error(
            rust_string_to_ansi_str(title).as_ptr(),
            rust_string_to_ansi_str(message).as_ptr(),
        );
    }
}

pub enum HKEY {
    HKEY_CLASSES_ROOT = 0x80000000,
    HKEY_CURRENT_USER = 0x80000001,
    HKEY_LOCAL_MACHINE = 0x80000002,
    HKEY_USERS = 0x80000003,
}


pub fn getRegistrValue(hKey: HKEY, subKey: String, valueKey: String) -> String {
    let mut result = String::new();
    unsafe {
        let mut c_result = _getRegistrValue(
            c_long::from(hKey as i32),
            rust_string_to_ansi_str(subKey).as_ptr(),
            rust_string_to_ansi_str(valueKey).as_ptr(),
        );
        result =c_string_to_rust_string(c_result);
    }
    result
}

// 判断wx进程是否存在
pub fn hasWeChat()->bool {
    let mut result = false;
    unsafe {
        result= _hasWeChat();
    }
    result
}

pub fn hasWeChatWin()->bool {
    let mut result = false;
    unsafe {
        let hwnd_01 = findWindow("WeChatMainWndForPC", "");
        if(hwnd_01!=0){
           return true;
        }

        let hwnd_02 = findWindow("ChatWnd", "");
        if(hwnd_02!=0){
            return true;
        }

        let hwnd_03 = findWindow("SubscriptionWnd", "");
        if(hwnd_03!=0){
            return true;
        }
    }
    result
}

// 把一个傀儡窗口变成主窗口的托盘
pub fn setTaskbarWin(hWnd: i128) {
    unsafe {
       _setTaskbarWin(hWnd as i32);
    }
}

pub fn setMinWindows(hWnd: i128) -> bool {
    unsafe {
        _setMinWindows(hWnd as i32)
    }
}


// 搜索窗口
pub fn findWindow(className: &str, titleName: &str)->i128 {
    let mut hwnd:i128 = 0;
    unsafe {
        let mut className = rust_string_to_ansi_str(className.to_string());
        let mut titleName = rust_string_to_ansi_str(titleName.to_string());
      
        hwnd= _findWindow(className.as_ptr(), titleName.as_ptr()/*,className_len as i32,titleName_len as i32 */).into();
    }
    return hwnd;
}

// // 搜索窗口
// pub fn findWindowU8(className: String, titleName: String)->i128 {
//     let mut hwnd:i128 = 0;
//     unsafe {
//         // let mut className = rust_string_to_c_string(className);
//         // let mut titleName = rust_string_to_c_string(titleName);
//         hwnd= _findWindowU8(encode_lpcstr(className.as_str()).as_ptr(), encode_lpcstr(titleName.as_str()).as_ptr() /*,className_len as i32,titleName_len as i32 */).into();
//     }
//     return hwnd;
// }

// 判断是否启用了自动更新（如果wx进程不存在也会返回false）
pub fn has_auto_sync() -> bool{
    let mut result = false;
    unsafe {
        result =_has_auto_sync();
    }

    result
}

// 判断是否启用了自动更新（如果wx进程不存在也会返回false）
pub fn has_auto_sync_all() -> bool{
    let mut result = false;
    unsafe {
        result = hasWeChat()&&hasWeChatWin()&&_has_auto_sync();
    }
    println!("has_auto_sync-> {}",&result);
    result
}

// 设置自动更新
pub fn set_auto_sync(value: bool){
    unsafe {
       _set_auto_sync(value);
    }
}

// 是否立即同步
pub fn has_sync_token()->bool{
    unsafe{
        _has_sync_token()
    }
}

// 是否立即同步
pub fn hasStartupGlobalVar()->bool{
    unsafe{
        _hasStartupGlobalVar()
    }
}

pub fn getFocusWindow()->i128{
    unsafe{
        _getFocusWindow() as i128
    }
}

pub fn getFocusTopWindow()->i128{
    unsafe{
        _getFocusTopWindow() as i128
    }
}