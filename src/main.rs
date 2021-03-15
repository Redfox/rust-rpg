mod components;
mod player;
mod reaper;
mod background;
mod frame;
mod event_handler;


use sdl2::rect::{Point, Rect};
use sdl2::image::{self, LoadTexture, InitFlag};

use specs::prelude::*;

use std::time::Duration;

use player::Player;
use reaper::Reaper;
use background::Backround;
use components::*;
use components::keyboard::{
  MovementCommand
};

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
      .with(ai::AI, "AI", &[])
      .with(physics::Physics, "Physics", &[])
      .with(animator::Animator, "Animator", &[])
      .build();

  let mut world = WorldExt::new();
  dispatcher.setup(&mut world);
  renderer::SystemData::setup(&mut world);

  let move_command: Option<MovementCommand> = None;
  world.insert(move_command);

  Backround::render(&mut world);
  
  Player::new().initialize(&mut world);
  Reaper::new().initialize(&mut world, Point::new(-150, -150));
  Reaper::new().initialize(&mut world, Point::new(150, -190));
  Reaper::new().initialize(&mut world, Point::new(-150, -170));

  let mut event_pump = sdl_context.event_pump()?;
  'running: loop {
    let mut movement_command = None;
    
    for event in event_pump.poll_iter() {
      let event_return = event_handler::keyboard_handler(event);
      match event_return {
        Some(value) => {
          movement_command = value.movement_command;
          if value.stop_loop { break 'running };
        }
        None => {}
      }
    }
    
    *world.write_resource() = movement_command;

    dispatcher.dispatch(&mut world);
    world.maintain();

    let textures = [
      texture_creator.load_texture("assets/bardo.png")?,
      texture_creator.load_texture("assets/reaper.png")?,
      texture_creator.load_texture("assets/background.png")?,
    ];
    
    renderer::render(&mut canvas, &textures, world.system_data())?;
    
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
  }

  Ok(())
}
