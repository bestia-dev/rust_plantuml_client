[comment]: # (auto_md_to_doc_comments segment start A)

# rust_plantuml_client

[comment]: # (auto_cargo_toml_to_md start)

**Rust client library and CLI for plantuml server**  
***version: 1.0.14 date: 2022-04-24 author: [bestia.dev](bestia.dev) repository: [Github](https://github.com/bestia-dev/rust_plantuml_client)***  

[comment]: # (auto_cargo_toml_to_md end)

[comment]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-81-green.svg)]()
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-65-blue.svg)]()
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-34-purple.svg)]()
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-13-yellow.svg)]()
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-30-orange.svg)]()

[comment]: # (auto_lines_of_code end)

[![Licence](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/rust_plantuml_client/blob/main/LICENSE) [![Rust](https://github.com/bestia-dev/rust_plantuml_client/workflows/RustAction/badge.svg)](https://github.com/bestia-dev/rust_plantuml_client/)

## Edit this README.md file

Edit this README file with your data. But leave the markers: auto_md_to_doc_comments, auto_lines_of_code, auto_cargo_toml_to_md and similar, because the automation tasks need them.  
Modify the title and description only in Cargo.toml. Automation tasks will copy that into README.md.  
Lines of code are filled automatically from automation tasks.  
Find `bestia.dev` everywhere and change it with your username.

## Motivation

It is very ofter needed to insert simple graphs inside the README markdown file of Rust projects.
There is a lot of different ways to do that.
I prefer to use <http://plantuml.com> server to render the graph from a simple text file.
Then I save the resulting SVG file in the `image` directory and add it to README.md as an image.
This is easy to do manually, but for me every problem needs a software solution.
So I want to make an automation task for this.
But first I need a simple Rust library or CLI app to call the <plantuml.com> server and save the image.
This web service API is pretty non-standard because the plantuml code needs to be compressed and base64 in an unusual way.
This `Rust plantuml client` will solve this. It can be used as a Rust library or as a standalone CLI app.

## cargo crev reviews and advisory

We live in times of danger with [supply chain attacks](https://en.wikipedia.org/wiki/Supply_chain_attack).

It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev) to verify the trustworthiness of each of your dependencies.

Please, spread this info.

You can also read crev reviews quickly on the web:

<https://web.crev.dev/rust-reviews/crates/>

## open-source and free as a beer

My open-source projects are free as a beer (MIT license).

I just love programming.

But I need also to drink. If you find my projects and tutorials helpful,please buy me a beer donating on my [paypal](https://paypal.me/LucianoBestia).

You know the price of a beer in your local bar ;-) So I can drink a free beer for your health :-)

[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[comment]: # (auto_md_to_doc_comments segment end A)
