use {
  alloc::{ vec::Vec, },
  crate::{
    program_state::ProgramState,
    message::{ ClientMessage, ServerMessage, DebugMessage, },
  },
  bouquet_ribbon::message::{ Message, MessageSendee, },
  rhai::{ Engine, INT, },
};

pub struct Simulation {
  engine: Engine,
}

impl Simulation {
  pub fn new() -> Simulation {
    Simulation {
      engine: Engine::new_raw(),
    }
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
        let increment =
          self.engine.eval_expression::<INT>("8 + 2").unwrap() as usize;
        ps.next_frame_data.frame = ps.last_frame_data.frame + increment;
      },
      _ => (),
    }
    result
  }
}

