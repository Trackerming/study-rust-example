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
