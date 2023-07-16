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
    /// Path to the target directory or file.
    #[clap(required = false)]
    path: Option<String>,
    /// Add BOM mark to text files without BOM except some file format.
    #[clap(long, required = false)]
    add: bool,
    /// Remove BOM mark from files with BOM.
    #[clap(long, required = false)]
    remove: bool,
    /// Add BOM mark to text files without BOM with no exception.
    #[clap(long, required = false, value_parser, value_delimiter = ',')]
    add_strict: bool,
    /// File extensions to add BOM mark. Delenited by comma.
    #[clap(long, required = false, value_parser, value_delimiter = ',')]
    add_bom: Option<Vec<String>>,
    /// File extensions to remove BOM mark. Delenited by comma.
    #[clap(long, required = false)]
    remove_bom: Option<Vec<String>>,
}
