use rand::Rng;
use std::cmp::Ordering;
use std::io;

mod basic_concept;

fn guess_game() {
    println!("Guess your number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // 循环处理
    loop {
        println!("Please input your guess.");
        // 变量默认是不可变的immutable，添加mut可以使变量可变；
        let mut guess = String::new();
        /*
         * read_line 的完整工作是获取任何内容用户键入标准输入并将其附加到字符串中
         * &说明传入的是一个引用
         * mut说明参数是可变的，默认情况下引用不可变，所以需要把编写&mut guess来使得引用可变
         * Result类型存在2种，变量是Ok和Err，如果是Err，expect将会使程序crash并且输出expect的msg
         */
        let _ = io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        /* 在没有use std::io的时候也可以直接使用下面的语句进行替代；
         * std::io::stdin()
         */
        // rust允许用新值来隐藏之前的值，所以这里可以又定义一次guess变量
        // 输出错误的时候继续
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("input err: {guess}");
                continue;
            }
        };
        // 看起来类似 js的``去格式化字符串
        println!("Your guess: {guess}");
        /*
         * match表达式由臂组成，匹配到对应的值就运行对应的代码
         * 注意rust的类型，虽然有类型推断
         */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("small."),
            Ordering::Greater => println!("big."),
            // 猜对终止循环
            Ordering::Equal => {
                println!("win!");
                break;
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    // guess_game();
    // basic_concept::basic_concept::variable::test_variable();
    // basic_concept::basic_concept::data_types::test_data_types();
    basic_concept::basic_concept::function::test_function_handle();
}
