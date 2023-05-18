pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
    prices.iter()
        .fold((0, i32::MIN+fee), |(mut tx1, mut tx2), price| {
            let old_tx1 = tx1;
            tx1 = i32::max(tx1, tx2 + price - fee);
            tx2 = i32::max(tx2, old_tx1 - price);
            (tx1, tx2)
        }).0
}

struct FSM {
    buy: i32,
    sell: i32,
}
pub fn fsm_max_profit(prices: Vec<i32>, fee: i32) -> i32 {
    prices.iter()
        .skip(1)
        .fold(FSM {buy: -prices[0]-fee, sell: 0},
        |prev, price| {
            FSM {
                buy: i32::max(prev.buy, prev.sell - price - fee),
                sell: i32::max(prev.sell, prev.buy + price)
            }
        }).sell
}

pub fn it_max_profit(prices: Vec<i32>, fee: i32) -> i32 {
    prices.iter().skip(1)
        .fold((0, -prices[0]), |(cash, hold), price| {
            (cash.max(price + hold - fee), hold.max(cash - price))
        }).0
}
fn main() {}

#[test]
fn test() {
    let prices = vec![1,3,2,8,4,9];
    let fee = 2;
    assert_eq!(8, fsm_max_profit(prices, fee));
}