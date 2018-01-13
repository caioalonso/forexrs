extern crate time;
use std::fs::File;
use std::io::prelude::*;
mod candle;
mod tick;
use candle::*;
use tick::*;

pub fn run(csv: &str) {
    let ticks = parse_csv(csv);
    build_candles(ticks);
}

fn parse_csv(filestr: &str) -> Vec<Tick> {
    let mut s = String::new();
    File::open(filestr)
        .expect("a CSV file")
        .read_to_string(&mut s)
        .expect("Couldn't read file");
    s.lines().filter_map(Tick::new).collect()
}

fn build_candles(ticks: Vec<Tick>) -> Vec<Candle> {
    let duration = time::Duration::minutes(1);
    let mut candles: Vec<Candle> = Vec::new();
    for tick in ticks {
        candles = update_candles(candles, tick, duration);
    }
    candles
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_valid_csv() {
        let ticks = parse_csv("oneday.csv");
        assert_eq!(build_candles(ticks).len(), 1440);
    }

    #[test]
    #[should_panic]
    fn doesnt_parse_nonexistant_file() {
        println!("{:?}", run("nonexistant.csv"));
    }

    #[test]
    fn updates_price() {
        let first_tick = Tick {
            time: time::now(),
            ask: 1.0,
            bid: 1.0,
            ask_volume: 10,
            bid_volume: 10
        };
        let mut c = Candle::new(first_tick);
        c.update_price(2.0);
        c.update_price(0.5);
        c.update_price(1.5);
        assert_eq!(c.o, 1.0);
        assert_eq!(c.h, 2.0);
        assert_eq!(c.l, 0.5);
        assert_eq!(c.c, 1.5);
    }
}
