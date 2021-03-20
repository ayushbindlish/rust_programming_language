mod variables;
mod numeric_operations;
mod floating_point;
mod bool;
mod char;
mod compound_types;

fn main() {
    println!("Hello, world!");
    variables::mn();
    numeric_operations::numeric_operation();
    floating_point::floating_point();
    bool::booln();
    char::chr();
    compound_types::compound_types();
}
