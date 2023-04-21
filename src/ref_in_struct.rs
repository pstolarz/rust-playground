//
// Generic approach: capable to store T, &T and &mut T.
//
struct S1<T> {
  t: T
}

impl<T> S1<T> {
    fn new(t: T) -> S1<T> {
        // same as S1::<T>
        S1 {t: t}
    }
}

//
// Specific approach: &mut T only.
//
struct S2<'a, T>
where T: 'a {
  t: &'a mut T
}

impl<'a, T> S2<'a, T> {
    fn new(t: &mut T) -> S2<'_, T> {
        // same as S1::<'a, T>
        S2 {t: t}
    }
}

#[derive(Debug)]
struct A(u32);

#[allow(unused_labels)]
pub fn test() {
    let s1;
    let s2;

    'a: {
        let mut a = A(0);

        // S1<&mut A> is deduced
        s1 = S1::new(&mut a);
        s1.t.0 += 1;
        println!("{:?}", s1.t);

        // S2<'a, A> is deduced
        s2 = S2::new(&mut a);
        s2.t.0 += 1;
        println!("{:?}", s2.t);
    }

    // ERROR: dangling references
    //s1.t;
    //s2.t;
}
