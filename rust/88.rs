use std::hashmap::{HashMap,HashSet};
use std::num::sqrt;

static MX:uint = 12000;

//calculate factors up to sqrt n
fn factors(n:uint) -> ~[uint] {
    let mut facts:~[uint] = ~[];
    for m in range(2u,sqrt(n as f32) as uint + 1) {
        if (n/m)*m == n {
            facts.push(m)
        }
    }
    facts
}

fn main() {
    let mut scores = HashMap::<uint, ~[uint]>::new();
    let mut best = [0u,..MX];
    let mut tally = 0;
    'outer : for n in range(1,2*MX) {
        let tmp = factors(n);
        for &f1 in tmp.iter() {
            let f2 = n/f1;
            let basescore = n-(f1+f2) + 2;
            //println!("{}: f1 {}, f2 {}, score {}", n, f1, f2, basescore);
            if basescore <= MX {
                scores.insert_or_update_with(n, ~[basescore], |_, pre| pre.push(basescore));
                let subs = scores.find_or_insert(f2, ~[]).clone();
                for &p in subs.iter() {
                    let scr = basescore + p - 1 ;
                    //add score to list
                    scores.insert_or_update_with(n, ~[scr], |_, pre| pre.push(scr) );
                }
            }
        }
        let scrs = scores.find_or_insert(n, ~[]);
        //println!("{}: {:?}", n, scrs);
        for &s in scrs.iter() {
            if s <= MX {
                if best[s-1] == 0 {
                    best[s-1] = n;
                    tally += 1;
                    if tally > MX - 2 {
                        break 'outer
                    }
                }
            }
        }
    }
    //println!("{:?}", best);
    let mut set = HashSet::<uint>::new();
    for &s in best.iter() {
        if s > 0 {
        set.insert(s);
        }
    }
    //println!("{:?}", set);
    let mut ct = 0;
    for &k in set.iter() {
        ct += k
    }
    println!("{}", ct);
}

