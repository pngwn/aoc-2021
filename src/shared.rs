use std::{
  env,
  fs::File,
  io::{prelude::*, BufReader},
  path::Path,
};

pub fn read(num: u8) -> Vec<u32> {
  let cwd = env::current_dir().unwrap();
  println!("{:?}", cwd);
  let filename = cwd.join("inputs").join(format!("day_{:02}.txt", num));
  lines_from_file(filename)
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<u32> {
  let file = File::open(filename).expect("no such file");
  let buf = BufReader::new(file);
  buf
    .lines()
    .map(|l| l.expect("Could not parse line").parse::<u32>().unwrap())
    .collect()
}
