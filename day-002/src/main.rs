//! A Simple Hello World Crate
fn main() {
    println!("{}", hello());
}

/// This function returns the greeting; Hello, world!
pub fn hello() -> String {
    ("Hello, world!").to_string()
}

#[cfg(test)]
mod tests {
    use super::hello;

    #[test]
    fn test_hello() {
        assert_eq!(hello(), "Hello, world!");
    }
}