pub fn max_profit(prices: Vec<i32>) -> i32 {
    let (mut lowest, mut profit) = (i32::MAX, i32::MIN);
    for price in prices {
        lowest = lowest.min(price);
        profit = profit.max(price - lowest);
    }
    profit
}

//Revisited on May 20th 2023 for how to use the fold function
pub fn max_profit1(prices: Vec<i32>) -> i32 {
    prices
        .iter()
        .fold((i32::MAX, i32::MIN), |(mut lowest, mut profit), price| {
            lowest = lowest.min(*price);
            profit = profit.max(*price - lowest);
            (lowest, profit)
        })
        .1
}
fn main() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    println!("{}", max_profit1(prices));
}
