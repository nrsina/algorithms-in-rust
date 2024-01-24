
// Write a function that reverses a string. The input string is given as an array of characters s.
// You must do this by modifying the input array in-place with O(1) extra memory.
// Approach: Two Pointers
fn reverse(s: &mut Vec<char>) -> () {
    let mut i = 0;
    let mut j = s.len() - 1;
    while i < j {
        s.swap(i, j);
        i += 1;
        j -= 1;
    }
    /*
    Other solutions:
        let l = s.len();
        for i in 0..l/2 {
            s.swap(i, l-1-i);
        }
        -----
        Or just simply use s.reverse()
     */
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reverse_test() {
        let mut s1 = vec!['a', 'b', 'C', 'd'];
        let mut s2 = vec!['r', 'a', 'c', 'e', 'c', 'a', 'r'];
        reverse(&mut s1);
        assert_eq!(s1, vec!['d', 'C', 'b', 'a']);
        reverse(&mut s2);
        assert_eq!(s2, vec!['r', 'a', 'c', 'e', 'c', 'a', 'r']);
    }

}
