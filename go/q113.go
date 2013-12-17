package main

import (
    "fmt"
    //"strconv"
)


func TriProg(n int) int {
    tot := 1
    for i:=0;i<n;i++ {
        tot *= 10 + i
        tot /= i+1
    }
    return tot
}

func Sum(narr []int) int {
    tot := 0
    for _, v := range narr {
        tot += v
    }
    return tot
}


func main() {
    narr := make([]int, 100)
    for x:=1;x<=len(narr);x++ {
        narr[x-1] = TriProg(x)
    }
   fmt.Println(Sum(narr[:100]) + narr[99] - 10 * (100) - 1)
}

