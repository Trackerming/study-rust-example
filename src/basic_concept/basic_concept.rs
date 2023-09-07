pub mod variable {
    pub fn mutable() {
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
}
