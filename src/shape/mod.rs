pub mod shape {
    pub struct Rectangle {
        pub width: u32,
        pub height: u32,
    }

    impl Rectangle {
        pub fn square(usize: u32) -> Self {
            Self {
                width: usize,
                height: usize,
            }
        }

        pub fn area(&self) -> u32 {
            self.width * self.height
        }

        #[allow(dead_code)]
        pub fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
}
