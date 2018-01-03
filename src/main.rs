use std::fs::File;
use std::io::prelude::*;
extern crate time;

#[derive(Debug)]
struct Tick {
    time: time::Tm,
    ask: f32,
    bid: f32,
    ask_volume: u32,
    bid_volume: u32,
}

fn main() {
    let mut file =
        File::open("/home/caio/dev/forex/tick/short.csv").expect("Failed to open CSV file.");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("Couldn't read file");
    let ticks: Vec<Tick> = s.lines().filter_map(parse_line).collect();
    println!("{:#?}", ticks);
}

fn parse_line(line: &str) -> Option<Tick> {
    let items: Vec<&str> = line.split(',').collect();
    let time = time::strptime(items[0], "%Y-%m-%d %H:%M:%S.%f");
    match time {
        Ok(t) => {
            let time = t;
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
