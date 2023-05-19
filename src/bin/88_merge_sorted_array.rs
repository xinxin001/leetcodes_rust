fn main() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];
    merge(&mut nums1, 3, &mut nums2, 3);
    dbg!(nums1);
}

fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let (mut p1, mut p2) = ((m - 1), (n - 1));
    for i in (0..nums1.len()).rev() {
        if p1 >= 0 && p2 >= 0 {
            if nums1[p1 as usize] > nums2[p2 as usize] {
                nums1[i] = nums1[p1 as usize];
                p1 -= 1;
            } else {
                nums1[i] = nums2[p2 as usize];
                p2 -= 1;
            }
        } else if p2 >= 0 {
            nums1[i] = nums2[p2 as usize];
            p2 -= 1;
        } else if p1 >= 0 {
            nums1[i] = nums1[p1 as usize];
            p1 -= 1;
        }
    }
}

fn alt_merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let (mut p1, mut p2) = (m as usize, n as usize);
    let mut i = (m + n) as usize;
    while p1 > 0 && p2 > 0 {
        i -= 1;
        if nums1[p1 - 1] > nums2[p2 - 1] {
            nums1[i] = nums1[p1 - 1];
            p1 -= 1;
        } else {
            nums1[i] = nums2[p2 - 1];
            p2 -= 1;
        }
    }
    nums1[..p2].copy_from_slice(&nums2[..p2]);
}
