use sdl2::rect::{ Point, Rect };

#[derive(Debug)]
pub struct Player {
  pub position: Point,
  pub sprite: Rect,
}
