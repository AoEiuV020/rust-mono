use mathlib::Calculator;
use stringlib::StringProcessor;

fn main() {
    println!("=================================================");
    println!("Rust 多模块项目演示 - 静态链接版本");
    println!("=================================================\n");

    // 数学运算演示
    println!("--- 数学运算演示 ---");
    let calc = Calculator::new();
    
    let add_result = calc.add(10, 20);
    let multiply_result = calc.multiply(5, 7);
    let factorial_result = calc.factorial(5);
    let max_result = calc.max_of_three(15, 8, 23);

    // 字符串处理演示
    println!("\n--- 字符串处理演示 ---");
    let proc = StringProcessor::new();
    
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
