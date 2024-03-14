open System

let riskFreeRate = 0.01
let volatility = 0.2
let initialStockPrice = 100.0
let strikePrice = 100.0
let timeToMaturity = 1.0 // 1 year
let numberOfSimulations = 1000000

let generateGaussianNoise () =
    let u1 = Random().NextDouble()
    let u2 = Random().NextDouble()
    Math.Sqrt(-2.0 * Math.Log(u1)) * Math.Cos(2.0 * Math.PI * u2)

let simulateBlackScholes () =
    let drift = (riskFreeRate - 0.5 * volatility * volatility) * timeToMaturity
    let diffusion = volatility * Math.Sqrt(timeToMaturity)
    let mutable sumPayoff = 0.0

    for i in 1 .. numberOfSimulations do
        let gauss = generateGaussianNoise()
        let stockPriceAtMaturity = initialStockPrice * Math.Exp(drift + diffusion * gauss)
        let payoff = Math.Max(stockPriceAtMaturity - strikePrice, 0.0)
        sumPayoff <- sumPayoff + payoff

    Math.Exp(-riskFreeRate * timeToMaturity) * (sumPayoff / float numberOfSimulations)

[<EntryPoint>]
let main argv =
    let stopwatch = System.Diagnostics.Stopwatch.StartNew()
    
    let callPrice = simulateBlackScholes ()
    printfn "European Call Option Price = %f" callPrice

    stopwatch.Stop()
    printfn "Execution time: %f seconds" (float stopwatch.ElapsedMilliseconds / 1000.0)
    0
