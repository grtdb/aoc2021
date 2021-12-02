// use std::io::prelude::*;
// mod day01 {

// }
pub fn aoc2021_test(pattern: &String, file: &String) -> usize {
  file.lines().filter(|l| l.contains(pattern)).count()
}

pub fn day01_p1(input: String) -> usize {
  input.lines()
    .filter_map(|l| l.parse::<usize>().ok())
    .collect::<Vec<usize>>()
    .windows(2).filter(|x| x[1] > x[0])
    .count()
  }


pub fn day01_p2(input: String) -> usize {
  input.lines()
    .filter_map(|l| l.parse::<usize>().ok())
    .collect::<Vec<usize>>()
    .windows(3).map(|d| d.iter().sum())
    .collect::<Vec<usize>>()
    .windows(2).filter(|x| x[1] > x[0])
    .count()
}


// pub fn day01_p1(input: String) -> 

// fn test(String: pattern, String: file) -> i32 {
//   reader.lines()
//     // .filter(|l| l.as_ref().unwrap().contains(&args.pattern))
//     .filter(|l| l.contains(&args.pattern))
//     .count();
// }