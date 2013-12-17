package main

import (
    "fmt"
    "math/big"
)

func main() {
    sumsq := big.NewInt(0)
    sqsum := big.NewInt(0)
    var i int64
    for i = 1; i<= 100; i++ {
        I := big.NewInt(i)
        sumsq.Add(sumsq, I)
        sqsum.Add(sqsum, I.Mul(I, I))
    }
    sumsq.Mul(sumsq, sumsq)
    fmt.Println(sumsq.Sub(sumsq,sqsum))
}

