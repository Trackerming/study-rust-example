// 完成两个 `impl` 语句块
// 不要修改 `main` 中的代码
trait Hello {
    fn say_hi(&self) -> String {
        String::from("hi")
    }
    fn say_something(&self) -> String;
}
struct Student {}
impl Hello for Student {
    fn say_something(&self) -> String {
        "I'm a good student".to_owned()
    }
}
struct Teacher {}
impl Hello for Teacher {
    fn say_hi(&self) -> String {
        String::from("Hi, I'm your new teacher")
    }
    fn say_something(&self) -> String {
        "I'm not a bad teacher".to_string()
    }
}
fn method_1() {
    let s = Student {};
    assert_eq!(s.say_hi(), "hi");
    assert_eq!(s.say_something(), "I'm a good student");

    let t = Teacher {};
    assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
    assert_eq!(t.say_something(), "I'm not a bad teacher");

    println!("Success!")
}

// `Centimeters`, 一个元组结构体，可以被比较大小
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);
// `Inches`, 一个元组结构体可以被打印
#[derive(Debug)]
struct Inches(i32);
impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;
        Centimeters(inches as f64 * 2.54)
    }
}
// 添加一些属性让代码工作
// 不要修改其它代码！
#[derive(Debug, PartialEq, PartialOrd)]
struct Seconds(i32);
fn method_2() {
    let _one_second = Seconds(1);

    println!("One second looks like: {:?}", _one_second);
    let _this_is_true = _one_second == _one_second;
    let _this_is_true = _one_second > _one_second;

    let foot = Inches(12);

    println!("One foot equals {:?}", foot);

    let meter = Centimeters(100.0);
    let cmp = if foot.to_centimeters() < meter {
        "smaller"
    } else {
        "bigger"
    };

    println!("One foot is {} than one meter.", cmp);
}

use std::ops::{self, Mul};
// 实现 fn multiply 方法
// 如上所述，`+` 需要 `T` 类型实现 `std::ops::Add` 特征
// 那么, `*` 运算符需要实现什么特征呢? 你可以在这里找到答案: https://doc.rust-lang.org/core/ops/
fn multiply<T>(a: T, b: T) -> T
where
    T: Mul<Output = T>,
{
    a * b
}
fn method_3() {
    assert_eq!(6, multiply(2u8, 3u8));
    assert_eq!(5.0, multiply(1.0, 5.0));
    println!("Success!")
}

// 修复错误，不要修改 `main` 中的代码!
struct Foo;
struct Bar;
#[derive(Debug, PartialEq)]
struct FooBar;
#[derive(Debug, PartialEq)]
struct BarFoo;
// 下面的代码实现了自定义类型的相加： Foo + Bar = FooBar
impl ops::Add<Bar> for Foo {
    type Output = FooBar;
    fn add(self, _rhs: Bar) -> FooBar {
        FooBar
    }
}
impl ops::Sub<Bar> for Foo {
    type Output = BarFoo;

    fn sub(self, _rhs: Bar) -> BarFoo {
        BarFoo
    }
}
fn method_4() {
    // 不要修改下面代码
    // 你需要为 FooBar 派生一些特征来让代码工作
    assert_eq!(Foo + Bar, FooBar);
    assert_eq!(Foo - Bar, BarFoo);
    println!("Success!")
}

// 实现 `fn summary`
// 修复错误且不要移除任何代码行
trait Summary {
    fn summarize(&self) -> String;
}
#[derive(Debug)]
struct Post {
    title: String,
    author: String,
    content: String,
}
impl Summary for Post {
    fn summarize(&self) -> String {
        format!("The author of post {} is {}", self.title, self.author)
    }
}
#[derive(Debug)]
struct Weibo {
    username: String,
    content: String,
}
impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{} published a weibo {}", self.username, self.content)
    }
}
fn method_5() {
    let post = Post {
        title: "Popular Rust".to_string(),
        author: "Sunface".to_string(),
        content: "Rust is awesome!".to_string(),
    };
    let weibo = Weibo {
        username: "sunface".to_string(),
        content: "Weibo seems to be worse than Tweet".to_string(),
    };

    summary(&post);
    summary(&weibo);

    println!("{:?}", post);
    println!("{:?}", weibo);
}
// 在下面实现 `fn summary` 函数
fn summary(_summary: &impl Summary) {}

struct Sheep {}
struct Cow {}
trait Animal {
    fn noise(&self) -> String;
}
impl Animal for Sheep {
    fn noise(&self) -> String {
        "baaaaah!".to_string()
    }
}
impl Animal for Cow {
    fn noise(&self) -> String {
        "moooooo!".to_string()
    }
}
// 返回一个类型，该类型实现了 Animal 特征，但是我们并不能在编译期获知具体返回了哪个类型
// 修复这里的错误，你可以使用虚假的随机，也可以使用特征对象
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}
fn method_6() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!(
        "You've randomly chosen an animal, and it says {}",
        animal.noise()
    );
}

fn method_7() {
    assert_eq!(sum(1, 2), 3);
}
// 通过两种方法使用特征约束来实现 `fn sum`
fn sum<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
    x + y
}
// 修复代码中的错误
struct Pair<T> {
    x: T,
    y: T,
}
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
impl<T: std::fmt::Debug + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {:?}", self.x);
        } else {
            println!("The largest member is y = {:?}", self.y);
        }
    }
}
#[derive(Debug, PartialEq, PartialOrd)]
struct Unit(i32);
fn method_8() {
    let pair = Pair {
        x: Unit(1),
        y: Unit(3),
    };
    pair.cmp_display();
}

// 填空
fn example1() {
    // `T: Trait` 是最常使用的方式
    // `T: Fn(u32) -> u32` 说明 `T` 只能接收闭包类型的参数
    struct Cacher<T: Fn(u32) -> u32> {
        calculation: T,
        value: Option<u32>,
    }
    impl<T: Fn(u32) -> u32> Cacher<T> {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }
        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }
    let mut cacher = Cacher::new(|x| x + 1);
    assert_eq!(cacher.value(10), 11);
    assert_eq!(cacher.value(15), 11);
}
fn example2() {
    // 还可以使用 `where` 来约束 T
    struct Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        calculation: T,
        value: Option<u32>,
    }
    impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }
        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }
    let mut cacher = Cacher::new(|x| x + 1);
    assert_eq!(cacher.value(20), 21);
    assert_eq!(cacher.value(25), 21);
}
fn method_9() {
    example1();
    example2();
    println!("Success!")
}

pub fn practice() {
    println!("Traits practice method1:");
    method_1();
    println!("Traits practice method2:");
    method_2();
    println!("Traits practice method3:");
    method_3();
    println!("Traits practice method4:");
    method_4();
    println!("Traits practice method5:");
    method_5();
    println!("Traits practice method6:");
    method_6();
    println!("Traits practice method7:");
    method_7();
    println!("Traits practice method8:");
    method_8();
    println!("Traits practice method9:");
    method_9();
}
