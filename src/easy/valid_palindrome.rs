fn is_palindrome(s: String) -> bool
{
    let mut lptr = 0;
    let mut rptr = s.len() - 1;

    let chars: Vec<char> = s.to_ascii_lowercase().chars().collect();

    while lptr < rptr
    {
        if !chars[lptr].is_alphanumeric()
        {
            lptr += 1;
            continue;
        }
        if !chars[rptr].is_alphanumeric()
        {
            rptr -= 1;
            continue;
        }
        if chars[lptr] != chars[rptr]
        {
            return false;
        }
        lptr += 1;
        rptr -= 1;
    }
    true
}

#[cfg(test)]
mod tests
{
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("A man, a plan, a canal: Panama", true)]
    #[case("race a car", false)]
    #[case(" ", true)]
    fn test_is_palindrome(#[case] s: String, #[case] expected: bool)
    {
        let result = is_palindrome(s);

        assert_eq!(result, expected);
    }
}
