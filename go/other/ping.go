package main

import (
    "time"
    "fmt"
)

type Ball struct{ hits int }

func main() {
    table := make(chan *Ball)
    go player1("ping", table)
    go player2("mong", table)
    fmt.Println("Throw the ball!")
    time.Sleep(1 * time.Second)
    table <- new(Ball) // game on; toss the ball
    time.Sleep(2 * time.Second)
    <-table // game over; grab the ball
}

func player1(name string, table chan *Ball) {
    for {
        ball := <-table
        ball.hits++
        fmt.Println(name, ball.hits)
        table <- ball
        time.Sleep(100 * time.Millisecond)
    }
}

func player2(name string, table chan *Ball) {
    for {
        ball := <-table
        ball.hits++
        fmt.Println(name, ball.hits)
        table <- ball
        time.Sleep(300 * time.Millisecond)
    }
}
