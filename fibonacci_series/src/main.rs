use std::io;
fn main() {
    println!("Hello, world!");

    let mut num = String::new();
    println!("Enter a point to which you want fibo. series: = ");
    io::stdin().read_line(&mut num).unwrap();

    let num:u32 = num.trim().parse().unwrap(); 

    let mut n1= 0;
    let mut n2 = 1;

    while  n2<num{

        println!("{}",n2);

        let n3 = n1+n2;
        n1= n2;
        n2=n3;
    }
   


}
