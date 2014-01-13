package tools

import (
	"math"
    "io/ioutil"
    "strings"
    "strconv"
)

// Returns the factorial of int n
func Factorial(n int) (fct int) {
	fct = 1
	for i := n; i > 1; i-- {
		fct *= i
	}
	return
}

// Splits an int into a slice of digits of the int
func Split(n int) (narr []int) {
	ndig := int(math.Log10(float64(n))) + 1
	narr = make([]int, ndig)
	for i := 1; i <= ndig; i++ {
		narr[ndig-i] = n / int(math.Pow10(i-1)) % 10
	}
	return narr
}

func Join(narr []int) (tot int) {
	tot = 0
	for i, v := range narr {
		tot += v * int(math.Pow10(len(narr)-1-i))
	}
	return
}

// Returns the sum of the factorials of digits in
// a slice of ints
func SumFact(narr []int) (tot int) {
	for _, v := range narr {
		tot += Factorial(v)
	}
	return
}

//Load from text file
func Load(path string, firstSplit string, secondSplit string) (narr [][]int) {
	s, _ := ioutil.ReadFile(path)
	sarr := strings.Split(string(s), firstSplit)
	sarr = sarr[:len(sarr)-1]
	narr = make([][]int, len(sarr))
	for i, v := range sarr {
		spl := strings.Split(v, secondSplit)
		narr[i] = make([]int, len(spl))
		for j, w := range spl {
			narr[i][j], _ = strconv.Atoi(w)
		}
	}
	return
}
