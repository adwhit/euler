package main

import (
	"fmt"
	"math/big"
)

func root2iter(f *big.Rat) {
	f.Inv(f)
	f.Add(f, big.NewRat(2, 1))
	return
}

func main() {
	var t big.Rat
	ct := 0
	f := big.NewRat(5, 2)
	for i := 0; i < 1000; i++ {
		root2iter(f)
		t.Add(f, big.NewRat(-1, 1))
		if len(t.Num().String()) > len(t.Denom().String()) {
			ct++
		}
	}
	fmt.Println(ct)
}
