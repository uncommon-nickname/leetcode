fn remove_duplicates(nums: &mut [i32]) -> usize
{
    let mut lptr = 1;

    for rptr in 1..nums.len()
    {
        if nums[rptr] != nums[rptr - 1]
        {
            nums[lptr] = nums[rptr];
            lptr += 1;
        }
    }
    lptr
}

#[cfg(test)]
mod tests
{
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(vec![1, 1, 2], 2, vec![1, 2, 2])]
    #[case(vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4], 5, vec![0, 1, 2, 3, 4, 2, 2, 3, 3, 4])]
    fn test_remove_duplicates(#[case] mut nums: Vec<i32>, #[case] expected: usize,
                              #[case] expected_nums: Vec<i32>)
    {
        let result = remove_duplicates(&mut nums);

        assert_eq!(result, expected);
        assert_eq!(nums, expected_nums);
    }
}
