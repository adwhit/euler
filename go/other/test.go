package main

import (
    "fmt"
    //"os"
    "flag"
)

var v = flag.Bool("v", false, "Verbosity")

func main () {
    flag.Parse()
    fmt.Println(*v)
    path := flag.Arg(0)
    fmt.Println(path)
}
