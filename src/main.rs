use crate::player::{ Player, Direction };
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::{ Point, Rect };
use sdl2::render::{ WindowCanvas, Texture };
use sdl2::image::{ self, LoadTexture, InitFlag };

mod player;

fn render(
  canvas: &mut WindowCanvas,
  color: Color,
  texture: &Texture,
  player: &Player,
) -> Result<(), String> {
  canvas.set_draw_color(color);
  canvas.clear();

  let (width, height) = canvas.output_size()?;

  let screen_position = player.position + Point::new(width as i32 / 2, height as i32 / 2);
  let screen_rect = Rect::from_center(screen_position, player.sprite.width(), player.sprite.height());
  
  canvas.copy(texture, player.sprite, screen_rect)?;

  canvas.present();

  Ok(())
}

fn main() -> Result<(), String> {

  let sdl_context = sdl2::init()?;
  let video_subsystem = sdl_context.video()?;

  let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

  let window = video_subsystem
      .window("Game", 800, 600)
      .position_centered()
      .build()
      .expect("could not initialize video subsystem");

  let mut canvas = window
      .into_canvas()
      .build()
      .expect("could not make a canvas");

  let texture_creator = canvas.texture_creator();
  let texture = texture_creator.load_texture("assets/bardo.png")?;

  let mut player = Player {
    position: Point::new(0, 0),
    sprite: Rect::new(0, 0, 26, 36),
    speed: 0,
    direction: Direction::Right,
  };

  let mut event_pump = sdl_context.event_pump()?;
  let mut i = 0;
  'running: loop {
    
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit {..} |
        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
          println!("quiting...");
          break 'running;
        },
        Event::KeyDown { keycode: Some(Keycode::A), repeat: false, .. } => {
          player.move_to(Direction::Left);
        },
        Event::KeyDown { keycode: Some(Keycode::D), repeat: false, .. } => {
          player.move_to(Direction::Right);
        },
        Event::KeyDown { keycode: Some(Keycode::W), repeat: false, .. } => {
          player.move_to(Direction::Up);
        },
        Event::KeyDown { keycode: Some(Keycode::S), repeat: false, .. } => {
          player.move_to(Direction::Down);
        },
        Event::KeyUp { keycode: Some(Keycode::A), repeat: false, .. } |
        Event::KeyUp { keycode: Some(Keycode::D), repeat: false, .. } |
        Event::KeyUp { keycode: Some(Keycode::W), repeat: false, .. } |
        Event::KeyUp { keycode: Some(Keycode::S), repeat: false, .. }  => {
          player.stop();
        },
        _ => {}
      }
    }

    i = (i + 1) % 255;
    player.update();

    render(&mut canvas, Color::RGB(i, 64, 255 - i), &texture, &player)?;
    
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
  }

  Ok(())
}
