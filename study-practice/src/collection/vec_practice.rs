fn method_1() {
    let arr: [u8; 3] = [1, 2, 3];
    let v = Vec::from(arr);
    is_vec(&v);
    let v = vec![1, 2, 3];
    is_vec(&v);
    // vec!(..) 和 vec![..] 是同样的宏，宏可以使用 []、()、{}三种形式，因此...
    let v = vec![1, 2, 3];
    is_vec(&v);
    // ...在下面的代码中, v 是 Vec<[u8; 3]> , 而不是 Vec<u8>
    // 使用 Vec::new 和 `for` 来重写下面这段代码
    // let v1 = vec!(arr);
    let mut v1 = Vec::new();
    for ele in arr {
        v1.push(ele);
    }
    is_vec(&v1);
    assert_eq!(v, v1);
    println!("Success!")
}
fn is_vec(v: &Vec<u8>) {}

// 填空
fn method_2() {
    let mut v1 = Vec::from([1, 2, 4]);
    v1.pop();
    v1.push(3);

    let mut v2 = Vec::new();
    v2.extend_from_slice(&v1[..]);

    assert_eq!(v1, v2);

    println!("Success!")
}

// 填空
fn method_3() {
    // array -> Vec
    // impl From<[T; N]> for Vec
    let arr = [1, 2, 3];
    let v1 = Vec::from(arr);
    let v2: Vec<i32> = arr.to_vec();

    assert_eq!(v1, v2);

    // String -> Vec
    // impl From<String> for Vec
    let s = "hello".to_string();
    let v1: Vec<u8> = s.into();

    let s = "hello".to_string();
    let v2 = s.into_bytes();
    assert_eq!(v1, v2);

    // impl<'_> From<&'_ str> for Vec
    let s = "hello";
    let v3 = Vec::from(s);
    assert_eq!(v2, v3);

    // 迭代器 Iterators 可以通过 collect 变成 Vec
    let v4: Vec<i32> = [0; 10].into_iter().collect();
    assert_eq!(v4, vec![0; 10]);

    println!("Success!")
}

// 修复错误并实现缺失的代码
fn method_4() {
    let mut v = Vec::from([1, 2, 3]);
    for i in 0..5 {
        println!("{:?}", v.get(i));
    }

    for i in 0..5 {
        // 实现这里的代码...
        if let Some(value) = v.get(i) {
            v[i] = value + 1;
        } else {
            v.push(i + 2);
        }
    }

    assert_eq!(v, vec![2, 3, 4, 5, 6]);

    println!("Success!")
}

// 修复错误
fn method_5() {
    let mut v = vec![1, 2, 3];

    let slice1 = &v[..];
    // 越界访问将导致 panic.
    // 修改时必须使用 `v.len`
    let slice2 = &v[0..v.len()];

    assert_eq!(slice1, slice2);

    // 切片是只读的
    // 注意：切片和 `&Vec` 是不同的类型，后者仅仅是 `Vec` 的引用，并可以通过解引用直接获取 `Vec`
    let vec_ref: &mut Vec<i32> = &mut v;
    (*vec_ref).push(4);
    let slice3 = &mut v[0..4];

    assert_eq!(slice3, &[1, 2, 3, 4]);

    println!("Success!")
}

// 修复错误
fn method_6() {
    let mut vec = Vec::with_capacity(10);

    assert_eq!(vec.len(), 0);
    assert_eq!(vec.capacity(), 10);

    // 由于提前设置了足够的容量，这里的循环不会造成任何内存分配...
    for i in 0..10 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 10);
    assert_eq!(vec.capacity(), 10);

    // ...但是下面的代码会造成新的内存分配
    vec.push(11);
    assert_eq!(vec.len(), 11);
    assert!(vec.capacity() >= 11);

    // 填写一个合适的值，在 `for` 循环运行的过程中，不会造成任何内存分配
    let mut vec = Vec::with_capacity(128);
    for i in 0..100 {
        vec.push(i);
    }

    assert_eq!(vec.len(), 100);
    assert_eq!(vec.capacity(), 128);

    println!("Success!")
}

#[derive(Debug, PartialEq)]
enum IpAddr {
    V4(String),
    V6(String),
}
fn method_7() {
    // 填空
    let v: Vec<IpAddr> = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string()),
    ];

    // 枚举的比较需要派生 PartialEq 特征
    assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
    assert_eq!(v[1], IpAddr::V6("::1".to_string()));

    println!("Success!")
}

trait IpAddr8 {
    fn display(&self);
}
struct V4(String);
impl IpAddr8 for V4 {
    fn display(&self) {
        println!("ipv4: {:?}", self.0)
    }
}
struct V6(String);
impl IpAddr8 for V6 {
    fn display(&self) {
        println!("ipv6: {:?}", self.0)
    }
}

fn method_8() {
    // 填空
    let v: Vec<Box<dyn IpAddr8>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];
    for ip in v {
        ip.display();
    }
}

pub fn practice() {
    println!("Collection Vector practice method1:");
    method_1();
    println!("Collection Vector practice method2:");
    method_2();
    println!("Collection Vector practice method3:");
    method_3();
    println!("Collection Vector practice method4:");
    method_4();
    println!("Collection Vector practice method5:");
    method_5();
    println!("Collection Vector practice method6:");
    method_6();
    println!("Collection Vector practice method7:");
    method_7();
    println!("Collection Vector practice method8:");
    method_8();
}
