#[derive(Debug, PartialEq)]
pub struct Vector2 {
  pub x: f32,
  pub y: f32
}

impl Vector2 {
  pub fn new(x:f32, y: f32) -> Self {
    Self {
      x,
      y
    }
  }

  pub fn zero() -> Self {
    Self::new(0.0, 0.0)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new_vector() {
    let x = 2.0;
    let y = 4.0;
    let v0 = Vector2::new(x, y);

    assert_eq!(v0.x, x);
    assert_eq!(v0.y, y);
  }

  #[test]
  fn zero_vector() {
    let v0 = Vector2::zero();

    assert_eq!(v0.x, 0.0);
    assert_eq!(v0.y, 0.0);
  }
}