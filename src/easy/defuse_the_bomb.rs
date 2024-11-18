fn build_needed_indicies(size: usize, k: i32, lookup_idx: usize) -> Vec<usize>
{
    let abs_k = k.unsigned_abs() as usize;
    let mut result = Vec::with_capacity(abs_k);

    if k > 0
    {
        for idx in 1..abs_k + 1
        {
            result.push((lookup_idx + idx) % size);
        }
    }
    else
    {
        for idx in 1..abs_k + 1
        {
            let calculated = (lookup_idx as i32 - idx as i32) % size as i32;

            if calculated < 0
            {
                result.push((calculated + size as i32) as usize);
            }
            else
            {
                result.push(calculated as usize);
            }
        }
    }

    result
}

fn defuse_the_bomb(code: &[i32], k: i32) -> Vec<i32>
{
    let size = code.len();
    let mut result = vec![0; size];

    if k == 0
    {
        return result;
    }

    for (idx, value) in result.iter_mut().enumerate()
    {
        let indicies = build_needed_indicies(size, k, idx);

        for calculated in indicies
        {
            *value += code[calculated];
        }
    }

    result
}

#[cfg(test)]
mod tests
{
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(vec![5, 7, 1, 4], 3, vec![12, 10, 16, 13])]
    #[case(vec![1, 2, 3, 4], 0, vec![0, 0, 0, 0])]
    #[case(vec![2, 4, 9, 3], -2, vec![12, 5, 6, 13])]
    fn test_defuse_the_bomb(#[case] code: Vec<i32>, #[case] k: i32, #[case] expected: Vec<i32>)
    {
        let result = defuse_the_bomb(&code, k);

        assert_eq!(result, expected);
    }
}
