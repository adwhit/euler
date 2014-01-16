extern mod extra;
use extra::bitv::BitvSet;
use extra::bitv::Bitv;
fn main() {
    let mut bv2 = Bitv::new(100, false);
    println!("Bit set, from bitv 2:");
    let mut bs2 = BitvSet::from_bitv(bv2.clone());
    println!("Length: {}", bs2.len())
    println!("Capacity: {}", bs2.capacity())
    println!("Contains 127? {:b}", bs2.contains(&127));
    for k in bs2.iter() {
        println!("{}", k)
    }
    println!("{}", std::uint::bits);
}
