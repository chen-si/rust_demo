use std::process::Command;

mod color;

// // 设计模式：
// //      针对接口编程
// //      组合优于继承
// //      分离变和不变
// //      委托代替继承
//
// // 7-34 建造者模式
// //      主要思想： 变和不变分离
// struct Circle{
//     x: f64,
//     y: f64,
//     radius: f64,
// }
//
// struct CircleBuilder {
//     x: f64,
//     y: f64,
//     radius: f64,
// }
//
// impl Circle {
//     fn area(&self) -> f64 {
//         std::f64::consts::PI * (self.radius * self.radius)
//     }
//
//     fn new() -> CircleBuilder{
//         CircleBuilder{
//             x: 0.0,
//             y: 0.0,
//             radius: 1.0,
//         }
//     }
// }
//
// impl CircleBuilder{
//     fn x(&mut self, coordinate: f64) -> &mut CircleBuilder {
//         self.x = coordinate;
//         self
//     }
//
//     fn y(&mut self, coordinate: f64) -> &mut CircleBuilder {
//         self.y = coordinate;
//         self
//     }
//
//     fn radius(&mut self, radius: f64) -> &mut CircleBuilder {
//         self.radius = radius;
//         self
//     }
//
//     fn build(&self) -> Circle {
//         Circle{
//             x: self.x,
//             y: self.y,
//             radius: self.radius,
//         }
//     }
// }

// #[derive(Copy, Clone, Debug)]
// struct Book<'a>{
//     name: &'a str,
//     isbn: i32,
//     version: i32,
// }

// RAII模式
pub struct Letter {
    text: String,
}

pub struct EmptyEnvelope{}

pub struct ClosedEnvelope{
    letter: Letter,
}

pub struct PickupLorryHandle{
    done: bool,
}

impl Letter {
    pub fn new(text: String) -> Self{
        Letter{
            text,
        }
    }
}

impl EmptyEnvelope {
    pub fn wrap(self, letter: Letter) -> ClosedEnvelope{
        ClosedEnvelope{
            letter,
        }
    }
}

pub fn buy_prestamped_envelope() -> EmptyEnvelope {
    EmptyEnvelope{}
}

impl PickupLorryHandle {
    pub fn pickup(&mut self, envelope: ClosedEnvelope) {
        /* give letter */
    }
    pub fn done(self){}
}

impl Drop for PickupLorryHandle {
    fn drop(&mut self) {
        println!("sent");
    }
}

pub fn order_pickup() -> PickupLorryHandle {
    PickupLorryHandle{
        done: false,
    }
}

fn main() {
    // let book = Book{
    //     name: "Rust 编程之道",
    //     isbn: 20181212,
    //     version: 1,
    // };
    // // .. 更新操作符
    // let book2 = Book{version: 2, ..book};
    // println!("{:?}", book);
    // println!("{:?}", book2);

    // let hi = "Rust".red().on_yellow();
    // println!("{}", hi)

    // // 7-34 建造者模式
    // let c = Circle::new()
    //     .x(1.0).y(2.0).radius(2.0)
    //     .build();
    // println!("{}", c.area());
    //
    // Command::new("ls")
    //     .arg("-hl")
    //     .spawn()
    //     .expect("ls command failed to start");

    // 7-43 RAII
    let letter = Letter::new(String::from("Dear Liu"));
    let envelope = buy_prestamped_envelope();
    let closed_envelope = envelope.wrap(letter);
    let mut lorry = order_pickup();
    lorry.pickup(closed_envelope);
}
