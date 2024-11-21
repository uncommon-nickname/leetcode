const STORAGE: [(u32, &str); 13] = [(1000, "M"),
                                    (900, "CM"),
                                    (500, "D"),
                                    (400, "CD"),
                                    (100, "C"),
                                    (90, "XC"),
                                    (50, "L"),
                                    (40, "XL"),
                                    (10, "X"),
                                    (9, "IX"),
                                    (5, "V"),
                                    (4, "IV"),
                                    (1, "I")];

fn integer_to_roman(mut num: u32) -> String
{
    let mut result = Vec::new();

    for (number, roman) in STORAGE
    {
        while num >= number
        {
            result.push(roman);
            num -= number;
        }
    }
    result.join("")
}

#[cfg(test)]
mod tests
{
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(3749, "MMMDCCXLIX")]
    #[case(58, "LVIII")]
    #[case(1994, "MCMXCIV")]
    fn test_integer_to_romans(#[case] num: u32, #[case] expected: String)
    {
        let result = integer_to_roman(num);

        assert_eq!(result, expected);
    }
}
