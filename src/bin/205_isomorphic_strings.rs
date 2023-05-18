use std::collections::HashMap;

pub fn is_isomorphic(s: String, t: String) -> bool {
    let mut s_to_t: HashMap<char, char> = HashMap::new();
    let mut t_to_s: HashMap<char, char> = HashMap::new();

    for (schar, tchar) in s.chars().zip(t.chars()) {
        if !s_to_t.contains_key(&schar) && !t_to_s.contains_key(&tchar) {
            s_to_t.insert(schar, tchar);
            t_to_s.insert(tchar, schar);
        } else {
            match (s_to_t.get(&schar), t_to_s.get(&tchar)) {
                (Some(x), _) => if x != &tchar { return false },
                (_, Some(y)) => if y != &schar { return false },
                (None, _) => return false,
            }
        }
    }
    return true;
}
fn main() {
    let s = "egg".to_string();
    let t = "att".to_string();
    println!("{:?}", is_isomorphic(s, t));
}

#[test]
fn test() {}