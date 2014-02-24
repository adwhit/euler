static niter : int = 50;
static inditer : int = 900;

fn main() {
    let probs = makeprobs();
    let mut list: ~[~[f64]] = ~[];
    for i in range(0, inditer) {
        let mut arr1 : ~[f64] = ~[0.0,..900];
        let mut arr2 : ~[f64] = ~[0.0,..900];
        arr1[i] = 1.0;
        for _ in range(0,niter) {
            step(arr1, arr2, probs);
            std::mem::swap(&mut arr2, &mut arr1);
        }
        list.push(arr1);
    }
    let mut res : ~[f64] = ~[1.0,..900];
    for arr in list.iter() {
        for ix in range(0,900) {
            res[ix] *= (1.0 - arr[ix])
        }
    }
    let mut sum = 0.0;
    for &val in res.iter() { sum += val }
    println!("{}", sum)
}

fn step(arr1 : &mut[f64], arr2: &mut[f64], probs: &[f64]) {
    for ix in range(0,900) {
        let v = arr1[ix];
        if v == 0.0 { continue }
        if ix >= 30 { arr2[ix-30] += v * probs[ix*4]};
        if ix > 0 { arr2[ix-1] += v * probs[ix*4 + 1]};
        if ix < 899 { arr2[ix+1] += v * probs[ix*4 + 2]};
        if ix < 870 { arr2[ix+30] += v * probs[ix*4 + 3]};
        arr1[ix] = 0.0;
    }
}

fn makeprobs() -> ~[f64] {
    let mut probs = ~[];
    for ix in range(0,900) {
        let i = ix / 30;
        let j = ix % 30;
        let mut totin = 0;

        let up = ix-30; 
        let left = ix-1; 
        let right = ix+1; 
        let down = ix+30; 

        let upin = { if (up/30 == i-1) && up >= 0 { totin += 1; true } else { false } };
        let leftin = { if (left % 30 == j-1) && left >= 0 { totin += 1; true } else { false } };
        let rightin = { if (right % 30 == j+1) && right < 900 { totin += 1; true } else { false } };
        let downin = { if (down/30 == i+1) && down < 900 { totin += 1; true } else { false } };

        let p = 1.0/(totin as f64);
        if upin { probs.push(p) } else { probs.push(0.0) };
        if leftin { probs.push(p) } else { probs.push(0.0) };
        if rightin { probs.push(p) } else { probs.push(0.0) };
        if downin { probs.push(p) } else { probs.push(0.0) };

    }
    probs
}



