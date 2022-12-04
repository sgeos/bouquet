use alloc::{ fmt::Debug, vec::Vec, };
use bouquet_ribbon::{ Message, MessageSendee, };

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

impl<C, S, D> MessageSendee::<C, S, D> for Simulation
  where C: Clone + Debug, S: Clone + Debug, D: Clone + Debug
{
  fn send(&mut self, message: Message::<C, S, D>) -> Vec<Message::<C, S, D>>
  {
    let result = Vec::new();
    match message {
      Message::Initialize => self.done = false,
      Message::Terminate => self.done = true,
      Message::Update(_) => self.frame += 1,
      _ => (),
    }
    result
  }
}

