//
// Test runner crate
//
mod for_iter;
mod borrow_mut;
mod mod_test;
mod lifetime;
mod refs;
mod ref_in_struct;
mod var_lifetime;
mod traits_prm;
mod closures;
mod closures_out;

fn main() {
    for_iter::test();
    borrow_mut::test();
    mod_test::test();
    lifetime::test();
    refs::test();
    ref_in_struct::test();
    var_lifetime::test();
    traits_prm::test();
    closures::test();
    closures_out::test();
}
