use std::rc::Rc;
use std::cell::RefCell;
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List{
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
//    let value = Rc::new(RefCell::new(5));
//    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
//
//    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
//    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
//
//    *value.borrow_mut() += 10;
//
//    println!("a after = {:?}", a);
//    println!("b after = {:?}", b);
//    println!("c after = {:?}", c);
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&a));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail(){
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

//    取消下面的注释将会看到引用循环，这会导致栈溢出
//    println!("a next item = {:?}", a.tail());
}
