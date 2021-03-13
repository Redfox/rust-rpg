use specs::prelude::*;

use crate::components::*;

use super::MovementCommand;

const PLAYER_MOVEMENT_SPEED: i32 = 20;

pub struct Keyboard;

impl<'a> System<'a> for Keyboard {
  type SystemData = (
    ReadExpect<'a, Option<MovementCommand>>,
    ReadStorage<'a, KeyboardControlled>,
    WriteStorage<'a, Velocity>,
    WriteStorage<'a, Moving>,
  );

  fn run(&mut self, mut data: Self::SystemData) {
    let movement_command = match &*data.0 {
      Some(movement_command) => movement_command,
      None => return,
    };

    for (_, vel, moving) in (&data.1, &mut data.2, &mut data.3).join() {
      use self::Direction::*;
      match movement_command {
        MovementCommand::Move(direction) => {
          match direction {
            Up => { moving.up = true },
            Down => { moving.down = true },
            Left => { moving.left = true },
            Right => { moving.right = true },
          };

          vel.speed = PLAYER_MOVEMENT_SPEED;
          vel.direction = direction.clone();
        },
        MovementCommand::Stop(direction) => {
          match direction {
            Up => { moving.up = false },
            Down => { moving.down = false },
            Left => { moving.left = false },
            Right => { moving.right = false },
          };

          if !moving.left && !moving.right && !moving.up && !moving.down {
            vel.speed = 0;
          }
        }
      }
    }
  }
}
