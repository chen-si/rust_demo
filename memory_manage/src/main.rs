// 4-20 内存泄漏
use std::rc::Rc;
use std::cell::RefCell;

type NodePtr<T> = Option<Rc<RefCell<Node<T>>>>;
struct Node<T>{
    data: T,
    next: NodePtr<T>,
}

impl<T> Drop for Node<T>{
    fn drop(&mut self) {
        println!("Dropping!");
    }
}

// // 4-11 4-15
// use std::ops::Drop;
//
// #[derive(Debug)]
// struct S(i32);
//
// impl Drop for S{
//     fn drop(&mut self){
//         println!("Drop {}", self.0);
//     }
// }

// // 4-13
// fn create_box(){
//     let box3 = Box::new(3);
// }

fn main() {
    // // 4-11
    // let x = S(1);
    // println!("crate x: {:?}", x);
    // {
    //     let y = S(2);
    //     println!("crate y: {:?}", y);
    //     println!("Exit inner scope");
    // }
    // println!("exit main");

    // // 4-13
    // let box1 = Box::new(1);
    // {
    //     let box2 = Box::new(2);
    // }
    //
    // for _ in 0..1_000{
    //     create_box();
    // }

    // // 4-14 使用花括号构造显示作用域主动析构局部变量
    // let mut v = vec![1, 2, 3];
    // {
    //     v
    // };
    //
    // v.push(4);

    // // 4-15 变量遮蔽不等于生命周期提前结束
    // let x = S(1);
    // println!("create x:{:?}", x);
    //
    // let x = S(2);
    // println!("create shadowing x:{:?}", x);

    // 4-20 内存泄漏
    let first = Rc::new(RefCell::new(Node{
        data: 1,
        next: None,
    }));
    let second = Rc::new(RefCell::new(Node{
        data: 2,
        next: Some(first.clone()),
    }));
    first.borrow_mut().next = Some(second.clone());
    second.borrow_mut().next = Some(first.clone());
}
