fn combine_str() {
    let s1 = String::from("spring.");
    let s2 = String::from("summer.");
    let s3 = String::from("winter.");
    let s4 = String::from("autumn.");
    let s5 = s1 + &s2;
    let s6 = s2 + "-" + &s3 + "_" + &s4;
    println!("s5 {s5} s6 {s6}");
    let s1 = String::from("spring.");
    let s2 = String::from("summer.");
    let s3 = String::from("winter.");
    let s7 = format!("{s1}-{s2}_{s3}");
    println!("s7 {s7}");
}

fn slice_str() {
    // 2byte
    let str_val = "Здравствуйте";
    let s = &str_val[0..4];
    println!("s: {s}");
}

fn iterator_str() {
    let str_val = "Здравствуйте";
    println!("iterator_str :");
    for c in str_val.chars() {
        print!("{c} ");
    }
    println!();
    for c in str_val.bytes() {
        print!("{c} ");
    }
}

pub fn string_usage_study() {
    let mut str_val = String::new();
    // s1 和 s2之间的操作等效；
    let s1 = "hello string".to_string();
    let s2 = String::from("[String::from] hello.");
    str_val.push_str("push str hello rust.");
    str_val.push('&');
    println!("str_val {}, s1 {}, s2 {}", str_val, s1, s2);
    let s3 = s1 + &s2;
    println!("s3: {s3}");
    combine_str();
    slice_str();
    iterator_str();
}
