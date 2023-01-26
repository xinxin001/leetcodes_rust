pub fn max_profit(prices: Vec<i32>) -> i32 {
    let n = prices.len();
    let (
        mut neutral,
        mut buy,
        mut sell
    ) = (vec![0; n], vec![-prices[0]; n], vec![i32::MIN; n]);

    for i in 1..n {
        neutral[i] = neutral[i-1].max(sell[i-1]);
        buy[i] = buy[i-1].max(neutral[i-1] - prices[i]);
        sell[i] = buy[i-1] + prices[i];
    }
    i32::max(sell[n-1], neutral[n-1])
}

struct FSM {
    no_stock: i32, //-> previously no_stock || cooldown ended
    stock: i32, //-> previously holding stock || bought stock
    cooldown: i32, //-> previously sold stock
}

pub fn fsm_max_profit(prices: Vec<i32>) -> i32 {
    let final_state = prices.iter()
        .fold(
        FSM {no_stock: 0, stock: i32::MIN, cooldown: i32::MIN},
        |prev, price| {
            FSM {
                no_stock: i32::max(prev.no_stock, prev.cooldown),
                stock: i32::max(prev.stock, prev.no_stock - price),
                cooldown: prev.stock + price
            }
        });
    i32::max(final_state.no_stock, final_state.cooldown)
}
fn main() {
    let prices = vec![1,2,3,0,2];
    println!("{}", max_profit(prices));
}

#[test]
fn test() {
    let prices = vec![1,2,3,0,2];
    assert_eq!(max_profit(prices.clone()), fsm_max_profit(prices.clone()));
}