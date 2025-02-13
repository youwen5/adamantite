use std::fs::File;

use adamantite::Ast;

fn main() {
    let result = Ast::from_file(&mut File::open("./tests/sample.md").unwrap());

    println!("{:?}", result);
}
