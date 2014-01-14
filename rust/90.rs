use std::hashmap::HashSet;
use std::vec::append;

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
    if c[5] == 9 { c[5] = 6 }
}

fn contains(c:&[uint],n:uint) -> bool {
    for &m in c.iter() {
        if m == n {
            return true
        }
    }
    false
}

fn checker(c:&[uint], d:&[uint], i:uint) -> int {
    let inc = contains(c, i);
    let ind = contains(d, i);
    if inc && ind { 
        return 0
    } else if !inc && !ind {
        return -1
    } else if inc {
        return 1
    } else {
        return 2
    }
}

fn legal(c:&[uint],d:&[uint]) -> bool {
    let mut ind:~[uint] = ~[];
    let mut inc:~[uint] = ~[];
    let mut flag:int;
    let reqs:~[~[uint]] = ~[~[1,4,6],~[6,8],~[5],~[6],~[0,6],~[2],~[3,4,0],~[],~[1]];
    for i in range(0,9u) {
        if i != 7 {
            flag = checker(c, d, i);
            if flag == -1 {
                return false
            } else if flag == 1 {
                ind.push_all(reqs[i])
            } else if flag == 2 {
                inc.push_all(reqs[i])
            }
        }
    }
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
    let rn = choose(10,6);
    let mut c = [0u, 0u,0u,0u,0u,0u];
    let mut d = [0u, 0u,0u,0u,0u,0u];
    let mut ct = 0;
    for i in range(0, rn) {
        combination(c, 10, 6, i+1);
        for j in range(i,rn) {
            combination(d, 10, 6, j+1);
            if legal(c,d){
                ct += 1;
                //println!("{} {:?},{:?}", ct,c,d);
            }
        }
    }
    println!("{}", ct);
}
