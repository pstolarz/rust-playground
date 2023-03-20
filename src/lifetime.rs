fn _f<'a, 'b>(_a: &'a i32, mut _b: &'b i32)
where 'a: 'b
{
    // ok: &'a i32 is subtype of &'b i32 (therefore may coerce into it)
    _b = _a;

    // &'b &'a i32 is well formed (coercion from base to subtype)
    let _r: &'b &'a i32 = &&0;
}

// ----------------------------------------------------------------------------

fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
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

        // 'b: 'a, therefore 'a is coerced into 'b,
        // so both choose_first() args have the same lifetime 'b
        res = choose_first(&y, &x);
        println!("{} is the first (2)", res);
    };
}

// ----------------------------------------------------------------------------

fn max<'a, 'b, 'c>(a: &'a i32, b: &'b i32, c: &'c i32) -> &'c i32
where
    // 'a,'b may coerce `c
    'a: 'c, 'b: 'c

    // Alternatively:
    // 'b may coerce into 'a, and 'a into 'c
    // 'b: 'a, 'a: 'c
    // 'a may coerce into 'b, and 'b into 'c
    // 'a: 'b, 'b: 'c

    // but error here ('a may coerce into 'c but not 'b):
    // 'a: 'b, 'a: 'c
{
    let ab = if a > b { a } else { b };
    if ab > c { ab } else { c }
}

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
            // lifetimes are confined to shortest lifetime c to fulfill
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
