use crate::shared;
// use std::str::Lines;

pub fn run() -> (fn() -> String, fn() -> String) {
  return (parta, partb);
}

fn parta() -> String {
  let lines = shared::read(1)
    .lines()
    .map(|l| l.parse::<u32>().expect("Could not parse line"))
    .collect();
  count_increases(lines).to_string()
}

fn partb() -> String {
  let lines = shared::read(1)
    .lines()
    .map(|l| l.parse::<u32>().expect("Could not parse line"))
    .collect();
  count_window_increases(lines).to_string()
}

fn count_increases(nums: Vec<u32>) -> usize {
  nums.windows(2).filter(|x| x[0] <= x[1]).count()
}

fn count_window_increases(nums: Vec<u32>) -> usize {
  let (_, count) =
    nums
      .windows(3)
      .map(|x| x.iter().sum())
      .fold((u32::MAX, 0), |(prev, count), curr| {
        if curr > prev {
          (curr, count + 1)
        } else {
          (curr, count)
        }
      });

  count
}

#[cfg(test)]
mod tests {
  #[test]

  fn increases() {
    let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(super::count_increases(input), 7)
  }
  #[test]
  fn window_increases() {
    let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(super::count_window_increases(input), 5)
  }
}
