// This allows us to use custom macros in rust, such as get, put, post etc. of rocket
#[macro_use]
extern crate rocket;
// serde -> Serialize and Deserialize library, that help us work with json in rust
use rocket::serde::{json::Json, Deserialize, Serialize};
// serde json is another package that is used for conversion of json to string and visa versa
use serde_json::to_string;
// as we want to store the user's name permanently, we want to perform read and write operation
// hence we use the fs lib and some of it's methods
use std::fs::{read_to_string, write as write_file};

// we must implement Serialize trait to return a Json response from rust, and convert a rust struct
// to json
#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    name: String,
}

#[get("/user")]
// rust is extremely type safe, so we must provide proper return type for the function, in this
// example we need to return json of type User struct, so we specify that type as the return type
fn get_json() -> Json<User> {
    // match works as switch case, we want to read the file users.json and if there is some data we
    // want to parse it and send the response otherwise we want to write our own data to file that
    // will be a default name "Himanshu Jangd" and then return a response, this will only happen
    // single time, and once the file is written with some data and saved, this case may never run
    // until we delete the file
    match read_to_string("./src/users.json") {
        Ok(user) => {
            let user: User = serde_json::from_str(user.as_str()).expect("couldn't parse sting");
            return Json(user);
        }
        Err(_) => {
            let user = User {
                name: String::from("Himanshu Jangid"),
            };

            let stringified_user = to_string(&user).expect("Couldn't convert");

            write_file("./src/users.json", stringified_user.as_bytes())
                .expect("Couldn't write file");

            return Json(user);
        }
    }
}

// using this route to update the name in the user.json file
#[get("/update/<name>")]
fn add_user(name: &str) -> String {
    let user = User {
        name: String::from(name),
    };

    // conversion of user object into string
    let stringified_user = to_string(&user).expect("Couldn't convert");

    println!("This is the string, {}", stringified_user);

    // writting the stringified user into the file
    write_file("./src/users.json", stringified_user.as_bytes()).expect("Couldn't write file");

    String::from("Added")
}

#[get("/hello")]
fn hello() -> String {
    String::from("Hello World!")
}

#[launch]
fn rocket() -> _ {
    // mounting various routes in the rocket server
    rocket::build().mount("/", routes![hello, get_json, add_user])
}
