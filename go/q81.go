package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func Load() (narr [][]int) {
	s, _ := ioutil.ReadFile("other/matrix.txt")
	sarr := strings.Split(string(s), "\n")
	sarr = sarr[:len(sarr)-1]
	narr = make([][]int, len(sarr))
	for i, v := range sarr {
		spl := strings.Split(v, ",")
		narr[i] = make([]int, len(spl))
		for j, w := range spl {
			narr[i][j], _ = strconv.Atoi(w)
		}
	}
	return
}

func Best(path [][]int, short [][]int, i int, j int) {
	toadd := path[i][j]
	up := short[i-1][j]
	left := short[i][j-1]
	if up < left {
		short[i][j] = toadd + up
	} else {
		short[i][j] = toadd + left
	}
}

func main() {
	path := Load()
	//create array
	short := make([][]int, len(path))
	for i := range short {
		short[i] = make([]int, len(path))
	}

	//fill knowns
	short[0][0] = path[0][0]
	for i := 1; i < len(path); i++ {
		short[i][0] = path[i][0] + short[i-1][0]
		short[0][i] = short[0][i-1] + path[0][i]
	}
	//start algo
	for shell := 1; shell < len(path); shell++ {
		//row
		for i := shell; i < len(path); i++ {
			Best(path, short, i, shell)
		}
		//column
		for j := shell; j < len(path); j++ {
			Best(path, short, shell, j)
		}
	}
	fmt.Println(short[79][79])
}
