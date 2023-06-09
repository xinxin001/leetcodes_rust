pub fn trap(height: Vec<i32>) -> i32 {
    let n = height.len();
    let left_max = height.iter().fold(vec![], |mut a, h| {
        a.push(*h.max(a.last().unwrap_or(&0)));
        a
    });
    let mut right_max = height.iter().rev().fold(vec![], |mut a, h| {
        a.push(*h.max(a.last().unwrap_or(&0)));
        a
    });
    right_max.reverse();
    (0..n).fold(0, |volume, idx| {
        let water = i32::min(left_max[idx], right_max[idx]) - height[idx];
        if water > 0 {
            water + volume
        } else {
            volume
        }
    })
}

pub fn alt_trap(nums: Vec<i32>) -> i32 {
    let index = nums
        .iter()
        .enumerate()
        .fold(
            (0, 0),
            |(i, max), (idx, val)| {
                if val > &max {
                    (idx, *val)
                } else {
                    (i, max)
                }
            },
        )
        .0;

    let sum = |part: &[i32]| {
        let mut max = 0;
        return part.iter().fold(0, |mut sum, e| {
            max = max.max(*e);
            sum += max - e;
            sum
        });
    };

    let mut right = nums[index..].to_vec();
    right.reverse();

    sum(&nums[0..index]) + sum(&right)
}

pub fn new_trap(height: Vec<i32>) -> i32 {
    let n = height.len();
    let mut l_max = vec![0; n];
    let mut r_max = vec![0; n];

    let mut left = 0;
    for i in 0..n {
        left = left.max(height[i]);
        l_max[i] = left;
    }

    let mut right = 0;
    for i in (0..n).rev() {
        right = right.max(height[i]);
        r_max[i] = right;
    }

    let mut sum = 0;
    for i in 0..n {
        let volume = l_max[i].min(r_max[i]) - height[i];
        if volume > 0 {
            sum += volume
        }
    }
    return sum;
}

fn main() {
    let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    println!("{}", alt_trap(height));
}

#[test]
fn test() {
    let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    assert_eq!(6, trap(height));
}
