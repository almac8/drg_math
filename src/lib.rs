mod drg_math;

use drg_math::Vector2;

pub fn start() {
  let vec_1 = Vector2::zero();

  println!("Vx: {}", vec_1.x);
  println!("Vy: {}", vec_1.y);
}