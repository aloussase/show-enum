mod options;
mod parser;

use std::env::args;
use std::fs;

use options::Options;
use parser::CEnum;

fn main() {
    let mut args = args();
    args.next().unwrap();

    let options = Options::parse(&mut args);

    let contents = fs::read_to_string(options.filename.as_str()).expect(&format!(
        "failed to read the contents of '{}'",
        options.filename.as_str()
    ));

    let start = options.start.map(|start| start as usize).unwrap_or(1);
    let end = options
        .end
        .map(|end| end as usize)
        .unwrap_or(contents.chars().filter(|c| *c == '\n').count());

    let lines: String = contents
        .lines()
        .skip(start - 1)
        .take(end - start + 1)
        .collect();

    let enum_ = CEnum::parse(&lines);

    println!("{}", enum_);
}
