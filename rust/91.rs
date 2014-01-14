static MX:int = 51;

fn main () {
    let mut ct = 0;
    for x1 in range(0, MX) {
        for x2 in range(x1, MX) {
            for y1 in range(0, MX) {
                for y2 in range(0, MX) {
                    if (x1 == 0 && y1 == 0) ||
                       (x2 == 0 && y2 == 0) ||
                       (x1 == x2 && y1 == y2) ||
                       ((x1 ==x2) && (y1 < y2)) {
                        continue
                    }
                    if x1*x2 + y1*y2 == 0 ||
                       x1*(x2-x1) + y1*(y2-y1) == 0 ||
                       x2*(x2-x1) + y2*(y2-y1) == 0 {
                       ct += 1;
                    }
                }
            }
        }
    }
    println!("{}", ct)
}
