/// **'rand'** is an external crate that can be added to the project using cargo
/// `cargo add rand`
/// Is is used for generating random numbers and ranges
use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    // In rust println is a macro, and macro's are used with ! sign at the end of them
    println!("Hello, world!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret_number}");
    loop {
        println!("please enter your guess : =");
        let mut guess = String::new();

        /* io is a module that is part to std module that contains the
        foundation apis of rust
        The `read_line` function is used to read input from console
        */
        io::stdin()
            .read_line(&mut guess)
            .expect("i can't  read your message");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Fuck yu ! enter the number only");
                continue;
            }
        };

        println!("Your guess is : {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
