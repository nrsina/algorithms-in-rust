// Given a binary array nums and an integer k, return the maximum number of consecutive 1's in the array if you can flip at most k 0's.
// Complexity: O(N)
// Approach: Sliding Window
fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
    let mut left = 0;
    let mut zeros = 0;
    let mut answer = 0;
    for (right, num) in nums.iter().enumerate() {
        if *num == 0 {
            zeros += 1;
        }
        while zeros > k {
            if nums[left] == 0 {
                zeros -= 1;
            }
            left += 1;
        }
        answer = answer.max(right - left + 1);
    }
    answer as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn longest_ones_test() {
        assert_eq!(longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2), 6);
        assert_eq!(longest_ones(vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1], 3), 10);
    }

}