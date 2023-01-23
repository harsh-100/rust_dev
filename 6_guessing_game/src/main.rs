use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("Hello, world!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret_number}");
   loop {
   
    println!("please enter your guess : =");
    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("i can't  read your message");
   let guess:u32 = match guess.trim().parse() {
     Ok(num)=> num,
     Err(_)=>{
        println!("Fuck yu ! enter the number only");
        continue;
    }       
   };

        println!("Your guess is : {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less=>println!("Too small"),
        Ordering::Greater=>println!("Too big"),
        Ordering::Equal=>{
            println!("You win!");
            break;
        },
    }
   }


}
