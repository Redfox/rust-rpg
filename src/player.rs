use sdl2::rect::{ Point, Rect };

const PLAYER_MOVEMENT_SPEED: i32 = 20;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right,
}

#[derive(Debug)]
pub struct Player {
  pub position: Point,
  pub sprite: Rect,
  pub speed: i32,
  pub direction: Direction,
}

impl Player {
  pub fn update(&mut self) {
    use self::Direction::*;
    match &self.direction {
      Left => {
        self.position = self.position.offset(-self.speed, 0);
      },
      Right => {
        self.position = self.position.offset(self.speed, 0);
      },
      Up => {
        self.position = self.position.offset(0, -self.speed);
      },
      Down => {
        self.position = self.position.offset(0, self.speed);
      }
    }
  }

  pub fn move_to(&mut self, direction: Direction) {
    self.speed = PLAYER_MOVEMENT_SPEED;
    self.direction = direction;
  }

  pub fn stop(&mut self) {
    self.speed = 0;
  }
}
