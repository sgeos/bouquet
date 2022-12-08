#![no_std]

#[macro_use]
extern crate alloc;

use {
  alloc::{ boxed::Box, },
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
    let simulation = Box::new(Simulation::new());
    result.message_bus.register("simulation", simulation);
    result.message_bus.send(
      Message::Debug(DebugMessage::Log(format!("Hello, Bouquet!"))),
      &mut result.program_state,
    );
    result.message_bus.send(Message::Initialize, &mut result.program_state);
    Ok(Box::new(result))
  }
}

impl Game for PlaydateProgram {
  fn update(&mut self, _playdate: &mut Playdate) -> Result<(), Error> {
    let graphics = Graphics::get();
    graphics.clear(LCDColor::Solid(LCDSolidColor::kColorWhite))?;
    graphics.draw_text("Hello Bouquet!", self.location)?;

    let delta_t: usize = 1;
    let ps = &mut self.program_state;
    self.message_bus.send(Message::Update(delta_t), ps);
    self.message_bus.send(Message::Terminate, ps);
    self.program_state.next_frame();
    Ok(())
  }
}

const INITIAL_X: i32 = (400 - TEXT_WIDTH) / 2;
const INITIAL_Y: i32 = (240 - TEXT_HEIGHT) / 2;

const TEXT_WIDTH: i32 = 86;
const TEXT_HEIGHT: i32 = 16;

crankstart_game!(PlaydateProgram);

