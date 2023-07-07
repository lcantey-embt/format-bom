#![allow(unused)]

mod checker;
mod explorer;
mod fixer;

use clap::Parser;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Result, Write};
use std::path::Path;

fn main() {
    let args = Args::parse();
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    path: Option<String>,
    #[arg(short, long, required = false)]
    option: Option<String>,
}
