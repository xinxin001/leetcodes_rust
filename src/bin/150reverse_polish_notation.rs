fn main() {
    let tokens: Vec<String> = vec![
        "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
    ]
    .iter()
    .map(|x| x.to_string())
    .collect();
    let ans = eval_rpn(tokens);
    println!("{ans}");
}

pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut buffer = vec![];
    for t in tokens {
        if let Ok(num) = t.parse::<i32>() {
            buffer.push(num);
        } else {
            let num2 = buffer.pop().unwrap();
            let num1 = buffer.pop().unwrap();
            let res = match &t[..] {
                "+" => num1 + num2,
                "-" => num1 - num2,
                "*" => num1 * num2,
                "/" => num1 / num2,
                _ => 0,
            };
            buffer.push(res)
        }
    }
    return buffer[0];
}
