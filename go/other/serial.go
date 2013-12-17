package main

import (
    "fmt"
    "encoding/gob"
    "math/rand"
    "os"
)


type Info struct {
    Name string
    Array [][]float64
}

func (info *Info) Serialize(to_file  string) (error) {
    f, err := os.Create(to_file)
    if err != nil {
        return err
    }

    defer f.Close()

    gob.NewEncoder(f).Encode(info)

    return nil
}

func Deserialize(from_file string) (*Info, error) {
    f, err := os.Open(from_file)
    if err != nil {
        return nil, err
    }
    defer f.Close()

    var info Info

    decoder := gob.NewDecoder(f)
    err = decoder.Decode(&info)
    return &info, err
}

func main() {
    nrows := 100
    ncols := 200
    //make array of random floats
    arr := make([][]float64, nrows)
    for i:=0;i<nrows;i++ {
        arr[i] = make([]float64,ncols)
        for j:=0;j<ncols;j++ {
            arr[i][j] = rand.Float64()
        }
    }

    //make object to serialize
    info := &Info{ Name : "My thing",Array : arr }

    fstr := "/home/alex/Temp/ser.gob"

    err := info.Serialize(fstr)

    if err != nil {
        panic(err.Error())
    }

    inf, err := Deserialize(fstr)

    if err != nil {
        panic(err.Error())
    }

    same := true
    for i:=0;i<ncols;i++ {
        if info.Array[0][i] == inf.Array[0][i] {
            fmt.Println(info.Array[0][i], inf.Array[0][i])
            same = false
        }
    }
    fmt.Println(same)
}
