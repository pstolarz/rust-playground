fn max<'a, 'b, 'c>(a: &'a i32, b: &'b i32, c: &'c i32) -> &'c i32
where
    // a,b data may flow into c
    'a: 'c, 'b: 'c

    // Alternatively:
    // b may flow into a, and a into c, so a,b may flow into c
    // 'b: 'a, 'a: 'c
    // a may flow into b, and b into c, so a,b may flow into c
    // 'a: 'b, 'b: 'c

    // but error here (a may flow into c but not b):
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

fn _f<'a, 'b>(_a: &'a i32, mut _b: &'b i32)
where 'a: 'b
{
    // ok: a data may flow into b
    _b = _a;

    // &'b &'a i32 is well formed because 'a: 'b
    let _r: &'b &'a i32 = &&0;
}

pub fn test()
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
