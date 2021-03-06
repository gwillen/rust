iface add {
    fn +(++x: self) -> self;
}

impl of add for int {
    fn +(++x: int) -> int { self + x }
}

fn do_add<A:add>(x: A, y: A) -> A { x + y }

fn main() {
    let x = 3 as add;
    let y = 4 as add;
    do_add(x, y); //~ ERROR a boxed iface with self types may not be passed as a bounded type
}
