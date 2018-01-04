extern crate clap;
extern crate time;
use std::fs::File;
use std::io::prelude::*;
use clap::{App, Arg};

#[derive(Debug)]
struct Tick {
    time: time::Tm,
    ask: f32,
    bid: f32,
    ask_volume: u32,
    bid_volume: u32,
}

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
    let ticks = parse_csv(csv);
}

fn parse_csv(filestr: &str) -> Vec<Tick> {
    let mut s = String::new();
    File::open(filestr)
        .expect("a CSV file")
        .read_to_string(&mut s)
        .expect("Couldn't read file");
    s.lines().filter_map(parse_line).collect()
}

fn parse_line(line: &str) -> Option<Tick> {
    let items: Vec<&str> = line.split(',').collect();
    match time::strptime(items[0], "%Y-%m-%d %H:%M:%S.%f") {
        Ok(time) => {
            let ask = items[1].parse::<f32>().unwrap();
            let bid = items[2].parse::<f32>().unwrap();
            let ask_volume = items[3].parse::<u32>().unwrap();
            let bid_volume = items[4].parse::<u32>().unwrap();
            Some(Tick {
                time,
                ask,
                bid,
                ask_volume,
                bid_volume,
            })
        }
        Err(_) => None,
    }
}

#[test]
fn test_something() {
    panic!("sei la");
}
