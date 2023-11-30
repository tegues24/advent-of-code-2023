fn main() {
    println!("Hello, world!");
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(2 + 2, 4); 
    }
}