use std::ops::Drop;
struct XY {
    x: i32,
    y: i32,
}

struct Point {
    xy: XY,
    ecc: String,
}

impl Drop for XY {
    fn drop(&mut self) {
        println!("dropping XY:({}, {})", self.x, self.y);
    }
}

impl Drop for Point {
    fn drop(&mut self) {
        println!(
            "dropping point ecc of {} with XY:({}, {})",
            self.ecc, self.xy.x, self.xy.y
        );
    }
}

/// 运行结果如下，这里的顺序与书本内容不符合？
/// dropping point ecc of ecdsa with XY:(1, 1)
/// dropping XY:(1, 1)
/// dropping point ecc of ecdsa with XY:(2, -1)
/// dropping XY:(2, -1)
/// 将变量名从_换成pointer1和pointer2结果与书本顺序介绍相同如下
/// ```txt
/// dropping point ecc of ecdsa with XY:(2, -1)
/// dropping XY:(2, -1)
/// dropping point ecc of ecdsa with XY:(1, 1)
/// dropping XY:(1, 1)
/// ```
/// 原因可能是编译器做的，当用_意味着后续不会使用提前释放了内存？
fn drop_struct_test() {
    let pointer1 = Point {
        xy: XY { x: 1, y: 1 },
        ecc: String::from("ecdsa"),
    };
    let pointer2 = Point {
        xy: XY { x: 2, y: -1 },
        ecc: String::from("ecdsa"),
    };
    // error[E0040]: explicit use of destructor method
    // pointer1.drop();
}

/// 运行结果如下
/// ```txt
/// [early_drop_with_stddrop_test] pointer crated.
/// dropping point ecc of ecdsa with XY:(1, 1)
/// dropping XY:(1, 1)
/// [early_drop_with_stddrop_test] pointer before the end of fn
/// ```
fn early_drop_with_stddrop_test() {
    let pointer = Point {
        xy: XY { x: 1, y: 1 },
        ecc: String::from("ecdsa"),
    };
    println!("[early_drop_with_stddrop_test] pointer crated.");
    std::mem::drop(pointer);
    println!("[early_drop_with_stddrop_test] pointer before the end of fn");
}

pub fn drop_trait_study() {
    drop_struct_test(); // 结束函数的scope的时候，调用对应的drop
    early_drop_with_stddrop_test();
}
