use std::{collections::HashMap, fs};

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "srt.pest"]
pub struct SRTParser;

pub struct TimeCode {
    milliseconds: i32,
}

impl TimeCode {
    fn build_from_str(timecode: &str) -> TimeCode {
        let mut mils: i32 = 0;
        if timecode.contains(",") {
            // from "00:03:15,167"
            let x: Vec<&str> = timecode.split(",").collect();
            mils = x.last().unwrap().parse::<i32>().unwrap();

            let t: Vec<&str> = x.first().unwrap().split(":").collect();

            match t.len() {
                2 => {
                    let seconds = t[2].parse::<i32>().unwrap();
                    mils = (seconds * 1000) + mils;

                    let minutes = t[1].parse::<i32>().unwrap();
                    mils = (minutes * 1000 * 60) + mils;

                    let hours = t[0].parse::<i32>().unwrap();
                    mils = (hours * 1000 * 60 * 60) + mils;
                }
                1 => {}
                0 => {}
                _ => {}
            }
        }
        TimeCode { milliseconds: mils }
    }
}

pub struct SRTClip {
    pub identifier: Option<i32>,
    pub start: TimeCode,
    pub stop: TimeCode,
    pub content: String,
}

fn main() {
    let unparsed_file = fs::read_to_string("sample.srt").expect("cannot read file");

    print!("{}", unparsed_file);

    let file = SRTParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next()
        .unwrap(); // get and unwrap the `file` rule; never fails

    // println!("{:?}", file);

    let mut clips: Vec<SRTClip> = vec![];

    for line in file.into_inner() {
        match line.as_rule() {
            Rule::clip => {
                // New Clip
                let mut identifier: Option<i32> = None;
                let mut start: TimeCode;
                let mut stop: TimeCode;
                let mut content: String;

                let mut header_and_content = line.into_inner(); // { header | content }

                for r in header_and_content {
                    match r.as_rule() {
                        Rule::header => {
                            let mut header = r.into_inner();

                            let identifier_str: &str = header.next().unwrap().as_str(); // identifier
                            let i = identifier_str.parse::<i32>();
                            if let Ok(foo) = i {
                                identifier = Some(foo);
                            }

                            let mut startstop = header.next().unwrap().into_inner(); //

                            let start_str: &str = startstop.next().unwrap().as_str(); // start
                            let stop_str: &str = startstop.next().unwrap().as_str();

                            start = TimeCode::build_from_str(start_str);
                            stop = TimeCode::build_from_str(stop_str);
                            // stop

                            // println!("id: {}, start: {}, stop: {}", identifier, start, stop);
                        }
                        Rule::content => {
                            content = r.as_span().as_str().to_string();
                            println!("{}", content);
                        }
                        Rule::EOI => (),
                        _ => unreachable!(),
                    }
                }

                let c = SRTClip {
                    identifier,
                    start,
                    stop,
                    content,
                };

                clips.push(c);
            }
            // Rule::property => {
            //     let mut inner_rules = line.into_inner(); // { name ~ "=" ~ value }

            //     let name: &str = inner_rules.next().unwrap().as_str();
            //     let value: &str = inner_rules.next().unwrap().as_str();

            //     // Insert an empty inner hash map if the outer hash map hasn't
            //     // seen this section name before.
            //     let section = properties.entry(current_section_name).or_default();
            //     section.insert(name, value);
            // }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    // let unsuccessful_parse = CSVParser::parse(Rule::field, "this is not a number");
    // println!("{:?}", unsuccessful_parse);
}
