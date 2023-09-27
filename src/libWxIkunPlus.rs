use std::env;
use std::ffi::CString;
use std::ffi::{c_int, c_long, c_void, OsStr};
use std::os::raw::c_char;

// #![crate_type = "staticlib"]
#[link(name = "libWxIkunPlus", kind = "static")]
extern "C" {
    fn _setWinIcon(_hWnd: c_long) -> c_void;
    fn _setShowWindows(_hWnd: c_long, visible: bool) -> bool;
    fn _set_tray() -> c_void;
    fn _createMutex(mutex: *const c_char) -> bool;
    fn _removeMutex(mutex: *const c_char) -> bool;
    fn _hasMutex(mutex: *const c_char) -> bool;
    fn _setStartup() -> bool;
    fn _openSelectFolder() -> c_void;
    fn _setWindowsTop(_hWnd: c_long, visible: bool) -> bool;
    fn _setCloseWindow(_hWnd: c_long, closeRoot: bool) -> bool;
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

// 文件夹选取器
pub fn openSelectFolder() -> String {
    unsafe {
        _openSelectFolder();
        let mut open_path = env::var("IKUN@SelectedFolderPath").unwrap_or_else(|_| "".to_owned());
        return open_path;
    };
}
