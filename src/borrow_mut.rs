use core::ops::AddAssign;

#[derive(Debug)]
struct S(i32);

impl AddAssign<i32> for S {
    fn add_assign(&mut self, rhs: i32) {
       self.0 += rhs;
    }
}

pub fn test()
{
    let mut s = S(0);           // s owns the object
    s += 1;

    let mut r1 = &mut s;        // mut-borrow (r1)
    *r1 += 1;

    // r1 borrow ends here

    let r2 = &mut s;            // mut-borrow (r2)
    *r2 += 1;

    // ERROR: cannot mut-borrow more than once
    // (only one mut-borrow may change the object at a time)
    //*r1 += 1;

    // r2 borrow ends here

    // re-mut-borrow using r1 (aka let r1 = &mut s)
    r1 = &mut s;
    *r1 += 1;

    s += 1;
    println!("{:?}", s);
}                               // s end of life
