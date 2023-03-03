// well have you 

struct User {
    name: String,
    age: i32
}
fn main() {
    let users = vec![User{name: gstr("Himanshu Jangid"), age: 20}, {User{name: gstr("Harsh Agarwal"), age: 21}}];
    let is_everyone_above = every(&users, |user, _index| {
        user.age > 20
    });

    println!("Is everyone greater than 20! {is_everyone_above}");

    let is_some_above = some(&users, |user, _index| {user.age > 20});

    println!("Is someone greater than 20! {is_some_above}");


    let names_only = map(&users, |user, _index| {
        return user.name.clone();
    });

    println!("Names only: {:?}", names_only);

}


/// Function to get String instance using String::from
fn gstr(input: &str) -> String{
    String::from(input)
} 

// in rust function are can be passed into another functions as arguments
// we can define the type of function right in the function definition
fn every<T>(iterable: &Vec<T>, matcher: fn (&T, usize) -> bool) -> bool{
    let mut i = 0;
    while i < iterable.len() {
        if !matcher(&iterable[i], i){
            return  false;
        }
        i+=1;
    }
    return true;
}


fn some<T>(iterable: &Vec<T>, matcher: fn (&T, usize) -> bool) -> bool{
    let mut i = 0;
    while i < iterable.len() {
        if matcher(&iterable[i], i){
            return  true;
        }
        i+=1;
    }
    return false;
}


fn map<T, U>(iterable: &Vec<T>, mapper: fn (&T, usize) -> U) -> Vec<U>{
    let mut i = 0;
    let mut result: Vec<U> = vec![];
    while i < iterable.len() {
       result.push(mapper(&iterable[i], i));
       i+=1;
    }
    return result;
}

