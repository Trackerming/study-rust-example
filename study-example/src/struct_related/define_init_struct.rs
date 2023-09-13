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
