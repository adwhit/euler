package main

import (
    "fmt"
    "tools"
)

func IsInc(n int) bool {
    narr := tools.FastSplit(n)
    for i := 1; i < len(narr); i++ {
        if narr[i] < narr[i-1] {
            return false
        }
    }
    return true
}

func IsDec(n int) bool {
    narr := tools.FastSplit(n)
    for i := 0; i < len(narr) - 1; i++ {
        if narr[i] < narr[i+1] {
            return false
        }
    }
    return true
}

/*
func main () {
    var i int
    ct := 0
    rat := 0.0
    for i=1; rat<0.99; i++ {
        if !IsInc(i) && !IsDec(i) {
            ct ++
        }
        rat = float64(ct)/float64(i)
    }
    fmt.Println(i, rat)
}
*/


func main () {
    var i int
    ct := 0
    rat := 0.0
    for i=1; i<10000; i++ {
        if !IsInc(i) && !IsDec(i) {
            ct ++
        }
        rat = float64(ct)/float64(i)
    }
    fmt.Println(i,i-ct, rat)
}


