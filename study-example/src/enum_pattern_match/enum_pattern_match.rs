pub mod define_enum {
    #[derive(Debug)]
    enum IpAddrType {
        V4,
        V6,
    }

    #[derive(Debug)]
    struct IpAddr {
        ip_addr_type: IpAddrType,
        address: String,
    }

    #[derive(Debug)]
    enum IpAddrEnum {
        V4(String),
        V6(String),
    }

    #[derive(Debug)]
    enum Message {
        Quit,                          // 没有数据关联
        Move { x: i32, y: i32 },       // 像struct做的，有命名的字段
        Write(String),                 // 包括一个单个的字符串
        ChangeColor(i32, i64, String), // 包括几个类型的数据
    }
    pub fn define_enum_study() {
        let home = IpAddr {
            ip_addr_type: IpAddrType::V4,
            address: String::from("127.0.0.1"), // home: IpAddr { ip_addr_type: V4, address: "127.0.0.1" }
        };
        println!("home: {:?}", home);
        let loopback = IpAddr {
            address: String::from("::1"),
            ..home // 不带逗号
        };
        println!("loopback: {:?}", loopback);
        let home_ip_addr = IpAddrEnum::V4(String::from("127.0.0.1"));
        println!("home_ip_addr: {:?}", home_ip_addr); // home_ip_addr: V4("127.0.0.1")
    }
}

pub mod enum_usage {

    #[derive(Debug)]
    enum Msg {
        Quit,                          // 没有数据关联
        Move { x: i32, y: i32 },       // 像struct做的，有命名的字段
        Write(String),                 // 包括一个单个的字符串
        ChangeColor(i32, i64, String), // 包括几个类型的数据
    }

    impl Msg {
        fn call(&self) {
            println!("msg call {:?}", &self);
        }
    }
    pub fn use_enum_study() {
        let msg = Msg::Write(String::from("enum msg write."));
        msg.call();
    }
}

pub mod option_usage {
    #[derive(Debug)]
    enum Option<T> {
        None,
        Some(T),
    }
    fn option_i32_add_ten(x: &Option<i32>) -> Option<i32> {
        match x {
            Option::None => Option::None,
            Option::Some(i) => Option::Some(i + 10),
        }
    }

    pub fn option_uasge_study() {
        let some_number = Some(5);
        let some_char = Some('S');
        let mut absent_number: Option<i32> = Option::None;
        println!("{:?}", &absent_number);
        let number: i32 = 32;
        /*
         error[E0277]: cannot add `option_usage::Option<i32>` to `i32`
          --> bin/enum_pattern_match/enum_pattern_match.rs:76:26
           |
        76 |         let sum = number + some_number;
           |                          ^ no implementation for `i32 + option_usage::Option<i32>`
         */
        // let sum = number + some_number;
        let result = option_i32_add_ten(&absent_number);
        println!("absent_number {:?} result: {:?}", absent_number, result);
        absent_number = Option::Some(100);
        println!("absent_number {:?} result: {:?}", absent_number, result);
        println!("result: {:?}", result);
        match absent_number {
            Option::None => (),
            Option::Some(i) => {
                println!("{:?}", i);
                return ();
            }
        }
    }
}

pub mod match_control_flow {
    #[derive(Debug)]
    enum Coin {
        BTC(String),
        ETH(String),
        USDT(String),
    }
    fn decimal_of_coin(coin: &Coin) -> u64 {
        match coin {
            Coin::BTC(str) => {
                println!("str {}", str);
                1_0000_0000
            }
            Coin::ETH(str) => 100_0000_0000_0000_0000,
            Coin::USDT(str) => 100_0000,
        }
    }
    pub fn match_control_flow_study() {
        let coin = Coin::BTC(String::from("BTC"));
        let decimal = decimal_of_coin(&coin);
        println!("coin {:?} decimal {}", coin, decimal)
    }
}

pub mod if_let_control {
    fn std_option_match_exercise(opt: Option<i32>) -> Option<i32> {
        match opt {
            Option::None => Option::None,
            Option::Some(int_val) => {
                println!("option int val {}", int_val);
                return Option::Some(int_val * 2);
            }
        }
    }
    fn std_option_if_let_exercise(opt: &Option<char>) -> Option<char> {
        if let Some(char_ele) = opt {
            println!("std option if let exercise: {char_ele}");
            return Option::Some(char_ele.to_ascii_lowercase());
        } else {
            return Option::None;
        }
    }
    pub fn if_let_control_study() {
        let config_max = Some(2u8);
        let mut count_a = 0;
        // match只想匹配一种情况执行的时候
        match config_max {
            Some(max) => println!("max {}", max),
            _ => count_a += 1,
        }
        // 等效于如下
        let mut count_b = 0;
        if let Some(max) = config_max {
            println!("if let max {max}");
        } else {
            count_b += 1;
        }
        println!("count_a {}, count_b {}", count_a, count_b);
        std_option_match_exercise(Option::None);
        let result = std_option_match_exercise(Some(12));
        println!("result {:?}", result);
        let opt = Some('A');
        let result = std_option_if_let_exercise(&opt);
        println!("result {:?}", result);
    }
}
