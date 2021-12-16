use std::{env, fs, path::Path};

pub fn read(num: u8) -> String {
  let cwd = env::current_dir().unwrap();
  let filename = cwd.join("inputs").join(format!("day_{:02}.txt", num));
  lines_from_file(filename)
}

fn lines_from_file(filename: impl AsRef<Path>) -> String {
  let file = fs::read_to_string(filename).expect("no such file");
  file.to_owned()
}
