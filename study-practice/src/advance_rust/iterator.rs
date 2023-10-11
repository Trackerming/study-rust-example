/// 模拟for in的语法糖
fn example_for_in() {
    let values = vec![1, 2, 4, 8];
    {
        let result = match IntoIterator::into_iter(values) {
            mut iter => loop {
                match iter.next() {
                    Some(v) => println!("{v}"),
                    None => break,
                }
            },
        };
        result
    }
}

/// into_iter 转移所有权了
fn into_iter() {
    println!("into_iter for move ownership: ");
    let values = vec!["hello", "world", "rust"];
    for v in values
        .into_iter()
        .into_iter()
        .into_iter()
        .into_iter()
        .into_iter()
    {
        println!("{}", v);
    }
}
/// iter
fn iter() {
    println!("iter for reference: ");
    let values = vec!["hello", "world", "rust"];
    for v in values.iter() {
        println!("{}", v);
    }
    println!("values: {:?}", values);
}
/// iter_mut
fn iter_mut() {
    println!("iter for mute reference: ");
    let mut values = vec!["hello", "world", "rust"];
    // v: &mut str
    for v in values.iter_mut() {
        let string_upper = v.to_uppercase().into_boxed_str();
        // borrowed value does not live long enough
        *v = Box::leak(string_upper);
    }
    println!("values: {:?}", values);
}
fn example_iter() {
    into_iter();
    iter();
    iter_mut();
}

/// 消费者
fn consumer() {
    let values = vec![1, 2, 4, 8];
    // 拿走所有权
    let sum_values: i32 = values.iter().sum();
    println!("{sum_values}");
    // 迭代适配器，可以继续返回一个迭代器
    let v2 = values.iter().map(|x| x * 2); //.collect();
    println!("v2: {:?}", v2);
}

use std::collections::HashMap;
fn consumer_iter() {
    let keys = ["key1", "key2", "key3"];
    let values = [12, 243, 56];
    let key_value_map_zip = keys.iter().zip(values.iter());
    // [(key1, 12), (key2, 243), (key3, 56)]
    println!("key_value_map_zip: {:#?}", key_value_map_zip);
    let map: HashMap<_, _> = key_value_map_zip.collect();
    println!("map: {:?}", map);
}

fn bench_for_iter(x: &[f64]) -> f64 {
    let mut result: f64 = 0.0;
    for v in x {
        result += v;
    }
    println!("result_for: {result}");
    result
}
fn bench_iterator(x: &[f64]) -> f64 {
    let result = x.iter().sum::<f64>();
    println!("result_for: {result}");
    result
}

pub fn practice() {
    example_for_in();
    example_iter();
    consumer();
    consumer_iter();
}
extern crate rand;
extern crate test;

#[cfg(test)]
mod bench {
    use super::*;
    use rand::{thread_rng, Rng};
    use test::Bencher;

    const LEN: usize = 1024 * 1024;
    fn rand_array(len: u32) -> Vec<f64> {
        let mut rng = thread_rng();
        (0..len).map(|_| rng.gen::<f64>()).collect()
    }

    /*
    test advance_rust::iterator::bench::bench_for  ... bench:   1,341,376 ns/iter (+/- 98,987)
    test advance_rust::iterator::bench::bench_iter ... bench:   1,271,270 ns/iter (+/- 64,704)
    */
    #[bench]
    fn bench_for(b: &mut Bencher) {
        let samples = rand_array(LEN as u32);
        b.iter(|| bench_for_iter(&samples))
    }

    #[bench]
    fn bench_iter(b: &mut Bencher) {
        let samples = rand_array(LEN as u32);
        b.iter(|| bench_iterator(&samples))
    }
}
