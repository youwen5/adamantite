use std::fs::File;

use adamantite::Ast;

#[test]
fn basic_parse_success() {
    let result = Ast::from_file(&mut File::open("./tests/sample.md").unwrap());

    println!("{:?}", result);
}
