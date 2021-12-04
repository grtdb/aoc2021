use transpose::transpose;

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

pub fn prep_diagnostic(input: String, &input_len: &usize, &bits: &usize) -> Vec<u32> {
  let input = input.lines()
    .collect::<Vec<_>>().join("")
    .chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>();
  
  let arr_len = input.iter().count();

  let mut output_array: Vec<u32> = vec![0; arr_len];
  transpose(&input, &mut output_array, bits, input_len);

  output_array
}

fn find_common_bit(bits: &[u32], if_eq: u32) -> u32{
  let len = bits.iter().count();
  let half = len as u32 / 2;
  let sum = bits.iter().sum::<u32>();

  if sum == half {if_eq} else { if (sum) > (half) {1} else {0} }
}

pub fn day03_p1(raw_input: String, bits: usize) -> u32 {
  let input_len = raw_input.lines().count();
  
  let output_array = prep_diagnostic(raw_input, &input_len, &bits);

  let mut common: Vec<u32> = vec![];
  for b in 0..bits {
    let row = b * input_len;
    let bs = &output_array[row..row + input_len];
    assert_eq!(
      if (bs.iter().sum::<u32>()) > (input_len as u32 / 2) {1} else {0},
      find_common_bit(bs, 1)
    );
    // common.push(
    //   if (bs.iter().sum::<u32>()) > (input_len as u32 / 2) {1} else {0}
    // );
    common.push(find_common_bit(bs, 1))
  }

  let common_bin_str = common.iter().map(|i| i.to_string())
    .collect::<Vec<_>>().join("");

  let gamma = u32::from_str_radix(&common_bin_str[..], 2).unwrap();
  // println!("{:?}, {:?}", common_bin_str, gamma);

  let epsilon_str = common.iter()
    .map(|&b| if b == 1 {"0"} else {"1"}) // Flip bits and "covert" to String
    .collect::<Vec<_>>().join("");
  let epsilon = u32::from_str_radix(&epsilon_str[..], 2).unwrap();
  // println!("{:?}, {:?}", epsilon_str, epsilon);

  gamma * epsilon
}

// pub fn day03_p2(input: String, bits: usize) -> u32 {

// }

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