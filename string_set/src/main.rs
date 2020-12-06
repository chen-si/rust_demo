// 8-1 字符串编码示例
// use std::str;

// // 8-34 为自定义结构体实现from_str
// use std::str::FromStr;
// use std::num::ParseIntError;
//
// #[derive(Debug, PartialEq)]
// struct Point{
//     x: i32,
//     y: i32,
// }
//
// impl FromStr for Point {
//     type Err = ParseIntError;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let coords = s.trim_matches(|p| p == '{' || p =='}')
//             .split(",")
//             .collect::<Vec<&str>>();
//         let x_fromstr = coords[0].parse::<i32>()?;
//         let y_fromstr = coords[1].parse::<i32>()?;
//         Ok(Point{x: x_fromstr, y: y_fromstr})
//     }
// }

// 8-38 自定义类型使用format!
use std::fmt::{self, Formatter, Display};
struct City{
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for City{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 {'N'} else {'S'};
        let lon_c = if self.lon >= 0.0 {'E'} else {'W'};
        write!(f, "{}:{:.3}'{} {:.3}'{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

fn main() {
    // // 8-1 字符串编码示例
    // let tao = str::from_utf8(&[0xE9u8, 0x81u8, 0x93u8]).unwrap();
    // assert_eq!("道", tao);
    // assert_eq!("道", String::from("\u{9053}"));
    //
    // let unicode_x = 0x9053;
    // let utf_x_hex = 0xe98193;
    // let utf_x_bin = 0b111010011000000110010011;
    // println!("unicode_x: {:b}", unicode_x);
    // println!("utf_x_hex: {:b}", utf_x_hex);
    // println!("utf_x_bin: 0x{:x}", utf_x_bin);

    // // 8-6 组成String
    // let mut a = String::from("foo@");
    // // 堆中字节序列的地址
    // println!("{:p}", a.as_ptr());
    // // 栈上指针的地址
    // println!("{:p}", &a);
    // a.reserve(10);
    // println!("{}", a.capacity());
    // println!("{}", a.len());

    // // 8-18 存在性判断
    // let bananas = "bananas";
    // assert!(bananas.contains('a'));
    // assert!(bananas.contains("an"));
    // assert!(bananas.contains(char::is_lowercase));
    // assert!(bananas.starts_with('b'));
    // assert!(bananas.ends_with("nanas"));

    // // rfind
    // let s = "abc刘dea";
    // println!("{:?}", s.rfind('a'));

    // // 8-32 类型转换
    // let four: u32 = "4".parse().unwrap();
    // assert_eq!(four, 4);
    //
    // let four = "4".parse::<u32>().unwrap();
    // assert_eq!(four, 4);

    // // 8-34 为自定义结构体实现from_str
    // let p = Point::from_str("{1,2}").unwrap();
    // println!("{:?}", p)

    // // 8-35 字符串format
    // let s = format!("{}Rust", "hello");
    // println!("{}", s);
    // println!("{}", format!("{:5}", s));
    // println!("{}", format!("{:5.3}", s));
    // println!("{}", format!("{:10}", s));
    // println!("{}", format!("{:<12}", s));
    // println!("{}", format!("{:>12}", s));
    // println!("{}", format!("{:^12}", s));
    // println!("{}", format!("{:^12.5}", s));
    // println!("{}", format!("{:=^12.5}", s));
    // println!("{}", format!("{:*^12.5}", s));
    // println!("{}", format!("{:5}", "th\u{e9}"));

    // // 8-36 整数format!
    // println!("{}", format!("{:+}", 1234));
    // println!("{}", format!("{:+x}", 1234));
    // println!("{}", format!("{:+#x}", 1234));
    // println!("{}", format!("{:b}", 1234));
    // println!("{}", format!("{:#b}", 1234));
    // println!("{}", format!("{:#20b}", 1234));
    // println!("{}", format!("{:<#20b}", 1234));
    // println!("{}", format!("{:^#20b}", 1234));
    // println!("{}", format!("{:>+#15x}", 1234));
    // println!("{}", format!("{:>+#015x}", 1234));

    // // 8-37 浮点数format!
    // println!("{}", format!("{:.4}", 1234.5678));
    // println!("{}", format!("{:.2}", 1234.5618));
    // println!("{}", format!("{:.2}", 1234.5678));
    // println!("{}", format!("{:<10.4}", 1234.5678));
    // println!("{}", format!("{:^12.2}", 1234.5678));
    // println!("{}", format!("{:0^12.2}", 1234.5678));
    // println!("{}", format!("{:e}", 1234.5678));

    // 8-38 自定义类型使用format!
    let city = City{
        name: "BeiJing",
        lat: 39.900469,
        lon: -166.40717
    };

    println!("{}", city);

}
