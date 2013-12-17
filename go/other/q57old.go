package main

import (
    "fmt"
    "tools"
)

type frac struct {
    numer int
    denom int
}

// Greatest Common Divisor algorithm
func GCD(n1, n2 int) (gcd int) {
    if n2 > n1 {
        n1, n2 = n2, n1
    }
    for {
        r := n1 % n2
        if r == 0 {
            return n2
        }
        n2, n1 = r, n2
    }
}

// Sum two fractions- returns simplified form
func SumFrac(f1, f2 frac) (res frac) {
    numer := f1.numer * f2.denom + f2.numer * f1.denom
    denom := f1.denom * f2.denom
    gcd := GCD(numer, denom)
    res.numer, res.denom = numer/gcd, denom/gcd
    return
}

func InvFrac(fr frac) frac {
    fr.numer, fr.denom = fr.denom, fr.numer
    return fr
}

func root2iter(f frac) frac {
        return SumFrac(InvFrac(f),frac{2,1})
}

func main() {
    ct := 0
    f := frac{5, 2}
    for i:=0;i<100;i++ {
        f = root2iter(f)
        t := SumFrac(f, frac{-1,1})
        fmt.Println(t)
        if len(tools.Splitter(t.numer)) > len(tools.Splitter(t.denom)) {
            ct += 1
        }
    }
    fmt.Println(ct)
}
