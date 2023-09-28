struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    // 完成 area 方法，返回矩形 Rectangle 的面积
    fn area
}
fn method_1() {
    let rect1 = Rectangle { width: 30, height: 50 };
    assert_eq!(rect1.area(), 1500);
}

// 只填空，不要删除任何代码行!
#[derive(Debug)]
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    pub fn show_state(__)  {
        println!("the current state is {}", __.color);
    }
}
fn method_2() {
    let light = TrafficLight{
        color: "red".to_owned(),
    };
    // 不要拿走 `light` 的所有权
    light.show_state();
    // 否则下面代码会报错
    println!("{:?}", light);
}

struct TrafficLight3 {
    color: String,
}

impl TrafficLight3 {
    // 使用 `Self` 填空
    pub fn show_state(__)  {
        println!("the current state is {}", self.color);
    }

    // 填空，不要使用 `Self` 或其变体
    pub fn change_state(__) {
        self.color = "green".to_string()
    }
}
fn method_3() {}

#[derive(Debug)]
struct TrafficLight4 {
    color: String,
}

impl TrafficLight4 {
    // 1. 实现下面的关联函数 `new`,
    // 2. 该函数返回一个 TrafficLight 实例，包含 `color` "red"
    // 3. 该函数必须使用 `Self` 作为类型，不能在签名或者函数体中使用 `TrafficLight`
    pub fn new() 

    pub fn get_state(&self) -> &str {
        &self.color
    }
}

fn method_4() {
    let light = TrafficLight::new();
    assert_eq!(light.get_state(), "red");
}

struct Rectangle5 {
    width: u32,
    height: u32,
}

// 使用多个 `impl` 语句块重写下面的代码
impl Rectangle5 {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


fn method_5() {}

#[derive(Debug)]
enum TrafficLightColor6 {
    Red,
    Yellow,
    Green,
}
// 为 TrafficLightColor 实现所需的方法
impl TrafficLightColor6 {

}

fn method_6() {
    let c = TrafficLightColor6::Yellow;
    assert_eq!(c.color(), "yellow");
    println!("{:?}",c);
}


pub fn practice() {
    println!("Method run method_1: ");
    method_1();
    println!("Method run method_2: ");
    method_2();
    println!("Method run method_3: ");
    method_3();
    println!("Method run method_4: ");
    method_4();
    println!("Method run method_5: ");
    method_5();
    println!("Method run method_6: ");
    method_6();
}
