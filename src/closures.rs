// non copyable struct
struct S(i32);

//
// In the following 3 functions it's crucial to pass a closure object by value.
// By using this it's possible to use common argument type for all of the
// Rust functional traits: FnOnce, FnMut and Fn, regardless the FnOnce moves
// the closure, while FnMut and Fn borrows it by &mut and & respectively.
// Additionally, since Fn-closure is (specifically in this example) a copyable
// object, passing the object by value allows to call Fn, FnMut and FnOnce
// methods on it.
//
// FnOnce and FnMut closures are not copyable, although for FnMut closure
// it's possible to call it multiple time by mutable reference.
//
fn call_fn_once(f: impl FnOnce() -> i32) -> i32 {
    f()
}

fn call_fn_mut<F>(mut f: F) -> i32 where
    F: FnMut() -> i32 {
    f()
}

fn call_fn<F>(f: F) -> i32 where
    F: Fn() -> i32 {
    f()
}

// FnMut-closure passed by mutable reference
fn call_fn_mut_ref<F>(f: &mut F) -> i32 where
    F: FnMut() -> i32 {
    f()
}

pub fn test()
{
    let s_moved = S(0);
    // FnOnce-closure, capture s_moved by value
    let cl_fn_once = || -> i32 {
        // move s_moved into closure
        let m = s_moved;
        m.0
    };

    call_fn_once(cl_fn_once);
    // ERROR: Fn and FnMut are supertraits of FnOnce
    //call_fn_mut(cl_fn_once);
    //call_fn(cl_fn_once);

    // ERROR: FnOnce-closure is not copyable (may be run only once)
    //let x = cl_fn_once;
    //cl_fn_once();

    let mut s_mut = S(1);
    // FnMut-closure, capture s_mut by &mut
    let mut cl_fn_mut = || -> i32 {
        s_mut.0 += 1;
        s_mut.0
    };

    // while passing FnMut by &mut reference it's possible to reuse it
    call_fn_mut_ref(&mut cl_fn_mut);
    call_fn_mut_ref(&mut cl_fn_mut);

    // ERROR: Fn is subtype on FnMut
    //call_fn(cl_fn_mut);

    call_fn_mut(cl_fn_mut);
    // ERROR: FnMut-closure is not copyable therefore can't be reused
    //let x = cl_fn_mut;
    //call_fn_once(cl_fn_mut);
    //call_fn_mut_ref(&mut cl_fn_mut);

    let s = S(2);
    // Fn-closure, capture s by &
    let cl_fn = || -> i32 {
        s.0
    };

    // since Fn is supertrait for FnMut and FnOnce and Fn-closure is copyable
    // (in this example) all these calls are allowed
    call_fn(cl_fn);
    call_fn_mut(cl_fn);
    call_fn_once(cl_fn);
}
