//Conway's game of life!!!!!

package main

import (
	"flag"
	"fmt"
	"math/rand"
	"os"
	"os/exec"
	"time"
)

var (
	gens = flag.Int("g", 50, "Number of generations to simulate")
	size = flag.Int("s", 70, "Size of side of board")
)

func BtoI(b bool) int {
	if !b {
		return 0
	} else {
		return 1
	}
}

func Step(board [][]bool) {
	//how best to calc changes? Can't do it in-place...
	//answer - use sparse matrix
	var ipr, inx, jpr, jnx int
	chgrow := make([]int, 0, *size)
	chgcol := make([]int, 0, *size)
	for i := 0; i < *size; i++ {
		if i == 0 {
			ipr = *size - 1
		} else {
			ipr = i - 1
		}
		if i == *size-1 {
			inx = 0
		} else {
			inx = i + 1
		}
		for j := 0; j < *size; j++ {
			if j == 0 {
				jpr = *size - 1
			} else {
				jpr = j - 1
			}
			if j == *size-1 {
				jnx = 0
			} else {
				jnx = j + 1
			}
			state := board[i][j]
			//assume it doesn't change
			newstate := state
			tot := 0
			tot += BtoI(board[ipr][jpr])
			tot += BtoI(board[ipr][j])
			tot += BtoI(board[ipr][jnx])
			tot += BtoI(board[i][jpr])
			tot += BtoI(board[i][jnx])
			tot += BtoI(board[inx][jpr])
			tot += BtoI(board[inx][j])
			tot += BtoI(board[inx][jnx])
			if tot < 2 || tot > 3 {
				newstate = false
			} else if tot == 3 && !state {
				newstate = true
			}
			if newstate != state {
				chgrow = append(chgrow, i)
				chgcol = append(chgcol, j)
			}
		}
	}
	for i := range chgrow {
		board[chgrow[i]][chgcol[i]] = !board[chgrow[i]][chgcol[i]]
	}
	return
}

func PrintBoard(board [][]bool) {
	for i := 0; i < *size; i++ {
		for j := 0; j < *size; j++ {
			if board[i][j] {
				fmt.Print("#")
			} else {
				fmt.Print(" ")
			}
		}
		fmt.Print("\n")
	}
	fmt.Print("\n\n")
}

func Randomize(board [][]bool) {
	for i := 0; i < *size; i++ {
		for j := 0; j < *size; j++ {
			if rand.Intn(2) == 0 {
				board[i][j] = false
			} else {
				board[i][j] = true
			}
		}
	}
}

func main() {
	//create board
	flag.Parse()

	c := exec.Command("printf \"\033c\"")
	c.Stdout = os.Stdout
	c.Run()

	board := make([][]bool, *size)
	for i := 0; i < *size; i++ {
		board[i] = make([]bool, *size)
	}
	Randomize(board)

	//create oscillator
	board[5][4] = true
	board[5][5] = true
	board[5][6] = true

	for i := 0; ; i++ {
		//iterate!
		Step(board)
		c.Run()
		PrintBoard(board)
		time.Sleep(100 * time.Millisecond)
	}
}
