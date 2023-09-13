fn iterator_vector(v: &Vec<i32>) {
    let v_iter = v.iter();
    for val in v_iter {
        println!("iterator_vector:: val = {val}");
    }
    // 等同于如下的测试函数test_iterator_demo
    // 测试拥有vector所有权的vector
    let mut v_iter_into = v.into_iter();
    assert_eq!(v_iter_into.next(), Some(&1));
    assert_eq!(v_iter_into.next(), Some(&2));
    assert_eq!(v_iter_into.next(), Some(&3));
    assert_eq!(v_iter_into.next(), Some(&4));
    assert_eq!(v_iter_into.next(), Some(&5));
    assert_eq!(v_iter_into.next(), Some(&6));
    assert_eq!(v_iter_into.next(), None);
}

fn iterator_mute_vector(v: &mut Vec<i32>) {
    let v_mute_iter = v.iter_mut();
    for value in v_mute_iter {
        *value = *value * 2;
    }
    println!("value_vector {:?}", v);
}

fn iterator_consume_method(v: &Vec<i32>) {
    let v_iter = v.iter();
    let total: i32 = v_iter.sum();
    assert_eq!(total, 42);
    println!("total {total}");
}

fn produce_iterator_method(v: &Vec<i32>) {
    let v2: Vec<_> = v.iter().map(|x| x + 1).collect();
    println!("produce_iterator_method vec: {:?}", v2);
}

pub fn iterator_usage() {
    let mut v1 = vec![1, 2, 3, 4, 5, 6];
    iterator_vector(&v1);
    iterator_mute_vector(&mut v1);
    println!("iterator_usage {:?}", v1);
    iterator_consume_method(&v1);
    produce_iterator_method(&v1);
}

#[derive(Debug, PartialEq)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoe_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_iterator_demo() {
        let v1 = vec![1, 2, 3];
        // 需要使 v1_iter 可变：在迭代器上调用 next 方法会更改迭代器用来跟踪其在序列中位置的内部状态。
        // 换句话说，此代码消耗或用完迭代器。每次调用 next 都会消耗迭代器中的一个项目。
        // 当我们使用 for 循环时，我们不需要使 v1_iter 可变，因为循环获取了 v1_iter 的所有权并使其在幕后可变
        let mut v1_iter = v1.iter();
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn test_shoe_in_size() {
        use super::*;
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];
        let in_my_size = shoe_in_size(shoes, 10);
        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ]
        )
    }
}
