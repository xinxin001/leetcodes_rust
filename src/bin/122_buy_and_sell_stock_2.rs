//Revisited this in May 20th 2023 for the fold function
pub fn max_profit(prices: Vec<i32>) -> i32 {
    prices
        .iter()
        .fold((prices[0], 0), |(mut holding, mut profit), price| {
            holding = holding.min(*price);
            let sell = *price - holding;
            if sell > 0 {
                profit += sell;
                holding = *price
            }
            (holding, profit)
        })
        .1
}

pub fn peak_valley_max_profit(prices: Vec<i32>) -> i32 {
    let mut i = 0;
    let mut valley = prices[0];
    let mut peak = prices[0];
    let mut max_profit = 0;
    while i < prices.len() - 1 {
        while i < prices.len() - 1 && prices[i] >= prices[i + 1] {
            i += 1;
        }
        valley = prices[i];
        while i < prices.len() - 1 && prices[i] <= prices[i + 1] {
            i += 1;
        }
        peak = prices[i];
        max_profit += peak - valley;
    }
    max_profit
}

fn main() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    println!("{}", max_profit(prices));
}
