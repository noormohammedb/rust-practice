mod attribute_like_macros;
mod declarative_macros;
mod function_like_macro;
mod procedural_macros;

fn main() {
    declarative_macros::run();
    procedural_macros::run();
    attribute_like_macros::run();
    function_like_macro::run();
}
