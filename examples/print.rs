use std::env;
use std::fs::read_to_string;
use std::io::BufReader;

use openaip::parse;

fn main() {
    let args: Vec<_> = env::args().collect();
    let content = read_to_string(&args[1]).unwrap();
    let file = parse(&content).unwrap();

    for airspace in file.airspaces.unwrap() {
        println!("{:?}", airspace);
    }
}
