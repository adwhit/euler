package main

import (
	"fmt"
	"tools"
)

func main() {
	var ct int
	resarr := make([]int, 10000000)
	resarr[1] = 1
	resarr[89] = 89
	for x := 2; x < 10000000; x++ {
		ct = x
		for {
			if resarr[ct] == 1 {
				resarr[x] = 1
				break
			}
			if resarr[ct] == 89 {
				resarr[x] = 89
				break
			}
			ctarr := tools.Split(ct)
			ct = 0
			for _, v := range ctarr {
				ct += v * v
			}
		}
	}
	tot := 0
	for _, v := range resarr {
		if v == 89 {
			tot++
		}
	}
	fmt.Println(tot)
}
