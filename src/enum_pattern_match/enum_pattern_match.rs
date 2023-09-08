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
          --> src/enum_pattern_match/enum_pattern_match.rs:76:26
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
