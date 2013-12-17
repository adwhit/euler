package main

import (
    "fmt"
    "tools"
    "prime"
    "math"
)

func ReplaceDigit(n int, index int, value int) int {
    narr := tools.Splitter(n)
    narr[index] = value
    changed := tools.Joiner(narr)
    return changed
}

func MakePerms(digits []int, pos []int) (perms []int) {
    perms = make([]int, 10)

    // Construct initial permutation
    p0 := 0
    i := 0
    flag := false
    for exp:=len(digits) + len(pos) -1;exp>=0;exp-- {
        for _, v := range pos {
            if v == exp {
                flag = true
            }
        }
        if flag == true {
            flag = false
            continue
        }
        p0 += digits[i]*int(math.Pow10(exp))
        i++
    }
    //use p0 to build the rest of the permutations
    perms[0] = p0
    toadd := 0
    for _, v := range pos {
        toadd += int(math.Pow10(v))
    }
    for j:=1;j<10;j++ {
        perms[j] = perms[j-1] + toadd
    }
    return perms
}


func Increment(narr []int) {
    for counter := len(narr)-1; counter >= 0; counter-- {
        if narr[counter] < 9 {
            narr[counter] ++
            break
        } else {
            narr[counter] = 0
        }
    }
    return
}


func main() {
    max := 0
    ndig := 4
    npos := 3
    var best []int
    parr := prime.PrimeSieve(int(math.Pow10(ndig+npos+1)))
    curdig := make([]int, ndig)

    for x:=1; x<int(math.Pow10(ndig));x++{
        Increment(curdig)
        for p1:=0;p1<ndig+npos-3;p1++ {
            for p2:=p1+1;p2<ndig+npos-2;p2++ {
                for p3:=p2+1;p3<ndig+npos-1;p3++ {
                    perms := MakePerms(curdig, []int{p1,p2,p3})
                    ct := 0
                    for _, p := range perms {
                        if parr[p] {
                            ct ++
                        }
                    }
                    if ct >= max {
                        max = ct
                        best = perms
                        fmt.Println(max, best)
                        for _, p := range perms {
                            if parr[p] {
                                print(p, "  ")
                            }
                        }
                    }
                }
            }
        }
    }
}
