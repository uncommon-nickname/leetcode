use std::cmp::{min, Reverse};
use std::collections::{BinaryHeap, HashSet};

fn minimum_obstacles(grid: &[Vec<u8>]) -> i32
{
    const POSSIBLE_DIR: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let mut heap = BinaryHeap::new();
    // We need a min heap, rusts default is a max heap.
    // A quick fix is to use reverse wrapped value.
    heap.push(Reverse((0, 0, 0)));

    let mut visited = HashSet::new();
    visited.insert((0, 0));

    while !heap.is_empty()
    {
        let Reverse((cost, r, c)) = heap.pop().unwrap();

        if r == rows - 1 && c == cols - 1
        {
            return cost;
        }

        for (dr, dc) in POSSIBLE_DIR
        {
            let new_r = r + dr;
            let new_c = c + dc;

            if min(new_r, new_c) < 0
               || new_r == rows
               || new_c == cols
               || visited.contains(&(new_r, new_c))
            {
                continue;
            }

            visited.insert((new_r, new_c));

            let new_cost = cost + grid[new_r as usize][new_c as usize] as i32;
            heap.push(Reverse((new_cost, new_r, new_c)));
        }
    }
    -1
}

#[cfg(test)]
mod tests
{
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(vec![vec![0, 1, 1], vec![1, 1, 0], vec![1, 1, 0]], 2)]
    #[case(vec![vec![0, 1, 0, 0, 0], vec![0, 1, 0, 1, 0], vec![0, 0, 0, 1, 0]], 0)]
    fn test_minimum_obstacles(#[case] grid: Vec<Vec<u8>>, #[case] expected: i32)
    {
        let result = minimum_obstacles(&grid);

        assert_eq!(result, expected);
    }
}
