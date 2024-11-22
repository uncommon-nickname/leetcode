use std::collections::hash_map::Entry;
use std::collections::HashMap;

fn max_equal_rows_after_flips(matrix: &[Vec<u8>]) -> usize
{
    let mut memory = HashMap::new();

    for row in matrix
    {
        let mut pattern = 0;
        let first = row[0];

        for (idx, element) in row.iter().enumerate()
        {
            pattern |= (element ^ first) << idx;
        }
        match memory.entry(pattern)
        {
            Entry::Occupied(mut e) =>
            {
                e.insert(e.get() + 1);
            }
            Entry::Vacant(e) =>
            {
                e.insert(1);
            }
        };
    }

    memory.values().max().unwrap().to_owned()
}

#[cfg(test)]
mod tests
{
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(vec![vec![0, 1], vec![1, 1]], 1)]
    #[case(vec![vec![0, 1], vec![1, 0]], 2)]
    #[case(vec![vec![0, 0, 0], vec![0, 0, 1], vec![1, 1, 0]], 2)]
    fn test_max_equal_rows(#[case] matrix: Vec<Vec<u8>>, #[case] expected: usize)
    {
        let result = max_equal_rows_after_flips(&matrix);

        assert_eq!(result, expected);
    }
}
