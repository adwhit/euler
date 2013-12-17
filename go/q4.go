package main

import (
    "fmt"
    "strconv"
)

func Reverse(n string) (string) {
    runes := []rune(n)
    for i, j := 0, len(runes)-1; i < j; i, j = i + 1, j - 1 {
        runes[i], runes[j] = runes[j], runes[i]
    }
    return string(runes)
}


func main() {
    var y int
    m, max := 0, 0
    for x:=100; x<1000; x++ {
        for y=x; y<1000; y++ {
            n := strconv.Itoa(x * y)
            nrev := Reverse(n)
            if n == nrev {
                m, _ = strconv.Atoi(n)
                if m > max {
                    max, _ = strconv.Atoi(n)
                }
            }
        }
    }
    fmt.Println(max)
}
