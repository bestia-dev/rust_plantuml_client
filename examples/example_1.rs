// examples/example_1.rs

//! A simple example how to use the `lib.rs`
//! You can run it with `cargo run --example example_1`

/// example
fn main() {
    let plantuml_code = "@startuml
    Bob -> Alice : hello
    @enduml";
    let svg_code = rust_plantuml_client::get_svg(plantuml_code);
    println!("{}", svg_code);
}
