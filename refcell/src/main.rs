//use std::rc::Rc;
//use std::cell::RefCell;
//use crate::List::{Cons, Nil};
//
//#[derive(Debug)]
//enum List {
//    Cons(i32, RefCell<Rc<List>>),
//    Nil,
//}
//
//impl List{
//    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
//        match self {
//            Cons(_, item) => Some(item),
//            Nil => None,
//        }
//    }
//}
//
//fn main() {
////    let value = Rc::new(RefCell::new(5));
////    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
////
////    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
////    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
////
////    *value.borrow_mut() += 10;
////
////    println!("a after = {:?}", a);
////    println!("b after = {:?}", b);
////    println!("c after = {:?}", c);
//    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
//    println!("a initial rc count = {}", Rc::strong_count(&a));
//    println!("a next item = {:?}", a.tail());
//
//    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
//    println!("a rc count after b creation = {}", Rc::strong_count(&a));
//    println!("b initial rc count = {}", Rc::strong_count(&a));
//    println!("b next item = {:?}", b.tail());
//
//    if let Some(link) = a.tail(){
//        *link.borrow_mut() = Rc::clone(&b);
//    }
//
//    println!("b rc count after changing a = {}", Rc::strong_count(&b));
//    println!("a rc count after changing a = {}", Rc::strong_count(&a));
//
////    取消下面的注释将会看到引用循环，这会导致栈溢出
////    println!("a next item = {:?}", a.tail());
//}
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node{
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
//    一旦创建了leaf，其Rc<Node>的强引用记数为1，弱引用记数为0
    let leaf = Rc::new(Node{
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
//        在内部作用域中创建了branch并与leaf关联，此时branch中Rc<Node>的强引用记数为1，弱引用记数为1
//        这里leaf的强引用记数为2，因为现在branch的branch.children中存储了leaf的Rc<Node>的拷贝，不过弱引用记数仍为0
        let branch = Rc::new(Node{
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
//    当内部作用域结束时，branch离开作用域，Rc<Node>的强引用记数减少为0，所以其Node被丢弃
//    来自leaf.parent的弱引用记数1与Node是否被丢弃无关，所以并没有产生任何内存泄漏。
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}