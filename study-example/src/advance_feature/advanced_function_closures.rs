fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

/// 运行结果如下
/// ```txt
/// do_twice answer: 8
/// ```
fn function_pointer() {
    let answer = do_twice(add_one, 3);
    println!("do_twice answer: {answer}");
}

/// 运行结果如下
/// ```txt
/// list of strings: ["1", "2", "3"]
/// ```
fn iterator_map_usage_closures() {
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    println!("list of strings: {:?}", list_of_strings);
}

/// 运行结果如下
/// ```txt
/// list of strings: ["1", "2", "3"]
/// ```
fn iterator_map_usage_fn_pointer() {
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    println!("list of strings: {:?}", list_of_strings);
}

/// 运行结果如下
/// ```txt
/// list of status: [Value(0), Value(1), Value(2), Value(3), Value(4)]
/// ```
fn init_enum_type() {
    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }
    let list_of_status: Vec<Status> = (0u32..5).map(Status::Value).collect();
    println!("list of status: {:?}", list_of_status);
}

/// 编译报如下的错
/// error[E0746]: return type cannot have an unboxed trait object
///  --> study-example/src/advance_feature/advanced_function_closures.rs:52:24
///   |
/// 52 | fn return_closure() -> dyn Fn(i32) -> i32 {
///    |                        ^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
/*fn return_closure() -> dyn Fn(i32) -> i32 {
    |x| x + 1
}*/

fn return_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

/// 运行结果如下
/// ```txt
/// return_closure i = 32: 33
/// ```
pub fn advanced_function_closures_study() {
    function_pointer();
    iterator_map_usage_closures();
    iterator_map_usage_fn_pointer();
    init_enum_type();
    let f = return_closure();
    println!("return_closure i = 32: {}", f(32));
}
