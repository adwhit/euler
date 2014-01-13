package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func Load() [][]int {
	s, _ := ioutil.ReadFile("other/triangle.txt")
	str := string(s)
	sarr := strings.Split(str, "\n")
	sarr = sarr[:len(sarr)-1]
	narr := make([][]int, len(sarr))
	for i, v := range sarr {
		spl := strings.Split(v, " ")
		narr[i] = make([]int, len(spl))
		for j, w := range spl {
			conv, _ := strconv.Atoi(w)
			narr[i][j] = conv
		}
	}
	return narr
}

func Shortest(up []int, down []int) []int {
	res := make([]int, len(down))
	size := len(down)
	for i := 0; i < size; i++ {
		if i == 0 {
			res[i] = up[0] + down[0]
		} else if i == size-1 {
			res[i] = up[i-1] + down[i]
		} else {
			s1 := down[i] + up[i]
			s2 := down[i] + up[i-1]
			if s1 > s2 {
				res[i] = s1
			} else {
				res[i] = s2
			}
		}
	}
	return res
}

func main() {
	path := Load()
	short := make([][]int, len(path))
	short[0] = path[0]
	for i := 1; i < len(path); i++ {
		short[i] = Shortest(short[i-1], path[i])
	}
	max := 0
	for _, v := range short[len(short)-1] {
		if v > max {
			max = v
		}
	}
	fmt.Println(max)
}
