fn divide(dividend: i32, divisor: i32) -> i32
{
    let mut result = 0;
    let mut abs_dividend = dividend.abs();
    let abs_divisor = divisor.abs();

    while abs_dividend >= abs_divisor
    {
        let mut temp = abs_divisor;
        let mut multiple = 1;

        while abs_dividend >= (temp << 1)
        {
            temp <<= 1;
            multiple <<= 1;
        }
        abs_dividend -= temp;
        result += multiple;
    }

    if dividend.is_negative() ^ divisor.is_negative()
    {
        return -result;
    }
    result
}

#[cfg(test)]
mod tests
{
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(10, 3, 3)]
    #[case(7, -3, -2)]
    fn test_divide_two_integers(#[case] dividend: i32, #[case] divisor: i32, #[case] expected: i32)
    {
        let result = divide(dividend, divisor);

        assert_eq!(result, expected);
    }
}
