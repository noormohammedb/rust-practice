mod box_pointer;
mod custom_pointer_drop;
mod interior_mutability;
mod memory_leak;
mod ref_counting;

fn main() {
    println!("Box Pointer : ");
    box_pointer::run();
    println!("\n\nSmart Pointer : ");
    custom_pointer_drop::run();
    println!("\n\nReference Counting : ");
    ref_counting::run();
    println!("\n\nInterior Mutability : ");
    interior_mutability::run();
    println!("\n\nMemory Leak : ");
    memory_leak::run();
    println!("\n\nMemory Leak (Tree) : ");
    memory_leak::tree_run();
}
