extern crate forexrs;
extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("forexrs")
        .version("0.0.1")
        .arg(
            Arg::with_name("CSV")
                .help("Sets the CSV file to use")
                .required(true)
                .index(1),
        )
        .get_matches();
    let csv = matches.value_of("CSV").unwrap();
    forexrs::run(csv);
}
