pub mod enum_match {
    use crate::{
        coin::coin::{check_coin_option, check_if_coin, Coin},
        message::message::Message,
    };

    pub fn run_enums_and_match() {
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
    }
}
