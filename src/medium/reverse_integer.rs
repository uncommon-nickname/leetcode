fn reverse_integer(mut value: i32) -> i32
{
    let mut reversed: i32 = 0;

    while value != 0
    {
        match reversed.checked_mul(10).map(|v| v.checked_add(value % 10)).flatten()
        {
            Some(v) => reversed = v,
            None => return 0,
        };
        value /= 10;
    }

    reversed
}

#[cfg(test)]
mod tests
{
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(123, 321)]
    #[case(-123, -321)]
    #[case(120, 21)]
    #[case(1534236469, 0)]
    fn test_reverse_integer(#[case] v: i32, #[case] expected: i32)
    {
        let result = reverse_integer(v);

        assert_eq!(result, expected);
    }
}
