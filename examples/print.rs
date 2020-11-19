use std::env;
use std::fs::File;
use std::io::BufReader;

use openaip::parse;

fn main() {
    let args: Vec<_> = env::args().collect();
    let f = File::open(&args[1]).unwrap();
    let reader = BufReader::new(f);

    let file = parse(reader);

    for airspace in file.unwrap().airspaces.unwrap() {
        println!("{:?}", airspace);
    }
}
