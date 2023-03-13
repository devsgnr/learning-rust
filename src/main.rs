pub mod coin;
pub mod message;
pub mod shape;

use std::collections::HashMap;

use coin::coin::{check_coin_option, check_if_coin, value_in_cents, Coin};
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

    /* Vectors */
    let mut vec: Vec<i32> = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(6);

    println!("{:?}", vec);

    let third = &vec[2];
    println!("Using indexing third element is: {}", third);

    let third_ = vec.get(10);
    match third_ {
        Some(value) => println!("Using get we found third element: {}", value),
        None => println!("We did not find anything"),
    };

    for i in &mut vec {
        println!("{}", *i * 2);
    }

    #[derive(Debug)]
    enum Spreadsheet {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let list = vec![
        Spreadsheet::Int(3),
        Spreadsheet::Float(0.5),
        Spreadsheet::Text(String::from("Hi")),
    ];

    println!("{:?}", list);

    /*
     String as a collection
    */
    let mut s = String::from("Emmanuel");
    s.push_str(" Watila");
    println!("{}", s);

    let s1 = String::from("Tic");
    let s2 = String::from("Tac");
    let s3 = String::from("Toe");
    let s_concat = format!("{s1}-{s2}-{s3}");

    println!("{}", s_concat);

    let s_literal = "Emmanuel";
    let s_bytes = &s_literal.as_bytes();
    for (i, &item) in s_bytes.iter().enumerate() {
        println!("{}: {}", i, item);
    }
    for (i, character) in s_literal.chars().enumerate() {
        println!("{}: {}", i, character)
    }
    println!("{:?}", s_bytes);

    /*
     Collections -> HashMap
    */
    let mut map: HashMap<&str, u32> = HashMap::new();

    map.insert("Blue", 10);
    map.insert("Yellow", 20);

    map.entry("Green").or_insert(25);

    println!("{:?}", map);

    for (key, value) in map {
        println!("{}: {}", key, value);
    }
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
