use core::slice;

/// 运行结果如下
/// ```txt
/// r1 is 10
/// r2 is 10
/// thread 'main' panicked at 'misaligned pointer dereference: address must be a multiple of 0x4 but is 0x12345', study-example/bin/advance_feature/unsafe_rust.rs:11:9
/// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
/// ```
fn define_raw_pointer() {
    let mut num = 10;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    let address = 0x012345usize;
    let r = address as *const i32;

    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
        // println!("r is {}", *r);
    }
}

/// 如果不在unsafe代码块执行则会编译时出现如下错误
/// error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
///  --> study-example/bin/advance_feature/unsafe_rust.rs:27:9
///    |
/// 27 |         dangerous();
///    |         ^^^^^^^^^^^ call to unsafe function
unsafe fn dangerous() {}

/// error[E0499]: cannot borrow `*values` as mutable more than once at a time
/*fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    assert!(mid <= len);
    (&mut values[..mid], &mut values[mid..])
}*/

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();
    assert!(mid <= len);
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

/// 运行结果如下
/// ```txt
/// a: [1, 2, 3]
/// b: [4, 5, 6]
/// ```
fn create_safe_abstract_over_unsafe_code() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    // split_at_mut中调用了不安全的函数from_raw_parts_mut，但是assert!(mid <= len);保证了范围说明指针式安全的
    let (a, b) = split_at_mut(r, 3);
    println!("a: {:?}", a);
    println!("b: {:?}", b);
}

// 调用C的对应函数签名的函数，C是比较常见的ABI
extern "C" {
    fn abs(input: i32) -> i32;
}

// 将 call_from_c 函数编译为共享库并从 C 链接后，从 C 代码访问该函数：
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just call a Rust function from C!");
}

static HELLO_RUST: &str = "Hello, Rust!";
static mut COUNTER: u32 = 0;
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

unsafe trait Foo {
    // method signature
}

unsafe impl Foo for i32 {}

/// 运行结果如下
/// ```txt
/// Absolute value of -3 according to C: 3
/// COUNTER: 3
/// global static string: Hello, Rust!
///
pub fn unsafe_rust_study() {
    define_raw_pointer();
    add_to_count(3);
    unsafe {
        dangerous();
        println!("Absolute value of -3 according to C: {}", abs(-3));
        println!("COUNTER: {}", COUNTER);
    }
    create_safe_abstract_over_unsafe_code();
    println!("global static string: {}", HELLO_RUST);
}
