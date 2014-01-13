package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
	"strings"
	"tools"
)

func Load() [][]int {
	s, _ := ioutil.ReadFile("other/keylog.txt")
	str := string(s)
	sarr := strings.Split(str, "\n")
	sarr = sarr[:len(sarr)-1]
	narr := make([][]int, len(sarr))
	// for each value, create array of integers,
	//put whole thing in (2D) array
	for i, v := range sarr {
		narr[i] = make([]int, 3)
		j, _ := strconv.Atoi(v)
		for k, w := range tools.FastSplit(j) {
			narr[i][k] = w
		}
	}
	return narr
}

func IsLegal(guess []int, keys [][]int) bool {
	var legal bool
	for _, v := range keys {
		i := 0
		legal = false
		for _, w := range guess {
			if v[i] == w {
				i++
				if i == 3 {
					legal = true
					break
				}
			}
		}
		if !legal {
			return false
		}
	}
	return true
}

func main() {
	keys := Load()
	for x := 1000; x < 1000000000; x++ {
		guess := tools.FastSplit(x)
		if IsLegal(guess, keys) {
			fmt.Println(guess)
			os.Exit(0)
		}
	}
	fmt.Println("None found")
}
