package main

import ("fmt"; "math")

func Tri(n int) (tri int) {
    return n * (n + 1) / 2
}

func Nfact(n int) (nf int) {
    nf = 0
    for i := 1; i<=int(math.Sqrt(float64(n))); i++ {
        if n%i == 0 {
            nf ++
        }
    }
    return nf*2
}


func main() {
    for x:=1;;x++ {
        tri := Tri(x)
        nf := Nfact(tri)
        fmt.Println(tri, nf)
        if nf > 500 {
            fmt.Println(tri)
            break
        }
    }
}
