fn search(nums: &[i32], target: i32) -> i32
{
    let mut lptr = 0;
    let mut rptr = nums.len() - 1;

    while lptr <= rptr
    {
        let mptr = (lptr + rptr) / 2;

        if nums[mptr] == target
        {
            return mptr as i32;
        }

        if nums[mptr] >= nums[lptr]
        {
            if nums[lptr] <= target && target <= nums[mptr]
            {
                rptr = mptr - 1;
            }
            else
            {
                lptr = mptr + 1;
            }
        }
        else
        {
            if nums[mptr] <= target && target <= nums[rptr]
            {
                lptr = mptr + 1;
            }
            else
            {
                rptr = mptr - 1;
            }
        }
    }
    -1
}

#[cfg(test)]
mod tests
{
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(vec![4, 5, 6, 7, 0, 1, 2], 0, 4)]
    #[case(vec![4, 5, 6, 7, 0, 1, 2], 3, -1)]
    #[case(vec![1], 0, -1)]
    fn test_search(#[case] nums: Vec<i32>, #[case] target: i32, #[case] expected: i32)
    {
        let result = search(&nums, target);

        assert_eq!(result, expected);
    }
}
