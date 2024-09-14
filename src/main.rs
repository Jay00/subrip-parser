use std::fs;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "srt.pest"]
pub struct SRTParser;

#[derive(Debug)]
pub struct TimeCode {
    milliseconds: i32,
}

impl TimeCode {
    fn to_string(self) -> String {
        let mils_per_hour = 1000 * 60 * 60;
        let mils_per_minute = 1000 * 60;
        let mils_per_second = 1000;

        println!("Total MILS: {}", self.milliseconds);

        let mut remainder = self.milliseconds;
        let h = remainder / mils_per_hour;

        remainder = remainder - (h * mils_per_hour);

        let m = remainder / mils_per_minute;

        remainder = remainder - (m * mils_per_minute);

        let s = remainder / mils_per_second;

        let mils = remainder - (s * mils_per_second);

        let s = format!("{:02}:{:02}:{:02},{:02}", h, m, s, mils);
        // println!("Output: {}", s);

        return s;
    }

    fn build_from_str(timecode: &str) -> TimeCode {
        let mut mils: i32 = 0;
        if timecode.contains(",") {
            // from "00:03:15,167"
            let x: Vec<&str> = timecode.split(",").collect();
            mils = x.last().unwrap().parse::<i32>().unwrap();

            let t: Vec<&str> = x.first().unwrap().split(":").collect();

            let seconds = t[2].parse::<i32>().unwrap();
            // println!("Seconds: {}", seconds);
            mils = (seconds * 1000) + mils;

            let minutes = t[1].parse::<i32>().unwrap();
            // println!("Minutes: {}", minutes);
            mils = (minutes * 1000 * 60) + mils;

            let hours = t[0].parse::<i32>().unwrap();
            mils = (hours * 1000 * 60 * 60) + mils;
        }
        TimeCode { milliseconds: mils }
    }
}

#[derive(Debug)]
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
                let mut start_str: &str = "";
                let mut stop_str: &str = "";
                let mut content: &str = "";

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

                            start_str = startstop.next().unwrap().as_str(); // start
                            stop_str = startstop.next().unwrap().as_str();

                            // stop

                            // println!("id: {}, start: {}, stop: {}", identifier, start, stop);
                        }
                        Rule::content => {
                            content = r.as_span().as_str();
                            println!("{}", content);
                        }
                        Rule::EOI => (),
                        _ => unreachable!(),
                    }
                }

                let start = TimeCode::build_from_str(&start_str);
                let stop = TimeCode::build_from_str(stop_str);

                // println!("{} = {}", &start_str, start_string);

                let c = SRTClip {
                    identifier,
                    start,
                    stop,
                    content: content.to_string(),
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
