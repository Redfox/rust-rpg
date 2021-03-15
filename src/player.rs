use crate::Point;
use crate::Rect;
use crate::frame::{ character_animation_frames };
use crate::KeyboardControlled;
use crate::components::*;
use specs::prelude::*;

pub struct Player {
  
}

impl Player {
  pub fn new() -> Player {
    println!("new player");

    Player {
    }
  }

  pub fn initialize(&self, world: &mut World) {
    let top_left_frame = Rect::new(0, 0, 26, 36);
    
    let animation = MovementAnimation {
      current_frame: 0,
      up_frames: character_animation_frames(0, top_left_frame, Direction::Up),
      down_frames: character_animation_frames(0, top_left_frame, Direction::Down),
      left_frames: character_animation_frames(0, top_left_frame, Direction::Left),
      right_frames: character_animation_frames(0, top_left_frame, Direction::Right),
    };

    world.create_entity()
      .with(KeyboardControlled)
      .with(Position(Point::new(0, 0)))
      .with(Moving { left: false, right: false, up: false, down: false })
      .with(Velocity { speed: 0, direction: Direction::Right })
      .with(animation.right_frames[0].clone())
      .with(animation)
      .build();
  }
}
