use std::hashmap::HashSet;

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

// get the [x]th lexicographically ordered set of [p] elements in [n]
// output is in [c], and should be sizeof(int)*[p] */
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
    for i in range(0,p) {
        c[i] -= 1
    }
}

fn counter(c:&mut [uint]) {
    let mut l = c.len();
    for i in range(0, l) {
        if c[l-1-i] >= 7 {
            c[l-1-i] = 0
        } else {
            c[l-1-i] += 1;
            for j in range(1,l) {
                if c[j] < c[j-1] {
                    c[j] = c[j-1]
                }
            }
            return
        }
    }
}

fn contains(c:&[uint],n:uint) -> bool {
    for &m in c.iter() {
        if m == n {
            return true
        }
    }
    false
}

fn legal(c:&[uint],d:&[uint]) -> bool {
    let mut set = HashSet::new();
    for n in c.iter().chain(d.iter()) { set.insert(n); }
    if set.len() < 8 {return false}

    let mut ind:~[uint] = ~[];
    let mut inc:~[uint] = ~[];
    if contains(c, 0) &! contains(d, 0) { ind.push(1); ind.push(4); ind.push(6); }
    if contains(d, 0) &! contains(c, 0) { inc.push(1); inc.push(4); inc.push(6); }
    if contains(c, 1) &! contains(d, 1){ ind.push(6); ind.push(7); }
    if contains(d, 1) &! contains(c, 1){ inc.push(6); inc.push(7); }
    if contains(c, 2) &! contains(d, 2){ ind.push(5); }
    if contains(d, 2) &! contains(c, 2){ inc.push(5); }
    if contains(c, 3) &! contains(d, 3){ ind.push(6); }
    if contains(d, 3) &! contains(c, 3){ inc.push(6); }
    if contains(c, 4) &! contains(d, 4){ ind.push(0); ind.push(6); }
    if contains(d, 4) &! contains(c, 4){ inc.push(0); inc.push(6); }
    if contains(c, 5) &! contains(d, 5){ ind.push(2); }
    if contains(d, 5) &! contains(c, 5){ inc.push(2); }
    if contains(c, 6) &! contains(d, 6){ ind.push(1); ind.push(3); ind.push(4); ind.push(0) }
    if contains(d, 6) &! contains(c, 6){ inc.push(1); inc.push(3); inc.push(4); inc.push(0) }
    if contains(c, 7) &! contains(d, 7){ ind.push(1); }
    if contains(d, 7) &! contains(c, 7){ inc.push(1); }
    //println!("{:?} {:?}", ind, inc);
    if ind.len() > 6 { return false }
    if inc.len() > 6 { return false }
    for &k in ind.iter() {
        if !contains(d, k) {
            return false
        }
    }
    for &k in inc.iter() {
        if !contains(c, k) {
            return false
        }
    }
    true
}

fn main() {
    let rn = choose(8,6);
    let mut c = [0u, 0u,0u,0u,0u,0u];
    let mut d = [0u, 0u,0u,0u,0u,0u];
    let mut ct = 0;
    for i in range(0, rn) {
        combination(c, 8, 6, i+1);
        //counter(c);
        for j in range(i,rn) {
            combination(d, 8, 6, j+1);
            //counter(d);
            //println!("{} {:?},{:?}", ct,c,d);
            if legal(c,d){
                ct += 1;
                println!("{} {:?},{:?}", ct,c,d);
            }
        }
    }
}
