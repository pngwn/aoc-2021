use crate::shared;

pub fn run() -> (fn() -> String, fn() -> String) {
  return (parta, partb);
}

fn parta() -> String {
  let moves = process_input(shared::read(3));
  get_power_consumption(moves).to_string()
}

fn partb() -> String {
  let moves = process_input(shared::read(3));
  get_life_support(moves).to_string()
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

#[derive(Debug, PartialEq)]
enum Frequency {
  Zeroes,
  Ones,
  Equal,
}

fn count_bit_frequency(bits: Vec<Vec<u32>>) -> Vec<Frequency> {
  let mut frequency = vec![];

  for i in 0..bits[0].len() {
    let (zeros, ones) = bits.iter().fold((0, 0), |acc, next| {
      if next[i] == 0 {
        (acc.0 + 1, acc.1)
      } else {
        (acc.0, acc.1 + 1)
      }
    });

    if zeros > ones {
      frequency.push(Frequency::Zeroes)
    } else {
      frequency.push(Frequency::Ones)
    }
  }

  frequency
}

fn get_power_consumption(bits: Vec<Vec<u32>>) -> isize {
  let mut gamma = "".to_owned();
  let mut epsilon = "".to_owned();

  for bit in count_bit_frequency(bits) {
    match bit {
      Frequency::Zeroes => {
        gamma.push('0');
        epsilon.push('1');
      }
      Frequency::Ones => {
        gamma.push('1');
        epsilon.push('0');
      }
      Frequency::Equal => {
        gamma.push('1');
        epsilon.push('0');
      }
    }
  }

  isize::from_str_radix(&gamma, 2).unwrap() * isize::from_str_radix(&epsilon, 2).unwrap()
}

fn get_life_support(bits: Vec<Vec<u32>>) -> isize {
  let mut co_two = bits.to_vec();
  let mut o_two = bits.to_vec();
  let mut i = 0;
  // println!("{:?} {:?}", bits_copy, bit_freq);

  while co_two.len() > 1 {
    let bit_freq = count_bit_frequency(co_two.to_vec());
    let most_frequent: u32 = match bit_freq[i] {
      Frequency::Zeroes => 0,
      Frequency::Ones => 1,
      Frequency::Equal => 9,
    };
    // println!("{:?}", bits_copy);
    co_two = co_two
      .into_iter()
      .filter(|x| x[i] == most_frequent)
      .collect();
    i += 1;
  }

  i = 0;
  while o_two.len() > 1 {
    let bit_freq = count_bit_frequency(o_two.to_vec());
    let least_frequent: u32 = match bit_freq[i] {
      Frequency::Zeroes => 1,
      Frequency::Ones => 0,
      Frequency::Equal => 9,
    };
    // println!("{:?}", bits_copy);
    o_two = o_two
      .into_iter()
      .filter(|x| {
        if x[i] == least_frequent {
          true
        } else if x[i] == 9 {
          true
        } else {
          false
        }
      })
      .collect();
    i += 1;
  }

  // println!("{:?} {:?} {:?}", bits_copy, bit_freq, i);

  isize::from_str_radix(&bits_to_str(co_two[0].to_vec()), 2).unwrap()
    * isize::from_str_radix(&bits_to_str(o_two[0].to_vec()), 2).unwrap()
}

fn bits_to_str(bits: Vec<u32>) -> String {
  bits
    .iter()
    .map(|x| x.to_string())
    .collect::<Vec<String>>()
    .join("")
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
  fn count_bit_frequency() {
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

    assert_eq!(
      super::count_bit_frequency(input.to_owned()),
      vec![
        super::Frequency::Ones,
        super::Frequency::Zeroes,
        super::Frequency::Ones,
        super::Frequency::Ones,
        super::Frequency::Zeroes
      ]
    )
  }
  #[test]
  fn get_power_consumption() {
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

    assert_eq!(super::get_power_consumption(input.to_owned()), 198)
  }
  #[test]
  fn get_life_support() {
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

    assert_eq!(super::get_life_support(input.to_owned()), 230)
  }
}
