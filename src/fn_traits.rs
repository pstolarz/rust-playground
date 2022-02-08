#![feature(fn_traits)]
#![feature(unboxed_closures)]

struct Closure(i32);

impl std::ops::FnOnce<(i32, i32)> for Closure {
    type Output = i32;
    extern "rust-call" fn call_once(self, args: (i32, i32)) -> Self::Output {
        self.0 + args.0 + args.1
    }
}
impl std::ops::FnMut<(i32, i32)> for Closure {
    extern "rust-call" fn call_mut(&mut self, args: (i32, i32)) -> Self::Output {
        self.0 + args.0 + args.1
    }
}
impl std::ops::Fn<(i32, i32)> for Closure {
    extern "rust-call" fn call(&self, args: (i32, i32)) -> Self::Output {
        self.0 + args.0 + args.1
    }
}

fn call<F>(f: &F, a: i32, b: i32) -> i32
where
    F: Fn(i32, i32) -> i32
    // alternativaly
    // F: Fn<(i32, i32), Output = i32>
{
    f(a, b)
}

fn main() {
    let c = Closure(1);
    assert!(call(&c, 1, 2) == 4);
}
