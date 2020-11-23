// // 5-20
// fn foo(mut v: [i32; 3]) -> [i32; 3]{
//     v[0] = 3;
//     assert_eq!([3, 2, 3], v);
//     v
// }

// // 5-22 bubble_sort
// fn bubble_sort(a: &mut Vec<i32>){
//     let mut n = a.len();
//     while n > 0 {
//         let (mut i, mut max_ptr) = (1, 0);
//         while i < n{
//             if a[i - 1] > a[i]{
//                 a.swap(i - 1, i);
//                 max_ptr = i;
//             }
//             i = i + 1
//         }
//         n = max_ptr;
//     }
// }

// // 5-29 无输入返回引用
// fn return_str<'a>() -> &'a str{
//     let mut s = "Rust".to_string();
//     for i in 0..3 {
//         s.push_str(" Good");
//     }
//     // returns a value referencing data owned by the current function
//     &s[..]
// }

// // 5-32 'b : 'a 表示 'b outlive 'a
// fn the_longest<'a, 'b : 'a>(s1: &'a str, s2: &'b str) -> &'a str{
//     if s1.len() > s2.len() {
//         s1
//     }else{
//         s2
//     }
// }
//
//
// use std::rc::Rc;

use std::rc::{Rc, Weak};
use std::cell::RefCell;

// 5-52 利用Weak<T>解决内存泄漏的问题
struct Node{
    next: Option<Rc<RefCell<Node>>>,
    head: Option<Weak<RefCell<Node>>>,
}

impl Drop for Node{
    fn drop(&mut self) {
        println!("Dropping!");
    }
}

fn main() {
    // // 5-14 match 匹配作用域
    // let a = Some("hello".to_string());
    // match a{
    //     Some(s) => println!("{:?}", s),
    //     _ => println!("None"),
    // }
    // // error[E0382]: borrow of moved value: `a`
    // println!("{:?}", a);

    // // 5-15 for 作用域
    // let v = vec![1, 2, 3];
    // for i in v{
    //     println!("{:?}", i);
    //     // error[E0382]: borrow of moved value: `v`
    //     println!("{:?}", v);
    // }

    // //5-20
    // let v = [1, 2, 3];
    // foo(v);
    // assert_eq!([1, 2, 3], v);

    // // 5-22 bubble_sort
    // let mut a = vec![1, 4, 5, 3, 2];
    // bubble_sort(&mut a);
    // println!("{:?}", a);

    // // 5-29
    // // cannot return value referencing local variable `s`
    // let x = return_str();

    // // 5-32
    // let s1 = String::from("Rust");
    // let s1_r = &s1;
    // {
    //     let s2 = String::from("C");
    //     let res = the_longest(s1_r, &s2);
    //     println!("{} is the longest", res);
    // }

    // // 5-51 Rc 示例
    // let x = Rc::new(45);
    // // 增加强引用计数
    // let y1 = x.clone();
    // let y2 = x.clone();
    // println!("{:?}", Rc::strong_count(&x));
    //
    // // 弱引用计数
    // let w = Rc::downgrade(&x);
    // println!("{:?}", Rc::weak_count(&x));
    //
    // // 不增加引用计数
    // let y3 = &*x;
    // println!("{}", 100 - *x);

    // 5-52 利用Weak<T>解决内存泄漏的问题
    let first = Rc::new(RefCell::new(Node{next: None, head: None}));
    let second = Rc::new(RefCell::new(Node{next: None, head: None}));
    let third = Rc::new(RefCell::new(Node{next: None, head: None}));
    first.borrow_mut().next = Some(second.clone());
    second.borrow_mut().next = Some(third.clone());
    third.borrow_mut().head = Some(Rc::downgrade(&first));
}
