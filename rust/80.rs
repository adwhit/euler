use std::iter::range_step_inclusive;
use std::fmt;

#[deriving (Clone)]
struct BigFloat {
    head : int,
    tail : ~[i8],
    prec : uint
}

impl BigFloat {
    pub fn new(head: int, mut tail: ~[i8], prec: uint) -> BigFloat {
        if tail.len() > prec {
            fail!("tail longer than precision!")
        } else if tail.len() < prec {
            for _ in range(0, prec - tail.len()) { tail.push(0) }
        }
        BigFloat{head: head, tail: tail, prec: prec}
    }

    pub fn add<'a>(&self, other: &BigFloat) -> &'a mut BigFloat {
        let mut res = &'a mut self.clone();
        res.head += other.head;
        for i in range_step_inclusive((res.prec-1) as int,0 as int,-1 as int) {
            res.tail[i] += other.tail[i];
            if res.tail[i] > 9 {
                match i {
                    0 => {res.head += 1; res.tail[i] -= 10},
                    _ => { res.tail[i-1] += 1; res.tail[i] -= 10}
                }
            }
        }
        res
    }

    pub fn sub<'a>(&'a mut self, other: &'a BigFloat) -> &'a mut BigFloat {
        assert!(self.head > other.head)
        self.head -= other.head;
        for i in range_step_inclusive((self.prec-1) as int,0,-1) {
            self.tail[i] -= other.tail[i];
            if self.tail[i] < 0 {
                match i {
                    0 => {self.head -= 1; self.tail[i] += 10},
                    _ => { self.tail[i-1] -= 1; self.tail[i] += 10}
                }
            }
        }
        self
    }

    pub fn div<'a>(&'a mut self, other: &'a BigFloat) -> &'a mut BigFloat {
        let mut tmp = &'a mut self.clone();
        tmp
    }
}

impl fmt::Default for BigFloat {
    fn fmt(obj: &BigFloat, f: &mut fmt::Formatter) {
        let mut v:~[u8] = ~[];
        for n in obj.tail.iter() {
           v.push((*n + 48) as u8)
        }
       write!(f.buf, "{}.{}", obj.head, std::str::from_utf8(v))
    }
}

fn main() {
    let mut f = BigFloat::new(2,~[6,0,0],3);
    let mut g = BigFloat::new(1,~[5,4,4],3);
    g.sub(&mut f);
    println!("{:?}", g);
    //println!("{:?}", f);
}
