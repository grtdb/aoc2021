// use std::env;
use structopt::StructOpt;
// use std::io::prelude::*;
// Req for BufReader method
// use std::io::BufReader;
// use std::fs::File;
use aoc2021::*;

// #[derive(StructOpt)]
#[derive(Debug, StructOpt)]
#[structopt(name = "Advent Of Code 2021", about = ":)")]
struct Cli {
    day: u8,
    #[structopt(parse(from_os_str))]
    file: std::path::PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Cli::from_args();

    // Simplier to handle, but loads whole file into memory
    // Adjust x in read_to_x() for desired results
    let reader = std::fs::read_to_string(&args.file)
        .expect("could not read file");
        
    match &args.day {
        01 => {println!( "{}\n{}", day01_p1(reader.clone()), day01_p2(reader.clone()) )},
        03 => {println!( "{}\n{}", day03_p1(reader.clone(), 12), day03_p2(reader.clone(), 12) )},
        04 => {println!( "{}\n{}", day04_p1(reader.clone()), day04_p2(reader.clone()) )},
        _ => {println!("No Solution found for Day {}", &args.day)},
    }
    Ok(())
}

