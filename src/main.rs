use rand::{thread_rng, Rng};
    const ALPHANUMERIC: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

    const NUMERIC: &str = "0123456789";
    const HEX: &str = "0123456789abcdef";

    pub struct UniqueIdGenerator {
        length: usize,
        charset: Vec<char>,
    }
    impl UniqueIdGenerator {
        pub fn new() -> Self {
            Self {
                length: 21,
                charset: ALPHANUMERIC.chars().collect(),
            }
        }
        pub fn length(mut self, length: usize) -> Self {
            self.length = length;
            self
        }
        pub fn charset(mut self, charset_str: &str) -> Self {
            self.charset = charset_str.chars().collect();
            self
        }
        pub fn numeric_only(mut self) -> Self {
            self.charset = NUMERIC.chars().collect();
            self
        }
        pub fn generate(&self) -> String {
            if self.charset.is_empty() {
                panic!("Empty characterset for {}", self.length);
            }
            let mut rng = thread_rng();
            (0..self.length)
                .map(|_| {
                    let idx = rng.gen_range(0..self.charset.len());
                    self.charset[idx]
                })
                .collect()
        }
    }
fn main() {
    let default = UniqueIdGenerator::new();
    println!("Default (21 chars): {}", default.generate());

    let short_gen = UniqueIdGenerator::new().length(8);
    println!("Short (length 8): {}", short_gen.generate());

    let pin_gen = UniqueIdGenerator::new().length(6).numeric_only();
    println!("PIN Code (length 6): {}", pin_gen.generate());

    let custom_gen = UniqueIdGenerator::new().length(12).charset("ABCDEFGHIJKLMNOPQRSTUVWXYZ23456789");
    println!("Custom Charset (length 12): {}", custom_gen.generate());
}
