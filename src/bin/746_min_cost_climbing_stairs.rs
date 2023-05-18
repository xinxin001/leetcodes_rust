#![allow(unused)]

use std::collections::HashMap;

fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    cost.iter()
        .chain(&[0])
        .fold((0, 0), |(x, y), t| (y, t + x.min(y)))
        .1
}

fn bdp_min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let mut minimum_cost = vec![0; cost.len() + 1 as usize];
    for i in 2..(cost.len() + 1) {
        let take_one_step = minimum_cost[i - 1] + cost[i - 1];
        let take_two_step = minimum_cost[i - 2] + cost[i - 2];
        minimum_cost[i] = take_one_step.min(take_two_step);
    }
    return *minimum_cost.last().unwrap();
}

fn tdp_min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    fn helper(i: usize, memo: &mut HashMap<usize, i32>, cost: &[i32]) -> i32 {
        if i <= 1 {
            return 0;
        }
        if memo.contains_key(&i) {
            return *memo.get(&i).unwrap();
        }
        let down_one = cost[i as usize - 1] + helper(i - 1, memo, cost);
        let down_two = cost[i as usize - 2] + helper(i - 2, memo, cost);
        memo.insert(i, down_one.min(down_two));
        return *memo.get(&i).unwrap();
    }
    let mut memo: HashMap<usize, i32> = HashMap::new();
    return helper(cost.len(), &mut memo, &cost);
}

fn main() {
    let cost = vec![10, 15, 20];
    /*
    (0,0)
    10 - (0, 10)
    15 - (10, 15)
    30 - (15, 30)
    0 - (30, 15).1
     */
    println!("{}", tdp_min_cost_climbing_stairs(cost));
    let cost1 = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
    println!("{}", tdp_min_cost_climbing_stairs(cost1));
}
