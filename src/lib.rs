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

// Solution to Day 02 - refactor into 
// let input: Vec<&str> = reader.lines().collect();
//   let mut inst: Vec<(&str, i32)> = vec!();
//   for i in input {
//       let mut i = i.splitn(2, " ");
//       inst.push((i.next().unwrap(), i.next().unwrap().parse().unwrap()))
//   }
  
//   // Part 1
//   let mut pos1: (i32, i32) = (0,0);
//   for i in inst.clone() {
//       match i {
//           ("forward", x) => pos1 = (pos1.0 + x, pos1.1),
//           ("down", x) => pos1 = (pos1.0, pos1.1 + x),
//           ("up", x) => pos1 = (pos1.0, pos1.1 - x),
//           _ => pos1 = pos1
//       }
//   }

//   println!("{:?}", pos1);
//   let answer1 = pos1.0 * pos1.1;
//   println!("{:?}", answer1);
  
//   // Part 2
//   let mut pos: (i32, i32, i32) = (0, 0, 0);
//   for i in inst.clone() {
//       match i {
//           ("down", x) => pos = (pos.0, pos.1, pos.2 + x),
//           ("up", x) => pos = (pos.0, pos.1, pos.2 - x),
//           ("forward", x) => pos = (pos.0 + x, pos.1 + (pos.2 * x), pos.2),
//           _ => pos = pos
//       }
//   }
//   println!("{:?}", pos);
//   let answer = pos.0 * pos.1;
//   println!("{:?}", answer);

// pub fn day01_p1(input: String) -> 

// fn test(String: pattern, String: file) -> i32 {
//   reader.lines()
//     // .filter(|l| l.as_ref().unwrap().contains(&args.pattern))
//     .filter(|l| l.contains(&args.pattern))
//     .count();
// }