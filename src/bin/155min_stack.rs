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
