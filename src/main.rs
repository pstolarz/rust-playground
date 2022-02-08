mod mod_test;
mod lifetime;
mod refs;

fn main() {
    mod_test::test();
    lifetime::test();
    refs::test();
}
