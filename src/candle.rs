extern crate time;
use tick::*;

#[derive(Debug)]
pub struct Candle {
    pub time: time::Tm,
    pub o: f32,
    pub h: f32,
    pub l: f32,
    pub c: f32,
    pub volume: u32,
}

impl Candle {
    pub fn new(tick: Tick) -> Candle {
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

    pub fn update_price(&mut self, bid: f32) {
        if self.h < bid {
            self.h = bid;
        } else if self.l > bid {
            self.l = bid;
        }
        self.c = bid;
    }
}
