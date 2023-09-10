fn find_largest_i32(list: &[i32]) -> i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    return *largest;
}

fn find_largest_char(list: &[char]) -> char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    *largest
}

fn find_largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        // error[E0369]: binary operation `>` cannot be applied to type `&T`
        // 限制T的范围为实现了std::cmp::PartialOrd的类型
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T, E> {
    x: T,
    y: E,
}

impl<T, E> Point<T, E> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &E {
        &self.y
    }

    // 相当于self代表<T, E> point
    fn mixup<T1, E1>(self, other: Point<T1, E1>) -> Point<T, E1> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point<i32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.y.powi(2)).sqrt()
    }
}

fn generic_struct() {
    let mix_point = Point { x: 32, y: 32.3 };
    println!("mix point x = {}, y = {}", mix_point.x(), mix_point.y());
    let x: i32 = 32;
    let y: f32 = 32.3;
    let mix_point2 = Point { x: x, y: y };
    println!("mix point distance: {}", mix_point2.distance_from_origin());
    let point2 = Point {
        x: String::from("point2x"),
        y: 'c',
    };
    let p3 = point2.mixup(mix_point);
    println!("p3 (x:{} y{})", p3.x(), p3.y());
}

pub fn generic_type_usage_study() {
    let nums = vec![12, 34, 56, 1234, 78, 8765, 6375];
    let mut chars: Vec<char> = Vec::new();
    chars.push('v');
    chars.push('a');
    let largest_num = find_largest_i32(&nums);
    let largest_char = find_largest_char(&chars);
    println!("larest_num: {}, largest char {}", largest_num, largest_char);
    let largest_num1 = find_largest(&nums);
    let largest_char1 = find_largest(&chars);
    println!(
        "larest_num1: {}, largest char1 {}",
        largest_num1, largest_char1
    );
    generic_struct();
}
