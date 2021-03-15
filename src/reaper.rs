use crate::Point;
use crate::Rect;
use crate::frame::{ character_animation_frames };
use crate::components::*;
use specs::prelude::*;

pub struct Reaper {

}

impl Reaper {
  pub fn new() -> Reaper {
    println!("new reaper");

    Reaper {
      
    }
  }

  pub fn initialize(&self, world: &mut World, position: Point) {
    let top_left_frame = Rect::new(0, 0, 32, 36);

    let animation = MovementAnimation {
      current_frame: 0,
      up_frames: character_animation_frames(1, top_left_frame, Direction::Up),
      down_frames: character_animation_frames(1, top_left_frame, Direction::Down),
      left_frames: character_animation_frames(1, top_left_frame, Direction::Left),
      right_frames: character_animation_frames(1, top_left_frame, Direction::Right),
    };
  
    world.create_entity()
      .with(Enemy)
      .with(Position(position))
      .with(Velocity { speed: 0, direction: Direction::Right })
      .with(animation.right_frames[0].clone())
      .with(animation)
      .build();
  }
}
