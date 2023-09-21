#[macro_export] // 指示每当定义该宏的包进入作用域时该宏就可以用，没有这个宏就无法进入作用域
macro_rules! vecs {
    /* $x:expr 匹配任何rust表达式并为表达式指定名称$x  `*`表示该模式与*之前的任何内容匹配0个或者多个 */
    ( $( $x:expr ),* ) => { // 模式的arm，待匹配, $()后面的逗号表示文字逗号可以选择出现与$()匹配的代码之后
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
                )*
            temp_vec
        }
    };
}

/*use proc_macro;
#[some_attribute]
pub fn some_name(input:TokenStream) -> TokenStream{}*/
pub trait HelloMacro {
    fn hello_macro();
}

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello macro! My name is Pancakes.");
    }
}

use hello_macro_derive::*;

#[derive(HelloMacro2)]
struct Pancakes2;

/// 运行结果如下
/// ```txt
/// [12, 34, 56, 98]
/// Hello macro! My name is Pancakes.
/// Hello, Macro! My name is Pancakes2!
/// ```
pub fn macros_study() {
    let vec_val = vecs![12, 34, 56, 98];
    println!("{:?}", vec_val);
    Pancakes::hello_macro();
    Pancakes2::hello_macro();
}
