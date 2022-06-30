use unsafe_rust;

mod external_call;
mod mut_static_and_trait;
fn main() {
    unsafe_rust::run();
    external_call::run();
    mut_static_and_trait::run();
}
