fn main() {}
pub fn is_palindrome(s: String) -> bool {
    let arr = s.as_bytes();
    let mut l = 0;
    let mut r = arr.len() - 1;
    while l < r {
        if !arr[l].is_ascii_alphanumeric() {
            l += 1;
            continue;
        }
        if !arr[r].is_ascii_alphanumeric() {
            r -= 1;
            continue;
        }
        if arr[l].to_ascii_lowercase() != arr[r].to_ascii_lowercase() {
            return false;
        }
        l += 1;
        r -= 1;
    }
    return true;
}
