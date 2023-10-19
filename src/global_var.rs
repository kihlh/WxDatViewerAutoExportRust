#![allow(warnings, unused)]

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicI64, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, MutexGuard, OnceLock, RwLock};

// 哈希表
static mut VARIABLE_U64: Option<HashMap<String, u64>> = Option::None;
static mut VARIABLE_STRING: Option<HashMap<String, String>> = Option::None;
static mut VARIABLE_I32: Option<HashMap<String, i32>> = Option::None;
static mut VARIABLE_I128: Option<HashMap<String, i128>> = Option::None;
static mut VARIABLE_BOOL: Option<HashMap<String, bool>> = Option::None;
static mut VARIABLE_VEC_STRING: Option<HashMap<String, Vec<String>>> = Option::None;
static mut VARIABLE_VEC_I32: Option<HashMap<String, Vec<i32>>> = Option::None;

// 绑定的原子锁
static VARIABLE_STRING_BIND: AtomicUsize = AtomicUsize::new(0);
static VARIABLE_I32_BIND: AtomicUsize = AtomicUsize::new(0);
static VARIABLE_I128_BIND: AtomicUsize = AtomicUsize::new(0);
static VARIABLE_BOOL_BIND: AtomicUsize = AtomicUsize::new(0);
static VARIABLE_VEC_STRING_BIND: AtomicUsize = AtomicUsize::new(0);
static VARIABLE_VEC_I32_BIND: AtomicUsize = AtomicUsize::new(0);
static VARIABLE_U64_BIND: AtomicUsize = AtomicUsize::new(0);


// 已经初始化哈希表了
static VARIABLE_INITIALIZE: OnceLock<bool> = OnceLock::new();

// 初始化全部类型哈希表
fn initialize() {
    if *(VARIABLE_INITIALIZE.get().unwrap_or_else(|| &false)) {
        return;
    }

    unsafe {
        if VARIABLE_U64.is_none() {
            VARIABLE_U64.replace(HashMap::new());
        }
    }

    unsafe {
        if VARIABLE_STRING.is_none() {
            VARIABLE_STRING.replace(HashMap::new());
        }
    }

    unsafe {
        if VARIABLE_I32.is_none() {
            VARIABLE_I32.replace(HashMap::new());
        }
    }

    unsafe {
        if VARIABLE_I128.is_none() {
            VARIABLE_I128.replace(HashMap::new());
        }
    }

    unsafe {
        if VARIABLE_BOOL.is_none() {
            VARIABLE_BOOL.replace(HashMap::new());
        }
    }

    unsafe {
        if VARIABLE_VEC_STRING.is_none() {
            VARIABLE_VEC_STRING.replace(HashMap::new());
        }
    }

    unsafe {
        if VARIABLE_VEC_I32.is_none() {
            VARIABLE_VEC_I32.replace(HashMap::new());
        }
    }

    VARIABLE_INITIALIZE.set(true);
}

// 母函数

// 设置 u64 键值对
pub fn set_u64(key: &str, value: u64) {
    initialize();
    let mutex = Arc::new(Mutex::new(&VARIABLE_U64_BIND));
    mutex.lock();
    let the_value: usize = VARIABLE_U64_BIND.load(Ordering::SeqCst);
    unsafe {
        let mut mut_hash = VARIABLE_U64.as_mut().unwrap();
        mut_hash.insert(key.to_string(), value);
    };

    VARIABLE_U64_BIND.store(the_value + 1, Ordering::SeqCst);
    drop(mutex);
}

// 获取 u64 键值
pub fn get_u64(key: &str) -> Option<u64> {
    initialize();
    let mutex = Arc::new(Mutex::new(&VARIABLE_U64_BIND));
    mutex.lock();
    let the_value: usize = VARIABLE_U64_BIND.load(Ordering::SeqCst);
    unsafe {
        let mut mut_hash = VARIABLE_U64.as_mut().unwrap();
        if let Some(value) = mut_hash.get(&key.to_string()) {
            return Some(value.clone());
        };
    };
    // 写入操作记录到原子并解锁
    VARIABLE_U64_BIND.store(the_value + 1, Ordering::SeqCst);
    drop(mutex);
    None
}

