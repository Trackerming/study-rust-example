use rand::Rng;
use std::cmp::Ordering;
use std::io;

use study_example::*;

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
    println!("study rust example: ");
    // guess_game();
    // basic_concept::basic_concept::variable::test_variable();
    // basic_concept::basic_concept::data_types::test_data_types();
    // basic_concept::basic_concept::function::test_function_handle();
    // basic_concept::basic_concept::control_flow::test_control_flow_handle();
    // ownership::ownership::ownership_test::ownership_test_handle();
    // ownership::ownership::reference_borrow::reference_borrow_study();
    // ownership::ownership::slice::slice_study();
    // struct_related::define_init_struct::define_init_struct_study();
    // struct_related::retangle::retangle_struct_study();
    // enum_pattern_match::enum_pattern_match::define_enum::define_enum_study();
    // enum_pattern_match::enum_pattern_match::enum_usage::use_enum_study();
    // enum_pattern_match::enum_pattern_match::option_usage::option_uasge_study();
    // enum_pattern_match::enum_pattern_match::match_control_flow::match_control_flow_study();
    // enum_pattern_match::enum_pattern_match::if_let_control::if_let_control_study();
    // package_crate_module::package_crate_module::package_crate_path::front_of_house_root();
    // common_collection::vector_study::vector_usage_study();
    // common_collection::string_study::string_usage_study();
    // common_collection::hash_map_study::hash_map_usage_study();
    // error_handle::panic_unrecoverable::panic_unrecover_error_study();
    // error_handle::result_recorverable::result_recover_study();
    // generic_traits_lifetime::generic_type_study::generic_type_usage_study();
    // generic_traits_lifetime::traits_study::traits_usage_study();
    // generic_traits_lifetime::lifetime_study::lifetime_kown_study();
    // generic_traits_lifetime::mix_usage::mix_usage_study();
    // iterator_closure::closure_study::closures_related_usage();
    // iterator_closure::iterator_process_study::iterator_usage();
    // smart_pointer::box_pointer::box_pointer_study();
    // smart_pointer::deref_trait::deref_trait_study();
    // smart_pointer::drop_trait::drop_trait_study();
    // smart_pointer::rc_t_count_pointer::rc_t_count_pointer_study();
    // smart_pointer::refcell_pointer::refcell_pointer_study();
    // smart_pointer::reference_cycle::reference_cycle_study();
    // concurrency::thread_create::thread_create_study();
    // concurrency::message_thread::messsage_thread_study();
    // concurrency::mutex_shared_state::mutex_shared_state_study();
    // concurrency::dead_lock::dead_lock_study();
    object_oriented::object_oriented_feature::object_oriented_feature_study();
    object_oriented::trait_object_for_diff_type::trait_object_for_diff_type_study();
}
