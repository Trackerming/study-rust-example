pub mod define_init_struct {
    #[derive(Debug)]
    struct User {
        user_id: String,
        user_name: String,
        age: u8,
        active: bool,
    }

    /*
     error[E0106]: missing lifetime specifier
       --> src/struct_related/struct_related.rs:11:20
       |
    11 |         user_name: &str,
       |                    ^ expected named lifetime parameter
       |
     help: consider introducing a named lifetime parameter
     */
    /*struct UserRef {
        user_id: &str,
        user_name: &str,
        age: u8,
        active: bool,
    }*/
    // 元组类型不需要命名的地方
    struct Color(i32, i32, i32);
    struct Point(i32, i32);
    fn build_user(user_name: String, user_id: String, age: u8) -> User {
        User {
            user_name,
            user_id,
            age,
            active: true,
        }
    }
    fn create_user_and_modify(name: &str) -> User {
        let mut user = build_user(String::from("user_name"), String::from("user_id"), 28);
        user.user_name = name.to_string();
        return user;
    }
    pub fn define_init_struct_study() {
        let name = String::from("Jack");
        let user = create_user_and_modify(&name);
        let user2 = User { age: 18, ..user };
        let color = Color(32, 34, 43);
        let point = Point(23, -23);
        dbg!(&user2);
    }
}

pub mod retangle_struct {
    // {:?}pretty-print打印时候没有实现`Rectangle` doesn't implement `Debug`
    #[derive(Debug)]
    struct Rectangle {
        width: i32,
        height: i32,
    }

    impl Rectangle {
        fn area(&self) -> i32 {
            self.width * self.height
        }

        // move了所有权，调用之后，后续该实例就没法使用；
        fn get_width(self) -> i32 {
            self.width
        }

        fn modify_width(&mut self, new_width: i32) {
            self.width = new_width;
        }

        fn valid_width(self: &Self) -> bool {
            self.width > 0
        }

        // Associated function，类似于静态方法？
        fn square(size: i32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }

    pub fn retangle_struct_study() {
        let mut rect = Rectangle {
            width: 12,
            height: 8,
        };
        let area_val = rect.area();
        // 直接打印没有实现标准的std::fmt::Display
        println!("area val {}, rect {:?}", area_val, rect);
        if rect.valid_width() {
            println!("width valid.");
        }
        rect.modify_width(-32);
        println!("area after modify val {}, rect {:?}", rect.area(), rect);
        let square_val = Rectangle::square(10);
        println!("square: {:?}", square_val);
        println!("rect width {}", rect.get_width());
        // error[E0382]: borrow of moved value: `rect`
        // rect.area();
    }
}