// 获取 u64 键值 没有则返回另外的值
pub fn get_u64_or(key: &str, or_value: u64) -> u64 {
    if let Some(value) = get_u64(key) {
        return value;
    }
    or_value
}

// 获取 u64 键值 没有则返回另外的值
pub fn get_u64_default(key: &str) -> u64 {
    let or_value: u64 = 0;
    if let Some(value) = get_u64(key) {
        return value;
    }
    or_value
}

// 判断 u64 键存在
pub fn has_u64(key: &str) -> bool {
    initialize();
    let mut result = false;
    let mutex = Arc::new(Mutex::new(&VARIABLE_U64_BIND));
    mutex.lock();
    let the_value: usize = VARIABLE_U64_BIND.load(Ordering::SeqCst);

    unsafe {
        let mut mut_hash = VARIABLE_U64.as_mut().unwrap();
        result = mut_hash.get(&key.to_string()).is_some();
    };

    // 写入操作记录到原子并解锁
    VARIABLE_U64_BIND.store(the_value + 1, Ordering::SeqCst);
    drop(mutex);
    result
}

// 设置值的宏
#[macro_export]
macro_rules! set_map_bind_variable {
    ($variable_mut_static: expr,$variable_atomic_bind: expr,$key:expr ,$value:expr) => {{
        initialize();
        let mutex = Arc::new(Mutex::new(&$variable_atomic_bind));
        mutex.lock();
        let the_value: usize = $variable_atomic_bind.load(Ordering::SeqCst);
        unsafe {
            let mut mut_hash = $variable_mut_static.as_mut().unwrap();
            mut_hash.insert($key.to_string(), $value);
        };

        $variable_atomic_bind.store(the_value + 1, Ordering::SeqCst);
        drop(mutex);
    }};
}

// 获取值的宏
#[macro_export]
macro_rules! get_map_bind_variable {
    ($variable_mut_static: expr,$variable_atomic_bind: expr,$key:expr ) => {{
        initialize();
        let mutex = Arc::new(Mutex::new(&$variable_atomic_bind));
        mutex.lock();
        let the_value: usize = $variable_atomic_bind.load(Ordering::SeqCst);
        unsafe {
            let mut mut_hash = $variable_mut_static.as_mut().unwrap();
            if let Some(value) = mut_hash.get(&$key.to_string()) {
                return Some(value.clone());
            };
        };
        // 写入操作记录到原子并解锁
        $variable_atomic_bind.store(the_value + 1, Ordering::SeqCst);
        drop(mutex);
        None
    }};
}

// 判断键的宏
#[macro_export]
macro_rules! has_map_bind_variable {
    ($variable_mut_static: expr,$variable_atomic_bind: expr,$key:expr ) => {{
        initialize();
        let mut result = false;
        let mutex = Arc::new(Mutex::new(&$variable_atomic_bind));
        mutex.lock();
        let the_value: usize = $variable_atomic_bind.load(Ordering::SeqCst);

        unsafe {
            let mut mut_hash = $variable_mut_static.as_mut().unwrap();
            result = mut_hash.get(&$key.to_string()).is_some();
        };

        // 写入操作记录到原子并解锁
        $variable_atomic_bind.store(the_value + 1, Ordering::SeqCst);
        drop(mutex);
        result
    }};
}

// 宏生成的

// 设置 i32 键值对
pub fn set_i32(key: &str, value: i32) {
    set_map_bind_variable!(VARIABLE_I32, VARIABLE_I32_BIND, key, value)
}

// 获取 i32 键值
pub fn get_i32(key: &str) -> Option<i32> {
    get_map_bind_variable!(VARIABLE_I32, VARIABLE_I32_BIND, key)
}

