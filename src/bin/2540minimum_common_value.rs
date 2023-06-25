fn main() {
    let nums1 = vec![1, 2, 3, 6];
    let nums2 = vec![2, 3, 4, 5];
    let ans = get_common(nums1, nums2);
    println!("{ans}");
}
use std::cmp::Ordering;

/*
Basic iterative solution
time: O(n+m)
space: O(1)
Can also do binary search, but time complexity is not better
 */
pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let (mut l1, mut l2) = (0, 0);
    while l1 < nums1.len() && l2 < nums2.len() {
        match nums1[l1].cmp(&nums2[l2]) {
            Ordering::Equal => return nums1[l1],
            Ordering::Less => l1 += 1,
            Ordering::Greater => l2 += 1,
        }
    }
    -1
}
