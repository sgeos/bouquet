use alloc::{ vec::Vec, };
use crate::program_state::{ ProgramState, };
use crate::message::{ ClientMessage, ServerMessage, DebugMessage, };
use bouquet_ribbon::message::{ Message, MessageSendee, };

pub struct Simulation { }

impl Simulation {
  pub fn new() -> Simulation {
    Simulation { }
  }
}

impl
  MessageSendee::<ProgramState, ClientMessage, ServerMessage, DebugMessage>
  for Simulation
{
  fn send(
    &mut self,
    message: Message::<ClientMessage, ServerMessage, DebugMessage>,
    ps: &mut ProgramState,
  ) -> Vec<Message::<ClientMessage, ServerMessage, DebugMessage>>
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