// 获取 i32 键值 没有则返回另外的值
pub fn get_i32_or(key: &str, or_value: i32) -> i32 {
    if let Some(value) = get_i32(key) {
        return value;
    }
    or_value
}

// 获取 i32 键值 没有则返回另外的值
pub fn get_i32_default(key: &str) -> i32 {
    let or_value: i32 = 0;
    if let Some(value) = get_i32(key) {
        return value;
    }
    or_value
}

// 判断 i32 键存在
pub fn has_i32(key: &str) -> bool {
    has_map_bind_variable!(VARIABLE_I32, VARIABLE_I32_BIND, key)
}

// 设置 i128 键值对
pub fn set_i128(key: &str, value: i128) {
    set_map_bind_variable!(VARIABLE_I128, VARIABLE_I128_BIND, key, value)
}

// 获取 i128 键值
pub fn get_i128(key: &str) -> Option<i128> {
    get_map_bind_variable!(VARIABLE_I128, VARIABLE_I128_BIND, key)
}

// 获取 i128 键值 没有则返回另外的值
pub fn get_i128_or(key: &str, or_value: i128) -> i128 {
    if let Some(value) = get_i128(key) {
        return value;
    }
    or_value
}

// 获取 i128 键值 没有则返回另外的值
pub fn get_i128_default(key: &str) -> i128 {
    let or_value: i128 = 0;
    if let Some(value) = get_i128(key) {
        return value;
    }
    or_value
}

// 判断 i128 键存在
pub fn has_i128(key: &str) -> bool {
    has_map_bind_variable!(VARIABLE_I128, VARIABLE_I128_BIND, key)
}

// 设置 bool 键值对
pub fn set_bool(key: &str, value: bool) {
    set_map_bind_variable!(VARIABLE_BOOL, VARIABLE_BOOL_BIND, key, value)
}

// 获取 bool 键值
pub fn get_bool(key: &str) -> Option<bool> {
    get_map_bind_variable!(VARIABLE_BOOL, VARIABLE_BOOL_BIND, key)
}

// 获取 bool 键值 没有则返回另外的值
pub fn get_bool_or(key: &str, or_value: bool) -> bool {
    if let Some(value) = get_bool(key) {
        return value;
    }
    or_value
}

// 获取 bool 键值 没有则返回另外的值
pub fn get_bool_default(key: &str) -> bool {
    let or_value: bool = false;
    if let Some(value) = get_bool(key) {
        return value;
    }
    or_value
}

// 判断 bool 键存在
pub fn has_bool(key: &str) -> bool {
    has_map_bind_variable!(VARIABLE_BOOL, VARIABLE_BOOL_BIND, key)
}

// 设置 string 键值对
pub fn set_string(key: &str, value: String) {
    set_map_bind_variable!(VARIABLE_STRING, VARIABLE_STRING_BIND, key, value)
}

// 获取 string 键值
pub fn get_string(key: &str) -> Option<String> {
    get_map_bind_variable!(VARIABLE_STRING, VARIABLE_STRING_BIND, key)
}

// 获取 string 键值 没有则返回另外的值
pub fn get_string_or(key: &str, or_value: String) -> String {
    if let Some(value) = get_string(key) {
        return value;
    }
    or_value
}

// 获取 String 键值 没有则返回另外的值
pub fn get_string_default(key: &str) -> String {
    let or_value: String = String::new();
    if let Some(value) = get_string(key) {
        return value;
    }
    or_value
}

// 判断 string 键存在
pub fn has_string(key: &str) -> bool {
    has_map_bind_variable!(VARIABLE_STRING, VARIABLE_STRING_BIND, key)
}

// 设置 Vec<String> 键值对
pub fn set_vec_string(key: &str, value: Vec<String>) {
    set_map_bind_variable!(VARIABLE_VEC_STRING, VARIABLE_VEC_STRING_BIND, key, value)
}

// 获取 Vec<String> 键值
pub fn get_vec_string(key: &str) -> Option<Vec<String>> {
    get_map_bind_variable!(VARIABLE_VEC_STRING, VARIABLE_VEC_STRING_BIND, key)
}

