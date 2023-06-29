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
                (Some(x), _) => {
                    if x != &tchar {
                        return false;
                    }
                }
                (_, Some(y)) => {
                    if y != &schar {
                        return false;
                    }
                }
                (None, _) => return false,
            }
        }
    }
    return true;
}

pub fn alt_is_isomorphic(s: String, t: String) -> bool {
    let mut s_map_t: HashMap<char, char> = HashMap::new();
    let mut t_map_s: HashMap<char, char> = HashMap::new();
    for (sc, tc) in s.chars().zip(t.chars()) {
        if !s_map_t.contains_key(&sc) && !t_map_s.contains_key(&tc) {
            s_map_t.insert(sc, tc);
            t_map_s.insert(tc, sc);
        } else {
            match (s_map_t.get(&sc), t_map_s.get(&tc)) {
                (Some(x), _) => {
                    if x != &tc {
                        return false;
                    }
                }
                (_, Some(y)) => {
                    if y != &sc {
                        return false;
                    }
                }
                (None, _) => return false,
            }
        }
    }
    return true;
}

pub fn alt1_is_isomorphic(s: String, t: String) -> bool {
    let mut fwd = HashMap::with_capacity(s.len());
    let mut bwd = HashMap::with_capacity(s.len());

    for (s, t) in s.chars().zip(t.chars()) {
        if let Some(old) = fwd.insert(s, t) {
            if old != t {
                return false;
            }
        }

        if let Some(old) = bwd.insert(t, s) {
            if old != s {
                return false;
            }
        }
    }

    true
}
fn main() {
    let s = "egg".to_string();
    let t = "att".to_string();
    println!("{:?}", is_isomorphic(s, t));
}

#[test]
fn test() {}
