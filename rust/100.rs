fn main() {
    let mut x = 2u;
    let mut y = 3u;
    let mut xpre;
    let mut ypre;
    loop {
        xpre = x;
        ypre = y;
        x = xpre + ypre;
        y = 2*xpre + ypre;
        if ypre*y > 1000000000000 {
            if ypre*x > xpre * y { println!("{}", ypre*x) }
            else { println!("{}", xpre*y) };
            break
        }
    }
}
