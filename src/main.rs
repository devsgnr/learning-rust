mod coin;
mod collections;
mod error_handling;
mod generics;
mod message;
mod shape;

use coin::coin::{check_coin_option, check_if_coin, value_in_cents, Coin};
use collections::maps::mine_maps::run_maps;
use collections::strings::mine_strings::run_string;
use collections::vectors::mine_vectors::run_vectors;
use error_handling::errorhandling::{open_a_file, run_file_error_handling};
use generics::generic_functions::run_largest;
use message::message::Message;
use shape::shape::Rectangle;

fn main() {
    let square = Rectangle::square(15);
    println!("Square area: {}", square.area());

    /*
    Get the value of the coin directly
    */
    println!("Value in cents: {}", value_in_cents(Coin::Penny));

    /*
     Using enums and pattern matching
     First one is using my custom enum and pattern matching
    Second one is using Option<T> enum and pattern matching
    */
    let coin = check_if_coin(Coin::NotFound); /* First */
    match coin {
        Message::Ok(value) => println!("The coin is: {}", value),
        Message::Err(err) => println!("{}", err),
    }

    let coin_option = check_coin_option(Coin::NotFound); /* Second */
    match coin_option {
        Some(coin) => println!("This coin is: {}", coin),
        None => println!("No such coin"),
    }

    let file_openin = open_a_file("hello_world.txt");
    match file_openin {
        Ok(result) => println!("{:?}", result),
        Err(error) => println!("Error: {:?}", error),
    }

    run_vectors();
    run_string();
    run_maps();
    run_file_error_handling();

    run_largest();
}

/*
 Learnt about Ownershipsm, References, and Slices as a type of reference
*/
#[allow(dead_code)]
fn first_word(str: &String) -> &str {
    let bytes = str.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &str[..i];
        }
    }
    &str[..]
}
