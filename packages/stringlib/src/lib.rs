use common::{Logger, to_upper};

pub struct StringProcessor {
    logger: Logger,
}

impl StringProcessor {
    pub fn new() -> Self {
        StringProcessor {
            logger: Logger::new("StringLib"),
        }
    }

    pub fn reverse(&self, s: &str) -> String {
        self.logger.logf(format!("反转字符串: \"{}\"", s));
        let result: String = s.chars().rev().collect();
        self.logger.logf(format!("结果: \"{}\"", result));
        result
    }

    pub fn concat(&self, strings: &[&str], separator: &str) -> String {
        self.logger.logf(format!("连接字符串: {:?}, 分隔符: \"{}\"", strings, separator));
        let result = strings.join(separator);
        self.logger.logf(format!("结果: \"{}\"", result));
        result
    }

    pub fn to_upper(&self, s: &str) -> String {
        self.logger.logf(format!("转换为大写: \"{}\"", s));
        let result = to_upper(s);
        self.logger.logf(format!("结果: \"{}\"", result));
        result
    }

    pub fn word_count(&self, s: &str) -> usize {
        self.logger.logf(format!("统计单词数: \"{}\"", s));
        let count = s.split_whitespace().count();
        self.logger.logf(format!("结果: {} 个单词", count));
        count
    }
}

// C兼容层
use std::collections::HashMap;
use std::sync::Mutex;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

static PROCESSOR_MAP: Mutex<Option<HashMap<usize, StringProcessor>>> = Mutex::new(None);
static mut NEXT_ID: usize = 1;

fn get_processor_map() -> &'static Mutex<Option<HashMap<usize, StringProcessor>>> {
    &PROCESSOR_MAP
}

#[no_mangle]
pub extern "C" fn stringlib_processor_new() -> usize {
    let processor = StringProcessor::new();
    let mut map_guard = get_processor_map().lock().unwrap();
    let map = map_guard.get_or_insert_with(HashMap::new);
    
    let id = unsafe {
        let id = NEXT_ID;
        NEXT_ID += 1;
        id
    };
    
    map.insert(id, processor);
    id
}

#[no_mangle]
pub extern "C" fn stringlib_processor_reverse(id: usize, s: *const c_char) -> *mut c_char {
    let s = unsafe {
        assert!(!s.is_null());
        CStr::from_ptr(s).to_str().unwrap()
    };

    let map_guard = get_processor_map().lock().unwrap();
    if let Some(ref map) = *map_guard {
        if let Some(proc) = map.get(&id) {
            let result = proc.reverse(s);
            return CString::new(result).unwrap().into_raw();
        }
    }
    CString::new("").unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn stringlib_processor_concat(
    id: usize,
    strings: *const *const c_char,
    count: usize,
    separator: *const c_char,
) -> *mut c_char {
    let separator = unsafe {
        assert!(!separator.is_null());
        CStr::from_ptr(separator).to_str().unwrap()
    };

    let strings_vec: Vec<&str> = unsafe {
        (0..count)
            .map(|i| {
                let ptr = *strings.add(i);
                CStr::from_ptr(ptr).to_str().unwrap()
            })
            .collect()
    };

    let map_guard = get_processor_map().lock().unwrap();
    if let Some(ref map) = *map_guard {
        if let Some(proc) = map.get(&id) {
            let result = proc.concat(&strings_vec, separator);
            return CString::new(result).unwrap().into_raw();
        }
    }
    CString::new("").unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn stringlib_processor_to_upper(id: usize, s: *const c_char) -> *mut c_char {
    let s = unsafe {
        assert!(!s.is_null());
        CStr::from_ptr(s).to_str().unwrap()
    };

    let map_guard = get_processor_map().lock().unwrap();
    if let Some(ref map) = *map_guard {
        if let Some(proc) = map.get(&id) {
            let result = proc.to_upper(s);
            return CString::new(result).unwrap().into_raw();
        }
    }
    CString::new("").unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn stringlib_processor_word_count(id: usize, s: *const c_char) -> usize {
    let s = unsafe {
        assert!(!s.is_null());
        CStr::from_ptr(s).to_str().unwrap()
    };

    let map_guard = get_processor_map().lock().unwrap();
    if let Some(ref map) = *map_guard {
        if let Some(proc) = map.get(&id) {
            return proc.word_count(s);
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn stringlib_processor_free(id: usize) {
    let mut map_guard = get_processor_map().lock().unwrap();
    if let Some(ref mut map) = *map_guard {
        map.remove(&id);
    }
}

#[no_mangle]
pub extern "C" fn stringlib_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}
