pub mod struct_and_traits {
    pub trait Summary {
        fn new(username: &str, content: &str) -> Tweet;
        fn summarize_author(&self) -> String;
        fn summarize(&self) -> String {
            format!("Read more from {}", self.summarize_author())
        }
    }
    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn new(username: &str, content: &str) -> Tweet {
            Tweet {
                username: String::from(username),
                content: String::from(content),
                reply: false,
                retweet: false,
            }
        }
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }
}
