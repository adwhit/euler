use std::hashmap::HashMap;
use std::num::sqrt;

fn factor(n:uint) -> uint {
    for m in range(2u,sqrt(n as f32) as uint) {
        if (n/m)*m == n {
            return m
        }
    }
    1
}

fn main() {
    let mut scores = HashMap::<uint, ~[uint]>::new();
    let mut best = [0u,..12000];
    let mut ct = 0;
    for n in range(0,100u) {
        let m = factor(n);
        if m > 1 {
            let scr = n-(m+n/m)+2;
            //add score to list
            scores.mangle(n, scr, |k,v| ~[v], |k, prev, v| { prev.push(v) } );
            //if score has not been set, set to n
            if best[scr] == 0 { 
                best[scr] = n;
                ct += 1;
                if ct > 12000 {
                    break
                }
            }
        } else { //prime
            score.push(0)
        }
    }
}

