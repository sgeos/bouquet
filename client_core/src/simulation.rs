use alloc::{ fmt::Debug, vec::Vec, };
use crate::program_state::{ ProgramState, };
use bouquet_ribbon::message::{ Message, MessageSendee, };

pub struct Simulation { }

impl Simulation {
  pub fn new() -> Simulation {
    Simulation { }
  }
}

impl<C, S, D>
  MessageSendee::<ProgramState, C, S, D>
  for Simulation
  where C: Clone + Debug, S: Clone + Debug, D: Clone + Debug
{
  fn send(
    &mut self,
    message: Message::<C, S, D>,
    ps: &mut ProgramState,
  ) -> Vec<Message::<C, S, D>>
  {
    let result = Vec::new();
    match message {
      Message::Initialize => ps.persistent_data.done = false,
      Message::Terminate => ps.persistent_data.done = true,
      Message::Update(_) => {
        ps.next_frame_data.frame = ps.last_frame_data.frame + 1;
      },
      _ => (),
    }
    result
  }
}

