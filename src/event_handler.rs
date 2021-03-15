use sdl2::event::Event;
use std::option::Option;
use sdl2::keyboard::Keycode;
use crate::components::{
  Direction,
  keyboard::MovementCommand
};

pub struct EventReturn {
  pub movement_command: Option<MovementCommand>,
  pub stop_loop: bool,
}

pub fn keyboard_handler(event: Event) -> Option<EventReturn> {
  match event {
    Event::Quit {..} |
    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
      println!("Quit");
      Some(EventReturn {
        movement_command: None,
        stop_loop: true
      })
    },
    Event::KeyDown { keycode: Some(Keycode::A), repeat: false, .. } => {
      Some(EventReturn {
        movement_command: Some(MovementCommand::Move(Direction::Left)),
        stop_loop: false
      })
    },
    Event::KeyDown { keycode: Some(Keycode::D), repeat: false, .. } => {
      Some(EventReturn {
        movement_command: Some(MovementCommand::Move(Direction::Right)),
        stop_loop: false
      })
    },
    Event::KeyDown { keycode: Some(Keycode::W), repeat: false, .. } => {
      Some(EventReturn {
        movement_command: Some(MovementCommand::Move(Direction::Up)),
        stop_loop: false
      })
    },
    Event::KeyDown { keycode: Some(Keycode::S), repeat: false, .. } => {
      Some(EventReturn {
        movement_command: Some(MovementCommand::Move(Direction::Down)),
        stop_loop: false
      })
    },
    Event::KeyUp { keycode: Some(Keycode::A), repeat: false, .. } => {
      Some(EventReturn {
        movement_command: Some(MovementCommand::Stop(Direction::Left)),
        stop_loop: false
      })
    },
    Event::KeyUp { keycode: Some(Keycode::D), repeat: false, .. } => {
      Some(EventReturn {
        movement_command: Some(MovementCommand::Stop(Direction::Right)),
        stop_loop: false
      })
    },
    Event::KeyUp { keycode: Some(Keycode::W), repeat: false, .. } => {
      Some(EventReturn {
        movement_command: Some(MovementCommand::Stop(Direction::Up)),
        stop_loop: false
      })
    }
    Event::KeyUp { keycode: Some(Keycode::S), repeat: false, .. } => {
      Some(EventReturn {
        movement_command: Some(MovementCommand::Stop(Direction::Down)),
        stop_loop: false
      })
    },
    _ => { None }
  }
}
