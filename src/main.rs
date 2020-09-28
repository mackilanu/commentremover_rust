use std::env;

use comment_remover::{read_file, remove_comments};
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        eprintln!("No input file provided.");
        exit(1);
    }

    let contents = read_file(&args[1]);
    let contents = remove_comments(contents);

    println!("{}", contents);
}