//! rust_plantuml_client/src/bin/rust_plantuml_client/main.rs

// This `main.rs` is the code for the CLI application.
// The build of this project will create the CLI application.
// The `main.rs` has all the stdin and stdout.
// The `lib.rs` must be in/out agnostic. That is the responsibility of the `main.rs`
// This `lib.rs` can be used as dependency crate for other projects.

// The `main.rs` uses the `anyhow` error library.
// The `lib.rs` uses the `thiserror` library.

/// entry point into the bin-executable
fn main() {
    // logging is essential for every project
    pretty_env_logger::init();

    // super simple argument parsing. There are crates that can parse more complex arguments.
    match std::env::args().nth(1).as_deref() {
        None | Some("--help") | Some("-h") => print_help(),
        Some("svg") => match std::env::args().nth(2).as_deref() {
            // second argument
            Some(file_path) => {
                render_svg(file_path);
            }
            None => println!("Missing arguments `greet_name`."),
        },
        _ => println!("Unrecognized arguments. Try `rust_plantuml_client --help`"),
    }
}

/// print help
fn print_help() {
    println!(
        r#"
rust_plantuml_client --help
rust_plantuml_client svg file_path
"#
    );
}

/// save svg file from plantuml file
fn render_svg(file_path: &str) {
    // shadowing to change type
    let file_path = std::path::Path::new(file_path);
    if !file_path.exists() {
        // early exit because of error
        panic!("The file {} does not exist.", file_path.to_string_lossy());
    }
    let plant_uml_code = std::fs::read_to_string(file_path).unwrap();
    let plant_uml = rust_plantuml_client::get_svg(&plant_uml_code);
    let new_file_path = file_path.with_extension("html");
    std::fs::write(new_file_path, plant_uml).unwrap();
}