// 获取 Vec<String> 键值 没有则返回另外的值
pub fn get_vec_string_or(key: &str, or_value: Vec<String>) -> Vec<String> {
    if let Some(value) = get_vec_string(key) {
        return value;
    }
    or_value
}

// 获取 Vec<String> 键值 没有则返回另外的值
pub fn get_vec_string_default(key: &str) -> Vec<String> {
    let or_value = Vec::new();
    if let Some(value) = get_vec_string(key) {
        return value;
    }
    or_value
}

// 判断 Vec<String> 键存在
pub fn has_vec_string(key: &str) -> bool {
    has_map_bind_variable!(VARIABLE_VEC_STRING, VARIABLE_VEC_STRING_BIND, key)
}

// 设置 Vec<i32> 键值对
pub fn set_vec_i32(key: &str, value: Vec<i32>) {
    set_map_bind_variable!(VARIABLE_VEC_I32, VARIABLE_VEC_I32_BIND, key, value)
}

// 获取 Vec<i32> 键值
pub fn get_vec_i32(key: &str) -> Option<Vec<i32>> {
    get_map_bind_variable!(VARIABLE_VEC_I32, VARIABLE_VEC_I32_BIND, key)
}

// 获取 Vec<i32> 键值 没有则返回另外的值
pub fn get_vec_i32_or(key: &str, or_value: Vec<i32>) -> Vec<i32> {
    if let Some(value) = get_vec_i32(key) {
        return value;
    }
    or_value
}

// 获取 Vec<i32> 键值 没有则返回另外的值
pub fn get_vec_i32_default(key: &str) -> Vec<i32> {
    let or_value = Vec::new();
    if let Some(value) = get_vec_i32(key) {
        return value;
    }
    or_value
}

// 判断 Vec<i32> 键存在
pub fn has_vec_i32(key: &str) -> bool {
    has_map_bind_variable!(VARIABLE_VEC_I32, VARIABLE_VEC_I32_BIND, key)
}

// 添加一个数组的所有值
pub fn insert_vec_i32(key: &str, value: Vec<i32>) {
    initialize();
    let mutex = Arc::new(Mutex::new(&VARIABLE_VEC_I32_BIND));
    mutex.lock();
    let the_value: usize = VARIABLE_VEC_I32_BIND.load(Ordering::SeqCst);
    unsafe {
        let mut mut_hash = VARIABLE_VEC_I32.as_mut().unwrap();

        if let Some(vec_value) = mut_hash.get_mut(&key.to_string()) {
            for data in value {
                vec_value.push(data);
            }
        } else {
            mut_hash.insert(key.to_string(), value);
        }
    };

    VARIABLE_VEC_I32_BIND.store(the_value + 1, Ordering::SeqCst);
    drop(mutex);
}

// 添加数组值
pub fn push_vec_i32(key: &str, value: i32) {
    initialize();
    let mutex = Arc::new(Mutex::new(&VARIABLE_VEC_I32_BIND));
    mutex.lock();
    let the_value: usize = VARIABLE_VEC_I32_BIND.load(Ordering::SeqCst);
    unsafe {
        let mut mut_hash = VARIABLE_VEC_I32.as_mut().unwrap();

        if let Some(vec_value) = mut_hash.get_mut(&key.to_string()) {
            vec_value.push(value);
        } else {
            mut_hash.insert(key.to_string(), Vec::from(vec![value]));
        }
    };

    VARIABLE_VEC_I32_BIND.store(the_value + 1, Ordering::SeqCst);
    drop(mutex);
}

