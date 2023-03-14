mod coin;
mod collections;
mod enum_and_match;
mod error_handling;
mod generics;
mod message;
mod shape;

use coin::coin::{value_in_cents, Coin};
use collections::maps::mine_maps::run_maps;
use collections::strings::mine_strings::run_string;
use collections::vectors::mine_vectors::run_vectors;
use enum_and_match::enum_match::run_enums_and_match;
use error_handling::errorhandling::open_a_file;
use generics::generic_functions::run_largest;
use generics::struct_and_traits::struct_and_traits::{Summary, Tweet};
use shape::shape::Rectangle;

fn main() {
    // Find area
    let square = Rectangle::square(15);
    println!("Square area: {}", square.area());

    // Get the value of the coin directly
    println!("Value in cents: {}", value_in_cents(Coin::Penny));

    // Run enum and matching
    run_enums_and_match();

    let file_openin = open_a_file("hello_world.txt");
    match file_openin {
        Ok(result) => println!("{:?}", result),
        Err(error) => println!("Error: {:?}", error),
    }

    // Collections - from collections modules
    run_vectors();
    run_string();
    run_maps();

    // Generics & Traits - from struct and traits modules
    run_largest();

    // Working with struct, traits and impl
    let tweet = Tweet::new("devsgnr_", "Hello World!!");
    println!("{}", tweet.summarize());
}

// Learnt about Ownershipsm, References, and Slices as a type of reference
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
