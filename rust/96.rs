extern mod extra;
use std::io::buffered::BufferedReader;
use std::io::File;
use extra::bitv::{BitvSet, Bitv};

static MX:u8 = 50;

fn reader(chan:Chan<[u8,..81]>) {
    let path = Path::new("../data/sudoku.txt");
    let mut file = BufferedReader::new(File::open(&path));
    let mut arr:[u8,..81] = [0, ..81];
    for _ in range(0,MX) {
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
    let mut opts:BitvSet = BitvSet::from_bitv(Bitv::new(10, false));
    for i in range(1,10u) {
        opts.insert(i);
    }
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

fn solve(poss: &mut[BitvSet]) {
    let hstep = |i:int, j:int| i*9 + j;
    let vstep = |i:int, j:int| i + j*9;
    let blkstep = |i:int, j:int| (i/3)*27 + (i%3)*3 + (j/3)*9 + (j%3);
    let fns = [hstep, vstep, blkstep];
    for i in range(0,9) {
        for j in range(0,9) {
            for f in fns.iter() {
                let ind = (*f)(i,j);
                // last-man-standing method
                let l = poss[ind].len();
                let mut cteq = 0;
                for k in range(j,9) {
                    //search for matches
                    if poss[(*f)(i,k)] == poss[ind] { cteq += 1 }
                    if cteq == l {
                        //iterate and change all
                        let bs = poss[ind].clone();
                        for n in range(0,9) {
                            let ind2 = (*f)(i,n);
                            if poss[ind2] != bs {
                                poss[ind2].difference_with(&bs);
                            }
                        }
                        break
                    }
                }
                //elimination method
                let mut bset = BitvSet::new();
                //construct set of can-be-in-rest-of-block
                for m in range(0,9) {
                    let ind3 = (*f)(i,m);
                    if ind3 != ind {
                        for p in poss[ind3].iter() {
                            bset.insert(p);
                        }
                    }
                }
                let mut newbs = BitvSet::new();
                for q in poss[ind].iter() {
                    if !bset.contains(&q) {
                        newbs.insert(q);
                        break
                    }
                }
                if newbs.len() == 1 {
                    poss[ind] = newbs
                }
            }
        }
    }
}

fn guess(poss: &mut[BitvSet], firstn: bool) -> (uint, u8){
    for i in range(0,81u) {
        if poss[i].len() == 2 {
            for (n,j) in poss[i].iter().enumerate() {
                if (n == 0 && firstn) || (n == 1 && !firstn) {
                    return (i, j as u8)
                }
            }
        }
    }
    return (0, 0)
}

fn updatearr(arr: &mut[u8,..81], poss: &mut[BitvSet]) -> bool {
    let mut arrchanged = false;
    for i in range(0,9) {
        for j in range(0,9) {
            if poss[i*9+j].len() == 1 && arr[i*9 +j] == 0 {
                for n in range(1u,10u) {
                    if poss[i*9+j].contains(&n) {
                        arr[i*9+j] = n as u8;
                        arrchanged = true;
                    }
                }
            }
        }
    }
    arrchanged
}

fn finished(arr:[u8, ..81]) -> bool {
    for &n in arr.iter() {
        if n == 0 {
            return false
        }
    }
    true
}

fn legal(arr:[u8, ..81]) -> bool {
    let hstep = |i:int, j:int| i*9 + j;
    let vstep = |i:int, j:int| i + j*9;
    let blkstep = |i:int, j:int| (i/3)*27 + (i%3)*3 + (j/3)*9 + (j%3);
    let fns = [hstep, vstep, blkstep];
    for i in range(0,9) {
        for f in fns.iter() {
            let mut ct = 0;
            for j in range(0,9) {
                ct += arr[(*f)(i,j)];
            }
            if ct != 45 {
                return false
            }
        }
    }
    true
}

fn printposs(poss:&[BitvSet]) {
    for (i,bs) in poss.iter().enumerate() {
        let c:~[uint] = bs.iter().collect();
        if i%9 == 0 {
            println!("-----------row {}-------------", i/9 + 1)
        }
        println!("{:?}", c);
    }
}

fn printarr(arr:[u8,..81]) {
    for i in range(0,81) {
        if i % 3 == 0 && i != 0 && !(i%9 == 0){
            print!("| ")
        }

        if i % 9 == 0 && i != 0 {
            println!("")
        }
        if i % 27 == 0 && i != 0 {
            println!("--------------------")
        }
        print!("{} ", arr[i]);
    }
    println!("\n")
}

fn main() {
    let (port, chan) : (Port<[u8,..81]>, Chan<[u8,..81]>) = Chan::new();
    reader(chan);
    let mut succ = 0;
    let mut score = 0;
    for _ in range(0,MX) {
        let arr = port.recv();
        let r = solveloop(arr);
        if r > 0 {
            succ += 1;
            score += r;
        }
    }
    println!("{} succeeded, {} failed", succ, MX-succ);
    println!("Score: {}", score);
}

fn solveloop(mut arr: [u8,..81]) -> uint {
    let mut poss = buildposs(arr);
    let mut nochgct = 0;
    loop {
        solve(poss);
        if !updatearr(&mut arr, poss) { 
            nochgct += 1;
            if nochgct >= 10 {
                //stuck - take a guess
                //locate change
                let bools = [true, false];
                for &b in bools.iter() {
                    let (i, n) = guess(poss, b);
                    let mut newarr = arr;
                    newarr[i] = n;
                    match solveloop(newarr) {
                        0 => continue,
                        n => return n
                    };
                }
                return 0
            }
        } else { nochgct = 0 }
        if finished(arr) { 
            if !legal(arr) {
                return 0
                //borked - rewind
            } else {
                return (arr[0] as uint) * 100 + (arr[1] as uint) * 10 + arr[2] as uint
            }
        }
    }
}

