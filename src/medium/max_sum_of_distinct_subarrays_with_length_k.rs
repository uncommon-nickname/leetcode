use std::cmp::max;
use std::collections::HashSet;

fn max_subarray_sum_sliding_window(nums: &[i32], k: usize) -> i32
{
    let mut memory = HashSet::new();
    let mut lptr = 0;
    let mut current_sum = 0;
    let mut max_sum = 0;

    for (rptr, item) in nums.iter().enumerate()
    {
        if !memory.contains(item)
        {
            current_sum += item;
            memory.insert(*item);

            if rptr - lptr + 1 == k
            {
                max_sum = max(max_sum, current_sum);
                current_sum -= nums[lptr];
                memory.remove(&nums[lptr]);
                lptr += 1;
            }
        }
        else
        {
            while nums[lptr] != *item
            {
                current_sum -= nums[lptr];
                memory.remove(&nums[lptr]);
                lptr += 1;
            }
            lptr += 1;
        }
    }
    max_sum
}

fn max_subarray_sum_naive(nums: &[i32], k: usize) -> i32
{
    let mut max_sum = 0;
    let mut lptr = 0;
    let mut rptr = k;

    while rptr < nums.len()
    {
        let window = &nums[lptr..rptr].iter().collect::<HashSet<&i32>>();

        if window.len() == k
        {
            let curr_sum: i32 = window.iter().copied().sum();
            max_sum = max(max_sum, curr_sum);
        }

        lptr += 1;
        rptr += 1;
    }
    max_sum
}

#[cfg(test)]
mod tests
{
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(vec![1, 5, 4, 2, 9, 9, 9], 3, 15)]
    #[case(vec![4, 4, 4], 3, 0)]
    fn test_max_subarray_sum_sliding_window(#[case] nums: Vec<i32>, #[case] k: usize,
                                            #[case] expected: i32)
    {
        let result = max_subarray_sum_sliding_window(&nums, k);

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(vec![1, 5, 4, 2, 9, 9, 9], 3, 15)]
    #[case(vec![4, 4, 4], 3, 0)]
    fn test_max_subarray_sum_naive(#[case] nums: Vec<i32>, #[case] k: usize, #[case] expected: i32)
    {
        let result = max_subarray_sum_naive(&nums, k);

        assert_eq!(result, expected);
    }
}
