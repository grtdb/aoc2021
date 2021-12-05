// Req for Day 04
#![feature(destructuring_assignment)]
use transpose::transpose;

pub fn aoc2021_test(pattern: &String, file: &String) -> usize {
  file.lines().filter(|l| l.contains(pattern)).count()
}

// Day 01
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

// Day 03
fn prep_diagnostic(input: String, &input_len: &usize, &bits: &usize) -> Vec<u32> {
  let input = input.lines()
    .collect::<Vec<_>>().join("")
    .chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>();
  
  let arr_len = input.iter().count();

  let mut output_array: Vec<u32> = vec![0; arr_len];
  transpose(&input, &mut output_array, bits, input_len);

  output_array
}

fn find_common_bit(bits: &[u32], if_eq: u32) -> u32 {
  let len = bits.iter().count();
  let half = len as u32 / 2;
  let sum = bits.iter().sum::<u32>();
  if len % 2 == 0 && sum == half {
    if_eq
  } else {
    if (sum) > (half) {1} else {0} 
  }
}
fn find_uncommon_bit(bits: &[u32], if_eq: u32) -> u32 {
  if find_common_bit(bits, if_eq) == 1 {0} else {1}
}

pub fn day03_p1(raw_input: String, bits: usize) -> u32 {
  let input_len = raw_input.lines().count();
  let output_array = prep_diagnostic(raw_input, &input_len, &bits);

  let mut common: Vec<u32> = vec![];
  for b in 0..bits {
    let row = b * input_len;
    let bs = &output_array[row..row + input_len];
    common.push(find_common_bit(bs, 1))
  }

  let common_bin_str = common.iter().map(|i| i.to_string()).collect::<Vec<_>>().join("");

  let gamma = u32::from_str_radix(&common_bin_str[..], 2).unwrap();

  // Flip bits and "cast" to String
  let epsilon_str = common.iter().map(|&b| if b == 1 {"0"} else {"1"}).collect::<Vec<_>>().join("");
  let epsilon = u32::from_str_radix(&epsilon_str[..], 2).unwrap();

  gamma * epsilon
}

fn day03_p2_process(mut input: Vec<&str>, bits: usize, finder: fn(&[u32], u32) -> u32) -> u32 {
  for b in 0..bits {
    let len = input.iter().count();
    if len == 1 { break; }

    let prepd = prep_diagnostic(input.join("\n"), &len, &bits);
    let row = b * len;
    let bs = &prepd[row..row + len];

    let common = finder(bs, 1);
    input = input.into_iter()
      .filter(|i| &i[b..b+1] == common.to_string())
      .collect();
  }
  u32::from_str_radix(input[0], 2).unwrap()
}

pub fn day03_p2(raw_input: String, bits: usize) -> u32 {
  let input: Vec<&str> = raw_input.lines().collect();
  let o2_gen = day03_p2_process(input.clone(), bits, find_common_bit);
  let co2_scrub = day03_p2_process(input.clone(), bits, find_uncommon_bit);
  
  o2_gen * co2_scrub
}

// Day 04
#[derive(Debug, Clone, Copy)]
enum BingoNum<T> {Unmarked(T), Marked(T)}
type BingoBoard<'a> = Vec<BingoNum<&'a str>>;
type BingoGame<'a> = (Vec<&'a str>, Vec<BingoBoard<'a>>);

fn build_bingo<'a>(input: &'a String) -> BingoGame<'a> {
  let mut input = input.lines();
  let draws: Vec<&str> = input.next().unwrap().split(",").collect();
  // Burn known blank line
  input.next();
  let mut boards: Vec<BingoBoard> = vec![];
  let mut board: BingoBoard = vec![];
  for l in input {
    if l == "" {
      // Board complete, push to collection
      boards.push(board);
      // Clear for next board
      board = vec![];
      continue;
    }
    let mut line: Vec<BingoNum<&str>> = l.split_whitespace()
      .map(|n| BingoNum::Unmarked(n))
      .collect();
    board.append(&mut line);
  }
  boards.push(board);

  (draws, boards)
}

fn mark_bingo_board<'a>(num: &str, board: BingoBoard<'a>) -> BingoBoard<'a> {
  use BingoNum::*;

  board.iter().map(|n| match n {
    Unmarked(x) => {if x == &num {Marked(*x)} else {Unmarked(*x)}},
    Marked(x) => {Marked(*x)},
  }).collect()
}

fn play_bingo_draw(game: BingoGame) -> (BingoGame, &str) {
  let (mut draws, boards) = game;
  // remove() is slow - consider storing reversed `draws` and pop()'ing if causing issue
  let draw = draws.remove(0);

  let mut marked_boards: Vec<BingoBoard> = vec![];
  for b in boards {
    marked_boards.push(mark_bingo_board(draw, b));
  }

  ((draws, marked_boards), draw)
}

fn check_bingo_line(line: &[BingoNum<&str>]) -> bool {
  use BingoNum::*;

  line.iter().all(|n| match n {
    Marked(_) => {true},
    _ => {false}
  })
}

fn check_bingo_board(board: &BingoBoard) -> bool {
  // Hardcode for now
  let size = 5;
  // Check horizontal
  for l in 0..size {
    let row_idx = l * size;
    let row = &board[row_idx..row_idx + size];
    if check_bingo_line(row) {return true};
  }
  // Check vertical
  let mut vert: BingoBoard = vec![BingoNum::Unmarked("0"); 25];
  transpose(&board, &mut vert, 5, 5);
  for l in 0..size {
    let row_idx = l * size;
    let row = &vert[row_idx..row_idx + size];
    if check_bingo_line(row) {return true};
  }
  false
}

fn score_bingo_board(board: BingoBoard, last_draw: &str) -> usize {
  use BingoNum::*;
  // Count the board
  // Refactor with filter_map() for cool points
  board.iter().filter(|n| match n {
    Unmarked(_) => {true},
    _ => {false},
    }).map(|n| if let Unmarked(v) = n {v.parse().unwrap()} else {0})
    .sum::<usize>() * last_draw.parse::<usize>().unwrap()
}

pub fn day04_p1(raw_input: String) -> usize {
  let mut game = build_bingo(&raw_input);
  
  let mut winning_board: Option<BingoBoard> = None;
  let mut last_draw: &str = "No Draw";
  while game.0.iter().count() != 0 {
    (game, last_draw) = play_bingo_draw(game);
    let bs = game.1.clone();
    for b in bs {
      if check_bingo_board(&b) { winning_board = Some(b); break; }
    }
    if let Some(_) = winning_board { break; }
  }
  if let None = winning_board {println!("No winners!"); return 0}
  println!("Winning Board!\n{:?}\nLast Drawn was {:?}", winning_board, last_draw);
  
  score_bingo_board(winning_board.unwrap(), last_draw)
}

pub fn day04_p2(raw_input: String) -> usize {
  let mut game = build_bingo(&raw_input);
  
  let mut winning_board: Option<BingoBoard> = None;
  let mut last_draw: &str = "No Draw";
  while game.0.iter().count() != 0 {
    (game, last_draw) = play_bingo_draw(game);
    let bs = game.1.clone();
    for b in bs {
      if check_bingo_board(&b) { winning_board = Some(b); break; }
    }
    if let Some(_) = winning_board { break; }
  }
  if let None = winning_board {println!("No winners!"); return 0}
  println!("Winning Board!\n{:?}\nLast Drawn was {:?}", winning_board, last_draw);
  
  score_bingo_board(winning_board.unwrap(), last_draw)
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