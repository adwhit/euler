package main

import "fmt"

type Shaper interface {
   Area() int
   Perimeter() int
}

type Rectangle struct {
   length, width int
}

func (r Rectangle) Area() int {
   return r.length * r.width
}

func (r Rectangle) Perimeter() int {
    return (r.length + r.width) * 2
}

type Square struct {
   side int
}

func (sq Square) Area() int {
   return sq.side * sq.side
}

func (sq Square) Perimeter() int {
    return sq.side * 4
}

func main() {
   r := Rectangle{length:5, width:3}
   q := Square{side:5}
   shapesArr := [...]Shaper{r, q}

   fmt.Println("Looping through shapes for area ...")
   for n, _ := range shapesArr {
       fmt.Println("Shape details: ", shapesArr[n])
       fmt.Println("Area of this shape is: ", shapesArr[n].Area())
       fmt.Println("Perimeter of this shape is: ", shapesArr[n].Perimeter())
   }
}
