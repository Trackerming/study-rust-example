use std::time::Duration;

#[derive(Debug, Clone, Copy)]
enum ShirtColor {
    Blue,
    Red,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // Option<T> 上的 unwrap_or_else 方法由标准库定义
        // 它需要一个参数：一个不带任何参数的闭包，返回一个值 T （与存储在 Option<T> 的 Some 变体中的类型相同
        // 如果 Option<T> 是 Some 变体，则 unwrap_or_else 返回 Some 中的值。如果 Option<T> 是 None 变体，则 unwrap_or_else 调用闭包并返回闭包返回的值
        // 闭包捕获对 self Inventory 实例的不可变引用，并将其与我们指定的代码一起传递给 unwrap_or_else 方法
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        for color in &self.shirts {
            match color {
                ShirtColor::Blue => num_blue += 1,
                ShirtColor::Red => num_red += 1,
            }
        }
        if num_blue > num_red {
            ShirtColor::Blue
        } else {
            ShirtColor::Red
        }
    }
}

fn inventory_usage() {
    // 初始化库存
    let store = Inventory {
        shirts: vec![
            ShirtColor::Blue,
            ShirtColor::Red,
            ShirtColor::Blue,
            ShirtColor::Red,
            ShirtColor::Blue,
        ],
    };
    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );
    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}

fn example_closures_study() {
    let example_closures = |x| x;
    let s = example_closures(String::from("closures."));
    // error[E0308]: mismatched types， 前面的调用使得推断了上面x的类型为String
    // let n = example_closures(5);
}

fn capture_immut_reference() {
    let list = vec![2, 433, 23];
    println!("capture_immut_reference Before define closure {:?}", list);
    let only_borrows = || println!("From closures: {:?}", list);
    println!(
        "capture_immut_reference before calling only_borrows closures: {:?}",
        list
    );
    only_borrows();
    println!(
        "capture_immut_reference after calling only_borrows closures: {:?}",
        list
    );
}

fn capture_mut_reference() {
    let mut list = vec![2, 433, 23];
    println!("capture_mut_reference Before define closure {:?}", list);
    let mut only_borrows = || list.push(32);
    // 可变借用存在的时候不能进行不可变的借用
    // println!("capture_immut_reference before calling only_borrows closures: {:?}", list);
    only_borrows();
    println!(
        "capture_mut_reference after calling only_borrows closures: {:?}",
        list
    );
}

fn move_ownership_to_closuer() {
    let list = vec![12, 43, 56];
    println!(
        "move_ownership_to_closuer before define closure, {:?}",
        list
    );
    // 不添加move的话：error[E0373]: closure may outlive the current function, but it borrows `list`, which is owned by the current function
    // 新线程可能会在主线程的其余部分完成之前完成，或者主线程可能会先完成。如果主线程保持 list 的所有权，但在新线程结束并删除 list 之前结束，则线程中的不可变引用将无效。
    // 因此，编译器要求将 list 移至新线程的闭包中，以便引用有效
    std::thread::spawn(move || println!("move_ownership_to_closuer from thread: {:?}", list))
        .join()
        .unwrap();
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn fn_trait_closures() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];
    list.sort_by_key(|r| r.width);
    /*let mut sort_operations = vec![];
    let value = String::from("by key called.");
    list.sort_by_key(
        |r|  {
            // error[E0507]: cannot move out of `value`, a captured variable in an `FnMut` closure，解决方式添加引用
            // 但是为什么输出4次字符串“by key called”？排序的次数？
            sort_operations.push(value);
            r.width
        }
    );
     println!("sort operations: {:?}", sort_operations);
    */
    let mut num = 0;
    list.sort_by_key(|r| {
        num += 1;
        r.width
    });
    println!("num: {num}");
    println!("{:#?}", list);
}

pub fn closures_related_usage() {
    inventory_usage();
    let expensive_closures = |num: u32| -> u32 {
        println!("caculating slowly...");
        std::thread::sleep(Duration::from_secs(2));
        return num;
    };
    let result = expensive_closures(32);
    println!("expensive_closures result {result}");
    capture_immut_reference();
    capture_mut_reference();
    move_ownership_to_closuer();
    fn_trait_closures();
}
