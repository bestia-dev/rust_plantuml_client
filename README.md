[comment]: # (auto_md_to_doc_comments segment start A)

# 03. Tutorial for coding a simple CLI application in Rust (rust_plantuml_client) (2022-04)

[comment]: # (auto_cargo_toml_to_md start)

**03. Tutorial for coding a simple CLI application in Rust (rust_plantuml_client) (2022-04)**  
***version: 1.0.16 date: 2022-05-12 author: [bestia.dev](https://bestia.dev) repository: [Github](https://github.com/bestia-dev/rust_plantuml_client)***  

[comment]: # (auto_cargo_toml_to_md end)

[comment]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-82-green.svg)](https://github.com/bestia-dev/rust_plantuml_client/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-65-blue.svg)](https://github.com/bestia-dev/rust_plantuml_client/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-34-purple.svg)](https://github.com/bestia-dev/rust_plantuml_client/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-13-yellow.svg)](https://github.com/bestia-dev/rust_plantuml_client/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-30-orange.svg)](https://github.com/bestia-dev/rust_plantuml_client/)

[comment]: # (auto_lines_of_code end)

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/rust_plantuml_client/blob/main/LICENSE)
[![Rust](https://github.com/bestia-dev/rust_plantuml_client/workflows/RustAction/badge.svg)](https://github.com/bestia-dev/rust_plantuml_client/)
[![Hits](https://hits.seeyoufarm.com/api/count/incr/badge.svg?url=https%3A%2F%2Fgithub.com%2Fbestia-dev%2Frust_plantuml_client&count_bg=%2379C83D&title_bg=%23555555&icon=&icon_color=%23E7E7E7&title=hits&edge_flat=false)](https://hits.seeyoufarm.com)

Hashtags: #rustlang #tutorial #plantuml #client  
My projects on Github are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).

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

## Try it

Watch the youtube video tutorial:

<!-- markdownlint-disable MD033 -->
[<img src="https://github.com/bestia-dev/rust_plantuml_client/raw/main/images/thumbnail.png" width="400px">](https://bestia.dev/youtube/rust_plantuml_client.html)
<!-- markdownlint-enable MD033 -->

## Video subtitles

Welcome to bestia.dev !
Learning Rust and Wasm programming and having fun.
I just love  programming !!

In my first video tutorial, we set up WSL 2 (Windows subsystem for Linux) with Debian 11 on Windows 10.
In the second video, we created a Docker container with a complete Rust development environment to use with VS Code.
Today we will create a simple CLI application and crate library using cargo-auto to demonstrate how Rust development works in real life.

After reboot we need to start the container properly using this bash script.
Open VSCode, press F1, type ssh and choose "Remote SSh: Connect to Host" then choose "rust dev pod" and type your SSH passphrase.
We are now inside the container. Change directory to rustprojects. Now create a new CLI project with cargo auto.  
CD into the new directory and and use code dot to open a new VSCode window for the new project.
Type your SSH passphrase again. We are now working in VSCode in the new project folder over SSH.
Now we can use cargo auto to run the automation task to build the template project.
After the task I added instructions for the developer how to continue. Copy and run it. You wil be greeted by "Hello, my name".

Now we can write some initial thoughts in the README markdown file.
I have already read the instructions of the template readme, so I will delete what I don't need.
But I will not delete the markers. The content inside those markers will be filled later by automation tasks.
Let start with the Motivation.

We can now create a folder for sample data and prepare a simple plantuml file.
In the main.rs file we can prepare the CLI to accept the command svg and a file path. We can rename some of the existing functions and delete the rest.

All the real code will be inside the plantuml module file. We can rename the template file and all occurrences of its name.
The render_svg function will read the file content. All the input output is done in the main.rs file.

The function does not exist yet. We will write it in the plantuml module.
For the url we will concatenate the base_url, the wanted format "svg" and the compressed parameter.
The parameter contains the complete text of the plantuml code. It can be pretty long.
A new function will take the plantuml code and compress and convert to base64.
We need to add some crate dependencies to Cargo.toml. I have already read the documentation and find the way how this libraries are called.
On the plantuml.com web page is a detailed description of how to prepare the parameter. It is pretty non-standard.

There are many crate libraries for web clients. Today I choose the reqwest library for no particular reason.
I use a spell checker extension to avoid typing mistakes. Some words are special and need to be added to the dictionary.
We can now delete the template code and write a test for the compress function.
It is convenient to collapse long comments in VSCode using the region and endregion syntax.

After we corrected the "pub use" statement, none of the files in the left Side bar is red anymore. THis is a quick check, we don't have errors.
Let run the cargo auto build command and see if something is missing.

There is no errors in red color. The yellow color is for warnings. We are not using a variable. We will print it to the screen just for debugging.

The suggested commands from automation tasks are old. We will modify them in the automation tasks rs folder.

In VSCode terminal press the arrow up, to get the last command from the bash history and re-run it.
Copy the suggested command, run it. Heureka, we got the svg code in the response.

Copy and run the next suggested command: cargo auto release. It takes more time to build, but the resulting binary is more compact and performant.
Run the binary executable with the new suggested command. Heureka, this works!

The next suggested command is cargo auto doc to create the documentation from the doc comments inside the code. Very nice.
Ctrl-click on the link will open the file for editing. Right click the index.html file in the Side bar and choose Live Preview:Show Preview. This works inside the container to preview html files.

The next command is cargo auto test. Here we see errors in red. This is the old code from the template, we need to replace with some new tests.

Ctrl-click the link to open the file for editing. We can rename the old symbols and change the old code.
Rerun cargo auto test and ctrl-click the next error we need to correct.

For this integration test, we will check only the first 50 characters, just for learning exercise.
All tests are green. Good!

Check all files if we forgot something. In Cargo.toml we must type the description. This means we need to start the automation tasks so the description can be copied into README.md.

We will now use the VSCode Source control to init and commit our code locally very simply.
Copy the title and the description into the windows multi item clipboard.
Now is time to create a new Github repository and to push the code of this project. We will enter only the title and description.
Github will show us the bash commands to connect to the remote repository.

We can refresh the github page and see our code in the remote repository.
I observe that the README file contains still some old text.
I can edit it in VSCode and use the integrated version control interface to commit and push to remote.

Refresh the page to see the change. It works!

This is all for today.
Thank you for watching and see you next time.
Feel free to contact me on bestia.dev or github.

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
