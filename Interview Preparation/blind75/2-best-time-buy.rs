//  Calculate the max profit
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_profit = 0;
    let mut buy_price = prices[0];

    for i in 1..prices.len() {
        let profit: i32 = prices[i] - buy_price;
        if profit > max_profit {
            max_profit = profit;
            continue;
        }
        if prices[i] < buy_price {
            buy_price = prices[i];
        }
    }

    return max_profit;
}

fn main() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    let profit = max_profit(prices);
    println!("Profit = {}", profit);
}
