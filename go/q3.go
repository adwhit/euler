package main

import (
	"fmt"
	"math"
	"prime"
)

const (
	x = 600851475143
)

func main() {
	var max int
	l := prime.GenPrimes(int(math.Sqrt(float64(x))))
	for _, n := range l {
		if x%n == 0 {
			max = n
		}
	}
	fmt.Println(max)
}
