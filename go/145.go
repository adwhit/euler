package main

import (
	"./src/tools"
	"fmt"
	"math"
)

func isrev(arr []int) bool {
	var base int
	l := len(arr)
	carry := 0
	for i := 0; i < l; i++ {
		digit := arr[i] + arr[l-i-1] + carry
		carry = digit / 10
		base = digit % 10
		if base%2 == 0 {
			return false
		}
	}
	return true
}

func trad() {
	ct := 0
	for i := 10; i < 1000; i++ {
		if i%10 == 0 {
			if i%1000000 == 0 {
				fmt.Println(i)
			}
			continue
		}
		if isrev(tools.Split(i)) {
			ct++
		}
	}
	fmt.Println(ct)
}

func sport() {
	var sense int
	ct := 0
	for i1 := 1; i1 < 10; i1++ {
		sense = i1 % 2
		for i2 := 0; i2 < 10; i2++ {
			if i2 != 0 {
				if isrev([]int{i1, i2}) {
					ct++
				}
				for i3 := 0; i3 < 10; i3++ {
                    if i3 != 0 {
					if isrev([]int{i1, i2, i3}) {
						ct++
					}
                    chk([]int{i1, i2, i3})
                }
					//    for i1:=0;i<10;i++{
					//       for i1:=0;i<10;i++{
					//          for i1:=0;i<10;i++{
					//             for i1:=0;i<10;i++{
					//                for i1:=0;i<10;i++{
				}
			}
		}
	}
	fmt.Println(ct)
	fmt.Println(sense)
}

func qjoin(arr []int) int {
	val := 0
	l := len(arr)
	for i := 0; i < l; i++ {
		val += int(math.Pow10(i)) * arr[l-i-1]
	}
	return val
}

func rev(arr []int) []int {
	var reva []int
	l := len(arr)
	for i := 0; i < l; i++ {
		reva = append(reva, arr[l-i-1])
	}
	return reva
}

func chk(arr []int) {
	reva := rev(arr)
	n := qjoin(arr)
	r := qjoin(reva)
	fmt.Println(n, r, n+r)
}

func main() {
	trad()
}
