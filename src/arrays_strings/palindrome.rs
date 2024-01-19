// Given a string str, return true if it is a palindrome, false otherwise.
// Complexity: O(N)

fn is_palindrome(str: &str) -> bool {
    let mut arr = str.chars();
    while let Some((i, j)) = arr.next().zip(arr.next_back()) {
        if i != j {
            return false;
        } 
    }
    true
}

fn is_palindrome_ignore_case(str: &str) -> bool {
    let mut arr = str.chars();
    while let Some((i, j)) = arr.next().zip(arr.next_back()) {
        if !i.eq_ignore_ascii_case(&j) {
            return false;
        } 
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn palindromes() {
        assert!(is_palindrome("radar"));
        assert!(is_palindrome("madam"));
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("rotator"));
        assert_eq!(is_palindrome("rAcecar"), false);
    }

    #[test]
    fn palindromes_ignore_case() {
        assert!(is_palindrome_ignore_case("radar"));
        assert!(is_palindrome_ignore_case("madAm"));
        assert!(is_palindrome_ignore_case("raceCar"));
        assert!(is_palindrome_ignore_case("rotator"));
    }
}
