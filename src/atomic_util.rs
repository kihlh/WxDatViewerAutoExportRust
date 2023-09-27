use std::hint;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicUsize, Ordering,AtomicI64};
use std::sync::{Arc, Condvar, Mutex};
static mut DATA_FDD: String = String::new();

#[macro_export]
macro_rules! get_bool {
    ($atomic_bool_static:expr) => {{
        let mut value_data: bool = false;
        let mutex = Arc::new(Mutex::new(&$atomic_bool_static));
        mutex.lock();
        value_data = $atomic_bool_static.load(Ordering::SeqCst);
        drop(mutex);
        value_data
    }};
}

#[macro_export]
macro_rules! set_bool {
    ($atomic_bool_static:expr,$value:expr) => {{
        let mut value_data: bool = false;
        let mutex = Arc::new(Mutex::new(&$atomic_bool_static));
        mutex.lock();
        $atomic_bool_static.store($value, Ordering::SeqCst);
        value_data = $atomic_bool_static.load(Ordering::SeqCst);
        drop(mutex);
        value_data
    }};
}

/// 获取 AtomicBool的内容
///
/// # 用法
///
///
/// ```
/// use std::sync::atomic::{AtomicBool};
///
/// static READY: AtomicBool = AtomicBool::new(false);
///
/// fn main() {
///   // 读取值
///   let mut ready = get_bool(&READY);
///   assert!(ready, false);
///
///   // 设置值
///   set_bool(&READY, true);
///   ready = get_bool(&READY);
///   assert!(ready, true);
///
///     }
///
/// ```
pub fn get_bool(atomic_bool_static: &AtomicBool) -> bool {
    get_bool!(atomic_bool_static)
}

/// 设置 AtomicBool的内容
///
/// # 用法
///
///
/// ```
/// use std::sync::atomic::{AtomicBool};
///
/// static READY: AtomicBool = AtomicBool::new(false);
///
/// fn main() {
///   // 读取值
///   let mut ready = get_bool(&READY);
///   assert!(ready, false);
///
///   // 设置值
///   set_bool(&READY, true);
///   ready = get_bool(&READY);
///   assert!(ready, true);
///
///     }
///
/// ```
pub fn set_bool(atomic_bool_static: &AtomicBool, value: bool) -> bool {
    set_bool!(atomic_bool_static, value)
}


/// 设置 AtomicI32的内容
///
/// # 用法
///
///
/// ```
/// use std::sync::atomic::{AtomicI32};
///
/// static READY: AtomicI32 = AtomicI32::new(0);
///
/// fn main() {
///   // 读取值
///   let mut ready = get_i32(&READY);
///   assert!(ready, 0);
///
///   // 设置值
///   set_i32(&READY, 1);
///   ready = get_i32(&READY);
///   assert!(ready, 1);
///   set_i32(&READY, 1);
///
///   // 加值
///   add_i32(&READY, 1);
///   assert!(ready, 2);
///
///   add_i32(&READY, 5);
///   assert!(ready, 7);
///     }
///
/// ```
pub fn set_i32(atomic_bool_static: &AtomicI32, value: i32) -> i32 {
    let mut value_data: i32 = value;
    let mutex = Arc::new(Mutex::new(&atomic_bool_static));
    mutex.lock();
    atomic_bool_static.store(value, Ordering::SeqCst);
    value_data = atomic_bool_static.load(Ordering::SeqCst);
    drop(mutex);
    value_data
}

/// 取 AtomicI32的内容
///
/// # 用法
///
///
/// ```
/// use std::sync::atomic::{AtomicI32};
///
/// static READY: AtomicI32 = AtomicI32::new(0);
///
/// fn main() {
///   // 读取值
///   let mut ready = get_i32(&READY);
///   assert!(ready, 0);
///
///   // 设置值
///   set_i32(&READY, 1);
///   ready = get_i32(&READY);
///   assert!(ready, 1);
///   set_i32(&READY, 1);
///
///   // 加值
///   add_i32(&READY, 1);
///   assert!(ready, 2);
///
///   add_i32(&READY, 5);
///   assert!(ready, 7);
///     }
///
/// ```
pub fn get_i32(atomic_bool_static: &AtomicI32) -> i32 {
    let mut value_data: i32 = 0;
    let mutex = Arc::new(Mutex::new(&atomic_bool_static));
    mutex.lock();
    // atomic_bool_static.store(value, Ordering::SeqCst);
    value_data = atomic_bool_static.load(Ordering::SeqCst);
    drop(mutex);
    value_data
}

