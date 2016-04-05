pub struct Vector2 {
  pub x: f32,
  pub y: f32,
}

impl Vector2 {
  pub fn new() -> Vector2 {
    Vector2 { x: 0.0, y: 0.0 }
  }

  pub fn getLength(&self) -> f32 {
    let length = (self.x * self.x + self.y * self.y).sqrt();
    length
  }

  pub fn setLength(&mut self, length: f32) {
    let frac = length / self.getLength();
    self.x *= frac;
    self.y *= frac;
  }

  pub fn set(&mut self, x: f32, y: f32) {
    self.x = x;
    self.y = y;
  }
}

pub struct Ball {
  time: f64,
  pub pos: Vector2,
}

impl Ball {
  pub fn new() -> Ball {
    Ball {
      time: 0.0,
      pos: Vector2::new(),
    }
  }

  pub fn update(&mut self, dt: f64) {
    self.time += dt;
  }
}

pub struct Player {
  time: f64,
  pub pos: Vector2,
}

impl Player {
  pub fn new() -> Player {
    Player {
      time: 0.0,
      pos: Vector2::new(),
    }
  }

  pub fn update(&mut self, dt: f64) {
    self.time += dt;
  }
}
