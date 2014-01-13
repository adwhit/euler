package main

import "fmt"

func fib(i int) int {
	if i == 1 {
		return 1
	} else if i == 0 {
		return 0
	} else {
		return fib(i-1) + fib(i-2)
	}
}

func main() {
	f := 0
	sum := 0
	for i := 1; f < 4000000; i++ {
		f = fib(i)
		fmt.Printf("The %dth Fibonacci number is %d\n", i, f)
		if f%2 == 0 {
			sum += f
		}
	}
	fmt.Printf("The sum of even 'i's is %d\n", sum)
}
