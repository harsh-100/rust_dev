use std::io;
fn main() {
    println!("Hello, world!");
    let mut num = String::new();
    println!("Enter a input : = ");

    io::stdin()
    .read_line(&mut num).unwrap();

    let num = num.trim();

    let reverse_num:String = num.chars().rev().collect();

    if num==reverse_num{
        println!("{} is palindrome",num);
    }else{
        println!("{} is not an palindrome",num);
    }

}



