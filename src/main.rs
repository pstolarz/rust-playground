//
// Test runner crate
//
mod borrow_mut;
mod mod_test;
mod lifetime;
mod refs;
mod var_lifetime;
mod traits_prm;
mod closures;
mod closures_out;

fn main() {
    borrow_mut::test();
    mod_test::test();
    lifetime::test();
    refs::test();
    var_lifetime::test();
    traits_prm::test();
    closures::test();
    closures_out::test();
}
