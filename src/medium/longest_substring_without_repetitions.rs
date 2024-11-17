use std::cmp::max;

fn find_longest_substring(s: String) -> usize
{
    let mut substr_length = 0;
    let mut idx_memory = vec![-1_i32; 256];

    let mut left = 0;

    for (right, c) in s.chars().enumerate()
    {
        left = max(left, idx_memory[c as usize] + 1);
        substr_length = max(substr_length, right - left as usize + 1);
        idx_memory[c as usize] = right as i32;
    }

    substr_length
}

#[cfg(test)]
mod tests
{
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("abcabcbb", 3)]
    #[case("bbbbb", 1)]
    #[case("pwwkew", 3)]
    fn test_longest_substring(#[case] s: String, #[case] expected: usize)
    {
        let result = find_longest_substring(s);

        assert_eq!(result, expected);
    }
}
