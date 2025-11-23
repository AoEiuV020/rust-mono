use common::{Logger, max};

pub struct Calculator {
    logger: Logger,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator {
            logger: Logger::new("MathLib"),
        }
    }

    pub fn add(&self, a: i32, b: i32) -> i32 {
        self.logger.logf(format!("计算: {} + {}", a, b));
        let result = a + b;
        self.logger.logf(format!("结果: {}", result));
        result
    }

    pub fn multiply(&self, a: i32, b: i32) -> i32 {
        self.logger.logf(format!("计算: {} × {}", a, b));
        let result = a * b;
        self.logger.logf(format!("结果: {}", result));
        result
    }

    pub fn factorial(&self, n: i32) -> i64 {
        self.logger.logf(format!("计算: {}!", n));
        let result = if n <= 1 {
            1
        } else {
            (2..=n as i64).product()
        };
        self.logger.logf(format!("结果: {}", result));
        result
    }

    pub fn max_of_three(&self, a: i32, b: i32, c: i32) -> i32 {
        self.logger.logf(format!("求最大值: {}, {}, {}", a, b, c));
        let result = max(max(a, b), c);
        self.logger.logf(format!("结果: {}", result));
        result
    }
}

// C兼容层
use std::collections::HashMap;
use std::sync::Mutex;

static CALCULATOR_MAP: Mutex<Option<HashMap<usize, Calculator>>> = Mutex::new(None);
static mut NEXT_ID: usize = 1;

fn get_calculator_map() -> &'static Mutex<Option<HashMap<usize, Calculator>>> {
    &CALCULATOR_MAP
}

#[no_mangle]
pub extern "C" fn mathlib_calculator_new() -> usize {
    let calculator = Calculator::new();
    let mut map_guard = get_calculator_map().lock().unwrap();
    let map = map_guard.get_or_insert_with(HashMap::new);
    
    let id = unsafe {
        let id = NEXT_ID;
        NEXT_ID += 1;
        id
    };
    
    map.insert(id, calculator);
    id
}

#[no_mangle]
pub extern "C" fn mathlib_calculator_add(id: usize, a: i32, b: i32) -> i32 {
    let map_guard = get_calculator_map().lock().unwrap();
    if let Some(ref map) = *map_guard {
        if let Some(calc) = map.get(&id) {
            return calc.add(a, b);
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn mathlib_calculator_multiply(id: usize, a: i32, b: i32) -> i32 {
    let map_guard = get_calculator_map().lock().unwrap();
    if let Some(ref map) = *map_guard {
        if let Some(calc) = map.get(&id) {
            return calc.multiply(a, b);
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn mathlib_calculator_factorial(id: usize, n: i32) -> i64 {
    let map_guard = get_calculator_map().lock().unwrap();
    if let Some(ref map) = *map_guard {
        if let Some(calc) = map.get(&id) {
            return calc.factorial(n);
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn mathlib_calculator_max_of_three(id: usize, a: i32, b: i32, c: i32) -> i32 {
    let map_guard = get_calculator_map().lock().unwrap();
    if let Some(ref map) = *map_guard {
        if let Some(calc) = map.get(&id) {
            return calc.max_of_three(a, b, c);
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn mathlib_calculator_free(id: usize) {
    let mut map_guard = get_calculator_map().lock().unwrap();
    if let Some(ref mut map) = *map_guard {
        map.remove(&id);
    }
}
