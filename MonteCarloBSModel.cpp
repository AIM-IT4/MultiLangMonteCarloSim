#include <iostream>
#include <cmath>
#include <cstdlib>
#include <chrono>

double riskFreeRate = 0.01;
double volatility = 0.2;
double initialStockPrice = 100.0;
double strikePrice = 100.0;
double timeToMaturity = 1.0; // 1 year
long numberOfSimulations = 1000000;

double generateGaussianNoise() {
    double u = (double)rand() / RAND_MAX;
    double v = (double)rand() / RAND_MAX;
    return sqrt(-2.0 * log(u)) * cos(2.0 * M_PI * v);
}

double simulateBlackScholes() {
    double drift = (riskFreeRate - 0.5 * volatility * volatility) * timeToMaturity;
    double diffusion = volatility * sqrt(timeToMaturity);
    double sumPayoff = 0.0;

    for (long i = 0; i < numberOfSimulations; ++i) {
        double gauss = generateGaussianNoise();
        double stockPriceAtMaturity = initialStockPrice * exp(drift + diffusion * gauss);
        double payoff = std::max(stockPriceAtMaturity - strikePrice, 0.0);
        sumPayoff += payoff;
    }

    return exp(-riskFreeRate * timeToMaturity) * (sumPayoff / numberOfSimulations);
}

int main() {
    srand((unsigned)time(0));
    auto start = std::chrono::high_resolution_clock::now();

    double callPrice = simulateBlackScholes();
    std::cout << "European Call Option Price = " << callPrice << std::endl;

    auto end = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> elapsed = end - start;
    std::cout << "Execution time: " << elapsed.count() << " seconds\n";

    return 0;
}


