// fix the error
struct Person1 {
    name: String,
    age: u8,
    hobby: String,
}
fn method_1() {
    /*
    let age = 30;
    let p = Person1 {
        name: String::from("sunface"),
        age,
    };
    */
    let age = 30;
    let p = Person1 {
        name: String::from("sunface"),
        age,
        hobby: String::from("Swimming"),
    };
}

/// 单元结构体
struct Unit;
trait SomeTrait {
    // ...定义一些行为
}

// 我们并不关心结构体中有什么数据( 字段 )，但我们关心它的行为。
// 因此这里我们使用没有任何字段的单元结构体，然后为它实现一些行为
impl SomeTrait for Unit {}
fn method_2() {
    let u = Unit;
    do_something_with_unit(u);
}

// 填空，让代码工作
fn do_something_with_unit(u: Unit) {}

/// 元组结构体
// 填空并修复错误
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
fn method_3() {
    /*
    let v = Point(__, __, __);
    check_color(v);
    */
    let v = Point(32, 24, 98);
    let color = Color(0, 127, 255);
    check_color(color);
}

fn check_color(p: Color) {
    /*
    let (x, _, _) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(__, 255);
    */
    // let (x, z) = (p.0, p.2);
    let Color(x, _, z) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(z, 255);
}

// 填空并修复错误，不要增加或移除代码行
struct Person4 {
    name: String,
    age: u8,
}
fn method_4() {
    /*
    let age = 18;
    let p = Person4 {
        name: String::from("sunface"),
        age,
    };
    // how can you believe sunface is only 18?
    p.age = 30;
    // 填空
    __ = String::from("sunfei");
    */
    let age = 18;
    let mut p = Person4 {
        name: String::from("sunface"),
        age,
    };

    // how can you believe sunface is only 18?
    p.age = 30;

    // 填空
    p.name = String::from("sunfei");
}

// 填空
struct Person5 {
    name: String,
    age: u8,
}
fn method_5() {
    let _person = build_person(String::from("zhangsan"), 18);
}

fn build_person(name: String, age: u8) -> Person5 {
    /*
    Person5 {
        age,
        __
    }
    */
    Person5 { age, name }
}

// 填空，让代码工作
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn method_6() {
    let u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    let u2 = set_email(u1);
}

fn set_email(u: User) -> User {
    /*
    User {
        email: String::from("contact@im.dev"),
        __
    }
    */
    User {
        email: String::from("contact@im.dev"),
        ..u
    }
}

// 填空，让代码工作
// #[__]
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn method_7() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // 打印 debug 信息到标准错误输出 stderr,并将 `30 * scale` 的值赋给 `width`
        height: 50,
    };

    dbg!(&rect1); // 打印 debug 信息到标准错误输出 stderr

    println!("{:#?}", rect1); // 打印 debug 信息到标准输出 stdout
}

// 修复错误
#[derive(Debug)]
struct File {
    name: String,
    data: String,
}
fn method_8() {
    let mut f = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string(),
    };

    let _name = f.name;

    // 只能修改这一行
    // println!("{}, {}, {:?}",f.name, f.data, f);
    // 换还回去，否则f就会因为丢失了name的所有权而失效
    f.name = _name;
    println!("{}, {}, {:?}", f.name, f.data, f);
}

pub fn practice() {
    println!("Struct run method_1: ");
    method_1();
    println!("Struct run method_2: ");
    method_2();
    println!("Struct run method_3: ");
    method_3();
    println!("Struct run method_4: ");
    method_4();
    println!("Struct run method_5: ");
    method_5();
    println!("Struct run method_6: ");
    method_6();
    println!("Struct run method_7: ");
    method_7();
    println!("Struct run method_8: ");
    method_8();
}
