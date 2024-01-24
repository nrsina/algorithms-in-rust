
// Given an integer array nums sorted in non-decreasing order, return an array of the squares of each number sorted in non-decreasing order.
// Two Pointers
fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut i = 0;
    let mut j = nums.len() - 1;
    let mut result = vec![0; nums.len()];
    for k in (0..nums.len()).rev() {
        let first = nums[i].abs();
        let last = nums[j].abs();
        if first > last {
            result[k] = first * first;
            i += 1;
        } else {
            result[k] = last * last;
            j -= 1;
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sorted_squares_test() {
        assert_eq!(sorted_squares(vec![-4, -1, 0, 3, 10]), vec![0, 1, 9, 16, 100]);
        assert_eq!(sorted_squares(vec![-10, -5, 1, 2, 4, 7]), vec![1, 4, 16, 25, 49, 100]);
    }

}
