mod dynamically_sized_types;
mod never_type;
mod new_type;
mod typed_aliases;

fn main() {
    new_type::run();
    typed_aliases::run();
    never_type::run();
    dynamically_sized_types::run();
}
