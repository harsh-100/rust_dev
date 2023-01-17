use std::io;

fn main() {
    
    loop {
        let mut operation = String::new();

        println!("Please choose from the below one operation ");
        println!("1. Add");
        println!("2. Subtract");
        println!("3. Multiply");
        println!("4. Divide");
        println!("5. Exit ");
    
        io::stdin()
        .read_line(&mut operation).unwrap();
    
        let operation = operation.trim().parse().unwrap();
        if operation==5{
            println!("We will see you soon ✌️");
            break;
    
        }
    
        let mut num1 = String::new();
        let mut num2 = String::new();
        println!("Please Enter first number : = ");
    
        io::stdin()
        .read_line(&mut num1)
        .unwrap();
    
     
        println!("Please Enter Second number : = " );
        io::stdin()
        .read_line(&mut num2)
        .unwrap();
    
        let num1: u32 = num1.trim().parse().unwrap();
        let num2: u32 = num2.trim().parse().unwrap();
    
    
        let result = match operation {
            1 => num1+num2,
            2 => num1-num2,
            3 => num1*num2,
            4 => num1/num2,
            _ => panic!("Invalid Operation"),        
        };
        println!("The result is {}",result)
    
    }


   
}
