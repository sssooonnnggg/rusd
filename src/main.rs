use std::fs::{read_to_string, write};
use rusd::*;

fn main() {
    let start_time = std::time::Instant::now();
    let usda = read_to_string("benches/Workspace.usda").unwrap();
    let _ = UsdParser::parse(Rule::usd, &usda).unwrap();
    let end_time = std::time::Instant::now();
    println!("Elapsed time: {:?}", end_time - start_time);
}