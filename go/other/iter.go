package main

import "fmt"

func xrange(from int, to int) chan int {
	yield := make(chan int)

	go func() {
		for i := from; i <= to; i++ {
			yield <- i
		}
		close(yield)
	}()

	return yield
}

func main() {
	for i := range xrange(0, 10) {
		fmt.Println(i)
	}
}
