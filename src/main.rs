use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
extern crate time;

fn main() {
    let path = Path::new("/home/caio/dev/forex/tick/short.csv");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    file.read_to_string(&mut s).expect("Couldn't read file");
    for line in s.split('\n') {
        let items: Vec<&str> = line.split(',').collect();
        match time::strptime(items[0], "%Y-%m-%d %H:%M:%S.%f")
            {
                Ok(v) => println!("{}", time::strftime("%Y/%m/%d %H:%M:%S.%f",
                                                       &v).unwrap()),
                Err(e) => println!("Error: {}", e),
            };
    }
}
