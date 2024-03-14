# Monte Carlo Simulations for Black-Scholes Model Across Languages

This repository hosts a comparative analysis of implementing Monte Carlo simulations for the Black-Scholes model using three different programming languages: C++, F#, and Rust. Our goal is to evaluate the performance in terms of execution time and accuracy of option pricing across these languages.

## Introduction

The Black-Scholes model is a fundamental concept in financial mathematics, used for pricing European call and put options. Monte Carlo simulations are widely employed to estimate the pricing of options where analytical solutions are complex or non-existent. This project aims to assess how different programming languages perform in executing a Monte Carlo simulation for the Black-Scholes model, focusing on execution time and the resulting option prices.

## Implementation

Each language's implementation adheres to the same algorithmic steps to ensure a fair comparison. The core of the simulation involves generating random paths for the underlying asset's price movement and then calculating the payoff for a European call option. Finally, we discount the average payoff to today's prices to estimate the option's value.

### Languages Used

- **C++**: Known for its high performance and control over system resources.
- **F#**: A functional-first programming language that excels in clarity and expressiveness, especially for mathematical computations.
- **Rust**: A language that offers memory safety without sacrificing performance, making it a strong candidate for system-level programming in finance.

## Results

The execution times and the estimated European Call Option prices obtained from the simulations are as follows:

| Language | Execution Time (seconds) | Option Price |
|----------|-------------------------|--------------|
| C++      | 0.274164                | 8.42912      |
| F#       | 7.189000                | 8.462657     |
| Rust     | 5.161593541             | 8.430981543673564 |

### Analysis

- **C++** demonstrated the fastest execution time, underscoring its efficiency and optimization capabilities for numerical computations.
- **Rust** offered a competitive balance between performance and safety, showcasing its potential for reliable and fast financial applications.
- **F#** lagged in terms of execution speed but provided a high degree of expressiveness and ease of development, which can be advantageous in complex financial modeling.

## Conclusion

This comparative analysis highlights the importance of choosing the right tool for specific computational tasks in quantitative finance. While C++ leads in performance, Rust and F# present compelling advantages in safety, reliability, and development experience. This repository invites further exploration and optimization in each language to better understand and leverage their unique strengths.

## License

This project is open-sourced under the MIT License. See the LICENSE file for more details.
