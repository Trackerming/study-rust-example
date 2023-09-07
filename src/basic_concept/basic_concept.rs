pub mod variable {
    fn mutable() {
        let mut mutable_var = 100;
        println!("before mutable var modify: {mutable_var}");
        mutable_var = 20;
        let immutable_var = 100;
        /*
        取消下行注释编译出错
        error[E0384]: cannot assign twice to immutable variable `immutable_var`
            --> src/basic_concept/basic_concept.rs:7:9
           |
         6 |         let immutable_var = 100;
           |             -------------
           |             |
           |             first assignment to `immutable_var`
           |             help: consider making this binding mutable: `mut immutable_var`
         7 |         immutable_var = 20;
           |         ^^^^^^^^^^^^^^^^^^ cannot assign twice to immutable variable
        */
        // immutable_var = 20;
        println!("mutable_var: {mutable_var} immtable_var: {immutable_var}");
    }

    fn constants_test() {
        const WEI: u64 = 1000000000000000000;
        let max64_int = 2 ^ 64 - 1;
        match WEI <= u64::MAX {
            true => println!("WEI lessEqual 2^64-1."),
            false => println!("WEI greater 2^64-1."),
        }
        println!("ethereum WEI {WEI}");
        /*
        若放开下面的注释将报错
        error: const globals cannot be mutable
           --> src/basic_concept/basic_concept.rs:33:15
           |
        33 |         const mut G_WEI:u32 = 1000000000;
           |         ----- ^^^ cannot be mutable
           |         |
           |         help: you might want to declare a static instead: `static`
           error: could not compile `study-rust-example` (bin "study-rust-example") due to previous error*/
        // const mut G_WEI:u32 = 1000000000;
    }

    fn name_scope() {
        let x: u16 = 10086;
        println!("x = {x}"); // x = 10086
        let x: u16 = 100;
        println!("x = {x}"); // x = 100
        {
            let x = x * 2;
            println!("inner scope x = {x}"); // output: inner scope x = 200
        }
        let x = "update string.";
        println!("x = {x}"); // x = update string.
    }

    fn diff_scope_mut() {
        let mut x: u16 = 10000;
        x = 10086;
        // x = 10086
        println!("x = {x}");
        /*
        若打开下一行注释
        error[E0308]: mismatched types
            --> src/basic_concept/basic_concept.rs:62:14
              |
           59 |         let mut x :u16  = 10000;
              |                    --- expected due to this type
            ...
           62 |          x = "update string.";
              |              ^^^^^^^^^^^^^^^^ expected `u16`, found `&str`
         For more information about this error, try `rustc --explain E0308`.
        */
        // x = "update string.";
        let x = "update string.";
        println!("x = {x}"); // x = update string.
    }

    pub fn test_variable() {
        mutable();
        constants_test();
        name_scope();
        diff_scope_mut();
    }
}

pub mod data_types {
    fn data_type_define() {
        /*
        如果不明确指定类型将编译告错如下
        error[E0282]: type annotations needed
          --> src/basic_concept/basic_concept.rs:90:13
           |
        90 |         let var= "32".parse().expect("string not a number.");
           |             ^^^
           |
        help: consider giving `var` an explicit type
           |
        90 |         let var: /* Type */= "32".parse().expect("string not a number.");
           |                ++++++++++++
        For more information about this error, try `rustc --explain E0282`.
        */
        let var: u16 = "32".parse().expect("string not a number.");
        println!("var {var}");
    }

    fn int_type_handle() {
        // 赋值
        let int1: u32 = 100_2400;
        let int2 = 1000_8600;
        let int3: i16 = -32;
        let int4: u8 = b'A';
        println!("int1: {int1}, int2: {int2}, int3: {int3}, int4: {int4}");
        // 算术操作
        let result1 = int1 + 32;
        let result2 = int2 * 10;
        let result3 = int3 - 32;
        let result4 = int4 / 8;
        let result5 = int4 % 8;
        println!("result1: {result1}, result2: {result2}, result3: {result3}, result4: {result4}, result5: {result5}");
    }

    fn char_type_handle() {
        let c = 'z';
        let z: char = 'ℤ';
        let heart_eyed_cat = '😻';
        println!("c {c}, z {z} heart_eyed_cat {heart_eyed_cat}");
    }

    fn scalar_type_handle() {
        int_type_handle();
        char_type_handle();
    }

    fn tuple_type_handle() {
        let tup: (u16, i32, bool, char) = (18, -32, false, 'a');
        println!("tup: ({{tup.0}}, {{tup.1}}, {{tup.2}}, {{tup.3}})"); // tup: ( {tup.0}, {tup.1}, {tup.2}, {tup.3})
        let ele = tup.1;
        println!("tup.1 {ele}");
        let (x, y, z, a) = tup;
        println!("tup x: {x}, y: {y}, z: {z}, a: {a}"); // tup x: 18, y: -32, z: false, a: a
    }

    fn array_type_handle() {
        let a0 = [0, 1, 2, 3, 4, 5, 6];
        // 定义类型为i32长度为8的数组；
        let a1: [i32; 8] = [32, 35, 35, 78, 98, 48, 34, 66];
        let a2 = [88; 10];
        for ele in a0 {
            print!("{ele} ");
        }
        println!();
        for num in (0..a1.len()) {
            let ele = a1[num];
            print!("a1[{num}]: {ele} ");
        }
        println!();
        let mut index = 0;
        // 访问数组越界时候：
        /*
        thread 'main' panicked at 'index out of bounds: the len is 10 but the index is 10', src/basic_concept/basic_concept.rs:161:23
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        */
        while index <= a2.len() {
            let ele = a2[index];
            print!("a2[{index}]: {ele} ");
            index+=1;
        }
    }

    fn compound_type_handle() {
        tuple_type_handle();
        array_type_handle();
    }

    pub fn test_data_types() {
        data_type_define();
        scalar_type_handle();
        compound_type_handle();
    }
}
