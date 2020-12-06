// // 9-5 Option<T>使用示例
// fn get_shortest(names: Vec<&str>) -> Option<&str>{
//     if names.len() > 0 {
//         let mut shortest = names[0];
//         for name in names.iter() {
//             if name.len() < shortest.len(){
//                 shortest = name;
//             }
//         }
//         Some(shortest)
//     }else{
//         None
//     }
// }
//
// fn show_shortest(names: Vec<&str>) -> &str{
//     match get_shortest(names) {
//         Some(shortest) => shortest,
//         None => "Not Found!",
//     }
// }

// 9-18 从文件中读取数字并计算和
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // // 9-5 Option<T>使用示例
    // assert_eq!(show_shortest(vec!["Uku", "Felipe"]), "Uku");
    // assert_eq!(show_shortest(Vec::new()), "Not Found!");

    // 9-18 从文件中读取数字并计算和
    let args = env::args().collect::<Vec<String>>();
    println!("{:?}", args);

    let filename = &args[1];
    let mut f  = File::open(filename).unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    let mut sum = 0;
    for c in contents.lines(){
        let n = c.parse::<i32>().unwrap();
        sum += n;
    }
    println!("{:?}", sum);
}
