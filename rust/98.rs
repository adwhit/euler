use std::hashmap::{HashMap, HashSet};
use std::io::File;

fn getdupes() -> HashMap<~[u8], ~[~[u8]]> {
    let bytes = File::open(&Path::new("../data/words.txt")).read_to_end();
    let mut words:~[~[u8]] = ~[];
    let mut word:~[u8] = ~[];

    for &b in bytes.iter() {
        if !(b == '"' as u8 || b == ',' as u8) {
            word.push(b);
        } else if b == ',' as u8 {
            words.push(word);
            word = ~[];
        }
    }
    let mut dupes = HashMap::<~[u8], ~[~[u8]]>::new();
    let mut map = HashMap::<~[u8], uint>::new();
    for word in words.iter() {
        let mut k = word.clone();
        k.sort();
        map.insert_or_update_with(k, 1, |_, v| *v+= 1);
    }
    for word in words.iter() {
        let mut k = word.clone();
        k.sort();
        if map.get(&k) > &1u {
            let w = word.clone();
            dupes.mangle(k, w, |_, v| ~[v], |k,v,w| v.push(w));
        }
    }
    dupes
}

fn concat(v:&[uint]) -> uint {
    let mut s = 0u;
    for &i in v.iter() {
        s = (s + i as uint)*10
    }
    s/10
}

fn combination(c:&mut [uint],n:uint,p:uint,x:uint){
    let mut r:uint;
    let mut k = 0;
    for i in range(0,p-1) {
        if i != 0 { 
            c[i] = c[i-1]
        } else {
            //init to zero
            c[i] = 0
        }
        loop {
            c[i] += 1;
            r = choose(n-c[i], p-(i+1));
            k += r;
            if k >= x { 
                break
            }
        }
        k -= r
    }
    c[p-1] = c[p-2] + x - k;
    for i in range(0,p) { c[i] -= 1; }
}

fn choose(n:uint, c:uint) -> uint{
    let mut r = 1;
    for m in range(c+1,n+1) {
        r *= m
    }
    for m in range(1, n-c+1) {
        r /= m
    }
    r
}

fn nextperm(a:&mut [uint]) {
    //println!("{:?}", a);
    let ln = a.len();
    let mut k:uint = ln;
    let mut l:uint = ln;
    let mut tmp:uint;
    for i in range(0u,ln-1) {
        if a[i] < a[i+1]  { k = i }
    }
    assert!(k != ln);
    for i in range(0u,ln) {
        if a[k] < a[i] { l = i }
    }
    tmp = a[k]; a[k] = a[l]; a[l] = tmp;
    let l = ln - k; 
    for i in range(0,l/2) {
        tmp = a[k+l-i-1]; a[k+l-i-1] = a[k+i+1]; a[k+i+1] = tmp;
    }
}

fn assign(v:&[u8], perm:&mut[uint], x:uint) -> HashMap<u8, uint> {
    //assume already removed dupes
    let mut score = HashMap::<u8, uint>::new();
    let l = v.len();
    let mut c:~[uint] = std::vec::from_elem(l, 0u);
    combination(c,10,l,x);
    //println!("combo {:?}", c)
    for i in range(0,l) {
        score.insert(v[i], c[perm[i]]);
    }
    score
}

fn rmdupe(a:&[u8]) -> ~[u8] {
    let mut v:~[u8] = ~[];
    for &val1 in a.iter() {
        for &val2 in v.iter() {
            if val1 == val2 {
                break
            }
        }
        v.push(val1)
    }
    v
}

fn issqr(x:uint) -> bool {
    let rt = std::num::sqrt(x as f64);
    if (rt as uint) * (rt as uint) == x {
        return true
    } else {
        return false
    }
}

fn fact(n:uint) -> uint {
    let mut s = 1u;
    for n in range(2, n+1) {
        s*=n
    }
    s
}

fn main() {
    let dupes = getdupes();
    let mut sqr:uint = 0;
    for (k, v) in dupes.iter() {
        let k2 = rmdupe(*k);
        //for each combination
        let mut perm:~[uint] = ~[];
        for i in range(0,k.len()) { perm.push(i) }
        for y in range(0, fact(k.len())) {
            if y != 0 {nextperm(perm);}
            'outer : for x in range(0,choose(10,k.len())) {
                let score = assign(k2, perm, x);
                for vec in v.iter() {
                    let mut nvec:~[uint] = ~[];
                    for i in vec.iter() {
                        nvec.push(*score.get(i));
                    }
                    if nvec[0] == 0 {
                        continue 'outer
                    }
                    sqr = concat(nvec);
                    if !issqr(sqr) {
                        continue 'outer
                    }
                    //println!("Square! {}, vec:{:?}, {}", sqr, nvec, std::str::from_utf8(*vec));
                }
                for vec in v.iter() {
                    let mut nvec:~[uint] = ~[];
                    for i in vec.iter() {
                        nvec.push(*score.get(i));
                    }
                    sqr = concat(nvec);
                    println!("Square pair! {}, vec:{:?}, {}", sqr, nvec, std::str::from_utf8(*vec));
                }
            }
        }
    }
}
