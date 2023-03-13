pub mod mine_maps {
    use std::collections::HashMap;

    pub fn run_maps() {
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
}
