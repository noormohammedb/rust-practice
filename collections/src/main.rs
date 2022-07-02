mod my_hashmap;
mod my_string;
mod vec_enum;
mod vector_one;
mod vector_two;

use my_hashmap::hashmap_one;
use my_string::string_one;
use vec_enum::vector_enum;
use vector_one::vector1;
use vector_two::vector2;

fn main() {
    vector1();
    vector2();
    vector_enum();
    string_one();
    hashmap_one();
}
