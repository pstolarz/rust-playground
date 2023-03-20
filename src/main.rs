mod mod_test;
mod lifetime;
mod refs;
mod var_lifetime;

fn main() {
    mod_test::test();
    lifetime::test();
    refs::test();
    var_lifetime::test();
}
