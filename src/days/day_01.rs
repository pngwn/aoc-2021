use crate::shared;
// use std::str::Lines;

pub fn run() -> (fn() -> u32, fn() -> u32) {
  return (parta, partb);
}

fn parta() -> u32 {
  let lines = shared::read(1)
    .lines()
    .map(|l| l.parse::<u32>().expect("Could not parse line"))
    .collect();
  count_increases(lines)
}

fn partb() -> u32 {
  let lines = shared::read(1)
    .lines()
    .map(|l| l.parse::<u32>().expect("Could not parse line"))
    .collect();
  count_window_increases(lines)
}

fn count_increases(nums: Vec<u32>) -> u32 {
  let mut count: i32 = -1;
  let mut prev = 0;

  for num in nums {
    if num > prev {
      count += 1
    }
    prev = num;
  }

  return count as u32;
}

fn count_window_increases(nums: Vec<u32>) -> u32 {
  let mut count: i32 = 0;
  let mut prev = 0;
  let mut i = 3;

  for _ in 3..nums.len() {
    let sum: u32 = [nums[i - 2], nums[i - 1], nums[i]].iter().sum();

    if sum > prev {
      count += 1
    }

    prev = sum;
    i += 1;
  }

  return count as u32;
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
