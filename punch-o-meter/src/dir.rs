pub enum Direction{
  Right, 
  Left,
  Forward,
  Backward,
  Up, 
  Down
}

const RIGHT:[[u8; 5]; 5] = [
  [0, 0, 1, 0, 0],
  [0, 0, 0, 1, 0],
  [1, 1, 1, 1, 1],
  [0, 0, 0, 1, 0],
  [0, 0, 1, 0, 0]
];

const LEFT: [[u8; 5]; 5]=[
  [0, 0, 1, 0, 0],
  [0, 1, 0, 0, 0],
  [1, 1, 1, 1, 1],
  [0, 1, 0, 0, 0],
  [0, 0, 1, 0, 0]
];

const FORWARD: [[u8; 5]; 5] =[ 
  [0, 0, 1, 0, 0],
  [0, 1, 1, 1, 0],
  [1, 0, 1, 0, 1],
  [0, 0, 1, 0, 0],
  [0, 0, 1, 0, 0]
];

const BACKWARD: [[u8; 5]; 5] =[
  [0, 0, 1, 0, 0],
  [0, 0, 1, 0, 0],
  [1, 0, 1, 0, 1],
  [0, 1, 1, 1, 0],
  [0, 0, 1, 0, 0]
];

const UP:  [[u8; 5]; 5] =[
  [1, 1, 0, 1, 1],
  [1, 1, 0, 1, 1],
  [0, 0, 1, 0, 0],
  [1, 1, 0, 1, 1],
  [1, 1, 0, 1, 1]
];

const DOWN:  [[u8; 5]; 5] =[
  [0, 0, 1, 0, 0],
  [0, 0, 1, 0, 0],
  [1, 1, 1, 1, 1],
  [0, 0, 1, 0, 0],
  [0, 0, 1, 0, 0]
];

pub fn dir(dir: Direction) -> [[u8; 5]; 5]{
  match dir {
      Direction::Right => RIGHT,
      Direction::Left => LEFT,
      Direction::Forward => FORWARD,
      Direction::Backward => BACKWARD,
      Direction::Up => UP,
      Direction::Down => DOWN
  }
}