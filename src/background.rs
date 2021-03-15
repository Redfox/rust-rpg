use crate::Point;
use crate::Rect;
use crate::components::*;
use specs::prelude::*;

pub struct Backround {

}

impl Backround {
  pub fn render(world: &mut World) {
    let top_left_frame = Rect::new(83, 43, 30, 30);
    let (frame_width, frame_height) = top_left_frame.size();
    let y_offset = top_left_frame.y() + frame_height as i32;
    
    for y in 0..20 {
      for x in 0..30 {
        let sprite = Sprite {
          spritesheet: 2,
          region: Rect::new(
            top_left_frame.x() + frame_width as i32,
            y_offset,
            frame_width,
            frame_height
          ),
        };
    
        world.create_entity()
          .with(Position(Point::new(-385 + (x * 30), -285 + (y * 30))))
          .with(sprite)
          .build();
      }
    }
  }
}
