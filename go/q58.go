package main

import (
	"fmt"
	"prime"
)

func DiagNums(n int) []int {
	sq := n * n
	return []int{sq, sq - n + 1, sq - 2*n + 2, sq - 3*n + 3}
}

func main() {
	var n int
	primect := 0
	frac := 1.0

	for n = 3; frac > 0.1; n = n + 2 {
		for _, v := range DiagNums(n) {
			if prime.Isprime(v) {
				primect++
			}
		}
		frac = float64(primect) / (float64(n)*2 - 1)
	}
	fmt.Println(n, frac)
}
