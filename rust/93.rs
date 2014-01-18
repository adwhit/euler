use std::hashmap::HashSet;

fn add(a:f32, b:f32) -> f32 { a + b }
fn sub(a:f32, b:f32) -> f32 { a - b }
fn mul(a:f32, b:f32) -> f32 { a * b }
fn div(a:f32, b:f32) -> f32 { a / b }

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
    let mut k:uint = 4;
    let mut l:uint = 4;
    let mut tmp:uint;
    for i in range(0u,3u) {
        if a[i] < a[i+1]  { k = i }
    }
    assert!(k != 4);
    for i in range(0u,4u) {
        if a[k] < a[i] { l = i }
    }
    tmp = a[k]; a[k] = a[l]; a[l] = tmp;
    let l = 4 - k; 
    for i in range(0,l/2) {
        tmp = a[k+l-i-1]; a[k+l-i-1] = a[k+i+1]; a[k+i+1] = tmp;
    }
}

fn score(nums:[uint,..4], numord:[uint,..4], algord:[uint,..3], pairs:bool) -> uint {
    let mut r:f32;
    let fns = [add,sub,mul,div];
    let v:[f32,..4] = [nums[numord[0]] as f32,nums[numord[1]] as f32,nums[numord[2]] as f32,nums[numord[3]] as f32];
    if pairs {
        let s1 = fns[algord[0]](v[0],v[1]);
        let s2 = fns[algord[1]](v[2],v[3]);
        r = fns[2](s1,s2);
    } else {
        r = v[0];
        for i in range(1,4) {
            r = fns[algord[i-1]](r, v[i])
        }
    }
    if (r as uint) as f32 == r {
        return r as uint
    } else {return 0}
}


fn main() {
    let mut best = 0u;
    let mut nums = [0u, ..4];
    for x in range(1,choose(9,4) + 1) {
    //for x in range(1,3u) {
        combination(nums, 9, 4, x);
        let mut numorder = [0u, 1u,2u,3u];
        let mut set = HashSet::new();
        for y in range(0,24) {
            if y != 0 { nextperm(numorder) }
            //println!("nums {:?} order {:?}", nums, numorder);
            for z1 in range(0,4u) {
                for z2 in range(0,4u) {
                    for z3 in range(0,4u) {
                        let algorder = [z1,z2,z3];
                        let mut s = score(nums, numorder, algorder, false);
                        set.insert(s);
                        s = score(nums, numorder, algorder, true);
                        set.insert(s);
                    }
                }
            }
        }
        let mut ct = 1u;
        loop {
            if !set.contains(&ct) { break }
            ct += 1
        }
        if ct -1 > best {
            best = ct -1;
            println!("best: {} combo: {:?}", best, nums);
        }
    }
}
