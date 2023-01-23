pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    cost.iter()
        .chain(&[0])
        .fold((0,0), |(x,y), t| (y, t + x.min(y))).1
}

fn main() {
    let cost = vec![10,15,20];
    /*
    (0,0)
    10 - (0, 10)
    15 - (10, 15)
    30 - (15, 30)
    0 - (30, 15).1
     */
    println!("{}", min_cost_climbing_stairs(cost));
    let cost1 = vec![1,100,1,1,1,100,1,1,100,1];
    println!("{}", min_cost_climbing_stairs(cost1));
}