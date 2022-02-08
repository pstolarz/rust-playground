pub fn test() {
    let mut m = 0;

    let r = &mut m;
    *r = *r + 1;

    // destructuring r (&mut i32) into mutable object m2; same as: let mut m2 = *r;
    let &mut mut m2 = r;    // m2 = m
    m2 = m2 - 1;
    assert!(m == 1 && m2 == 0);

    // same as: let r2 = &mut m;
    // no idea how to declare let mut r2 = &mut m; using this syntax
    let ref mut r2 = m;
    *r2 = *r2 + 1;  // m == 2

    // ERROR: r2 is immutable
    // r2 = &mut m;

    // destructuring r2 (&mut i32) into mutable ref pointing into m
    // same as: let r3 = r2;
    let &mut ref mut r3 = r2;
    *r3 = *r3 + 1;
    assert!(m == 3);
}
