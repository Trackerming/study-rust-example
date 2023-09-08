pub mod ownership_test {
    fn var_scope_study() {
        // 这里因为s没有定义所以是非法的
        {
            let s = "hello"; // s从这里开始变得有效
            println!("s {s}");
        }
        // 离开了上面的scope，s已经失效
        /*
        error[E0425]: cannot find value `s` in this scope
           --> src/ownership/ownership.rs:9:22
           |
         9 |         println!("s {s}");
           |                      ^ not found in this scope*/
        // println!("s {s}");
    }

    fn string_type_owner_study() {
        // 双冒号 :: 运算符允许我们在 String 命名空间这个类型下的特定的 from 函数
        // 这句会请求需要的内存分配；
        {
            let mut s = String::from("hello");
            /*
             * String比起文本可以动态的原因：
             */
            s.push_str(" rust");
            println!("s: {s}");
        }
        // 离开scope调用drop，释放内存；
    }

    fn string_mul_to_one_var_study() {
        let mut s1 = String::from("hello");
        let s2 = s1;
        let mut s3 = s2.clone();
        s3.push_str(" rust");
        println!("s2 {} s3 {}", s2, s3);
        /*
         error[E0382]: borrow of moved value: `s1`
          --> src/ownership/ownership.rs:35:23
           |
        33 |         let mut s1 = String::from("hello");
           |             ------ move occurs because `s1` has type `String`, which does not implement the `Copy` trait
        34 |         let s2 = s1;
           |                  -- value moved here
        35 |         println!("s1: {s1}");
           |                       ^^^^ value borrowed here after move
         */
        // println!("s1: {s1}");
        let x: u16 = 88;
        let y = x;
        println!("x = {} y = {}", x, y);
        // 如果在不同的scope下呢
        {
            let mut s4 = s3;
            println!("s4 {s4}");
        }
        // value borrowed here after move
        // println!("{s3}");
    }

    fn take_ownership(str: String) {
        println!("str {} take_ownership", str);
    }
    fn makes_copy(int: i32) {
        println!("int {}", int);
    }
    fn move_function_study() {
        let mut str = String::from("move_function_study hello");
        take_ownership(str);
        // str value borrowed here
        // println!("{str}");
        let mut int1 = 188;
        makes_copy(int1);
        println!("move_function_study int1 {int1}");
    }

    fn gives_ownership() -> String {
        let str = String::from("gives_ownership");
        str
    }
    fn takes_and_gives_ownership(a_string: String) -> String {
        a_string // a_string is returned and moves out to the calling function
    }
    fn caculate_length(str:String) ->(String, usize){
        let length = str.len();
        (str, length)
    }
    fn move_function_ownership_study_2() {
        let str_from_gives_ownership = gives_ownership();
        println!("{str_from_gives_ownership}");
        let str_from_takes_and_gives = takes_and_gives_ownership(str_from_gives_ownership);
        println!("{str_from_takes_and_gives}");
        let (str_ret, str_ret_len) = caculate_length(str_from_takes_and_gives);
        println!("{str_ret} length {str_ret_len}");
    }

    pub fn ownership_test_handle() {
        println!("ownership test handle.");
        var_scope_study();
        string_type_owner_study();
        string_mul_to_one_var_study();
        move_function_study();
        move_function_ownership_study_2();
    }
}
