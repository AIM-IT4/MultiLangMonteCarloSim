use rand::rngs::ThreadRng;
use rand::Rng;
use rand_distr::{Distribution, Normal};
use std::time::Instant;

fn main() {
    let risk_free_rate: f64 = 0.01;
    let volatility: f64 = 0.2;
    let initial_stock_price: f64 = 100.0;
    let strike_price: f64 = 100.0;
    let time_to_maturity: f64 = 1.0; // 1 year
    let number_of_simulations: usize = 1_000_000;

    let now = Instant::now();

    let option_price = simulate_black_scholes(
        risk_free_rate,
        volatility,
        initial_stock_price,
        strike_price,
        time_to_maturity,
        number_of_simulations,
    );

    println!("European Call Option Price = {}", option_price);
    println!("Execution time: {} seconds", now.elapsed().as_secs_f64());
}

fn simulate_black_scholes(
    risk_free_rate: f64,
    volatility: f64,
    initial_stock_price: f64,
    strike_price: f64,
    time_to_maturity: f64,
    number_of_simulations: usize,
) -> f64 {
    let drift = (risk_free_rate - 0.5 * volatility.powi(2)) * time_to_maturity;
    let diffusion = volatility * (time_to_maturity.sqrt());
    let normal_dist = Normal::new(0.0, 1.0).unwrap();

    let sum_payoff: f64 = (0..number_of_simulations)
        .map(|_| {
            let gauss = normal_dist.sample(&mut rand::thread_rng());
            let stock_price_at_maturity = initial_stock_price * f64::exp(drift + diffusion * gauss);
            f64::max(stock_price_at_maturity - strike_price, 0.0)
        })
        .sum();

    f64::exp(-risk_free_rate * time_to_maturity) * (sum_payoff / number_of_simulations as f64)
}
