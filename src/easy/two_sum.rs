use std::collections::HashMap;

fn find_two_sum(nums: &[i32], target: i32) -> (usize, usize)
{
    let mut visited = HashMap::new();

    for (idx, value) in nums.iter().enumerate()
    {
        let current = target - value;
        let search = visited.get(&current);

        match search
        {
            Some(value) => return (*value, idx),
            None => visited.insert(*value, idx),
        };
    }
    (0, 0)
}

#[cfg(test)]
mod tests
{
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(vec![2, 7, 11, 15], 9, (0, 1))]
    #[case(vec![3, 2, 4], 6, (1, 2))]
    #[case(vec![3, 3], 6, (0, 1))]
    fn test_find_two_sum(#[case] v1: Vec<i32>, #[case] target: i32,
                         #[case] expected: (usize, usize))
    {
        let result = find_two_sum(&v1, target);

        assert_eq!(result, expected);
    }
}
