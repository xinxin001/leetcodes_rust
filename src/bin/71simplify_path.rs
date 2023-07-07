fn main() {
    let s1 = "/home/../foo/".to_string();
    let a1 = simplify_path(s1);
    println!("{a1}");
}

pub fn simplify_path(path: String) -> String {
    let chunks = path.split("/").collect::<Vec<&str>>();
    dbg!(&chunks);
    let mut output = Vec::with_capacity(chunks.len());
    for chunk in chunks {
        match chunk {
            ".." => {
                output.pop();
            }
            "" | "." => (),
            _ => output.push(chunk),
        }
    }
    let path = output.join("/");
    return format!("/{}", path);
}

pub fn iter_simplify_path(path: String) -> String {
    let mut stack = Vec::new();
    path.split('/')
        .filter(|s| !s.is_empty() && s != &".")
        .for_each(|s| {
            if s == ".." {
                stack.pop();
            } else {
                stack.push(s);
            }
        });
    let mut path = stack.join("/");
    path.insert(0, '/');
    path
}
