pub mod coin {
    use crate::message::message::Message;

    #[allow(dead_code)]
    pub enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
        NotFound,
    }

    pub fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
            Coin::NotFound => 0,
        }
    }

    pub fn check_if_coin(coin: Coin) -> Message {
        match coin {
            Coin::Penny => Message::Ok(1),
            Coin::Nickel => Message::Ok(5),
            Coin::Dime => Message::Ok(10),
            Coin::Quarter => Message::Ok(25),
            _ => Message::Err(String::from("No such coin, try again")),
        }
    }

    pub fn check_coin_option(coin: Coin) -> Option<u8> {
        match coin {
            Coin::Penny => Some(1),
            Coin::Nickel => Some(5),
            Coin::Dime => Some(10),
            Coin::Quarter => Some(25),
            Coin::NotFound => None,
        }
    }
}
