package fraction

type Frac struct {
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

