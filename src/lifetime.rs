fn _f<'a, 'b>(_a: &'a i32, mut _b: &'b i32)
where 'a: 'b
{
    // ok: 'a has longer lifetime than 'b i32 (therefore may coerce to it)
    _b = _a;

    // &'b &'a i32 is well formed (coercion from longer to shorter lifetime)
    let _r: &'b &'a i32 = &&0;
}

// ----------------------------------------------------------------------------

fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

// better approach (using newer notation)
fn choose_first2<'a>(first: &'a i32, _: &'a i32) -> &'a i32 {
    first
}

fn test_1() {
    let x = 2; // longer lifetime ('a)
    let mut res: &i32;

    {
        let y = 3; // shorter lifetime ('b)

        // 'a: 'b, so their fits choose_first() arguments w/o coercion
        res = choose_first(&x, &y);
        println!("{} is the first (1)", res);
        assert!(res == choose_first2(&x, &y));

        // 'b: 'a, therefore 'b is coerced to 'a,
        // so both choose_first() args have the same lifetime 'a
        res = choose_first(&y, &x);
        println!("{} is the first (2)", res);
        assert!(res == choose_first2(&y, &x));
    };
}

// ----------------------------------------------------------------------------

fn max<'a, 'b, 'c>(a: &'a i32, b: &'b i32, c: &'c i32) -> &'c i32
where
    // 'a,'b may coerce to `c
    'a: 'c, 'b: 'c

    // Alternatively:
    // 'b may coerce to 'a, and 'a into 'c
    // 'b: 'a, 'a: 'c
    // 'a may coerce to 'b, and 'b into 'c
    // 'a: 'b, 'b: 'c

    // but error here ('a may coerce to 'c but not 'b):
    // 'a: 'b, 'a: 'c
{
    let ab = if a > b { a } else { b };
    if ab > c { ab } else { c }
}

// better approach (using newer notation)
fn max2<'a>(a: &'a i32, b: &'a i32, c: &'a i32) -> &'a i32
{
    let ab = if a > b { a } else { b };
    if ab > c { ab } else { c }
}

fn test_2()
{
    let a = 1;
    let res: &i32;
    let res2: &i32;

    {
        let b = 2;

        {
            let c = 3;

            // Both max() and max2() are equivalent
            //
            // NOTE: Regardless of lifetimes ranges passed to max() the
            // lifetimes are coerced to shortest lifetime c to fulfill
            // bounds as specified by the where-clause.
            res = max(&c, &b, &a);
            res2 = max2(&c, &b, &a);
            assert!(*res == 3 && *res2 == 3);
        }
        // ERROR: refs does not live long enough
        // res; res2;
    }
    // ERROR: refs does not live long enough
    // res; res2;
}

// ----------------------------------------------------------------------------

pub fn test()
{
    test_1();
    test_2();
}
