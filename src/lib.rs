// rust_plantuml_client/src/lib.rs

// You can collapse the long region below using VSCode. It is only the copy of the README.md file, because it gets compiled into docs.

// region: auto_md_to_doc_comments include README.md A //!
//! # rust_plantuml_client
//!
//! **Rust client library and CLI for plantuml server**  
//! ***version: 1.0.16 date: 2022-05-12 author: [bestia.dev](https://bestia.dev) repository: [Github](https://github.com/bestia-dev/rust_plantuml_client)***  
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-82-green.svg)](https://github.com/bestia-dev/rust_plantuml_client/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-65-blue.svg)](https://github.com/bestia-dev/rust_plantuml_client/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-34-purple.svg)](https://github.com/bestia-dev/rust_plantuml_client/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-13-yellow.svg)](https://github.com/bestia-dev/rust_plantuml_client/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-30-orange.svg)](https://github.com/bestia-dev/rust_plantuml_client/)
//!
//! [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/rust_plantuml_client/blob/main/LICENSE)
//! [![Rust](https://github.com/bestia-dev/rust_plantuml_client/workflows/RustAction/badge.svg)](https://github.com/bestia-dev/rust_plantuml_client/)
//! ![Hits](https://bestia.dev/webpage_hit_counter/get_svg_image/722419866.svg)
//!
//! Hashtags: #rustlang #tutorial #plantuml #client
//!
//! ## Motivation
//!
//! It is very ofter needed to insert simple graphs inside the README markdown file of Rust projects.
//! There is a lot of different ways to do that.
//! I prefer to use <http://plantuml.com> server to render the graph from a simple text file.
//! Then I save the resulting SVG file in the `image` directory and add it to README.md as an image.
//! This is easy to do manually, but for me every problem needs a software solution.
//! So I want to make an automation task for this.
//! But first I need a simple Rust library or CLI app to call the <plantuml.com> server and save the image.
//! This web service API is pretty non-standard because the plantuml code needs to be compressed and base64 in an unusual way.
//! This `Rust plantuml client` will solve this. It can be used as a Rust library or as a standalone CLI app.
//!
//! ## Try it
//!
//! Watch the youtube video tutorial:
//!
//! <!-- markdownlint-disable MD033 -->
//! [<img src="https://github.com/bestia-dev/rust_plantuml_client/raw/main/images/thumbnail.png" width="400px">](https://bestia.dev/youtube/rust_plantuml_client.html)
//! <!-- markdownlint-enable MD033 -->
//!
//! ## cargo crev reviews and advisory
//!
//! We live in times of danger with [supply chain attacks](https://en.wikipedia.org/wiki/Supply_chain_attack).
//!
//! It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev) to verify the trustworthiness of each of your dependencies.
//!
//! Please, spread this info.
//!
//! You can also read crev reviews quickly on the web:
//!
//! <https://web.crev.dev/rust-reviews/crates/>
//!
//! ## open-source and free as a beer
//!
//! My open-source projects are free as a beer (MIT license).
//!
//! I just love programming.
//!
//! But I need also to drink. If you find my projects and tutorials helpful,please buy me a beer donating on my [paypal](https://paypal.me/LucianoBestia).
//!
//! You know the price of a beer in your local bar ;-) So I can drink a free beer for your health :-)
//!
//! [Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻
//!
// endregion: auto_md_to_doc_comments include README.md A //!

// The `main.rs` has all the stdin and stdout.
// The `lib.rs` must be in/out agnostic. That is the responsibility of the `main.rs`

// The `lib.rs` does not have any real code. All the code is in modules in separate files.
// The `lib.rs` has just the list of modules, it publishes module's functions or class for the caller
// and it has some global stuff like the Error enum.

// access to modules
mod plantuml_mod;

// `pub use` allows the caller of the lib to access modules functions, structs or all(*)
pub use plantuml_mod::*;

// The `main.rs` uses the `anyhow` error library.
// The `lib.rs` uses the `thiserror` library.
use thiserror::Error;

/// all possible library errors for `thiserror`
#[derive(Error, Debug)]
pub enum LibraryError {
    #[error("name `{0}` is already uppercase")]
    Uppercase(String),
    #[error("unknown error")]
    Unknown,
}
