
struct Container(i32, i32);

// 使用关联类型实现重新实现以下特征
// trait Contains {
//    type A;
//    type B;

trait Contains<A, B> {
    fn contains(&self, _: &A, _: &B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}
impl Contains<i32, i32> for Container {
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    // Grab the first number.
    fn first(&self) -> i32 { self.0 }

    // Grab the last number.
    fn last(&self) -> i32 { self.1 }
}
fn difference<A, B, C: Contains<A, B>>(container: &C) -> i32 {
    container.last() - container.first()
}
fn method_1() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!("Does container contain {} and {}: {}",
        &number_1, &number_2,
        container.contains(&number_1, &number_2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}

use std::ops::Sub;
#[derive(Debug, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}
// 用三种方法填空: 其中两种使用默认的泛型参数，另外一种不使用
impl __ {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
fn method_2() {
    assert_eq!(Point { x: 2, y: 3 } - Point { x: 1, y: 0 },
        Point { x: 1, y: 3 });

    println!("Success!")
}

trait Pilot {
    fn fly(&self) -> String;
}
trait Wizard {
    fn fly(&self) -> String;
}
struct Human;
impl Pilot for Human {
    fn fly(&self) -> String {
        String::from("This is your captain speaking.")
    }
}
impl Wizard for Human {
    fn fly(&self) -> String {
        String::from("Up!")
    }
}
impl Human {
    fn fly(&self) -> String {
        String::from("*waving arms furiously*")
    }
}
fn method_3() {
    let person = Human;

    assert_eq!(__, "This is your captain speaking.");
    assert_eq!(__, "Up!");
    assert_eq!(__, "*waving arms furiously*");

    println!("Success!")
}

trait Person {
    fn name(&self) -> String;
}
// Person 是 Student 的 supertrait .
// 实现 Student 需要同时实现 Person.
trait Student: Person {
    fn university(&self) -> String;
}
trait Programmer {
    fn fav_language(&self) -> String;
}
// CompSciStudent (computer science student) 是 Programmer 
// 和 Student 的 subtrait. 实现 CompSciStudent 需要先实现这两个 supertraits.
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}
fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}
struct CSStudent {
    name: String,
    university: String,
    fav_language: String,
    git_username: String
}
// 为 CSStudent 实现所需的特征
impl ...
fn method_4() {
    let student = CSStudent {
        name: "Sunfei".to_string(),
        university: "XXX".to_string(),
        fav_language: "Rust".to_string(),
        git_username: "sunface".to_string()
    };

    // 填空
    println!("{}", comp_sci_student_greeting(__));
}

use std::fmt;
// 定义一个 newtype `Pretty`
impl fmt::Display for Pretty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.0.clone() + ", world")
    }
}
fn main() {
    let w = Pretty("hello".to_string());
    println!("w = {}", w);
}

pub fn practice(){
    println!("AdvancedTrait practice method1:");
    method_1();
    println!("AdvancedTrait practice method2:");
    method_2();
    println!("AdvancedTrait practice method3:");
    method_3();
    println!("AdvancedTrait practice method4:");
    method_4();
    println!("AdvancedTrait practice method5:");
    method_5();
}

