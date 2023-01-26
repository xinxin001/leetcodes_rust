pub fn max_profit(prices: Vec<i32>) -> i32 {
    prices.iter()
        .fold((prices[0], 0), |(mut holding, mut profit), price| {
            holding = holding.min(*price);
            let sell = *price - holding;
            if sell > 0 { profit += sell; holding = *price }
            (holding,profit)
        }).1
}
fn main() {
    let prices = vec![7,1,5,3,6,4];
    println!("{}", max_profit(prices));
}