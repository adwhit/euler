extern mod extra;
use std::num::sqrt;
use extra::bitv::Bitv;
use std::iter::range_step;


static MX:uint = 1000000;

fn sieve(to:uint) -> Bitv {
    let mut v = Bitv::new(to, true);
    v.set(0, false);
    v.set(1, false);
    for x in range(1, to) {
        // for each number, if not prime then stop
        if !v.get(x) {
            continue
        }
        for k in range_step(2*x, to, x) {
            v.set(k, false)
        }
    }
    v
}

fn div_sum(n:uint) -> uint {
    let mut ct = 1u;
    for m in range(2,sqrt(n as f64) as uint +1) {
        if n%m == 0 {
            ct += m;
            ct += n/m;
        }
    }
    ct
}

fn contains(v:&[uint], n:uint) -> int {
    for (i,&m) in v.iter().enumerate() {
        if m == n {
            return i as int
        }
    }
    -1
}

fn main() {
    let mut dies = sieve(MX);
    dies.set(1, true);
    let mut best = 0;
    for n in range(2u,MX) {
        if !dies.get(n) {
            let mut chain:~[uint] = ~[];
            chain.push(n);
            let mut k = n;
            loop {
                k = div_sum(k);
                //println!("{} {} {:?}",n, k, chain);
                if k >= MX || dies.get(k) { 
                    for &d in chain.iter() {
                        dies.set(d, true);
                    }
                    break
                }
                if contains(chain, k) >= 0 { //We have hit a chain
                    //println!("{} {} {:?}",n, k, chain);
                    for &d in chain.iter() {
                        dies.set(d, true);
                    }
                    if chain.len() > best {
                        let i = contains(chain, k);
                        best = chain.len() - i as uint;
                        println!("{} {} {} {:?}",best, n, k, chain.slice(i as uint,chain.len()));
                    }
                    break
                }
                chain.push(k);
                //println!("{} {:?}",k, chain);
            }
        }
    }
}
