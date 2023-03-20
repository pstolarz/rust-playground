#[derive(Debug)]
struct S();

impl Drop for S {
    fn drop(&mut self) {
        println!("Dropping {:?}...", self);
    }
}

fn test_shadow() {
    let _s = S();
    println!("test_shadow: var created");

    let _s = S();               // s scope finishes here
    println!("test_shadow: shadowed-var created");
}                               // s, s-shadowed lifetimes finishes here

fn test_move() {
    let m1 = S();
    println!("test_move: var created");

    let _m2 = m1;               // m1 lifetime finishes here
    println!("test_move: vars moved move");
}                               // m2 lifetime finishes here

pub fn test() {
    test_shadow();
    println!("---");
    test_move();
}
