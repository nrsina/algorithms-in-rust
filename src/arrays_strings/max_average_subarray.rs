// Find a contiguous subarray whose length is equal to k that has the maximum average value and return this value.
// Complexity: O(N)
// Approach: Sliding Window
fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let k = k as usize;
    let mut current: i32 = nums.iter().take(k).sum();
    let mut answer: i32 = current;
    for (i, num) in nums.iter().enumerate().skip(k) {
        current += num - nums[i - k];
        answer = i32::max(answer, current);
    }
    answer as f64 / k as f64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_max_ave_test() {
        assert_eq!(find_max_average(vec![1, 12, -5, -6, 50, 3], 4), 12.75);
        assert_eq!(find_max_average(vec![5], 1), 5.0);
    }

}