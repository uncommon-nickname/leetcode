fn merge_sorted_arrays_naive(nums1: &mut [i32], m: usize, nums2: &[i32], n: usize)
{
    nums1[m..(m + n)].copy_from_slice(&nums2[..n]);
    nums1.sort();
}

fn merge_sorted_arrays_two_pointers(nums1: &mut [i32], m: usize, nums2: &[i32], n: usize)
{
    let mut n1_ptr = (m - 1) as i32;
    let mut n2_ptr = (n - 1) as i32;
    let mut merged_ptr = (n + m - 1) as i32;

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
    use super::*;

    #[test]
    fn validate_naive_approach()
    {
        let mut v1 = vec![1_i32, 2, 3, 4, 5, 0, 0, 0];
        let v2 = vec![1_i32, 5, 9];

        merge_sorted_arrays_naive(&mut v1, 5, &v2, 3);

        assert_eq!(v1, vec![1, 1, 2, 3, 4, 5, 5, 9]);
    }

    #[test]
    fn validate_two_pointer_approach()
    {
        let mut v1 = vec![1_i32, 2, 3, 4, 5, 0, 0, 0];
        let v2 = vec![1_i32, 5, 9];

        merge_sorted_arrays_two_pointers(&mut v1, 5, &v2, 3);

        assert_eq!(v1, vec![1, 1, 2, 3, 4, 5, 5, 9]);
    }
}
