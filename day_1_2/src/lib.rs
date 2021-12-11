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
  let mut previous_sum = -1;
  let mut current_sum = -1;
  let mut window = (-1, -1, -1);

  for (_, line) in reader.lines().enumerate() {
    let line = line?;

    // slide the window
    window.0 = window.1;
    window.1 = window.2;
    window.2 = line.parse::<i32>().unwrap();

    if window.0 != -1 && window.1 != -1 && window.2 != -1 {
      current_sum = window.0 + window.1 + window.2;

      // check for increasing sum
      if previous_sum < current_sum && previous_sum != -1 {
        counter += 1;
      }
    }

    previous_sum = current_sum;
  }

  println!(
    "{} measurements are larger than the previous measurement",
    counter
  );

  Ok(())
}
