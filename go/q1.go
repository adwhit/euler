package main

func main() {
    y := 0
    for i := 1; i < 1000; i++ {
        switch {
        case i%3==0:
            y += i
        case i%5==0:
            y += i
        }
    }
    println(y)
}
