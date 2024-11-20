use std::cmp::min;

fn take_characters(s: String, k: usize) -> i32
{
    let a_count = s.chars().filter(|c| *c == 'a').count();
    let b_count = s.chars().filter(|c| *c == 'b').count();
    let c_count = s.chars().filter(|c| *c == 'c').count();

    if a_count < k || b_count < k || c_count < k
    {
        return -1;
    }

    let chars: Vec<char> = s.chars().collect();
    let size = s.len();
    let mut result = size as i32;

    let mut a = 0;
    let mut b = 0;
    let mut c = 0;

    let mut lptr = 0;
    let mut rptr = 0;

    while rptr < size
    {
        if chars[rptr] == 'a'
        {
            a += 1;
        }
        else if chars[rptr] == 'b'
        {
            b += 1;
        }
        else
        {
            c += 1;
        }
        rptr += 1;

        while a > a_count - k || b > b_count - k || c > c_count - k
        {
            if chars[lptr] == 'a'
            {
                a -= 1;
            }
            else if chars[lptr] == 'b'
            {
                b -= 1;
            }
            else
            {
                c -= 1;
            }
            lptr += 1;
        }
        result = min(result, size as i32 - (rptr as i32 - lptr as i32));
    }
    result
}

#[cfg(test)]
mod tests
{
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("aabaaaacaabc", 2, 8)]
    #[case("a", 1, -1)]
    fn test_take_characters(#[case] s: String, #[case] k: usize, #[case] expected: i32)
    {
        let result = take_characters(s, k);

        assert_eq!(result, expected);
    }
}
