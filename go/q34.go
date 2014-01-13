package main

import (
	"fmt"
	"tools"
)

func main() {
	var sf, tot int
	limit := tools.Factorial(9)
	for i := 3; i < limit; i++ {
		narr := tools.Splitter(i)
		sf = tools.SumFact(narr)
		if sf == i {
			tot += i
		}
	}
	fmt.Println(tot)
}
