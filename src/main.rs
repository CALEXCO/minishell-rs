use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./parser_files/code.pest"] // relative to src
struct MyParser;

fn main() {
    println!("Hello, world!");
}
