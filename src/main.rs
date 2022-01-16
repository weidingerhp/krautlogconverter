use std::{fs::File, io::{stdin, BufRead, BufReader, Write, LineWriter, stdout, Read, BufWriter}};

use clap::{App, Arg};
use logdata::LogLine;

mod logdata;


#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

fn main() {
    let cmd_line_matches = App::new("Kraut-Log-Analyzer")
    .version(env!("CARGO_PKG_VERSION"))
    .author("hans-peter.weidinger <hpw@hp-weidinger.at>")
    .about("Transcode log-data to human readable format")
    .arg(
        Arg::new("INPUT")
            .short('i')
            .long("input")
            .required(false)
            .takes_value(true)
            .default_value("")
            .help("Input file"),
    )
    .arg(
        Arg::new("OUTPUT")
            .short('o')
            .long("output")
            .required(false)
            .takes_value(true)
            .default_value("")
            .help("Output Directory. If none is specified the current directory will be used."),
    )
    // .arg(
    //     Arg::new("FORMAT")
    //         .short('f')
    //         .long("format")
    //         .required(false)
    //         .takes_value(true)
    //         .default_value("json")
    //         .help("Input format. Possible values are \"json\", \"csv\" and \"yaml\""),
    // )
    .get_matches();

let input =  cmd_line_matches.value_of("INPUT").unwrap_or("");
let mut input_reader: Box<dyn BufRead> = match input {
    "" => Box::new(BufReader::new(stdin())),
    _ => Box::new(BufReader::new(File::open(input).unwrap())),
};

let output = cmd_line_matches.value_of("OUTPUT").unwrap_or("");
let mut output_writer: Box<dyn Write> = match output {
    "" => Box::new(BufWriter::new(stdout())),
    _ => Box::new(BufWriter::new(File::create(output).unwrap())),
};

let deserializer = serde_json::Deserializer::from_reader(&mut input_reader);
for item_res in deserializer.into_iter::<LogLine>() {
    let item = item_res.unwrap();
    let outline = format!("{:>28} [{:>3}] [{:05x}] {}", item.time.to_string(), item.level, item.pid, item.msg.unwrap());
    output_writer.write(outline.as_bytes()).unwrap();
    output_writer.write(LINE_ENDING.as_bytes()).unwrap();
    output_writer.flush().unwrap();
}
}
