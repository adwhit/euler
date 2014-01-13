package main

import (
	"fmt"
	"prime"
)

func main() {
	l := prime.GenNprimes(10001)
	fmt.Println(l[len(l)-1])
}
