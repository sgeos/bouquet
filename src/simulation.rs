use alloc::{ vec::Vec, };

use crate::message::{ Message, };
use crate::message_bus::{ MessageSendee, };

pub struct Simulation {
  pub done: bool,
  frame: usize,
}

impl Simulation {
  pub fn new() -> Simulation {
    Simulation {
      done: false,
      frame: 0,
    }
  }
}

impl MessageSendee::<Message> for Simulation {
  fn send(&mut self, message: Message) -> (bool, Vec<Message>) {
    let result = Vec::new();
    match message {
      Message::Initialize => self.done = false,
      Message::Terminate => self.done = true,
      Message::Update(_) => self.frame += 1,
      //_ => (),
    }
    (self.done, result)
  }
}

