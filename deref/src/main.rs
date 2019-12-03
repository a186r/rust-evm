use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T>{
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello {}", name);
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
//    rust有解引用强制多态，所以可以这样写
//    hello(&m);
//    如果rust没有解引用强制多态必须要编写如下的代码
    hello(&(*m)[..]);
}