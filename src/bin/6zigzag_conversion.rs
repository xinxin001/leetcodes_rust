fn main() {
    let s = "AB".to_string();
    let ans = convert(s, 1);
    println!("{ans}");
}

pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows < 2 {
        return s;
    }
    let s = s.chars();
    let mut ans = vec![String::new(); num_rows as usize];
    let mut idx = 0;
    let mut asc = true;
    for c in s {
        ans[idx as usize].push(c);
        idx += if asc { 1 } else { -1 };
        if idx == num_rows - 1 {
            asc = false
        } else if idx == 0 {
            asc = true
        }
    }
    ans.join("")
}

pub fn iter_convert(s: String, num_rows: i32) -> String {
    let mut zigzags: Vec<_> = (0..num_rows)
        .chain((1..num_rows - 1).rev())
        .cycle()
        .zip(s.chars())
        .collect();
    zigzags.sort_by_key(|&(row, _)| row);
    zigzags.into_iter().map(|(_, c)| c).collect()
}

pub fn elevator_convert(s: String, num_rows: i32) -> String {
    if num_rows < 2 {
        return s.into();
    }
    let mut floors: Vec<String> = vec![String::from(""); num_rows as usize];
    let mut floor = 0i32;
    let mut down = true;

    for c in s.chars() {
        floors[floor as usize].push(c);
        floor += if down { 1 } else { -1 };
        if floor == num_rows - 1 {
            down = false;
        } else if floor == 0 {
            down = true;
        }
    }

    floors.concat()
}
