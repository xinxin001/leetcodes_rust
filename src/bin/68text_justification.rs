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
    let ans = full_justify(words, 16);
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

pub fn alt_full_justify(words: Vec<String>, max_len: usize) -> Vec<String> {
    let mut result = vec![];

    let mut line_len = 0;
    let mut from = 0;

    for idx in 0..words.len() {
        line_len += words[idx].len();

        if idx < words.len() - 1 {
            if line_len + words[idx + 1].len() + (idx - from) + 1 <= max_len {
                continue;
            }
        }

        let mut line = String::with_capacity(max_len);

        // do full justification if it's not the last line
        if idx < words.len() - 1 {
            let word_count = idx - from + 1;
            let all_spaces = max_len - line_len;

            let mut eq_spaces = 0;
            let mut additional = 0;
            if word_count > 1 {
                eq_spaces = all_spaces / (word_count - 1);
                additional = all_spaces % (word_count - 1);
            }

            for word in &words[from..=idx] {
                if !line.is_empty() {
                    let mut spaces = eq_spaces;
                    if additional > 0 {
                        spaces += 1;
                        additional -= 1;
                    }

                    (0..spaces).for_each(|_| line.push(' '));
                }

                line.push_str(word);
            }
        } else {
            for word in &words[from..] {
                if !line.is_empty() {
                    line.push(' ');
                }
                line.push_str(word);
            }
        }

        // in case of the last row, which should be left-justified
        // or in case of a single word on the line
        while line.len() < max_len {
            line.push(' ');
        }

        result.push(line);

        // reset the state for the next row
        from = idx + 1;
        line_len = 0;
    }

    result
}
