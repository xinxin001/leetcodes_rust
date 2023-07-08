fn main() {
    let s = "(1+(4+5+2)-3)+(6+8)".to_string();
    let ans = calculate(s);
    dbg!(ans);
}

pub fn calculate(s: String) -> i32 {
    let mut stack = Vec::with_capacity(s.len());
    let mut total = 0;

    let mut cur_num = 0;
    let mut sign = 1;
    for c in s.chars() {
        if c.is_numeric() {
            cur_num = cur_num * 10 + (c.to_digit(10).unwrap() as i32);
            continue;
        }
        match c {
            '+' => {
                total += cur_num * sign;
                sign = 1;
                cur_num = 0;
            }
            '-' => {
                total += cur_num * sign;
                sign = -1;
                cur_num = 0
            }
            '(' => {
                stack.push(total);
                stack.push(sign);
                sign = 1;
                total = 0;
            }
            ')' => {
                total += cur_num * sign;
                total *= stack.pop().unwrap();
                total += stack.pop().unwrap();
                cur_num = 0;
            }
            _ => (),
        }
    }
    return total + (sign * cur_num);
}

pub fn alt_calculate(s: String) -> i32 {
    let mut stack: Vec<i32> = Vec::new();
    let mut answer = 0;
    let mut num = 0;
    let mut sign = 1;
    stack.push(sign);

    for ch in s.chars() {
        match ch {
            ' ' => {}
            '+' => {
                answer += num * sign;
                sign = *stack.last().unwrap();
                num = 0;
            }
            '-' => {
                answer += num * sign;
                sign = -*stack.last().unwrap();
                num = 0;
            }
            '(' => {
                stack.push(sign);
            }
            ')' => {
                stack.pop();
            }
            _ => {
                num = num * 10 + (ch as i32 - '0' as i32);
            }
        }
    }

    answer += num * sign;
    answer
}
