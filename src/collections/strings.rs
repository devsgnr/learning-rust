pub mod mine_strings {
    pub fn run_string() {
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
    }
}
