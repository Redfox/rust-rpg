use crate::Rect;
use crate::components::*;

fn direction_spritesheet_row(direction: Direction) -> i32 {
  use self::Direction::*;
  match direction {
      Up => 3,
      Down => 0,
      Left => 1,
      Right => 2,
  }
}

pub fn character_animation_frames(spritesheet: usize, top_left_frame: Rect, direction: Direction) -> Vec<Sprite> {
  let (frame_width, frame_height) = top_left_frame.size();
  let y_offset = top_left_frame.y() + frame_height as i32 * direction_spritesheet_row(direction);

  let mut frames = Vec::new();
  for i in 0..3 {
    frames.push(Sprite {
      spritesheet,
      region: Rect::new(
        top_left_frame.x() + frame_width as i32 * i,
        y_offset,
        frame_width,
        frame_height
      ),
    });
  }

  frames
}
