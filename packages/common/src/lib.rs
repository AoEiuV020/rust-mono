use chrono::Local;
use std::sync::Mutex;

pub struct Logger {
    prefix: String,
}

impl Logger {
    pub fn new(prefix: &str) -> Self {
        Logger {
            prefix: prefix.to_string(),
        }
    }

    pub fn log(&self, message: &str) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        println!("[{}] [{}] {}", timestamp, self.prefix, message);
    }

    pub fn logf(&self, message: String) {
        self.log(&message);
    }
}

// 工具函数
pub fn to_upper(s: &str) -> String {
    s.to_uppercase()
}

pub fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

pub fn min(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

// C兼容层 - 用于动态库导出
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::collections::HashMap;

static LOGGER_MAP: Mutex<Option<HashMap<usize, Logger>>> = Mutex::new(None);
static mut NEXT_ID: usize = 1;

fn get_logger_map() -> &'static Mutex<Option<HashMap<usize, Logger>>> {
    &LOGGER_MAP
}

#[no_mangle]
pub extern "C" fn common_logger_new(prefix: *const c_char) -> usize {
    let prefix = unsafe {
        assert!(!prefix.is_null());
        CStr::from_ptr(prefix).to_str().unwrap()
    };

    let logger = Logger::new(prefix);
    let mut map_guard = get_logger_map().lock().unwrap();
    let map = map_guard.get_or_insert_with(HashMap::new);
    
    let id = unsafe {
        let id = NEXT_ID;
        NEXT_ID += 1;
        id
    };
    
    map.insert(id, logger);
    id
}

#[no_mangle]
pub extern "C" fn common_logger_log(id: usize, message: *const c_char) {
    let message = unsafe {
        assert!(!message.is_null());
        CStr::from_ptr(message).to_str().unwrap()
    };

    let map_guard = get_logger_map().lock().unwrap();
    if let Some(ref map) = *map_guard {
        if let Some(logger) = map.get(&id) {
            logger.log(message);
        }
    }
}

#[no_mangle]
pub extern "C" fn common_logger_free(id: usize) {
    let mut map_guard = get_logger_map().lock().unwrap();
    if let Some(ref mut map) = *map_guard {
        map.remove(&id);
    }
}

#[no_mangle]
pub extern "C" fn common_to_upper(input: *const c_char) -> *mut c_char {
    let input = unsafe {
        assert!(!input.is_null());
        CStr::from_ptr(input).to_str().unwrap()
    };

    let result = to_upper(input);
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn common_max(a: i32, b: i32) -> i32 {
    max(a, b)
}

#[no_mangle]
pub extern "C" fn common_min(a: i32, b: i32) -> i32 {
    min(a, b)
}

#[no_mangle]
pub extern "C" fn common_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}
