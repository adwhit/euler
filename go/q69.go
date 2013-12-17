package main

import (
    "fmt"
    "prime"
)

func main() {
    //make list of primes
    p := prime.PrimeSieve(1000)
    pr := make([]int, 0, 1000)
    for i, v := range p {
        if v {
            pr = append(pr, i)
        }
    }
    tot := 1
    for _, v := range pr {
        tot *= v
        if tot > 1000000 {
            tot /= v
            break
        }
    }
    fmt.Println(tot)
}
