package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func Sum(sl []int) (sum int) {
	for _, n := range sl {
		sum += n
	}
	return sum
}

func Prod(sl []int) (prod int) {
	prod = 1
	for _, n := range sl {
		prod *= n
	}
	return prod
}

func Readtoints(path string) (nums []int) {
	b, _ := ioutil.ReadFile(path)
	s := strings.Split(string(b), "")
	nums = make([]int, 0, len(s))
	for _, l := range s {
		n, _ := strconv.Atoi(l)
		nums = append(nums, n)
	}
	return nums
}

func main() {
	nums := Readtoints("other/q8text.text")
	max := 0
	for i := 0; i < len(nums)-5; i++ {
		sl := nums[i : i+5]
		prod := Prod(sl)
		if prod > max {
			max = prod
		}
	}
	fmt.Println(max)
}
