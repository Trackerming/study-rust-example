/// for while loop

// 填空
fn method_1() {
    /*
    let n = 5;
    if n < 0 {
        println!("{} is negative", n);
    } __ n > 0 {
        println!("{} is positive", n);
    } __ {
        println!("{} is zero", n);
    }
    */
    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }
}

// 修复错误
fn method_2() {
    /*
    let n = 5;
    let big_n =
        if n < 10 && n > -10 {
            println!(" 数字太小，先增加 10 倍再说");
            10 * n
        } else {
            println!("数字太大，我们得让它减半");
            n / 2.0 ;
        }

    println!("{} -> {}", n, big_n);
    */
    let n = 5;
    let big_n = if n < 10 && n > -10 {
        println!(" 数字太小，先增加 10 倍再说");
        10 * n
    } else {
        println!("数字太大，我们得让它减半");
        n / 2 as i32
    };
    println!("{} -> {}", n, big_n);
}

fn method_3() {
    // for n in 1..=100 { // 修改此行，让代码工作
    for n in 1..100 {
        // 修改此行，让代码工作
        if n == 100 {
            panic!("NEVER LET THIS RUN")
        }
    }
}

// 修复错误，不要新增或删除代码行
fn method_4() {
    /*
    let names = [String::from("liming"),String::from("hanmeimei")];
    for name in names {
        // do something with name...
    }
    println!("{:?}", names);
    let numbers = [1, 2, 3];
    // numbers中的元素实现了 Copy，因此无需转移所有权
    for n in numbers {
        // do something with name...
    }
    println!("{:?}", numbers);
    */
    let names = [String::from("liming"), String::from("hanmeimei")];
    for name in &names {
        // do something with name...
    }

    println!("{:?}", names);

    let numbers = [1, 2, 3];
    // numbers中的元素实现了 Copy，因此无需转移所有权
    for n in numbers {
        // do something with name...
    }

    println!("{:?}", numbers);
}

fn method_5() {
    /*
    let a = [4,3,2,1];
    // 通过索引和值的方式迭代数组 `a`
    for (i,v) in a.__ {
        println!("第{}个元素是{}",i+1,v);
    }
    */
    let a = [4, 3, 2, 1];
    // 通过索引和值的方式迭代数组 `a`
    for (i, v) in a.iter().enumerate() {
        println!("第{}个元素是{}", i + 1, v);
    }
}

// 填空，让最后一行的  println! 工作 !
fn method_6() {
    /*
    // 当条件为真时，不停的循环
    while n __ 10 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        __;
    }
    */
    // 一个计数值
    let mut n = 1;

    // 当条件为真时，不停的循环
    while n < 10 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        n += 1;
    }

    println!("n 的值是 {}, 循环结束", n);
}

// 填空，不要修改其它代码
fn method_7() {
    /*
    let mut n = 0;
    for i in 0..=100 {
        if n == 66 {
            __
        }
        n += 1;
    }
    assert_eq!(n, 66);
    */
    let mut n = 0;
    for i in 0..=100 {
        if n == 66 {
            break;
        }
        n += 1;
    }
    assert_eq!(n, 66);
}

// 填空，不要修改其它代码
fn method_8() {
    /*
    let mut n = 0;
    for i in 0..=100 {
        if n != 66 {
            n+=1;
            __;
        }
        __
    }
    assert_eq!(n, 66);
    */
    let mut n = 0;
    for i in 0..=100 {
        if n != 66 {
            n += 1;
            continue;
        }
        break;
    }
    assert_eq!(n, 66);
}

// 填空，不要修改其它代码
fn method_9() {
    /*
    let mut count = 0u32;
    println!("Let's count until infinity!");
    // 无限循环
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            // 跳过当此循环的剩余代码
            __;
        }
        println!("{}", count);
        if count == 5 {
            println!("OK, that's enough");
            __;
        }
    }
    assert_eq!(count, 5);
    */
    let mut count = 0u32;
    println!("Let's count until infinity!");
    // 无限循环
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            // 跳过当此循环的剩余代码
            continue;
        }
        println!("{}", count);
        if count == 5 {
            println!("OK, that's enough");
            break;
        }
    }
    assert_eq!(count, 5);
}

// 填空
fn method_10() {
    /*
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            __;
        }
    };
    assert_eq!(result, 20);
    */
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break 20;
        }
    };

    assert_eq!(result, 20);
}

// 填空
fn method_11() {
    let mut count = 0;
    let mut count_inner1 = 0;
    let mut count_outer = 0;
    let mut count_inner2 = 0;
    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                // 这只会跳出 inner1 循环
                break 'inner1; // 这里使用 `break` 也是一样的
            }
            count_inner1 += 1;
            println!("current 'inner: count_inner1 = {count_inner1}");
            count += 2;
        }

        count += 5;
        count_outer += 1;
        println!("current 'outer: count_outer = {count_outer}");

        'inner2: loop {
            if count >= 30 {
                break 'outer;
            }
            count_inner2 += 1;
            println!("current 'inner: count_inner2 = {count_inner2}");
            continue 'outer;
        }
    }
    // assert!(count == __)
    assert!(count == 30);
}

pub fn practice() {
    println!("Flow control run method_1: ");
    method_1();
    println!("Flow control run method_2: ");
    method_2();
    println!("Flow control run method_3: ");
    method_3();
    println!("Flow control run method_4: ");
    method_4();
    println!("Flow control run method_5: ");
    method_5();
    println!("Flow control run method_6: ");
    method_6();
    println!("Flow control run method_7: ");
    method_7();
    println!("Flow control run method_8: ");
    method_8();
    println!("Flow control run method_9: ");
    method_9();
    println!("Flow control run method_10: ");
    method_10();
    println!("Flow control run method_11: ");
    method_11();
}
