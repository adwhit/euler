package main

import (
    "fmt"
    "io/ioutil"
    "strings"
    "strconv"
    "os"
)

func loader() []int {
    b, _ := ioutil.ReadFile("other/cipher1.txt")
    s := strings.Split(string(b), ",")
    nums := make([]int, len(s))
    for i, v := range s {
        nums[i], _ = strconv.Atoi(v)
    }
    nums[len(nums)-1] = 73
    return nums
}

func Decrypt(code []int, key[]int) ([]int, string) {
    xor := make([]int, len(code))
    b := make([]string, len(code))

    for i, v := range code {
        xor[i] = key[i % 3]^v
    }
    for i, v := range xor {
        b[i] = string(v)
    }
    return xor, strings.Join(b, "")
}

func main() {
    var key []int
    ct := 0
    code := loader()
    for l1:=97;l1<=122;l1++{
        for l2:=97;l2<=122;l2++{
            for l3:=97;l3<=122;l3++{
                key = []int{l1, l2, l3}
                ascii, dec := Decrypt(code, key)
                if strings.Contains(dec, "God") && strings.Contains(dec, "John") {
                    for _, v := range ascii {
                        ct += v
                    }
                    fmt.Println(dec)
                    fmt.Println(ct)
                    os.Exit(0)
                }
            }
        }
    }
}
