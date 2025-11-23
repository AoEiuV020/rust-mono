use libloading::{Library, Symbol};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

struct CalculatorWrapper {
    lib: Library,
    id: usize,
}

impl CalculatorWrapper {
    fn new(lib_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        unsafe {
            let lib = Library::new(lib_path)?;
            let new_fn: Symbol<extern "C" fn() -> usize> = lib.get(b"mathlib_calculator_new")?;
            let id = new_fn();
            Ok(CalculatorWrapper { lib, id })
        }
    }

    fn add(&self, a: i32, b: i32) -> i32 {
        unsafe {
            let add_fn: Symbol<extern "C" fn(usize, i32, i32) -> i32> =
                self.lib.get(b"mathlib_calculator_add").unwrap();
            add_fn(self.id, a, b)
        }
    }

    fn multiply(&self, a: i32, b: i32) -> i32 {
        unsafe {
            let multiply_fn: Symbol<extern "C" fn(usize, i32, i32) -> i32> =
                self.lib.get(b"mathlib_calculator_multiply").unwrap();
            multiply_fn(self.id, a, b)
        }
    }

    fn factorial(&self, n: i32) -> i64 {
        unsafe {
            let factorial_fn: Symbol<extern "C" fn(usize, i32) -> i64> =
                self.lib.get(b"mathlib_calculator_factorial").unwrap();
            factorial_fn(self.id, n)
        }
    }

    fn max_of_three(&self, a: i32, b: i32, c: i32) -> i32 {
        unsafe {
            let max_fn: Symbol<extern "C" fn(usize, i32, i32, i32) -> i32> =
                self.lib.get(b"mathlib_calculator_max_of_three").unwrap();
            max_fn(self.id, a, b, c)
        }
    }
}

impl Drop for CalculatorWrapper {
    fn drop(&mut self) {
        unsafe {
            let free_fn: Symbol<extern "C" fn(usize)> =
                self.lib.get(b"mathlib_calculator_free").unwrap();
            free_fn(self.id);
        }
    }
}

struct StringProcessorWrapper {
    lib: Library,
    id: usize,
}

impl StringProcessorWrapper {
    fn new(lib_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        unsafe {
            let lib = Library::new(lib_path)?;
            let new_fn: Symbol<extern "C" fn() -> usize> = lib.get(b"stringlib_processor_new")?;
            let id = new_fn();
            Ok(StringProcessorWrapper { lib, id })
        }
    }

    fn reverse(&self, s: &str) -> String {
        unsafe {
            let c_str = CString::new(s).unwrap();
            let reverse_fn: Symbol<extern "C" fn(usize, *const c_char) -> *mut c_char> =
                self.lib.get(b"stringlib_processor_reverse").unwrap();
            let result_ptr = reverse_fn(self.id, c_str.as_ptr());
            let result = CStr::from_ptr(result_ptr).to_string_lossy().into_owned();
            let free_fn: Symbol<extern "C" fn(*mut c_char)> =
                self.lib.get(b"stringlib_free_string").unwrap();
            free_fn(result_ptr);
            result
        }
    }

    fn concat(&self, strings: &[&str], separator: &str) -> String {
        unsafe {
            let c_strings: Vec<CString> = strings.iter().map(|s| CString::new(*s).unwrap()).collect();
            let c_ptrs: Vec<*const c_char> = c_strings.iter().map(|cs| cs.as_ptr()).collect();
            let separator_c = CString::new(separator).unwrap();
            
            let concat_fn: Symbol<extern "C" fn(usize, *const *const c_char, usize, *const c_char) -> *mut c_char> =
                self.lib.get(b"stringlib_processor_concat").unwrap();
            let result_ptr = concat_fn(self.id, c_ptrs.as_ptr(), c_ptrs.len(), separator_c.as_ptr());
            let result = CStr::from_ptr(result_ptr).to_string_lossy().into_owned();
            let free_fn: Symbol<extern "C" fn(*mut c_char)> =
                self.lib.get(b"stringlib_free_string").unwrap();
            free_fn(result_ptr);
            result
        }
    }

    fn to_upper(&self, s: &str) -> String {
        unsafe {
            let c_str = CString::new(s).unwrap();
            let to_upper_fn: Symbol<extern "C" fn(usize, *const c_char) -> *mut c_char> =
                self.lib.get(b"stringlib_processor_to_upper").unwrap();
            let result_ptr = to_upper_fn(self.id, c_str.as_ptr());
            let result = CStr::from_ptr(result_ptr).to_string_lossy().into_owned();
            let free_fn: Symbol<extern "C" fn(*mut c_char)> =
                self.lib.get(b"stringlib_free_string").unwrap();
            free_fn(result_ptr);
            result
        }
    }

    fn word_count(&self, s: &str) -> usize {
        unsafe {
            let c_str = CString::new(s).unwrap();
            let word_count_fn: Symbol<extern "C" fn(usize, *const c_char) -> usize> =
                self.lib.get(b"stringlib_processor_word_count").unwrap();
            word_count_fn(self.id, c_str.as_ptr())
        }
    }
}

impl Drop for StringProcessorWrapper {
    fn drop(&mut self) {
        unsafe {
            let free_fn: Symbol<extern "C" fn(usize)> =
                self.lib.get(b"stringlib_processor_free").unwrap();
            free_fn(self.id);
        }
    }
}

fn main() {
    println!("=================================================");
    println!("Rust 多模块项目演示 - 动态链接版本");
    println!("=================================================\n");

    // 确定库路径
    let lib_dir = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("lib");

    let mathlib_path = if cfg!(target_os = "macos") {
        lib_dir.join("libmathlib.dylib")
    } else {
        lib_dir.join("libmathlib.so")
    };

    let stringlib_path = if cfg!(target_os = "macos") {
        lib_dir.join("libstringlib.dylib")
    } else {
        lib_dir.join("libstringlib.so")
    };

    // 数学运算演示
    println!("--- 数学运算演示 ---");
    let calc = CalculatorWrapper::new(mathlib_path.to_str().unwrap()).unwrap();
    
    let add_result = calc.add(10, 20);
    let multiply_result = calc.multiply(5, 7);
    let factorial_result = calc.factorial(5);
    let max_result = calc.max_of_three(15, 8, 23);

    // 字符串处理演示
    println!("\n--- 字符串处理演示 ---");
    let proc = StringProcessorWrapper::new(stringlib_path.to_str().unwrap()).unwrap();
    
    let reverse_result = proc.reverse("Hello World");
    let concat_result = proc.concat(&["Rust", "Mono", "Project"], " - ");
    let upper_result = proc.to_upper("rustlang");
    let word_count_result = proc.word_count("This is a test string");

    // 输出结果摘要
    println!("\n=================================================");
    println!("结果摘要");
    println!("=================================================");
    println!("\n数学运算:");
    println!("  10 + 20 = {}", add_result);
    println!("  5 × 7 = {}", multiply_result);
    println!("  5! = {}", factorial_result);
    println!("  Max(15, 8, 23) = {}", max_result);
    
    println!("\n字符串处理:");
    println!("  反转 \"Hello World\" = \"{}\"", reverse_result);
    println!("  连接 [\"Rust\", \"Mono\", \"Project\"] = \"{}\"", concat_result);
    println!("  大写 \"rustlang\" = \"{}\"", upper_result);
    println!("  单词数 \"This is a test string\" = {}", word_count_result);
    
    println!("\n=================================================");
}
