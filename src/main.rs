use comment_remover::{read_file, remove_comments};

fn main() {
    let contents = read_file("test.rs");
    let contents = remove_comments(contents);

    println!("{}", contents);
}