/// 加加 AtomicI32的内容
///
/// # 用法
///
///
/// ```
/// use std::sync::atomic::{AtomicI32};
///
/// static READY: AtomicI32 = AtomicI32::new(0);
///
/// fn main() {
///   // 读取值
///   let mut ready = get_i32(&READY);
///   assert!(ready, 0);
///
///   // 设置值
///   set_i32(&READY, 1);
///   ready = get_i32(&READY);
///   assert!(ready, 1);
///   set_i32(&READY, 1);
///
///   // 加值
///   add_i32(&READY, 1);
///   assert!(ready, 2);
///
///   add_i32(&READY, 5);
///   assert!(ready, 7);
///     }
///
/// ```
pub fn add_i32(atomic_bool_static: &AtomicI32,add_value:i32 ) -> i32 {
    let mut value_data: i32 =0;
    let mutex = Arc::new(Mutex::new(&atomic_bool_static));
    mutex.lock();
    atomic_bool_static.fetch_add(add_value,Ordering::SeqCst);
    value_data = atomic_bool_static.load(Ordering::SeqCst);
    drop(mutex);
    value_data
}

/// 设置 AtomicI64的内容
///
/// # 用法
///
///
/// ```
/// use std::sync::atomic::{AtomicI64};
///
/// static READY: AtomicI64 = AtomicI64::new(0);
///
/// fn main() {
///   // 读取值
///   let mut ready = get_i64(&READY);
///   assert!(ready, 0);
///
///   // 设置值
///   set_i64(&READY, 1);
///   ready = get_i64(&READY);
///   assert!(ready, 1);
///   set_i64(&READY, 1);
///
///   // 加值
///   add_i64(&READY, 1);
///   assert!(ready, 2);
///
///   add_i64(&READY, 5);
///   assert!(ready, 7);
///     }
///
/// ```
pub fn set_i64(atomic_bool_static: &AtomicI64, value: i64) -> i64 {
    let mut value_data: i64 = value;
    let mutex = Arc::new(Mutex::new(&atomic_bool_static));
    mutex.lock();
    atomic_bool_static.store(value, Ordering::SeqCst);
    value_data = atomic_bool_static.load(Ordering::SeqCst);
    drop(mutex);
    value_data
}

/// 取 AtomicI64的内容
///
/// # 用法
///
///
/// ```
/// use std::sync::atomic::{AtomicI64};
///
/// static READY: AtomicI64 = AtomicI64::new(0);
///
/// fn main() {
///   // 读取值
///   let mut ready = get_i64(&READY);
///   assert!(ready, 0);
///
///   // 设置值
///   set_i64(&READY, 1);
///   ready = get_i64(&READY);
///   assert!(ready, 1);
///   set_i64(&READY, 1);
///
///   // 加值
///   add_i64(&READY, 1);
///   assert!(ready, 2);
///
///   add_i64(&READY, 5);
///   assert!(ready, 7);
///     }
///
/// ```
pub fn get_i64(atomic_bool_static: &AtomicI64) -> i64 {
    let mut value_data: i64 = 0;
    let mutex = Arc::new(Mutex::new(&atomic_bool_static));
    mutex.lock();
    // atomic_bool_static.store(value, Ordering::SeqCst);
    value_data = atomic_bool_static.load(Ordering::SeqCst);
    drop(mutex);
    value_data
}

