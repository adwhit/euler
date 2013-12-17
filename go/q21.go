package main

import (
    "fmt"
)



func DivSum(n int) (sum int) {
    for i:=1; i<=n/2; i++ {
        if n%i == 0 {
            sum += i
        }
    }
    return
}

func main() {
    var tot int
    m := make(map[int]int)
    for x:=2; x<10000; x++ {
        m[x] = DivSum(x)
    }
    for key, value := range m {
        if m[value] == key && key != value {
            fmt.Println(key, value)
            tot += key
        }
    }
    fmt.Println(tot)
}

