/// 运行结果如下
/// ```txt
/// x + y = 8
/// ```
fn type_alias() {
    type Kilometers = i32;
    let x: i32 = 3;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);
}

fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {}

fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
    return Box::new(|| println!("returns_long_type"));
}

type Thunk = Box<dyn Fn() + Send + 'static>;

fn takes_long_type_short(f: Thunk) {}
fn returns_long_type_short() -> Thunk {
    return Box::new(|| println!("returns_long_type_short"));
}

fn long_type() {
    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
    takes_long_type(f);
    let _ = returns_long_type();
    let _f2 = returns_long_type_short();
}

fn never_fn() -> ! {
    println!("forever ");
    let mut i = 0;
    loop {
        i += 1;
        println!("and ever {i}");
    }
}

fn generic<T>(t: T) {}
// = 编译时大小已知
fn generic_sized<T: Sized>(t: T) {}
// 编译时大小未知的T放宽如下
fn generic_sized_2<T: ?Sized>(t: &T) {}

pub fn advanced_type_study() {
    type_alias();
    long_type();
    // never_fn();
}
