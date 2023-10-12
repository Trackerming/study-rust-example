use std::rc::Rc;
use std::sync::Arc;

fn mem_convert() {
    // i32 占4Bytes
    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address = p1 as usize;
    let second_address = first_address + 4;
    let p2 = second_address as *mut i32;
    unsafe {
        *p2 *= 10;
    }
    assert_eq!(values[1], 20);
    println!("values: {:?}", values);
}

// use std::convert::TryInto;
fn try_into_example() {
    let a: u8 = 10;
    let b: u16 = 1500;
    let b_: u8 = match b.try_into() {
        Ok(b_) => b_,
        Err(msg) => {
            println!("error:{:?}", msg.to_string());
            0
        }
    };
    if a < b_ {
        println!("Ten is less than {b_} ");
    }
}

/*
1. 首先，编译器检查它是否可以直接调用 T::foo(value)，称之为值方法调用
2. 如果上一步调用无法完成(例如方法类型错误或者特征没有针对 Self 进行实现，上文提到过特征不能进行强制转换)，那么编译器会尝试增加自动引用，例如会尝试以下调用： <&T>::foo(value) 和 <&mut T>::foo(value)，称之为引用方法调用
3. 若上面两个方法依然不工作，编译器会试着解引用 T ，然后再进行尝试。这里使用了 Deref 特征 —— 若 T: Deref<Target = U> (T 可以被解引用为 U)，那么编译器会使用 U 类型进行尝试，称之为解引用方法调用
4. 若 T 不能被解引用，且 T 是一个定长类型(在编译期类型长度是已知的)，那么编译器也会尝试将 T 从定长类型转为不定长类型，例如将 [i32; 2] 转为 [i32]
*/
fn example_find_func_call() {
    let array: Rc<Box<[i32; 3]>> = Rc::new(Box::new([32, 45, 87]));
    /*
    1. 首先， array[0] 只是Index特征的语法糖：编译器会将 array[0] 转换为 array.index(0) 调用，当然在调用之前，编译器会先检查 array 是否实现了 Index 特征。
    2. 接着，编译器检查 Rc<Box<[T; 3]>> 是否有实现 Index 特征，结果是否，不仅如此，&Rc<Box<[T; 3]>> 与 &mut Rc<Box<[T; 3]>> 也没有实现。
    3. 上面的都不能工作，编译器开始对 Rc<Box<[T; 3]>> 进行解引用，把它转变成 Box<[T; 3]>
        此时继续对 Box<[T; 3]> 进行上面的操作 ：Box<[T; 3]>， &Box<[T; 3]>，和 &mut Box<[T; 3]> 都没有实现 Index 特征，
    4. 所以编译器开始对 Box<[T; 3]> 进行解引用，然后我们得到了 [T; 3]
        [T; 3] 以及它的各种引用都没有实现 Index 索引(是不是很反直觉:D，在直觉中，数组都可以通过索引访问，实际上只有数组切片才可以!)，它也不能再进行解引用，
    5. 因此编译器只能祭出最后的大杀器：将定长转为不定长，因此 [T; 3] 被转换成 [T]，也就是数组切片，它实现了 Index 特征，因此最终我们可以通过 index 方法访问到对应的元素
    */
    println!("array[0] = {}", array[0]);
}

#[derive(Clone)]
struct Container<T>(Arc<T>);
fn clone_containers<T>(foo: &Container<i32>, bar: &Container<T>) {
    let foo_container = foo.clone();
    let bar_container = bar.clone();
}

fn foo() -> i32 {
    0
}
fn unsafe_func_pointer_example() {
    let pointer = foo as *const ();
    let function = unsafe {
        // 裸指针转换为函数指针
        std::mem::transmute::<*const (), fn() -> i32>(pointer)
    };
    let result = function();
    println!("{result}");
}

pub fn practice() {
    mem_convert();
    try_into_example();
    example_find_func_call();
    unsafe_func_pointer_example();
}
