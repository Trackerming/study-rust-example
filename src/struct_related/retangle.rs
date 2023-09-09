// {:?}pretty-print打印时候没有实现`Rectangle` doesn't implement `Debug`
#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    // move了所有权，调用之后，后续该实例就没法使用；
    fn get_width(self) -> i32 {
        self.width
    }

    fn modify_width(&mut self, new_width: i32) {
        self.width = new_width;
    }

    fn valid_width(self: &Self) -> bool {
        self.width > 0
    }

    // Associated function，类似于静态方法？
    fn square(size: i32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

pub fn retangle_struct_study() {
    let mut rect = Rectangle {
        width: 12,
        height: 8,
    };
    let area_val = rect.area();
    // 直接打印没有实现标准的std::fmt::Display
    println!("area val {}, rect {:?}", area_val, rect);
    if rect.valid_width() {
        println!("width valid.");
    }
    rect.modify_width(-32);
    println!("area after modify val {}, rect {:?}", rect.area(), rect);
    let square_val = Rectangle::square(10);
    println!("square: {:?}", square_val);
    println!("rect width {}", rect.get_width());
    // error[E0382]: borrow of moved value: `rect`
    // rect.area();
}
