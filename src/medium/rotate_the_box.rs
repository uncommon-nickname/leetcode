type Matrix<T> = Vec<Vec<T>>;

fn rotate_the_box(_box: &Matrix<char>) -> Matrix<char>
{
    let rows = _box.len();
    let cols = _box[0].len();

    let mut result = vec![vec!['.'; rows]; cols];

    for (r, row) in _box.iter().enumerate().take(rows)
    {
        let mut i = cols - 1;

        for (c, col) in row.iter().enumerate().rev()
        {
            if *col == '#'
            {
                result[i][rows - r - 1] = '#';
                i = i.saturating_sub(1);
            }
            else if *col == '*'
            {
                result[c][rows - r - 1] = '*';
                i = c.saturating_sub(1);
            }
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
    #[case(
        vec![vec!['#', '.' ,'#']],
        vec![
            vec!['.'],
            vec!['#'],
            vec!['#']
        ],
    )]
    #[case(
        vec![
            vec!['#', '.', '*', '.'],
            vec!['#', '#', '*', '.']
        ],
        vec![
            vec!['#', '.'],
            vec!['#', '#'],
            vec!['*', '*'],
            vec!['.', '.']
        ]
    )]
    #[case(
        vec![
            vec!['#', '#', '*', '.', '*', '.'],
            vec!['#', '#', '#', '*', '.', '.'],
            vec!['#', '#', '#', '.', '#', '.'],
        ],
        vec![
            vec!['.', '#', '#'],
            vec!['.', '#', '#'],
            vec!['#', '#', '*'],
            vec!['#', '*', '.'],
            vec!['#', '.', '*'],
            vec!['#', '.', '.']
        ],
    )]
    fn test_rotate_the_box(#[case] _box: Matrix<char>, #[case] expected: Matrix<char>)
    {
        let result = rotate_the_box(&_box);

        assert_eq!(result, expected);
    }
}
