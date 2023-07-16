mod arg_parser;
mod explorer;
mod formatter;

use clap::Parser;
use std::path::PathBuf;

fn main() {
    let args = Args::parse();
    let fix_rule = arg_parser::parse_args(&args).unwrap();
    let files =
        explorer::get_file_list(&PathBuf::from(args.path.unwrap_or_else(|| ".".to_string())));
    _ = formatter::format_bom(&files, &fix_rule);
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[clap(required = false)]
    path: Option<String>,
    #[clap(long, required = false)]
    add: bool,
    #[clap(long, required = false)]
    remove: bool,
    #[clap(long, required = false, value_parser, value_delimiter = ',')]
    add_strict: bool,
    #[clap(long, required = false, value_parser, value_delimiter = ',')]
    add_bom: Option<Vec<String>>,
    #[clap(long, required = false)]
    remove_bom: Option<Vec<String>>,
}
