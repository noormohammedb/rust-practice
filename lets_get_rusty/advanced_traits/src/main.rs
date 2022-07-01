mod associated_types;
mod call_methods_same_name;
mod default_gen_type_param;
mod super_trait_and_new_type;

fn main() {
    associated_types::run();
    default_gen_type_param::run();
    call_methods_same_name::run();
    super_trait_and_new_type::run();
}
