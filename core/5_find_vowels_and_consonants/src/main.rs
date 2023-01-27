use std::io;

fn main() {
    println!("Program to count the number of vowel and Consonants in a String entered ");
    
    let mut user_input = String::new();
    println!("Please Enter any String : = ");
    io::stdin().read_line(&mut user_input).unwrap();

    let  user_input = user_input.trim();

    let  total_char_count = user_input.chars().count();

    let vowels_list = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

    let mut vowel_count = 0;
    let mut consonant_count = 0;

    for c in user_input.chars(){
        if vowels_list.contains(&c){
            vowel_count +=1;
        }else{
            consonant_count +=1;
        }
    }

    println!("Total Number of characters : {}",total_char_count);
    println!(" Number of vowel  characters : {}",vowel_count);
    println!(" Number of Consonant characters : {}",consonant_count);
    
}
