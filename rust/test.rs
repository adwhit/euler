extern mod extra;
use extra::bitv::BitvSet;
use extra::bitv::Bitv;

fn main() {
    let mut a: int = 0;
    let adder = |x: int| { a += 1; x + a };

    a = 10;

    println!("adder(1) = {} ", adder(1));
    println!("a = {}", a);
}