// 取出并清空数组
pub fn retrieve_vec_i32(key: &str) -> Vec<i32> {
    let mut result = Vec::new();
    initialize();
    let mutex = Arc::new(Mutex::new(&VARIABLE_VEC_I32_BIND));
    mutex.lock();
    let the_value: usize = VARIABLE_VEC_I32_BIND.load(Ordering::SeqCst);
    unsafe {
        let mut mut_hash = VARIABLE_VEC_I32.as_mut().unwrap();

        if let Some(vec_value) = mut_hash.get_mut(&key.to_string()) {
            for value in &mut *vec_value {
                result.push(value.clone());
            }
            vec_value.clear();
        } else {
            mut_hash.insert(key.to_string(), Vec::new());
        }
    };

    VARIABLE_VEC_I32_BIND.store(the_value + 1, Ordering::SeqCst);
    drop(mutex);
    result
}

// 获取数组的数量
pub fn get_vec_i32_len(key: &str) -> usize {
    let mut result: usize = 0;
    initialize();
    let mutex = Arc::new(Mutex::new(&VARIABLE_VEC_I32_BIND));
    mutex.lock();
    let the_value: usize = VARIABLE_VEC_I32_BIND.load(Ordering::SeqCst);
    unsafe {
        let mut mut_hash = VARIABLE_VEC_I32.as_mut().unwrap();

        if let Some(vec_value) = mut_hash.get_mut(&key.to_string()) {
            result = vec_value.len();
        } else {
            mut_hash.insert(key.to_string(), Vec::new());
        }
    };

    VARIABLE_VEC_I32_BIND.store(the_value + 1, Ordering::SeqCst);
    drop(mutex);
    result
}

// 获取数组的指定索引
pub fn get_vec_i32_from_index(key: &str, index: usize) -> Option<i32> {
    let mut result: Option<i32> = Option::None;
    initialize();
    let mutex = Arc::new(Mutex::new(&VARIABLE_VEC_I32_BIND));
    mutex.lock();
    let the_value: usize = VARIABLE_VEC_I32_BIND.load(Ordering::SeqCst);
    unsafe {
        let mut mut_hash = VARIABLE_VEC_I32.as_mut().unwrap();

        if let Some(vec_value) = mut_hash.get_mut(&key.to_string()) {
            if let Some(value) = vec_value.get(index) {
                result = Some(value.clone());
            }
        } else {
            mut_hash.insert(key.to_string(), Vec::new());
        }
    };

    VARIABLE_VEC_I32_BIND.store(the_value + 1, Ordering::SeqCst);
    drop(mutex);
    result
}

macro_rules! insert_map_vec_bind_variable {
    ($variable_mut_static:expr,$variable_atomic_bind:expr,$key:expr, $value: expr) => {{
        initialize();
        let mutex = Arc::new(Mutex::new(&$variable_atomic_bind));
        mutex.lock();
        let the_value: usize = $variable_atomic_bind.load(Ordering::SeqCst);
        unsafe {
            let mut mut_hash = $variable_mut_static.as_mut().unwrap();

            if let Some(vec_value) = mut_hash.get_mut(&$key.to_string()) {
                for data in $value {
                    vec_value.push(data);
                }
            } else {
                mut_hash.insert($key.to_string(), $value);
            }
        };

        $variable_atomic_bind.store(the_value + 1, Ordering::SeqCst);
        drop(mutex);
    }};
}

#[macro_export]
macro_rules! push_map_vec_bind_variable {
    ($variable_mut_static:expr,$variable_atomic_bind:expr,$key:expr, $value:expr) => {{
        initialize();
        let mutex = Arc::new(Mutex::new(&$variable_atomic_bind));
        mutex.lock();
        let the_value: usize = $variable_atomic_bind.load(Ordering::SeqCst);
        unsafe {
            let mut mut_hash = $variable_mut_static.as_mut().unwrap();

            if let Some(vec_value) = mut_hash.get_mut(&$key.to_string()) {
                vec_value.push($value);
            } else {
                mut_hash.insert($key.to_string(), Vec::from(vec![$value]));
            }
        };

        $variable_atomic_bind.store(the_value + 1, Ordering::SeqCst);
        drop(mutex);
    }};
}

