/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
fn main() {}
struct MinStack {
    stack: Vec<i32>,
    mins: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: vec![],
            mins: vec![],
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);
        if val <= *self.mins.last().unwrap_or(&i32::MAX) {
            self.mins.push(val);
        }
    }

    fn pop(&mut self) {
        if let Some(x) = self.stack.pop() {
            if x == *self.mins.last().unwrap() {
                self.mins.pop();
            }
        }
        dbg!(&self.stack);
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.mins.last().unwrap()
    }
}

struct MinStackPairs {
    s: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStackPairs {
    fn new() -> Self {
        MinStackPairs {
            s: Vec::with_capacity(30000),
        }
    }

    fn push(&mut self, val: i32) {
        if self.s.is_empty() {
            self.s.push((val, val));
        } else {
            let min = std::cmp::min(val, self.s[self.s.len() - 1].1);
            self.s.push((val, min));
        }
    }

    fn pop(&mut self) {
        self.s.pop();
    }

    fn top(&self) -> i32 {
        self.s.last().unwrap().0
    }

    fn get_min(&self) -> i32 {
        self.s.last().unwrap().1
    }
}
