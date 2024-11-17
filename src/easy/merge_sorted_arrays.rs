fn merge_sorted_arrays_naive(nums1: &mut [i32], m: usize, nums2: &[i32], n: usize)
{
    nums1[m..(m + n)].copy_from_slice(&nums2[..n]);
    nums1.sort();
}

fn merge_sorted_arrays_two_pointers(nums1: &mut [i32], m: usize, nums2: &[i32], n: usize)
{
    let mut n1_ptr = m as i32 - 1;
    let mut n2_ptr = n as i32 - 1;
    let mut merged_ptr = n as i32 + m as i32 - 1;

    while n2_ptr >= 0
    {
        if n1_ptr >= 0 && nums1[n1_ptr as usize] > nums2[n2_ptr as usize]
        {
            nums1[merged_ptr as usize] = nums1[n1_ptr as usize];
            n1_ptr -= 1;
        }
        else
        {
            nums1[merged_ptr as usize] = nums2[n2_ptr as usize];
            n2_ptr -= 1;
        }
        merged_ptr -= 1;
    }
}

#[cfg(test)]
mod tests
{
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(vec![1, 2, 3, 0, 0, 0], 3, vec![2, 5, 6], 3, vec![1, 2, 2, 3, 5, 6])]
    #[case(vec![1], 1, vec![], 0, vec![1])]
    #[case(vec![0], 0, vec![1], 1, vec![1])]
    fn validate_naive_approach(#[case] mut v1: Vec<i32>, #[case] m: usize, #[case] v2: Vec<i32>,
                               #[case] n: usize, #[case] expected: Vec<i32>)
    {
        merge_sorted_arrays_naive(&mut v1, m, &v2, n);

        assert_eq!(v1, expected);
    }

    #[rstest]
    #[case(vec![1, 2, 3, 0, 0, 0], 3, vec![2, 5, 6], 3, vec![1, 2, 2, 3, 5, 6])]
    #[case(vec![1], 1, vec![], 0, vec![1])]
    #[case(vec![0], 0, vec![1], 1, vec![1])]
    fn validate_two_pointer_approach(#[case] mut v1: Vec<i32>, #[case] m: usize,
                                     #[case] v2: Vec<i32>, #[case] n: usize,
                                     #[case] expected: Vec<i32>)
    {
        merge_sorted_arrays_two_pointers(&mut v1, m, &v2, n);

        assert_eq!(v1, expected);
    }
}