/// 加加 AtomicI64的内容
///
/// # 用法
///
///
/// ```
/// use std::sync::atomic::{AtomicI64};
///
/// static READY: AtomicI64 = AtomicI64::new(0);
///
/// fn main() {
///   // 读取值
///   let mut ready = get_i64(&READY);
///   assert!(ready, 0);
///
///   // 设置值
///   set_i64(&READY, 1);
///   ready = get_i64(&READY);
///   assert!(ready, 1);
///   set_i64(&READY, 1);
///
///   // 加值
///   add_i64(&READY, 1);
///   assert!(ready, 2);
///
///   add_i64(&READY, 5);
///   assert!(ready, 7);
///     }
///
/// ```
pub fn add_i64(atomic_bool_static: &AtomicI64,add_value:i64 ) -> i64 {
    let mut value_data: i64 =0;
    let mutex = Arc::new(Mutex::new(&atomic_bool_static));
    mutex.lock();
    atomic_bool_static.fetch_add(add_value,Ordering::SeqCst);
    value_data = atomic_bool_static.load(Ordering::SeqCst);
    drop(mutex);
    value_data
}


/// 设置 AtomicUsize的内容
///
/// # 用法
///
///
/// ```
/// use std::sync::atomic::{AtomicUsize};
///
/// static READY: AtomicUsize = AtomicUsize::new(0);
///
/// fn main() {
///   // 读取值
///   let mut ready = get_usize(&READY);
///   assert!(ready, 0);
///
///   // 设置值
///   set_usize(&READY, 1);
///   ready = get_usize(&READY);
///   assert!(ready, 1);
///   set_usize(&READY, 1);
///
///   // 加值
///   add_usize(&READY, 1);
///   assert!(ready, 2);
///
///   add_usize(&READY, 5);
///   assert!(ready, 7);
///     }
///
/// ```
pub fn set_usize(atomic_bool_static: &AtomicUsize, value: usize) -> usize {
    let mut value_data: usize = value;
    let mutex = Arc::new(Mutex::new(&atomic_bool_static));
    mutex.lock();
    atomic_bool_static.store(value, Ordering::SeqCst);
    value_data = atomic_bool_static.load(Ordering::SeqCst);
    drop(mutex);
    value_data
}

/// 取 AtomicUsize的内容
///
/// # 用法
///
///
/// ```
/// use std::sync::atomic::{AtomicUsize};
///
/// static READY: AtomicUsize = AtomicUsize::new(0);
///
/// fn main() {
///   // 读取值
///   let mut ready = get_usize(&READY);
///   assert!(ready, 0);
///
///   // 设置值
///   set_usize(&READY, 1);
///   ready = get_usize(&READY);
///   assert!(ready, 1);
///   set_usize(&READY, 1);
///
///   // 加值
///   add_usize(&READY, 1);
///   assert!(ready, 2);
///
///   add_usize(&READY, 5);
///   assert!(ready, 7);
///     }
///
/// ```
pub fn get_usize(atomic_bool_static: &AtomicUsize) -> usize {
    let mut value_data: usize = 0;
    let mutex = Arc::new(Mutex::new(&atomic_bool_static));
    mutex.lock();
    // atomic_bool_static.store(value, Ordering::SeqCst);
    value_data = atomic_bool_static.load(Ordering::SeqCst);
    drop(mutex);
    value_data
}

/// 加加 AtomicUsize的内容
///
/// # 用法
///
///
/// ```
/// use std::sync::atomic::{AtomicUsize};
///
/// static READY: AtomicUsize = AtomicUsize::new(0);
///
/// fn main() {
///   // 读取值
///   let mut ready = get_usize(&READY);
///   assert!(ready, 0);
///
///   // 设置值
///   set_usize(&READY, 1);
///   ready = get_usize(&READY);
///   assert!(ready, 1);
///   set_usize(&READY, 1);
///
///   // 加值
///   add_usize(&READY, 1);
///   assert!(ready, 2);
///
///   add_usize(&READY, 5);
///   assert!(ready, 7);
///     }
///
/// ```
pub fn add_usize(atomic_bool_static: &AtomicUsize,add_value:usize ) -> usize {
    let mut value_data: usize =0;
    let mutex = Arc::new(Mutex::new(&atomic_bool_static));
    mutex.lock();
    atomic_bool_static.fetch_add(add_value,Ordering::SeqCst);
    value_data = atomic_bool_static.load(Ordering::SeqCst);
    drop(mutex);
    value_data
}

