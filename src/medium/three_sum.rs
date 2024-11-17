fn three_sum(nums: &mut [i32]) -> Vec<(i32, i32, i32)>
{
    nums.sort();

    let size = nums.len();
    let mut result = Vec::new();

    for i in 0..size
    {
        let mut j = i + 1;
        let mut k = size - 1;

        while j < k
        {
            let sum = nums[i] + nums[j] + nums[k];

            if sum == 0 && !result.contains(&(nums[i], nums[j], nums[k]))
            {
                result.push((nums[i], nums[j], nums[k]));
            }
            else if sum < 0
            {
                j += 1;
            }
            else
            {
                k -= 1;
            }
        }
    }

    result.into_iter().collect()
}

#[cfg(test)]
mod tests
{
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(vec![-1, 0, 1, 2, -1, 4], vec![(-1, -1, 2), (-1, 0, 1)])]
    #[case(vec![0, 1, 1], vec![])]
    #[case(vec![0, 0, 0], vec![(0, 0, 0)])]
    fn test_three_sum(#[case] mut v: Vec<i32>, #[case] expected: Vec<(i32, i32, i32)>)
    {
        let result = three_sum(&mut v);

        assert_eq!(result, expected);
    }
}
