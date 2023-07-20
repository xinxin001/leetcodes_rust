use std::collections::VecDeque;

fn main() {}

struct HitCounter {
    q: VecDeque<(i32, usize)>,
    total_hits: usize,
}

impl HitCounter {
    fn new() -> HitCounter {
        HitCounter {
            q: VecDeque::new(),
            total_hits: 0,
        }
    }
    fn hit(&mut self, timestamp: i32) {
        match self.q.back_mut() {
            Some((t, n)) if *t == timestamp => {
                *n += 1;
            }
            _ => self.q.push_back((timestamp, 1)),
        }
        self.total_hits += 1;
    }

    fn get_hits(&mut self, timestamp: i32) -> i32 {
        while self.q.front().map_or(false, |&(t, _)| timestamp - 300 >= t) {
            let (_, n) = self.q.pop_front().unwrap();
            self.total_hits -= n;
        }
        return self.total_hits as i32;
    }
}
