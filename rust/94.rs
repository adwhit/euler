use std::num::gcd;
static MX:uint = 1000000000;

fn main() {
    let mut sum = 0u;
    let mut n = 1u;
    'nloop : loop { //n loop
        let mut m = n+1;
        'mloop : loop { //m loop
            if gcd(m,n) == 1 {
                let mut k = 1u;
                'kloop : loop {
                    let a = k*(m*m - n*n);
                    let b = k*2*m*n;
                    let c = k*(m*m + n*n);
                    //println!("n:{} m:{} k:{} a:{} b:{} c:{} L:{}", n, m, k,a,b,c, a+b+c)
                    if 2*(a+c) > MX && 2*(b+c) > MX {
                        if k > 1 {              //increment m, loop k again
                        break 'kloop
                        } else if m - n > 1  {   //increment n, loop m again
                            break 'mloop
                        } else {
                            break 'nloop
                        }
                    }
                    if a * 2 + 1 == c || a * 2 - 1 == c {
                        //println!("twin:  {}\t single:  {}",c, 2*a);
                        sum += 2*(a+c)
                    } else if b * 2 + 1 == c || b * 2 - 1 == c {
                        //println!("twin:  {}\t single:  {}",c, 2*b)
                        sum += 2*(b+c)
                    }
                    k += 1
                }
            }
            m += 2
        }
        n += 1
    }
    println!("Sum of perimeters: {}", sum)
}

