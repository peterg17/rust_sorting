#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use sorting::helpers;

    #[test]
    fn normal_sorting() {
        let mut nums = vec![5, 3, 20, 1, 4];
        sorting::insertion_sort(&mut nums);
        assert_eq!(nums, [1, 3, 4, 5, 20]);
    }

    #[test]
    fn empty_edge_case() {
        let mut nums = Vec::new();
        sorting::insertion_sort(&mut nums);
        assert_eq!(nums, []);
    }

    #[test]
    fn one_element_edge_case() {
        let mut nums = vec![1];
        sorting::insertion_sort(&mut nums);
        assert_eq!(nums, [1]);
    }

    #[test]
    fn test_mergesort_normal() {
        let mut nums = helpers::random_vec(1000);
        let nums_len = nums.len();
        sorting::merge_sort(&mut nums, 0, nums_len - 1);
        assert!(helpers::is_sorted(&nums));
    }
}
