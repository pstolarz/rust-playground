trait Tr<T> {
    type F;             // associated type
    const C: u32;       // associated const
    fn m(&self) -> T;   // method
}

struct S<T> { v: T }

impl<T: Copy> Tr<T> for S<T> {
    type F = T;
    const C: u32 = 1;
    fn m(&self) -> T { self.v }
}

fn test_1(_: &impl Tr<u8>) {}
fn _test_2(_: &impl Tr<u16>) {}

fn test_3(_: &impl Tr<u8, F = u8>) {}
fn _test_4(_: &impl Tr<u8, F = u16>) {}

// assoc. const not supported: https://github.com/rust-lang/rust/issues/70256
// fn test_6(_: &impl Tr<u8, C = 1>) {}

pub fn test() {
    // not sure why S<u8> is not allowed
    let s = S::<u8> { v: 1 };

    test_1(&s);
    // _test_2(&s);     // ERROR: Tr<u16> not implemented for S
    test_3(&s);
    // _test_4(&s);     // ERROR: assoc. type mismatch
}
