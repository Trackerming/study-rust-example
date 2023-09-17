pub trait Draw {
    fn draw(&self);
}

// 这与定义使用带有特征边界的泛型类型参数的结构不同。
// 泛型类型参数一次只能替换为一种具体类型，而特征对象允许在运行时用多种具体类型填充特征对象
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

#[derive(Debug)]
pub struct Button {
    pub width: i32,
    pub height: i32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("draw Button: {:?}", &self);
    }
}

#[derive(Debug)]
pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    options: String,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("draw SelectBox: {:?}.", &self);
    }
}

/// 运行结果如下
/// ```txt
/// draw SelectBox: SelectBox { width: 32, height: 32, options: "select box" }.
/// draw Button: Button { width: 64, height: 64, label: "button" }
/// ```
pub fn trait_object_for_diff_type_study() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 32,
                height: 32,
                options: String::from("select box"),
            }),
            Box::new(Button {
                width: 64,
                height: 64,
                label: String::from("button"),
            }),
        ],
    };
    screen.run();
}
