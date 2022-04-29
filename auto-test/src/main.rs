fn main() {
    println!("Auto Test");
}

pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!(is_even(2));
        assert!(is_even(22));
        assert!(is_even(314));
        assert!(is_even(1006));
        assert!(is_even(21118));
    }

    #[test]
    fn is_false_when_odd() {
        assert!(!is_even(9));
        assert!(!is_even(31));
        assert!(!is_even(223));
        assert!(!is_even(3425));
        assert!(!is_even(82227));
    }
}
