fn create_vector() -> Vec<i32> {
    // 创建，在这里添加了i32类型注释。因为没有向这个向量插入任何值，Rust 不知道我们打算存储什么类型的元素
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 23, 456];
    println!("v1 {:?}", v1);
    println!("v2 {:?}", v2);
    return v1;
}

fn update_vector_get_first_ele(v: &mut Vec<i32>, value: i32) -> Option<&i32> {
    v.push(value);
    v.push(value + 2);
    v.push(value * 3);
    return v.get(0);
}

fn get_ele_from_index_get_method(v: &Vec<i32>, index: usize) -> &i32 {
    let value = v.get(index);
    match value {
        None => {
            println!("index ele is null.");
            return &(-1);
        }
        Some(value) => {
            return value;
        }
    }
}

fn get_ele_from_index(v: &Vec<i32>, index: usize) -> i32 {
    let value = v[index];
    return value;
}

fn drop_vec_end_ele(v: &mut Vec<i32>) {
    v.pop();
}

fn iterator_vector_twice(v: &mut Vec<i32>) {
    println!("iterator_vector_twice for ... in:");
    for val_ref in v {
        *val_ref *= 2;
    }
}

fn iterator_vector(v: &Vec<i32>) {
    println!("iterator_vector for ... in:");
    for val in v {
        print!("{} ", val);
    }
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn enum_store_mul_type_vector() {
    let row = vec![
        SpreadsheetCell::Int(23),
        SpreadsheetCell::Float(3.12),
        SpreadsheetCell::Text(String::from("hello vector.")),
    ];
    for val in row {
        println!("{:?}", val);
    }
}

pub fn vector_usage_study() {
    let mut vec1: Vec<i32> = Vec::new();
    vec1 = create_vector();
    let values: [i32; 3] = [12, 23, 43];
    {
        let first_value = update_vector_get_first_ele(&mut vec1, 32);
        println!("first value: {:?}", first_value);
    }
    println!("vec1 {:?}", vec1);
    let ele = get_ele_from_index_get_method(&vec1, 0);
    println!("vec1 {:?}, ele: {}", vec1, ele);
    let ele2 = get_ele_from_index(&vec1, 0);
    println!("vec1 {:?}, ele2: {}", vec1, ele2);
    {
        iterator_vector_twice(&mut vec1);
        println!("vec1 twice: {:?}", vec1);
    }
    iterator_vector(&vec1);
    enum_store_mul_type_vector();
    {
        drop_vec_end_ele(&mut vec1);
        println!("vec1 drop: {:?}", vec1);
    }
}
