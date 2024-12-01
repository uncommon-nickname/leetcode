use std::collections::HashSet;

fn check_if_exists(arr: &[i32]) -> bool
{
    let mut memory = HashSet::with_capacity(arr.len());

    for item in arr.iter()
    {
        if memory.contains(&(item * 2)) || (*item % 2 == 0 && memory.contains(&(item / 2)))
        {
            return true;
        }
        memory.insert(*item);
    }
    false
}

#[cfg(test)]
mod tests
{
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(vec![10, 2, 5, 3], true)]
    #[case(vec![3, 1, 7, 11], false)]
    fn test_check_if_exists(#[case] arr: Vec<i32>, #[case] expected: bool)
    {
        let result = check_if_exists(&arr);

        assert_eq!(result, expected);
    }
}
