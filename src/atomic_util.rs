use std::hint;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicUsize, Ordering,AtomicI64};
use std::sync::{Arc, Condvar, Mutex,RwLock};

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
        let mut oid_data: bool = false;
        let mut value_data: bool = false;
        let mutex = Arc::new(Mutex::new(&$atomic_bool_static));
        mutex.lock();
        
        oid_data = $atomic_bool_static.load(Ordering::SeqCst);
        
        $atomic_bool_static.store($value, Ordering::SeqCst);
        
        value_data = $atomic_bool_static.load(Ordering::SeqCst);
        drop(mutex);

        value_data!=oid_data
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

///  写入 RwLock<Arc<any>> 的内容
///
/// # 用法
///
///
/// ```
///  use std::sync::{Arc, RwLock};
///     thread_local! {
///     static CURRENT_CONFIG: RwLock<Arc<String>> = RwLock::new(Arc::new(String::new()));
///     }
///
///     write_rw_lock!(CURRENT_CONFIG, "写入的文本".to_string());
///
///    // 直接读
///  if let Option::Some(read_value) = read_rw_lock!(CURRENT_CONFIG) {
///         println!("Current config: {}", read_value);
///     }
///
///   // 通过设置默认值
///
///  println!("Current config: {}", read_rw_lock!( CURRENT_CONFIG , String::fmt("默认值") ));
/// ```

#[macro_export]
macro_rules! write_rw_lazy_lock {
    ($rw_lock_var: expr,$value:expr) => {{
        let mut result = false;
        
        // 写入
        $rw_lock_var.with(|config| {
            if let Result::Ok(mut write_lock) = config.try_write() {
                *write_lock = Arc::new($value);
                result = true;
                // write_lock.clear_poison();
            }


        });
        result
    }};
}

#[macro_export]
macro_rules! write_rw_lazy_lock_insert {
    ($rw_lock_var: expr,$value:expr) => {{
        let mut result = false;
        let mut oid_value = read_rw_lock!($rw_lock_var, $value).to_vec(); 

        for value in $value {
            oid_value.push(value);
        }

        // write_rw_lock!($rw_lock_var,oid_value);
        
        $rw_lock_var.with(|config| {
            if let Result::Ok(mut write_lock) = config.try_write() {
                *write_lock = Arc::new(oid_value);
                result = true;
                // write_lock.clear_poison();
            }


        });

        result
    }};
}

///  读取 RwLock<Arc<any>> 的内容
///
/// # 用法
///
///
/// ```
///  use std::sync::{Arc, RwLock};
///     thread_local! {
///     static CURRENT_CONFIG: RwLock<Arc<String>> = RwLock::new(Arc::new(String::new()));
///     }
///
///     write_rw_lock!(CURRENT_CONFIG, "写入的文本".to_string());
///
///    // 直接读
///  if let Option::Some(read_value) = read_rw_lock!(CURRENT_CONFIG) {
///         println!("Current config: {}", read_value);
///     }
///
///   // 通过设置默认值
///
///  println!("Current config: {}", read_rw_lock!( CURRENT_CONFIG , String::fmt("默认值") ));
/// ```
/// 
#[macro_export]
macro_rules! read_rw_lazy_lock {
    ($rw_lock_var: expr) => {{
        let mut read_value = Default::default();
        IMG_PREVIEW_LIST_ARCLAZY.with(|config| {
            read_value=  config.load_full()
        });
        read_value
    }};
}

// // static mut IMG_PREVIEW_LIST_ARC: String =String::from("123");
// static mut CURRENT_CONFIGC:String = String::new();

// impl<T> Shared<T> {
//     /// Create a new [`Shared`], backed by an `Arc` and poisoning on panic.
//     pub fn new(t: T) -> Self {
//         Self {
//             atom:AtomicUsize::new(0),
//             data: t,
//         }
//     }
// }

// pub struct Shared<T> {
//     atom: AtomicUsize,
//     data: T,
// }

#[macro_export]
macro_rules! write_rw_lock {
    ($rw_lock_var: expr,$value:expr) => {{
        let mut result = false;
        
        // 写入
        $rw_lock_var.with(|config| {
            if let Result::Ok(mut write_lock) = config.try_write() {
                *write_lock = Arc::new($value);
                result = true;
                // write_lock.clear_poison();
            }


        });
        result
    }};
}

#[macro_export]
macro_rules! write_rw_lock_insert {
    ($rw_lock_var: expr,$value:expr) => {{
        let mut result = false;
        let mut oid_value = read_rw_lock!($rw_lock_var, $value).to_vec(); 

        for value in $value {
            oid_value.push(value);
        }

        // write_rw_lock!($rw_lock_var,oid_value);
        
        $rw_lock_var.with(|config| {
            if let Result::Ok(mut write_lock) = config.try_write() {
                *write_lock = Arc::new(oid_value);
                result = true;
                // write_lock.clear_poison();
            }


        });

        result
    }};
}

#[macro_export]
macro_rules! read_rw_lock {
    ($rw_lock_var: expr) => {{
        let mut read_value = Default::default();
        // 写入
        $rw_lock_var.with(|config| {
            if let Result::Ok(mut read_data) = config.try_read() {
                read_value = Some(Arc::clone(&read_data));
            } else {
                read_value = None;
            }
        });
        read_value
    }};
    ($rw_lock_var: expr,$default: expr) => {{
        let mut read_value = Arc::new($default);
        // 写入
        $rw_lock_var.with(|config| {
            if let Result::Ok(mut read_data) = config.try_read() {
               read_value =Arc::clone(&read_data);
            }
        });
        read_value
    }};
}

#[macro_export]
macro_rules! set_arc_bind_variable{
    ($static_var: expr,$static_atomic: expr,$value:expr)=>{{
        let mutex = Arc::new(Mutex::new(&$static_atomic));
        mutex.lock();
        let the_value:usize = $static_atomic.load(Ordering::SeqCst);
        $static_atomic.store(the_value+1, Ordering::SeqCst);
    
        unsafe{
            $static_var = $value; 
        }
        
        drop(mutex);}
    }
}


#[macro_export]
macro_rules! set_arc_bind_variable_insert {
    ($static_var: expr,$static_atomic: expr,$value:expr)=>{{
        let mutex = Arc::new(Mutex::new(&$static_atomic));
        mutex.lock();
        let the_value:usize = $static_atomic.load(Ordering::SeqCst);
        $static_atomic.store(the_value+1, Ordering::SeqCst);
    
        unsafe{
            // $static_var = $value; 
            for value in $value {
                $static_var.push(value);
            }
        }
        
        drop(mutex);}
    }
}

#[macro_export]
macro_rules! get_arc_bind_variable{
    ($static_var: expr,$static_atomic: expr)=>{{
        let mutex = Arc::new(Mutex::new(&$static_atomic));
        mutex.lock();
        let the_value:usize = $static_atomic.load(Ordering::SeqCst);
        $static_atomic.store(the_value+1, Ordering::SeqCst);
    
        let data = unsafe{&$static_var};
        drop(mutex);
        data}
    }
}

