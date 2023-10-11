/// 实现缓存功能的结构体
#[derive(Debug)]
struct Cacher<T, E>
where
    T: Fn(E) -> E,
    E: Copy,
{
    query: T,
    value: Option<E>,
}
impl<T, E> Cacher<T, E>
where
    T: Fn(E) -> E,
    E: Copy,
{
    fn new(query: T) -> Cacher<T, E> {
        Cacher {
            query: query,
            value: None,
        }
    }
    fn value(&mut self, arg: E) -> E {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.query)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
fn example_generic() {
    // 示例实现
    let mut cacher1 = Cacher::new(|str| str);
    let ele1 = cacher1.value("hello");
    println!("ele1 {ele1}, cacher1: {:?}", cacher1.value);
    let mut cacher1 = Cacher::new(|num| num);
    let ele2 = cacher1.value(10086);
    println!("ele2 {ele2}, cacher2: {:?}", cacher1.value);
}

/// FnOnce,FnMut,Fn
/// FnOnce:该类型的闭包会拿走被捕获变量的所有权;如果想强制闭包取得捕获变量的所有权，可以在参数列表前添加 move 关键字
/// FnMut:以可变借用的方式捕获了环境中的值，因此可以修改该值
/// 闭包自动实现Copy特征的规则是，只要闭包捕获的类型都实现了Copy特征的话，这个闭包就会默认实现Copy特征
/// Fn 特征，它以不可变借用的方式捕获环境中的值
// 所有的闭包都自动实现了 FnOnce 特征，因此任何一个闭包都至少可以被调用一次
fn exec_fn_once<F: FnOnce()>(f: F) {
    f()
}
// 没有移出所捕获变量的所有权的闭包自动实现了 FnMut 特征
fn exec_fn_mute<F: FnMut()>(mut f: F) {
    f()
}
// 不需要对捕获变量进行改变的闭包自动实现了 Fn 特征
fn exec_fn<F: Fn()>(f: F) {
    f()
}
/*
pub trait Fn<Args> : FnMut<Args> {
    extern "rust-call" fn call(&self, args: Args) -> Self::Output;
}
pub trait FnMut<Args> : FnOnce<Args> {
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
}
pub trait FnOnce<Args> {
    type Output;

    extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
}
*/
fn example_fn_trait() {
    let s = String::from("hello");
    let update_str = || println!("{}", s);
    exec_fn_once(update_str);
    exec_fn_mute(update_str);
    exec_fn(update_str);
}
pub fn practice() {
    example_generic();
    example_fn_trait();
}
