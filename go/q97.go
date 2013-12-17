package main

import (
    "fmt"
    "tools"
)

func Time2(xarr[]int) {
    var dbl int
    ndig := len(xarr)
    carry := 0
    for i:=0;i<ndig;i++ {
        dbl = xarr[i] * 2 + carry
        carry = 0
        if dbl > 9 {
            carry = 1
            xarr[i] = dbl - 10
        } else {
            xarr[i] = dbl
            carry = 0
        }
    }
    return
}



func main() {
    pow := 7830457
    xarr := make([]int,11)
    xarr[0] = 1
    for i:=1;i<=pow;i++ {
        Time2(xarr)
    }

    yarr := make([]int, len(xarr))
    for i, v := range xarr {
        yarr[len(xarr)-1-i] = v
    }
    ans := tools.Join(yarr) * 28433 + 1
    fmt.Println(ans)
}


