// // 6-2 按值传递参数使用mut关键字
// fn modify(mut v: Vec<u32>) -> Vec<u32>{
//     v.push(32);
//     v
// }

// // 6-3 按引用传递参数时mut的用法
// fn modify(v: &mut [u32]) {
//     v.reverse();
// }

// // 6-4 作用域内的函数会屏蔽作用域外的同名函数
// fn f(){
//     println!("1");
// }

// // 6-41 把闭包作为trait对象
// fn boxed_closure(c: &mut Vec<Box<dyn Fn()>>){
//     let s = "second";
//     c.push(Box::new(|| println!("first")));
//     c.push(Box::new(move || println!("{}", s)));
//     c.push(Box::new(|| println!("third")));
// }

// // 6-51 impl trait 示例
// fn square() -> impl FnOnce(i32) -> i32{
//     |i| i*i
// }

// 6-87 自定义迭代器适配器
#[derive(Clone, Debug)]
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct Step<I>{
    iter: I,
    skip: usize,
}

impl<I> Iterator for Step<I> where I: Iterator{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        let elt = self.iter.next();
        if self.skip > 0{
            self.iter.nth(self.skip - 1);
        }
        elt
    }
}

pub fn step<I>(iter: I, step: usize) -> Step<I> where I: Iterator{
    assert_ne!(step, 0);
    Step{
        iter: iter,
        skip: step - 1,
    }
}

pub trait IterExt: Iterator{
    fn step(self, n: usize) -> Step<Self> where Self: Sized,{
        step(self, n)
    }
}

impl<T: ?Sized> IterExt for T where T: Iterator{

}


fn main() {
    // // 6-2 按值传递参数使用mut关键字
    // let v = vec![1, 2, 3];
    // let v = modify(v);
    // println!("{:?}", v)

    // // 6-3 按引用传递参数时mut的用法
    // let mut v = vec![1, 2, 3];
    // modify(&mut v);
    // println!("{:?}", v);

    // // 6-4 作用域内的函数会屏蔽作用域外的同名函数
    // f();
    // {
    //     f();
    //     fn f(){
    //         println!("3");
    //     }
    // }
    // f();
    // fn f(){
    //     println!("2");
    // }

    // // 6-36 move关键字——移动语义
    // let s = "hello".to_string();
    // let c = move ||{println!("{:?}", s)};
    // c();
    // c();
    // // // error[E0382]: borrow of moved value: `s`
    // // println!("{:?}", s);

    // // 6-38 修改环境变量的闭包来自动实现FnMut
    // let mut s = "rush".to_string();
    // {
    //     let mut c = ||{s += " rust"};
    //     c();
    //     c();
    //
    //     // NLL导致不报错
    //     println!("{:?}", s);
    // }
    // println!("{:?}", s);

    // 闭包规则总结
    // 如果闭包中没有捕获如何环境变量，则默认自动实现Fn
    // 如果闭包中捕获了复制语义的环境变量，则：
    //     如果不需要修改环境变量，无论是否使用move关键字，都自动实现Fn
    //     如果需要修改环境变量，则自动实现FnMut
    // 如果闭包中捕获了移动语义的环境变量，则：
    //     如果不需要修改环境变量，且没有使用move关键字，则自动实现FnOnce
    //     如果不需要修改环境变量，且使用了move关键字，则自动实现Fn
    //     如果需要修改环境变量，则自动实现FnMut
    // 对于FnMut的闭包使用move关键字，如果捕获的变量时复制语义类型的，则闭包会自动实现Copy/Clone
    // 否则不会自动实现Copy/Clone

    // // 6-41 把闭包作为trait对象
    // let mut c: Vec<Box<dyn Fn()>> = vec![];
    // boxed_closure(&mut c);
    // for f in c {
    //     f();
    // }

    // // 6-51 impl trait 示例
    // let square = square();
    // assert_eq!(4, square(2));

    // let x = Box::new(&2usize);

    // // 6-74 部分迭代器适配器使用示例
    // let arr1 = [1, 2, 3, 4, 5];
    // let c1 = arr1.iter().map(|x| 2*x).collect::<Vec<i32>>();
    // assert_eq!(&c1[..], [2, 4, 6, 8, 10]);
    //
    // let arr2 = ["1", "2", "3", "h"];
    // let c2 = arr2.iter().filter_map(|x| x.parse().ok()).
    //     collect::<Vec<i32>>();
    // assert_eq!(&c2[..], [1, 2, 3]);
    //
    // let arr3 = ['a', 'b', 'c'];
    // for (idx, val) in arr3.iter().enumerate(){
    //     println!("idx: {:?}, val: {:?}", idx, val.to_uppercase());
    // }

    // 6-87 自定义迭代器适配器
    let arr = [1, 2, 3, 4, 5, 6];
    let sum = arr.iter().step(2).fold(0, |acc, x| acc + x);
    assert_eq!(9, sum);
}
