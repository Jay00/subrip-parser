use std::fs;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "srt.pest"]
pub struct CSVParser;

fn main() {
    let unparsed_file = fs::read_to_string("sample.srt").expect("cannot read file");

    print!("{}", unparsed_file);

    let file = CSVParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next()
        .unwrap(); // get and unwrap the `file` rule; never fails

    println!("{:?}", file);

    // let unsuccessful_parse = CSVParser::parse(Rule::field, "this is not a number");
    // println!("{:?}", unsuccessful_parse);
}
