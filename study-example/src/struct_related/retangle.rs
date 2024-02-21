//! **{:?}pretty-print打印时候没有实现`Rectangle` doesn't implement `Debug`**
//! 定义矩形结构体，具有高和宽的属性
//
#[derive(Debug)]
pub struct Rectangle {
    pub width: i32,
    pub height: i32,
}

impl Rectangle {
    /// 计算矩形的面积
    /// - 示例如下
    /// ```
    ///     use study_example::retangle::Rectangle;
    ///     let mut rect = Rectangle {
    ///         width: 12,
    ///         height: 8,
    ///     };
    ///     let area_val = rect.area();
    /// ```
    pub fn area(&self) -> i32 {
        self.width * self.height
    }

    /// 获取矩形的宽
    /// **move了所有权，调用之后，后续该实例就没法使用；**
    /// - 示例如下
    /// ```
    ///     use study_example::retangle::Rectangle;
    ///     let mut rect = Rectangle {
    ///         width: 12,
    ///         height: 8,
    ///     };
    ///     let width = rect.get_width();
    /// ```
    pub fn get_width(self) -> i32 {
        self.width
    }

    /// 更新矩形的宽
    /// - 示例如下
    /// ```
    ///     use study_example::retangle::Rectangle;
    ///     let mut rect = Rectangle {
    ///         width: 12,
    ///         height: 8,
    ///     };
    ///     rect.modify_width(-32);
    ///
    pub fn modify_width(&mut self, new_width: i32) {
        self.width = new_width;
    }

    /// 判断矩形的宽是否合法
    /// - 示例如下
    /// ```
    ///     use study_example::retangle::Rectangle;
    ///     let mut rect = Rectangle {
    ///         width: 12,
    ///         height: 8,
    ///     };
    ///     if rect.valid_width() {
    ///         println!("width valid.");
    ///     }
    ///
    pub fn valid_width(self: &Self) -> bool {
        self.width > 0
    }

    /// 生成正方形
    /// - 示例如下
    /// ```
    /// use study_example::retangle::Rectangle;
    /// let square_val = Rectangle::square(10);
    /// println!("square: {:?}", square_val);
    // Associated function，类似于静态方法？
    pub fn square(size: i32) -> Self {
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
