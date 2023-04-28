#![no_std]

#[macro_use]
extern crate alloc;

const HELLO_MESSAGE: &str = "Hello Bouquet!";
const INITIAL_X: i32 = (400 - TEXT_WIDTH) / 2;
const INITIAL_Y: i32 = (240 - TEXT_HEIGHT) / 2;
const TEXT_WIDTH: i32 = 86;
const TEXT_HEIGHT: i32 = 16;

use {
  alloc::{ boxed::Box, sync::Arc, },
  anyhow::Error,
  bouquet_ribbon::message::{ Message, MessageBus, MessageSendee, },
  client_core::{
    program_state::ProgramState, simulation::Simulation,
    message::{ ClientMessage, DebugMessage, ServerMessage, },
  },
  crankstart::{
    crankstart_game, Game, geometry::ScreenPoint,
    graphics::{ Graphics, LCDColor, LCDSolidColor, },
    Playdate,
  },
  euclid::point2,
  rhai::{ INT, },
};

struct PlaydateProgram {
  location: ScreenPoint,
  message_bus: MessageBus::
    <ProgramState, ClientMessage, ServerMessage, DebugMessage>,
  program_state: ProgramState,
}

impl PlaydateProgram {
  pub fn new(_playdate: &Playdate) -> Result<Box<Self>, Error> {
    crankstart::display::Display::get().set_refresh_rate(50.0)?;
    let mut result = PlaydateProgram {
      location: point2(INITIAL_X, INITIAL_Y),
      message_bus: MessageBus::new(),
      program_state: ProgramState::new(),
    };
/*
    let mut simulation = Box::new(Simulation::new());
    // simulation.send(Arc::new(Message::Initialize), &mut result.program_state);
    result.message_bus.register("simulation", simulation);
    result.message_bus.send(
      Message::Debug(Arc::new(DebugMessage::Log(format!("Hello, Bouquet!").into()))),
      &mut result.program_state,
    );
    result.message_bus.send(Arc::new(Message::Initialize), &mut result.program_state);
*/
    Ok(Box::new(result))
  }
}

impl Game for PlaydateProgram {
  fn update(&mut self, _playdate: &mut Playdate) -> Result<(), Error> {
    let graphics = Graphics::get();
    graphics.clear(LCDColor::Solid(LCDSolidColor::kColorWhite))?;
    let frame = self.program_state.last_frame_data.frame;
    let message = format!("{} {}", HELLO_MESSAGE, frame);
    graphics.draw_text(&message, self.location)?;

/*
    let delta_t: INT = 1;
    let ps = &mut self.program_state;
    self.message_bus.send(Arc::new(Message::Update(delta_t)), ps);
    self.message_bus.send(Arc::new(Message::Terminate), ps);
    self.program_state.next_frame();
*/
    Ok(())
  }

/*
  fn update(&mut self, _playdate: &mut Playdate) -> Result<(), Error> {
    let graphics = Graphics::get();
    graphics.clear(LCDColor::Solid(LCDSolidColor::kColorWhite))?;
    graphics.draw_text("Hello World Rust", self.location)?;

    self.location.x += 1;
    self.location.y += 2;

    if self.location.x < 0 || self.location.x > crankstart_sys::LCD_COLUMNS as i32 - TEXT_WIDTH {
      self.location.x = 0;
    }

    if self.location.y < 0 || self.location.y > crankstart_sys::LCD_ROWS as i32 - TEXT_HEIGHT {
      self.location.y = 0;
    }

    crankstart::system::System::get().draw_fps(0, 0)?;

    Ok(())
  }
*/
}

crankstart_game!(PlaydateProgram);

