use std::collections::HashMap;

fn valid_arrangement(pairs: &[Vec<i32>]) -> Vec<Vec<i32>>
{
    let size = pairs.len();
    let mut adj = HashMap::with_capacity(size);
    let mut in_out = HashMap::<i32, i32>::with_capacity(size * 2);

    for pair in pairs
    {
        adj.entry(pair[0]).or_insert_with(Vec::new).push(pair[1]);

        *in_out.entry(pair[0]).or_default() += 1;
        *in_out.entry(pair[1]).or_default() -= 1;
    }

    let mut starting_node = pairs[0][0];

    for (node, deg) in &in_out
    {
        if *deg == 1
        {
            starting_node = *node;
            break;
        }
    }

    let mut path = Vec::with_capacity(size);
    let mut stack = vec![starting_node];

    while !stack.is_empty()
    {
        if let Some(neighbors) = adj.get_mut(stack.last().unwrap())
        {
            if neighbors.is_empty()
            {
                path.push(stack.pop().unwrap());
            }
            else
            {
                stack.push(neighbors.pop().unwrap());
            }
        }
        else
        {
            path.push(stack.pop().unwrap());
        }
    }

    let mut arrangement = Vec::with_capacity(size);

    for i in (1..path.len()).rev()
    {
        arrangement.push(vec![path[i], path[i - 1]]);
    }
    arrangement
}

#[cfg(test)]
mod tests
{
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        vec![vec![5, 1], vec![4, 5], vec![11, 9], vec![9, 4]],
        vec![vec![11, 9], vec![9, 4], vec![4, 5], vec![5, 1]],
    )]
    #[case(
        vec![vec![1, 3], vec![3, 2], vec![2, 1]],
        vec![vec![1, 3], vec![3, 2], vec![2, 1]],
    )]
    #[case(
        vec![vec![1, 2], vec![1, 3], vec![2, 1]],
        vec![vec![1, 2], vec![2, 1], vec![1, 3]],
    )]
    fn test_valid_arrangement(#[case] pairs: Vec<Vec<i32>>, #[case] expected: Vec<Vec<i32>>)
    {
        let result = valid_arrangement(&pairs);

        assert_eq!(result, expected);
    }
}
