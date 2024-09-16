//
// All the following closures are returned as Fn, FnMut, FnOnce implementers.
// The objects are not copyable since contain non-copyable object `s'.
//
#[derive(Debug)]
struct S(i32);

fn create_fn() -> impl Fn() {
    let s = S(1);
    move || println!("Fn has: {}", s.0)
}

fn create_fnmut() -> impl FnMut() {
    let s = S(2);
    move || println!("FnMut has: {}", s.0)
}

fn create_fnonce() -> impl FnOnce() {
    let s = S(3);
    move || println!("FnOnce has: {}", s.0)
}

// normally would consume the closure
fn test_fn_once<F>(_f: &F) where F: FnOnce() {
    print!("test_fn_once(): ");
    // ERROR: closure is not copyable
    //_f();
    println!();
}
fn test_fn_mut<F>(_f: &mut F) where F: FnMut() {
    print!("test_fn_mut(): ");
    _f();
}
fn test_fn<F>(_f: F) where F: Fn() {
    print!("test_fn(): ");
    _f();
}

//
// The following versions of test routines won't work. They pass closure
// object by value and then uses reference of various types to it.
//
//fn test_fn_once(ref _f: impl FnOnce()) {}
//fn test_fn_mut(ref mut _f: impl FnMut()) {}
//fn test_fn(ref _f: impl Fn()) {}

pub fn test()
{
    let fn_ = create_fn();
    let mut fn_m = create_fn();

    let mut fn_mut = create_fnmut();

    let fn_once = create_fnonce();
    let mut _fn_once_mut = create_fnonce();

    fn_();
    fn_();
    test_fn(&fn_);
    test_fn_mut(&mut fn_m);
    test_fn_once(&fn_);

    fn_mut();
    fn_mut();
    // ERROR: Fn is not subtrait of FnMut
    //test_fn(&fn_mut);
    test_fn_mut(&mut fn_mut);
    test_fn_once(&fn_mut);

    fn_once();
    // ERROR: Fn, FnMut are not subtraits of FnOnce
    //test_fn(&fn_once);
    //test_fn_mut(&mut _fn_once_mut);
    // ERROR: closure already consumed
    //test_fn_once(&fn_once);
    //fn_once();
}
