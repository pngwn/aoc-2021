use crate::shared;

pub fn run() -> (fn() -> String, fn() -> String) {
  return (parta, partb);
}

fn parta() -> String {
  let moves = process_input(shared::read(3));
  count_bits(moves).to_string()
}

fn partb() -> String {
  "".to_string()
}

fn process_input(str: String) -> Vec<Vec<u32>> {
  const RADIX: u32 = 10;
  str
    .lines()
    .map(|x| {
      x.chars()
        .map(|c| c.to_digit(RADIX).expect("could not parse"))
        .collect::<Vec<u32>>()
    })
    .collect()
}

fn count_bits(bits: Vec<Vec<u32>>) -> isize {
  let mut gamma = "".to_owned();
  let mut epsilon = "".to_owned();

  for i in 0..bits[0].len() {
    let (zeros, ones) = bits.iter().fold((0, 0), |acc, next| {
      if next[i] == 0 {
        (acc.0 + 1, acc.1)
      } else {
        (acc.0, acc.1 + 1)
      }
    });

    if zeros > ones {
      gamma.push('0');
      epsilon.push('1');
    } else {
      gamma.push('1');
      epsilon.push('0');
    }
  }

  isize::from_str_radix(&gamma, 2).unwrap() * isize::from_str_radix(&epsilon, 2).unwrap()
}

#[cfg(test)]
mod tests {
  #[test]
  fn process_input() {
    let input =
      "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";

    assert_eq!(
      super::process_input(input.to_owned()),
      vec![
        vec![0, 0, 1, 0, 0],
        vec![1, 1, 1, 1, 0],
        vec![1, 0, 1, 1, 0],
        vec![1, 0, 1, 1, 1],
        vec![1, 0, 1, 0, 1],
        vec![0, 1, 1, 1, 1],
        vec![0, 0, 1, 1, 1],
        vec![1, 1, 1, 0, 0],
        vec![1, 0, 0, 0, 0],
        vec![1, 1, 0, 0, 1],
        vec![0, 0, 0, 1, 0],
        vec![0, 1, 0, 1, 0],
      ]
    )
  }
  #[test]
  fn count_bits() {
    let input = vec![
      vec![0, 0, 1, 0, 0],
      vec![1, 1, 1, 1, 0],
      vec![1, 0, 1, 1, 0],
      vec![1, 0, 1, 1, 1],
      vec![1, 0, 1, 0, 1],
      vec![0, 1, 1, 1, 1],
      vec![0, 0, 1, 1, 1],
      vec![1, 1, 1, 0, 0],
      vec![1, 0, 0, 0, 0],
      vec![1, 1, 0, 0, 1],
      vec![0, 0, 0, 1, 0],
      vec![0, 1, 0, 1, 0],
    ];

    assert_eq!(super::count_bits(input.to_owned()), 198)
  }
}
