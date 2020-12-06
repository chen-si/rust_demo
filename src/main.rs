// use std::fmt::DebugList;

use std::ops::Add;

#[derive(Debug)]
struct Point{
    x: i32,
    y: i32,
}

impl Add for Point{
    type Output = Point;
    fn add(self, other: Point) -> Point{
        Point{
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    //
    // println!("Guess the number!");
    //
    // let secret_number = rand::thread_rng().gen_range(1,101);
    //
    // println!("the secret number is {}",secret_number);
    // loop {
    //     println!("Please input your guess.");
    //
    //     let mut guess = String::new();
    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");
    //
    //     let guess: u32 = match guess.trim().parse(){
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };
    //
    //     println!("You guessed: {}", guess);
    //
    //     match guess.cmp(&secret_number) {
    //         Ordering::Less => println!("Too small!"),
    //         Ordering::Greater => println!("Too big!"),
    //         Ordering::Equal => {
    //             println!("You win!");
    //             break;
    //         }
    //     }
    // }

    // // 2-23
    // let mut v = vec![1, 2, 3, 4, 5];
    // while let Some(x) = v.pop(){
    //     println!("{}",x)
    // }

    // // 2-28
    // assert_eq!((1..5), std::ops::Range{start: 1, end: 5});
    // assert_eq!((1..=5), std::ops::RangeInclusive::new(1, 5));
    // assert_eq!(3 + 4 + 5,(3..6).sum());
    // assert_eq!(3 + 4 + 5 + 6,(3..=6).sum());
    //
    // for i in 1..5 {
    //     println!("{}", i);
    // }
    //
    // for i in 1..=5 {
    //     println!("{}", i);
    // }

    // // 2-50
    // # [derive(PartialEq, Debug)]
    // struct Point{
    //     x: f64,
    //     y: f64,
    // }
    //
    // let box_point = Box::new(Point{x:0.0, y:0.0});
    // let unboxed_point: Point = *box_point;
    // assert_eq!(unboxed_point, Point{x:0.0, y:0.0})

    // let str = "Hello Rust";
    // let ptr = str.as_ptr();
    // let len = str.len();
    //
    // println!("{:p}",ptr);
    // println!("{:?}",len);

    // let mut arr = [1, 2, 3, 4, 5];
    //
    // println!("{:?}",arr);
    // {
    //     let mut_arr : &mut[u32] = &mut arr;
    //     reset(mut_arr)
    // }
    // println!("{:?}", arr);

    // let v = vec![(); 10];
    // for i in v{
    //     println!("{:?}", i)
    // }

    // // 3-14 turbofish operator
    // let x = "1";
    // assert_eq!(x.parse::<i32>().unwrap(), 1);

    // // 3-16
    // assert_eq!(foo(1), 1);
    // assert_eq!(foo("hello"), "hello");

    // 3-30
    // println!("{:?}", Point{x: 1, y: 0} + Point{ x: 2, y: 3 })
}

// 3-16
// fn foo<T>(x : T) -> T{
//     return x;
// }

// fn reset(arr: &mut [u32]){
//     arr[0] = 5;
//     arr[1] = 4;
//     arr[2] = 3;
//     arr[3] = 2;
//     arr[4] = 1;
//
//     println!("{:?}",arr)
// }
