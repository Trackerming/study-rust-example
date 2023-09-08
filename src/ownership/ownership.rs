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
    fn caculate_length(str: String) -> (String, usize) {
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

pub mod reference_borrow {

    fn caculate_length(str_ref: &String) -> usize {
        str_ref.len()
    }
    // 如果引用的数据要更新如何处理
    /*
     error[E0596]: cannot borrow `*str_ref` as mutable, as it is behind a `&` reference
    --> src/ownership/ownership.rs:115:9
     |
     115 |         str_ref.push_str(" modify str");
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `str_ref` is a `&` reference, so the data it refers to cannot be borrowed as mutable
     */
    fn modify_str(str_ref: &String) {
        // str_ref.push_str(" modify str");
    }
    // 上述方法解决如下
    fn modify_str_correct(str_ref: &mut String) {
        str_ref.push_str(" modify str");
    }
    fn reference_study() {
        let mut str = String::from("reference_borrow->reference_study hello");
        let len = caculate_length(&str);
        // str 离开了caculate_length的scope，但是str依然能使用；
        println!("str after call: {str}, len = {len}");
        modify_str_correct(&mut str);
        println!("str after modify_str_correct(str_ref: &mut String): {str}");
    }
    fn reference_mutable_study() {
        let mut str = String::from("reference_borrow->reference_mutable_study hello");
        let ref1 = &mut str;
        /*
          error[E0499]: cannot borrow `str` as mutable more than once at a time
            --> src/ownership/ownership.rs:139:20
            |
        138 |         let ref1 = &mut str;
            |                    -------- first mutable borrow occurs here
        139 |         let ref2 = &mut str;
            |                    ^^^^^^^^ second mutable borrow occurs here
          */
        // let ref2 = &mut str;
        println!("ref1 {ref1}"); // ref1 reference_borrow->reference_mutable_study hello
                                 // println!("ref2 {ref2}");
                                 // 解决办法如下
        {
            let ref2 = &mut str;
            println!("ref2 {ref2}");
        }
    }
    fn reference_mutable_immutable_study() {
        let mut str = String::from("reference_borrow->reference_mutable_study hello");
        let ref1 = &str;
        let ref2 = &str;
        // error[E0502]: cannot borrow `str` as mutable because it is also borrowed as immutable
        // let ref3 = &mut str;
        println!("ref1: {}, ref2: {}", ref1, ref2);
        // 引用的范围从引入它的地方开始，一直持续到上次使用该引用时为止，所以因为前面是最后一次使用两个不变引用，这里可以正常定义可变引用；
        let ref3 = &mut str;
        println!("ref3: {ref3}");
    }

    /*
    error[E0106]: missing lifetime specifier
      --> src/ownership/ownership.rs:169:36
        |
    169 |     fn dangle_reference_study() -> &String {
        |                                    ^ expected named lifetime parameter
    */
    /*fn dangle_reference_study() -> &String { // returns a reference to a String
        let str = String::from("dangle hello."); // 创建一个字符串
        &str 返回字符串的引用
    }*/ //离开scope，drop了，内存释放了，引用就dangle了；

    pub fn reference_borrow_study() {
        reference_study();
        reference_mutable_study();
        reference_mutable_immutable_study();
    }
}
