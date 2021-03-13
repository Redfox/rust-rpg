mod components;
mod physics;
mod animator;
mod keyboard;
mod renderer;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use sdl2::image::{self, LoadTexture, InitFlag};

use specs::prelude::*;

use std::time::Duration;

use crate::components::*;

pub enum MovementCommand {
  Stop,
  Move(Direction),
}

fn direction_spritesheet_row(direction: Direction) -> i32 {
  use self::Direction::*;
  match direction {
      Up => 3,
      Down => 0,
      Left => 1,
      Right => 2,
  }
}

fn character_animation_frames(spritesheet: usize, top_left_frame: Rect, direction: Direction) -> Vec<Sprite> {
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

  let mut dispatcher = DispatcherBuilder::new()
      .with(keyboard::Keyboard, "Keyboard", &[])
      .with(physics::Physics, "Physics", &[])
      .with(animator::Animator, "Animator", &[])
      .build();

  let mut world = WorldExt::new();
  dispatcher.setup(&mut world);
  renderer::SystemData::setup(&mut world);

  let move_command: Option<MovementCommand> = None;
  world.insert(move_command);

  let textures = [
    texture_creator.load_texture("assets/bardo.png")?,
  ];

  let player_spritesheet = 0;
  let player_top_left_frame = Rect::new(0, 0, 26, 36);

  let player_animation = MovementAnimation {
    current_frame: 0,
    up_frames: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Up),
    down_frames: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Down),
    left_frames: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Left),
    right_frames: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Right),
  };
  
  world.create_entity()
    .with(KeyboardControlled)
    .with(Position(Point::new(0, 0)))
    .with(Velocity { speed: 0, direction: Direction::Right })
    .with(player_animation.right_frames[0].clone())
    .with(player_animation)
    .build();

  let mut event_pump = sdl_context.event_pump()?;
  let mut i = 0;
  'running: loop {
    let mut movement_command = None;
    
    for event in event_pump.poll_iter() {
      match event {
          Event::Quit {..} |
          Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
              break 'running;
          },
          Event::KeyDown { keycode: Some(Keycode::A), repeat: false, .. } => {
              movement_command = Some(MovementCommand::Move(Direction::Left));
          },
          Event::KeyDown { keycode: Some(Keycode::D), repeat: false, .. } => {
              movement_command = Some(MovementCommand::Move(Direction::Right));
          },
          Event::KeyDown { keycode: Some(Keycode::W), repeat: false, .. } => {
              movement_command = Some(MovementCommand::Move(Direction::Up));
          },
          Event::KeyDown { keycode: Some(Keycode::S), repeat: false, .. } => {
              movement_command = Some(MovementCommand::Move(Direction::Down));
          },
          Event::KeyUp { keycode: Some(Keycode::A), repeat: false, .. } |
          Event::KeyUp { keycode: Some(Keycode::D), repeat: false, .. } |
          Event::KeyUp { keycode: Some(Keycode::W), repeat: false, .. } |
          Event::KeyUp { keycode: Some(Keycode::S), repeat: false, .. } => {
              movement_command = Some(MovementCommand::Stop);
          },
          _ => {}
      }
    }

    *world.write_resource() = movement_command;

    i = (i + 1) % 255;
    dispatcher.dispatch(&mut world);
    world.maintain();

    renderer::render(&mut canvas, Color::RGB(i, 64, 255 - i), &textures, world.system_data())?;
    
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
  }

  Ok(())
}
