package main

import (
	"math"
)

func main() {
	ct := 0
	for dig := 1; dig < 10; dig++ {
		for exp := 1; ; exp++ {
			y := math.Pow(float64(dig), float64(exp))
			ndig := int(math.Log10(y)) + 1
			if ndig == exp {
				ct++
				println(dig, exp, y, ct)
			}
			if exp > ndig {
				break
			}
		}
	}
}
