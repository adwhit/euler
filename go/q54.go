package main

import (
    "fmt"
    "os"
    "bufio"
    "strings"
)

func main() {
    var text []string
    var hand [][]string
    file, _ := os.Open("other/poker.txt")
    scanner := bufio.NewScanner(file)
    for scanner.Scan() {
        text = append(text, scanner.Text())
    }
    for _, line := range text {
       hand = append(hand, strings.Split(line, " "))
    }
    fmt.Println(hand)
}

