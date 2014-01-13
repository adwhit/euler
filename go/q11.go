package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func MakeGrid() (grid Grid) {
	b, _ := ioutil.ReadFile("other/q11text")
	flat := strings.Split(string(b), "\n")
	for i, line := range flat {
		if i <= 19 {
			chrs := strings.Split(line, " ")
			nums := Toi(chrs)
			for j, num := range nums {
				grid[i][j] = num
			}
		}
	}
	return grid
}

func Toi(chrs []string) (nums []int) {
	nums = make([]int, len(chrs))

	for i, c := range chrs {
		n, _ := strconv.Atoi(c)
		nums[i] = n
	}
	return nums
}

func Prod(n1, n2, n3, n4 int) (prod int) {
	return n1 * n2 * n3 * n4
}

func Perms(grid Grid, inc, jnc int, pmax *int) {
	var js int
	var je int
	if jnc >= 0 {
		js = 0
		je = 3
	} else {
		js = 3
		je = 0
	}
	for i := 0; i < len(grid)-3*inc; i++ {
		for j := js; j < len(grid[i])-je*jnc; j++ {
			p := Prod(grid[i][j],
				grid[i+1*inc][j+1*jnc],
				grid[i+2*inc][j+2*jnc],
				grid[i+3*inc][j+3*jnc])
			if p > *pmax {
				*pmax = p
			}
		}
	}
	return
}

type Grid [20][20]int

func main() {
	grid := MakeGrid()
	max := 0
	Perms(grid, 1, 0, &max)
	Perms(grid, 0, 1, &max)
	Perms(grid, 1, 1, &max)
	Perms(grid, 1, -1, &max)

	fmt.Println(max)
}
