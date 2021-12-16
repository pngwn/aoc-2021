use crate::shared;

pub fn run() -> (fn() -> String, fn() -> String) {
  return (parta, partb);
}

#[derive(Debug, PartialEq)]
enum Move {
  Forward(i32),
  Backward(i32),
  Up(i32),
  Down(i32),
  Skip,
}

fn parta() -> String {
  let moves: Vec<Move> = create_moves(shared::read(2));
  let (x, y) = follow_moves(moves);
  (x * y).to_string()
}

fn partb() -> String {
  let moves: Vec<Move> = create_moves(shared::read(2));
  let (x, y, _) = follow_moves_with_aim(moves);
  (x * y).to_string()
}

fn create_moves(s: String) -> Vec<Move> {
  s.lines()
    .map(|l| {
      let s: Vec<&str> = l.split(' ').collect();
      let n = s[1].parse::<i32>().unwrap();
      match s[0] {
        "forward" => Move::Forward(n),
        "backward" => Move::Backward(n),
        "up" => Move::Up(n),
        "down" => Move::Down(n),
        _ => Move::Skip,
      }
    })
    .collect()
}

fn follow_moves(moves: Vec<Move>) -> (i32, i32) {
  moves.iter().fold((0, 0), |(x, y), m| match m {
    Move::Forward(n) => (x + n, y),
    Move::Backward(n) => (x - n, y),
    Move::Up(n) => (x, y - n),
    Move::Down(n) => (x, y + n),
    Move::Skip => (x, y),
  })
}

fn follow_moves_with_aim(moves: Vec<Move>) -> (i32, i32, i32) {
  moves.iter().fold((0, 0, 0), |(x, y, a), m| match m {
    Move::Forward(n) => (x + n, y + (a * n), a),
    Move::Backward(n) => (x - n, y - (a * n), a),
    Move::Up(n) => (x, y, a - n),
    Move::Down(n) => (x, y, a + n),
    Move::Skip => (x, y, a),
  })
}

#[cfg(test)]
mod tests {
  #[test]
  fn create_moves() {
    let s = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2".to_owned();
    let output = vec![
      super::Move::Forward(5),
      super::Move::Down(5),
      super::Move::Forward(8),
      super::Move::Up(3),
      super::Move::Down(8),
      super::Move::Forward(2),
    ];
    assert_eq!(super::create_moves(s), output)
  }
  #[test]
  fn follow_moves() {
    let input = vec![
      super::Move::Forward(5),
      super::Move::Down(5),
      super::Move::Forward(8),
      super::Move::Up(3),
      super::Move::Down(8),
      super::Move::Forward(2),
    ];
    assert_eq!(super::follow_moves(input), (15, 10))
  }

  #[test]
  fn follow_moves_with_aim() {
    let input = vec![
      super::Move::Forward(5),
      super::Move::Down(5),
      super::Move::Forward(8),
      super::Move::Up(3),
      super::Move::Down(8),
      super::Move::Forward(2),
    ];
    assert_eq!(super::follow_moves_with_aim(input), (15, 60, 10))
  }
}
