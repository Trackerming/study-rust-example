// 使用嵌套的结构使用具有同一父模块的模块
use std::io::{self, Write};
// 纳入路径中定义的所有的公共的结构
use std::collections::*;

pub mod package_crate_path {
    mod front_of_house {
        mod hosting {
            fn add_to_whitelist() {
                println!("add_to_whitelist");
            }
            fn seat_at_table() {
                println!("seat_at_table");
            }
            pub fn hosting_external() {
                add_to_whitelist();
                seat_at_table();
            }
        }
        mod serving {
            fn take_order() {
                println!("take_order");
            }
            fn serve_order() {
                println!("serve_order");
            }
            fn take_payment() {
                println!("take_payment");
            }
            pub fn serving_external() {
                take_order();
                serve_order();
                take_payment();
            }
        }
        pub fn to_external() {
            // relative path
            hosting::hosting_external();
            serving::serving_external();
        }
    }
    pub fn front_of_house_root() {
        // absolute path
        crate::package_crate_module::package_crate_module::package_crate_path::front_of_house::to_external();
        // relative path
        front_of_house::to_external();
        // super到上一级的父模块
        super::package_crate_path::front_of_house::to_external();
    }
}
