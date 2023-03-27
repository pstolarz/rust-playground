//
// All closures returned by the 3 functions below are Fn-closures, which are
// casted to a specific functional trait (Fn, FnMut, FnOnce).
//
// Fn-closure object in this example in not copyable since contains non-copyable
// String object.
//
fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();

    move || println!("This is a: {}", text)
}

fn test_fn_once<F>(_f: &F) where F: FnOnce() {}
fn test_fn_mut<F>(_f: &mut F) where F: FnMut() {}
fn test_fn<F>(_f: F) where F: Fn() {}

//
// The following versions of test routines won't work. They pass closure
// object by value and then uses reference of various types to it.
//
//fn test_fn_once(ref _f: impl FnOnce()) {}
//fn test_fn_mut(ref mut _f: impl FnMut()) {}
//fn test_fn(ref _f: impl Fn()) {}

pub fn test()
{
    let fn_plain = create_fn();
    let mut fn_plain_mut = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();
    let mut _fn_once_mut = create_fnonce();

    test_fn(&fn_plain);
    test_fn_mut(&mut fn_plain_mut); // need mutable reference here
    test_fn_once(&fn_plain);
    fn_plain();
    fn_plain();

    // ERROR: Fn is supertrait of FnMut
    //test_fn(&fn_mut);
    test_fn_mut(&mut fn_mut);
    test_fn_once(&fn_mut);
    fn_mut();
    fn_mut();

    // ERROR: Fn, FnMut are supertraits of FnOnce
    //test_fn(&fn_once);
    //test_fn_mut(&mut _fn_once_mut);
    //test_fn_once(&fn_once);
    fn_once();
    // ERROR: called on moved object
    //fn_once();
}