#[macro_export]
macro_rules! retrieve_map_vec_bind_variable {
    ($variable_mut_static:expr,$variable_atomic_bind:expr,$key: expr) => {{
        let mut result = Vec::new();
        initialize();
        let mutex = Arc::new(Mutex::new(&$variable_atomic_bind));
        mutex.lock();
        let the_value: usize = $variable_atomic_bind.load(Ordering::SeqCst);
        unsafe {
            let mut mut_hash = $variable_mut_static.as_mut().unwrap();

            if let Some(vec_value) = mut_hash.get_mut(&$key.to_string()) {
                for value in &mut *vec_value {
                    result.push(value.clone());
                }
                vec_value.clear();
            } else {
                mut_hash.insert($key.to_string(), Vec::new());
            }
        };

        $variable_atomic_bind.store(the_value + 1, Ordering::SeqCst);
        drop(mutex);
        result
    }};
}

#[macro_export]
macro_rules! from_index_map_vec_bind_variable {
    ($variable_mut_static:expr,$variable_atomic_bind:expr,$key: expr, $index: expr) => {{
        let mut result = Option::None;
        initialize();
        let mutex = Arc::new(Mutex::new(&$variable_atomic_bind));
        mutex.lock();
        let the_value: usize = $variable_atomic_bind.load(Ordering::SeqCst);
        unsafe {
            let mut mut_hash = $variable_mut_static.as_mut().unwrap();

            if let Some(vec_value) = mut_hash.get_mut(&$key.to_string()) {
                if let Some(value) = vec_value.get($index) {
                    result = Some(value.clone());
                }
            } else {
                mut_hash.insert($key.to_string(), Vec::new());
            }
        };

        $variable_atomic_bind.store(the_value + 1, Ordering::SeqCst);
        drop(mutex);
        result
    }};
}

#[macro_export]
macro_rules! len_map_vec_bind_variable {
    ($variable_mut_static:expr,$variable_atomic_bind:expr,$key:expr) => {{
        let mut result: usize = 0;
        initialize();
        let mutex = Arc::new(Mutex::new(&$variable_atomic_bind));
        mutex.lock();
        let the_value: usize = $variable_atomic_bind.load(Ordering::SeqCst);
        unsafe {
            let mut mut_hash = $variable_mut_static.as_mut().unwrap();

            if let Some(vec_value) = mut_hash.get_mut(&$key.to_string()) {
                result = vec_value.len();
            } else {
                mut_hash.insert($key.to_string(), Vec::new());
            }
        };

        $variable_atomic_bind.store(the_value + 1, Ordering::SeqCst);
        drop(mutex);
        result
    }};
}

// 添加一个数组的所有值
pub fn insert_vec_string(key: &str, value: Vec<String>) {
    insert_map_vec_bind_variable!(VARIABLE_VEC_STRING, VARIABLE_VEC_STRING_BIND, key, value)
}

// 添加数组值
pub fn push_vec_string(key: &str, value: String) {
    push_map_vec_bind_variable!(VARIABLE_VEC_STRING, VARIABLE_VEC_STRING_BIND, key, value)
}

// 取出并清空数组
pub fn retrieve_vec_string(key: &str) -> Vec<String> {
    retrieve_map_vec_bind_variable!(VARIABLE_VEC_STRING, VARIABLE_VEC_STRING_BIND, key)
}

// 获取数组的数量
pub fn get_vec_string_len(key: &str) -> usize {
    len_map_vec_bind_variable!(VARIABLE_VEC_STRING, VARIABLE_VEC_STRING_BIND, key)
}

// 获取数组的指定索引
pub fn get_vec_string_from_index(key: &str, index: usize) -> Option<String> {
    from_index_map_vec_bind_variable!(VARIABLE_VEC_STRING, VARIABLE_VEC_STRING_BIND, key, index)
}

#[macro_export]
macro_rules! console_log {
    ($message:expr) => {{
        println!("{}", $message);
        use crate::util;
        global_var::push_vec_string("console_log", util::to_string_default($message));
        // handle_dat::push_console_message($message);
    }};
}