mod all_includes;
mod my_generics;
mod my_lifetime;
mod my_trait_one;
mod my_trait_two;
mod struct_generics;
fn main() {
    my_generics::my_gen();
    struct_generics::my_struct();
    my_trait_one::trait_one();
    my_trait_two::trait_two();
    my_lifetime::lifetime_one();
    all_includes::all_generics();
}
