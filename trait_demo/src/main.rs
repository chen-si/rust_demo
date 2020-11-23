// #[derive(Debug)]
// struct Foo;
//
// trait Bar{
//     fn baz(&self);
// }
//
// impl Bar for Foo{
//     fn baz(&self) {
//         println!("{:?}", self)
//     }
// }
//
// fn static_dispatch<T> (t : &T) where T : Bar{
//     t.baz()
// }
//
//
// /// trait的Self类型参数不能被限定为Sized
// /// trait中的所有方法都必须是对象安全的
// /// 对象安全： 没有额外的Self类型参数的非泛型成员方法
// fn dynamic_dispatch(t : &dyn Bar){
//     t.baz()
// }
//
// fn main() {
//     let foo = Foo;
//     static_dispatch(&foo);
//     dynamic_dispatch(&foo);
// }
// use std::fmt::Debug;
// use std::pin::Pin;
//
// pub trait Fly{
//     fn fly(&self) -> bool;
// }
//
// #[derive(Debug)]
// struct Duck;
// #[derive(Debug)]
// struct Pig;
//
// impl Fly for Duck{
//     fn fly(&self) -> bool {
//         true
//     }
// }
//
// impl Fly for Pig{
//     fn fly(&self) -> bool {
//         false
//     }
// }
//
// fn fly_static(s : impl Fly + Debug) -> bool{
//     s.fly()
// }
//
// fn can_fly(s : impl Fly + Debug) -> impl Fly{
//     if s.fly() {
//         println!("{:?} can fly", s)
//     }else{
//         println!("{:?} can't fly", s)
//     }
//     s
// }
//
// fn main() {
//     let pig = Pig;
//     let pig = can_fly(pig);
//     let duck = Duck;
//     let duck = can_fly(duck);
// }

// use std::thread;
//
// fn main() {
//     let x = vec![1, 2, 3, 4];
//
//     thread::spawn( || x);
// }

fn main() {
    let a = "hello".to_string();
    let b = " world".to_string();
    let c = a + &b;
    println!("{:?}", c);
}