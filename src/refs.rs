pub fn test() {
    let mut m = 0;

    let r = &mut m;
    *r = *r + 1;

    // ERROR: m is mutable borrowed by r, bute r is used below
    //println!("{}", m);

    // destructuring r (&mut i32) into mutable m2; same as: let mut m2 = *r;
    let &mut mut m2 = r;    // m2 is a copy
    m2 = m2 - 1;
    assert!(m == 1 && m2 == 0);

    // same as: let r2 = &mut m;
    let ref mut r2 = m;
    *r2 = *r2 + 1;  // m == 2

    // ERROR: r2 is immutable
    //r2 = &mut m;

    // r2 lifetime ends here, m may be accessed
    assert!(m == 2);

    // destructuring &mut i32 into mutable ref m
    // same as: let r3 = &mut m OR let ref mut r3 = m;
    let &mut ref mut r3 = &mut m;
    *r3 = *r3 + 1;

    // r3 liefetime ends here, m may be accessed
    assert!(m == 3);

    //ERROR: mutable by-reference bindings are experimental
    //let mut ref r4 = m;

    // borrow m by immutable reference
    let r4 = &m;
    assert!(*r4 == 3);
}
