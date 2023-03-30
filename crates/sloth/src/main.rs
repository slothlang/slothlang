#![feature(test, let_chains)]
#![warn(
    clippy::wildcard_imports,
    clippy::string_add,
    clippy::string_add_assign,
    clippy::manual_ok_or,
    unused_lifetimes
)]

pub mod lexer;

use std::{env, fs};

use itertools::Itertools;

fn main() {
    let args = env::args().collect_vec();

    if args.len() < 2 {
        println!("Sloth programming language interpreter\n");
        println!("Usage: sloth <file>");
        return;
    }

    let source_path = &args[1];
    let Ok(_source) = fs::read_to_string(source_path) else {
        println!("Error while reading '{source_path}'");
        return;
    };

    // TODO:
}