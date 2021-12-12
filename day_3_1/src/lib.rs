use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

  let mut vec = vec![0; 12];

  // increment place if '1' found, decrement otherwise
  for line in reader.lines() {
    let line = line?;

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

  // convert binary string to binary value
  let gamma_as_int = isize::from_str_radix(&gamma, 2).unwrap();
  let epsilon_as_int = isize::from_str_radix(&epsilon, 2).unwrap();

  println!("power consumption: {}", gamma_as_int * epsilon_as_int);

  Ok(())
}
