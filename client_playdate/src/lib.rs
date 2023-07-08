#![no_std]

#[macro_use]
extern crate alloc;

mod logger;

const HELLO_MESSAGE: &str = "Hello Bouquet!";
const FONT_PATH: &str = "/System/Fonts/Asheville-Sans-14-Bold.pft";
const INITIAL_X: i32 = (400 - TEXT_WIDTH) / 2;
const INITIAL_Y: i32 = (240 - TEXT_HEIGHT) / 2;
const TEXT_WIDTH: i32 = 180;
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
    graphics::{ Font, Graphics, LCDColor, LCDSolidColor, },
    Playdate,
  },
  crate::{
    logger::Logger,
  },
  euclid::point2,
  rhai::{ INT, },
};

struct PlaydateProgram {
  font: Font,
  location: ScreenPoint,
  speed: ScreenPoint,
  message_bus: MessageBus::
    <ProgramState, ClientMessage, ServerMessage, DebugMessage>,
  program_state: ProgramState,
}

impl PlaydateProgram {
  pub fn new(_playdate: &Playdate) -> Result<Box<Self>, Error> {
    let graphics = Graphics::get();
    crankstart::display::Display::get().set_refresh_rate(50.0)?;
    let mut result = PlaydateProgram {
      font: graphics.load_font(FONT_PATH)?,
      location: point2(INITIAL_X, INITIAL_Y),
      speed: point2(1, 1),
      message_bus: MessageBus::new(),
      program_state: ProgramState::new(),
    };
    let logger = Box::new(Logger::new());
    let simulation = Box::new(Simulation::new());
    //crankstart::system::System::log_to_console(HELLO_MESSAGE);
    //logger.send(Arc::new(Message::Initialize), &mut result.program_state);
    //simulation.send(Arc::new(Message::Initialize), &mut result.program_state);
    result.message_bus.register("logger", logger);
    result.message_bus.register("simulation", simulation);
    result.message_bus.send(
      Arc::new(Message::Debug(DebugMessage::Log(HELLO_MESSAGE.into()))),
      &mut result.program_state,
    );
    result.message_bus.send(Arc::new(Message::Initialize), &mut result.program_state);
    Ok(Box::new(result))
  }

  fn update_render_position(&mut self, text_width: i32) {
    let x = self.location.x;
    let x_speed = self.speed.x;
    let max_x = crankstart_sys::LCD_COLUMNS as i32 - text_width;
    if (0 < x_speed && max_x < x) || (x_speed < 0 && x < 0) {
      self.speed.x *= -1;
    }
    self.location.x += self.speed.x;

    let y = self.location.y;
    let y_speed = self.speed.y;
    let max_y = crankstart_sys::LCD_ROWS as i32 - TEXT_HEIGHT;
    if (0 < y_speed && max_y < y) || (y_speed < 0 && y < 0) {
      self.speed.y *= -1;
    }
    self.location.y += self.speed.y;
  }

  fn render(&mut self) -> Result<(), Error> {
    let graphics = Graphics::get();
    graphics.clear(LCDColor::Solid(LCDSolidColor::kColorWhite))?;
    let frame = self.program_state.last_frame_data.frame;
    let message = format!("{} {}", HELLO_MESSAGE, frame);
    let text_width = graphics.get_text_width(&self.font, &message, 0)?;
    self.update_render_position(text_width);
    graphics.draw_text(&message, self.location)?;
    Ok(())
  }
}

impl Game for PlaydateProgram {
  fn update(&mut self, _playdate: &mut Playdate) -> Result<(), Error> {
    let delta_t: INT = 1;
    let ps = &mut self.program_state;
    self.message_bus.send(Arc::new(Message::Update(delta_t)), ps);
    self.program_state.next_frame();


    self.render()
  }
}

crankstart_game!(PlaydateProgram);

