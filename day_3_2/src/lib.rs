use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;

pub struct Config {
  pub filename: String,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &str> {
    if args.len() < 2 {
      return Err("not enough arguments");
    }

    let filename = args[1].clone();
    Ok(Config { filename })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let file = File::open(config.filename)?;
  let reader = BufReader::new(file);

  let mut oxygen_candidates = Vec::<String>::new();
  let mut co2_candidates = Vec::<String>::new();

  let mut vec = vec![0; 12];

  // increment place if '1' found, decrement otherwise
  for line in reader.lines() {
    let line = line?;

    oxygen_candidates.push(line.clone());
    co2_candidates.push(line.clone());

    for (j, bit) in line.chars().enumerate() {
      if bit == '1' {
        vec[j] += 1;
      } else if bit == '0' {
        vec[j] -= 1
      }
    }
  }

  let mut gamma = "".to_owned();
  let mut epsilon = "".to_owned();

  // generate binary string
  for v in vec.iter() {
    if *v >= 0 {
      gamma += "0";
      epsilon += "1";
    } else {
      gamma += "1";
      epsilon += "0";
    }
  }

  let mut oxygen_len = oxygen_candidates.len();
  let mut co2_len = co2_candidates.len();

  for (i, greatest_bit) in gamma.chars().enumerate() {
    println!("{}", oxygen_len);
    println!("{}", co2_len);

    oxygen_candidates.retain(|candidate| {
      let bits: Vec<char> = candidate.chars().collect();
      if bits[i] == greatest_bit || oxygen_len == 1 {
        true
      } else {
        oxygen_len -= 1;
        false
      }
    });

    co2_candidates.retain(|candidate| {
      let bits: Vec<char> = candidate.chars().collect();
      if bits[i] != greatest_bit || co2_len == 1 {
        true
      } else {
        co2_len -= 1;
        false
      }
    });
  }

  let oxygen_rating = isize::from_str_radix(&oxygen_candidates[0], 2).unwrap();
  let co2_rating = isize::from_str_radix(&co2_candidates[0], 2).unwrap();

  println!("ratings multiplied are: {}", oxygen_rating * co2_rating);

  Ok(())
}
