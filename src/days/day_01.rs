use crate::shared;

pub fn run() -> (fn() -> u32, fn() -> u32) {
  return (parta, partb);
}

fn parta() -> u32 {
  let lines = shared::read(1);
  count_increases(lines)
}

fn partb() -> u32 {
  2
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

#[cfg(test)]
mod tests {
  #[test]
  fn works() {
    let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(super::count_increases(input), 7)
  }
}
