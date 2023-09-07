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
