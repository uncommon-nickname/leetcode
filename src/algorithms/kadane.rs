use std::cmp::max;

pub fn largest_subarray_sum(nums: &[i32]) -> i32
{
    if nums.is_empty()
    {
        return 0;
    }

    let mut max_global = nums[0];
    let mut max_curr = nums[0];

    for item in nums.iter().skip(1)
    {
        max_curr = max(*item, max_curr + item);
        max_global = max(max_curr, max_global);
    }
    max_global
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_largest_subarray_sum()
    {
        let nums = vec![-2, -3, 4, -1, -2, 1, 5, -3];
        let result = largest_subarray_sum(&nums);

        assert_eq!(result, 7);
    }
}
