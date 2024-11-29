fn is_valid(s: String) -> bool
{
    let mut stack = Vec::new();

    for c in s.chars()
    {
        if c == '{' || c == '[' || c == '('
        {
            stack.push(c);
        }
        else
        {
            let Some(last_char) = stack.last()
            else
            {
                return false;
            };

            if (c == '}' && *last_char != '{')
               || (c == ']' && *last_char != '[')
               || (c == ')' && *last_char != '(')
            {
                return false;
            }
            stack.pop();
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests
{
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("()", true)]
    #[case("()[]{}", true)]
    #[case("(]", false)]
    #[case("([])", true)]
    fn test_valid_parentheses(#[case] s: String, #[case] expected: bool)
    {
        let result = is_valid(s);

        assert_eq!(result, expected);
    }
}
