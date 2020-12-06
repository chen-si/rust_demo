fn main(){
    let number = 42;
    match number {
        0 => println!("Original"),
        1 | 3 => println!("ALL"),
        5 | 7 | 13 => println!("Bad Luck"),
        n @ 42 => println!("The answer is {}", n),
        _ => println!("Common"),
    }
}