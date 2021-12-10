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

  let mut counter = 0;
  let mut previous = 0;

  for (index, line) in reader.lines().enumerate() {
    let line = line?;

    counter = if line.parse::<i32>().unwrap() > previous && index != 0 {
      counter + 1
    } else {
      counter
    };

    previous = line.parse::<i32>().unwrap();
  }

  println!(
    "{} measurements are larger than the previous measurement",
    counter
  );

  Ok(())
}
