pub mod struct_and_traits;
pub mod generic_functions {
    pub fn largest(list: &[u8]) -> &u8 {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }

        return largest;
    }

    pub fn run_largest() {
        let list = [255, 128, 117, 102];
        let largest_item = largest(&list);
        println!("Largest bytes in string is: {}", largest_item);
        println!("Bytes in string: {:?}", &list);
    }
}
