fn main() {}
/*
- left right pointer
- calculate volume: (r-l) * min(height[l], height[r])
- store if max
- move smallest ptr closer
time: O(n)
space: O(1)
 */
pub fn max_area(height: Vec<i32>) -> i32 {
    let mut maxvol = 0;
    let mut l = 0;
    let mut r = height.len() - 1;
    while l < r {
        let vol = (r - l) as i32 * height[l].min(height[r]);
        maxvol = maxvol.max(vol);
        if height[l] <= height[r] {
            l += 1
        } else {
            r -= 1
        }
    }
    return maxvol;
}
