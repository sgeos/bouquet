use {
  alloc::{ vec::Vec, },
  crate::{
    program_state::{ ProgramState, },
    message::{ ClientMessage, ServerMessage, DebugMessage, },
    script::{ ScriptingEngine, },
  },
  bouquet_ribbon::message::{ Message, MessageSendee, },
};

pub struct Simulation {
  scripting_engine: ScriptingEngine,
}

impl Simulation {
  pub fn new() -> Simulation {
    let script = include_str!("../rhaiscript/simulation.rhai");
    Simulation {
      scripting_engine: ScriptingEngine::new(script),
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
    let result = self.scripting_engine.send(message.clone(), ps);
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

