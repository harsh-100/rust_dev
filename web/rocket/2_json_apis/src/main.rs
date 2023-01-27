#[macro_use]
extern crate rocket;
use rocket::serde::{json::Json, Deserialize, Serialize};
use serde_json::to_string;
use std::fs::{read_to_string, write as write_file};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    name: String,
}

#[get("/user")]
fn get_json() -> Json<User> {
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

#[get("/update/<name>")]
fn add_user(name: &str) -> String {
    let user = User {
        name: String::from(name),
    };

    let stringified_user = to_string(&user).expect("Couldn't convert");

    println!("This is the string, {}", stringified_user);

    write_file("./src/users.json", stringified_user.as_bytes()).expect("Couldn't write file");

    String::from("Added")
}

#[get("/hello")]
fn hello() -> String {
    String::from("Hello World!")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello, get_json, add_user])
}
