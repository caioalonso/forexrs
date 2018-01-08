extern crate time;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Tick {
    time: time::Tm,
    ask: f32,
    bid: f32,
    ask_volume: u32,
    bid_volume: u32,
}

impl Tick {
    fn new(line: &str) -> Option<Tick> {
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
}

#[derive(Debug)]
struct Candle {
    time: time::Tm,
    o: f32,
    h: f32,
    l: f32,
    c: f32,
    volume: u32,
}

impl Candle {
    fn new(tick: Tick) -> Candle {
        let mut time = tick.time.clone();
        time.tm_sec = 0;
        time.tm_nsec = 0;
        Candle {
            time,
            o: tick.bid,
            h: tick.bid,
            l: tick.bid,
            c: tick.bid,
            volume: tick.bid_volume,
        }
    }

    fn update_price(&mut self, bid: f32) {
        if self.h < bid {
            self.h = bid;
        } else if self.l > bid {
            self.l = bid;
        }
        self.c = bid;
    }
}

pub fn run(csv: &str) {
    let ticks = parse_csv(csv);
    let duration = time::Duration::minutes(1);
    let mut candles: Vec<Candle> = Vec::new();
    for tick in ticks {
        candles = update_candles(candles, tick, duration);
    }
}

fn parse_csv(filestr: &str) -> Vec<Tick> {
    let mut s = String::new();
    File::open(filestr)
        .expect("a CSV file")
        .read_to_string(&mut s)
        .expect("Couldn't read file");
    s.lines().filter_map(Tick::new).collect()
}

fn update_candles(mut candles: Vec<Candle>, tick: Tick, duration: time::Duration) -> Vec<Candle> {
    match candles.pop() {
        None => {
            candles.push(Candle::new(tick))
        },
        Some(mut candle) => {
            // if tick is inside candle
            if candle.time + duration > tick.time {
                candle.update_price(tick.bid);
                candles.push(candle);
            } else {
                candles.push(candle);
                candles.push(Candle::new(tick));
            }
        }
    }
    candles
}

#[test]
fn test_something() {
    panic!("sei la");
}
