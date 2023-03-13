pub mod mine_vectors {
    pub fn run_vectors() {
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
    }
}
