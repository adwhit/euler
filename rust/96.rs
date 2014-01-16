extern mod extra;
use std::io::buffered::BufferedReader;
use std::io::File;
use extra::bitv::{BitvSet, Bitv};

static MX:u8 = 1;

fn reader(chan:Chan<[u8,..81]>) {
    let path = Path::new("../data/sudoku.txt");
    let mut file = BufferedReader::new(File::open(&path));
    let mut arr:[u8,..81] = [0, ..81];
    for i in range(0,MX) {
        for (lnum,line) in file.lines().enumerate() {
            if lnum == 0 {
                continue
            }
            let tmp = str2u8(line.trim());
            for (ct,&u) in tmp.iter().enumerate() {//str2u8(line.trim()).iter().enumerate() {
                arr[(lnum-1)*9 + ct] = u
            }
            if lnum == 9 {
                chan.send(arr);
                break 
            }
        }
    }
}

fn str2u8(s:&str) -> [u8,..9] {
    let mut v:[u8,..9] = [0, ..9];
    for (i,c) in s.bytes().enumerate() {
        v[i] = c - 48;
    }
    v
}


// build possibilities
fn buildposs(arr:[u8,..81]) -> ~[BitvSet] {
    let mut opts:BitvSet = BitvSet::from_bitv(Bitv::new(10, true));
    print!("{}", opts.len());
    for b in opts.iter() {
        print!("{}", b);
    }
    opts.remove(&0);
    let mut poss:~[BitvSet] = ~[];
    for i in range(0,81) {
        if arr[i] == 0 {
            poss.push(opts.clone())
        } else {
            let mut bs = BitvSet::new();
            bs.insert(arr[i] as uint);
            poss.push(bs)
        }
    }
    poss
}

fn solve(arr:[u8,..81], poss: &mut[BitvSet]) {
    for i in range(0,9) {
        for j in range(0,9) {
            //horizontal pass
            /*
            if poss[i*9+j].len() == 1 {
                let bs = poss[i*9+j].clone();
                for k in range(0,9) {
                    if k != j {
                        poss[i*9+k].difference_with(&bs);
                    }
                }
            }
            //vertical pass
            if poss[j*9+i].len() == 1 {
                let bs = poss[j*9+i].clone();
                for k in range(0,9) {
                    if k != i {
                        poss[j*9+k].difference_with(&bs);
                    }
                }
            }
            //block pass
            if poss[step(i,j)].len() == 1 {
                let bs = poss[step(i,j)].clone();
                for k in range(0,9) {
                    if k != j {
                        poss[step(i,k)].difference_with(&bs);
                    }
                }
            }
            */
        }
    }
}

fn step(i:int, j:int) -> int {
    (i/3)*27 + (i%3)*3 + (j/3)*9 + (j%3)
}

fn print(bs:&BitvSet) {
    println!("{}", bs.len());
    for c in bs.iter() {
        print!("{} ", c)
    }
    println!("");
}


fn main() {
    let (port, chan) : (Port<[u8,..81]>, Chan<[u8,..81]>) = Chan::new();
    reader(chan);
    for _ in range(0,MX) {
        let arr = port.recv();
        let mut poss = buildposs(arr);
        solve(arr, poss);
        for bs in poss.iter() {
     //       print(bs);
        }
    }
}

