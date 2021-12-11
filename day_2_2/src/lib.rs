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

  let mut x = 0;
  let mut y = 0;
  let mut aim = 0;

  for (_, line) in reader.lines().enumerate() {
    let line = line?;

    let mut iter = line.split_whitespace();
    let direction = iter.next().unwrap();
    let magnitude = iter.next().unwrap().parse::<i32>().unwrap();

    match direction {
      "forward" => {
        x = x + magnitude;
        y = y + (aim * magnitude);
      }
      "up" => aim = aim - magnitude,
      "down" => aim = aim + magnitude,
      _ => println!("I sure hope we don't hit this!"),
    }
  }

  println!("x * y: {}", x * y);

  Ok(())
}
