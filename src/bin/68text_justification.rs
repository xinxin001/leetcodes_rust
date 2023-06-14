fn main() {
    let words = vec![
        "This".to_string(),
        "is".to_string(),
        "an".to_string(),
        "example".to_string(),
        "of".to_string(),
        "text".to_string(),
        "justification.".to_string(),
    ];
    let ans = Solution::full_justify(words, 16);
    println!("{:?}", ans);
    for word in ans {
        println!("{}", word.len());
    }
}

/*
1: Calculate total len of words and divide my maxlen to get number of lines (tlen/maxlen).ceil() -> done
2: Greedy pack in as much words per line: if currlen + wordlen < maxlen -> pack in, else newline -> done
3: justify by putting spaces evenly, for line in output:
    - Get total str len
    - (maxwidth - stringlen) = extra spaces needed
    - join by extra whitespaces needed
 */
pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
    let total_chars = words.iter().fold(0, |acc, word| acc + word.len());
    let line_count = (total_chars as f64 / max_width as f64).ceil() as usize;
    let mut output = vec![];
    let mut w_idx = 0;
    for _ in 0..line_count {
        let mut line_string = words[w_idx].to_string();
        w_idx += 1;
        while w_idx < words.len() && (line_string.len() + words[w_idx].len()) <= max_width as usize
        {
            line_string = format!("{} {}", line_string, words[w_idx]);
            w_idx += 1;
        }
        output.push(line_string);
    }
    for line in &mut output {
        let ws: Vec<&str> = line.split(" ").collect();
        let s_len = ws.iter().fold(0, |acc, w| acc + w.len());
        let spaces_needed = max_width as usize - s_len;
        if ws.len() == 1 {
            *line = format!("{}{}", ws[0], " ".repeat(spaces_needed));
            break;
        }
        let gaps = if ws.len() > 1 { ws.len() - 1 } else { 1 };
        let base_spaces = spaces_needed / gaps;
        let extra_spaces = spaces_needed % gaps;
        let mut out = String::new();
        for (i, w) in ws.iter().enumerate() {
            out.push_str(w);
            if i < ws.len() - 1 {
                let spaces = base_spaces + if i < extra_spaces { 1 } else { 0 };
                out.push_str(&" ".repeat(spaces));
            }
        }
        *line = out
    }
    return output;
}

struct Solution {}

impl Solution {
    fn last_line(words: &[String], max_width: i32) -> String {
        let mut res = "".to_owned();
        for (i, w) in words.iter().enumerate() {
            res.push_str(w);
            if i < words.len() - 1 {
                res.push(' ');
            }
        }
        for _ in 0..(max_width - res.len() as i32) {
            res.push(' ');
        }
        res
    }

    fn line_output(words: &[String], words_len: i32, max_width: i32) -> String {
        let mut res = "".to_owned();
        let space_slot_count = (words.len() as i32 - 1).max(1);
        let space_count = max_width - words_len;
        let mut slot_count = space_slot_count;
        let space_per_slot = space_count / space_slot_count;
        let mut rem_space = space_count - space_per_slot * space_slot_count;
        for w in words {
            res.push_str(w);
            if slot_count > 0 {
                for _ in 0..space_per_slot {
                    res.push(' ');
                }
                slot_count -= 1;
            }
            if rem_space > 0 {
                res.push(' ');
                rem_space -= 1;
            }
        }
        res
    }

    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut res = vec![];
        let len_vec: Vec<i32> = words.iter().map(|i| i.len() as i32).collect();
        let mut i = 0;
        while i < words.len() {
            let mut j = i;
            let mut word_count = 0;
            let mut word_len = 0;
            let mut line_width = 0;
            while j < words.len() {
                let insert_len = if word_count == 0 {
                    line_width + len_vec[j]
                } else {
                    line_width + len_vec[j] + 1
                };
                if insert_len <= max_width {
                    line_width += len_vec[j];
                    word_len += len_vec[j];
                    word_count += 1;
                    if word_count > 1 {
                        line_width += 1;
                    }
                } else {
                    break;
                }
                j += 1;
            }
            let s = if j < words.len() {
                Solution::line_output(&words[i..j], word_len, max_width)
            } else {
                Solution::last_line(&words[i..j], max_width)
            };
            res.push(s);
            i = j;
        }
        res
    }
}
