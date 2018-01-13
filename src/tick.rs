extern crate time;

#[derive(Debug)]
pub struct Tick {
    pub time: time::Tm,
    pub ask: f32,
    pub bid: f32,
    pub ask_volume: u32,
    pub bid_volume: u32,
}

impl Tick {
    pub fn new(line: &str) -> Option<Tick> {
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
