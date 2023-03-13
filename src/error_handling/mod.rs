pub mod errorhandling {
    use std::{
        fs::File,
        io::{self, ErrorKind, Read},
    };

    pub fn run_file_error_handling() {
        /*
           Error Handling using Match
        */
        let new_file = File::open("hello.txt");
        match new_file {
            Ok(file) => println!("{:?}", file),
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => println!("{:?}", fc),
                    Err(error) => println!("Error - Problem creating file: {:?}", error),
                },
                other_error => println!("Error - Problem opening file: {:?}", other_error),
            },
        }

        /*
            Error Handline using Closures
        */
        let _another_file = File::open("another.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("another.txt")
                    .unwrap_or_else(|error| panic!("Error - Problem creating file: {:?}", error))
            } else {
                panic!("Error - Problem creating file: {:?}", error);
            }
        });
    }

    /*
        Error Propagation using ? operator
    */
    pub fn open_a_file(path: &str) -> Result<String, io::Error> {
        let mut file_result = File::open(path)?;
        let mut content = String::new();
        file_result.read_to_string(&mut content)?;
        Ok(content)
    }
}
