use seremban::add_one;

mod convert_temperature;
mod fibonacci;
mod guessing_game;
mod string;

fn main() {
    // converted_temperature::convert();
    // guessing_game::start();
    // let username_result = string::read_username_from_file();
    //
    // match username_result {
    //     Ok(username) => println!("username: {}", username),
    //     Err(error) => println!("error: {}", error),
    // }
    let res = add_one(4);
    println!("res: {res}");
}
