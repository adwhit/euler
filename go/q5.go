package main

import (
	"fmt"
	"os"
)

func main() {
	for x := 21; ; x++ {
		for y := 2; y <= 20; y++ {
			if x%y != 0 {
				break
			}
			if y == 20 {
				fmt.Println(x)
				os.Exit(1)
			}
		}
	}
}
